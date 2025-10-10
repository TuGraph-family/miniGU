use std::os::unix::io::AsRawFd;

use io_uring::{IoUring, opcode, types};

use crate::common::{ANNError, ANNResult};
use crate::model::IOContext;

pub const MAX_IO_CONCURRENCY: usize = 128;

// Re-defining AlignedRead and DISK_IO_ALIGNMENT as they are not public in the windows version
pub const DISK_IO_ALIGNMENT: usize = 512;

pub struct AlignedRead<'a, T> {
    offset: u64,
    aligned_buf: &'a mut [T],
}

impl<'a, T> AlignedRead<'a, T> {
    pub fn new(offset: u64, aligned_buf: &'a mut [T]) -> ANNResult<Self> {
        Self::assert_is_aligned(offset as usize)?;
        Self::assert_is_aligned(std::mem::size_of_val(aligned_buf))?;

        Ok(Self {
            offset,
            aligned_buf,
        })
    }

    fn assert_is_aligned(val: usize) -> ANNResult<()> {
        match val % DISK_IO_ALIGNMENT {
            0 => Ok(()),
            _ => Err(ANNError::log_disk_io_alignment_error(format!(
                "The offset or length of AlignedRead request is not {DISK_IO_ALIGNMENT} bytes aligned"
            ))),
        }
    }

    pub fn aligned_buf(&self) -> &[T] {
        self.aligned_buf
    }
}

pub struct LinuxAlignedFileReader {
    ring: IoUring,
}

impl LinuxAlignedFileReader {
    pub fn new() -> ANNResult<Self> {
        let ring = IoUring::new(MAX_IO_CONCURRENCY as u32).map_err(ANNError::log_io_error)?;
        Ok(Self { ring })
    }

    pub fn read<T>(
        &mut self,
        read_requests: &mut [AlignedRead<T>],
        ctx: &IOContext,
    ) -> ANNResult<()> {
        let n_requests = read_requests.len();
        let fd = ctx.file_handle.file().as_raw_fd();

        let mut submitted_count = 0;
        while submitted_count < n_requests {
            let batch_size = std::cmp::min(n_requests - submitted_count, MAX_IO_CONCURRENCY);
            {
                let mut sq = self.ring.submission();

                for i in 0..batch_size {
                    let req = &mut read_requests[submitted_count + i];
                    let read_e = opcode::Read::new(
                        types::Fd(fd),
                        req.aligned_buf.as_mut_ptr() as *mut _,
                        req.aligned_buf.len() as u32 * std::mem::size_of::<T>() as u32,
                    )
                    .offset(req.offset)
                    .build()
                    .user_data((submitted_count + i) as u64);

                    unsafe {
                        sq.push(&read_e).map_err(|_| {
                            ANNError::log_io_queue_error(
                                "Failed to push to submission queue".to_string(),
                            )
                        })?;
                    }
                }
                sq.sync();
            }

            self.ring
                .submit_and_wait(batch_size)
                .map_err(ANNError::log_io_error)?;
            submitted_count += batch_size;

            let mut cq = self.ring.completion();
            for _ in 0..batch_size {
                let cqe = cq.next().ok_or_else(|| {
                    ANNError::log_io_queue_error("Completion queue is empty".to_string())
                })?;
                if cqe.result() < 0 {
                    return Err(ANNError::log_io_queue_error(format!(
                        "Async read failed with error: {}",
                        cqe.result()
                    )));
                }
            }
            cq.sync();
        }

        Ok(())
    }
}
