pub struct FastCompressor {
    base: u64,
    counts: Vec<u64>,
    threads: usize,
}

/// Build bucket bounds for base and max value; used when encoding a block to get bucket index per frac.
pub fn build_bounds(base: u64, max_value: u64) -> Vec<u64> {
    let mut bounds = Vec::new();
    let mut x = 1;
    while x < max_value {
        bounds.push(x);
        x *= base;
    }
    bounds.push(x);
    bounds
}

/// Returns the bucket index for `value` given precomputed `bounds`.
#[inline]
pub fn get_bucket_index(bounds: &[u64], value: u64) -> usize {
    if value == 0 {
        return 0;
    }
    bounds.partition_point(|&b| value > b)
}

impl FastCompressor {
    pub fn new(base: u64, threads: usize) -> Self {
        assert!(base > 1, "底数必须大于 1.0");
        assert!(threads >= 1, "线程数必须大于1");
        Self {
            base,
            counts: Vec::new(),
            threads,
        }
    }

    pub fn compress(&mut self, data: &Vec<u64>) {
        if data.is_empty() {
            return;
        }
        let thread_num = self.threads.min(data.len());
        let base = self.base;
        let max = data.iter().copied().max().unwrap_or(0);

        let bucket_len = if max == 0 {
            1
        } else {
            let v = max as f64;
            let base_f = base as f64;
            let len =  (v.ln() / base_f.ln()).ceil() as usize + 1;
            len.max(1)
        };
        let bounds = build_bounds(self.base, max);
        let bounds_ref = &bounds;
        let chunk_size = (data.len() + thread_num - 1) / thread_num;
        let partials: Vec<Vec<u64>> = std::thread::scope(|s| {
            let mut handles = Vec::new();
            for chunk in data.chunks(chunk_size) {
                handles.push(s.spawn(move || {
                    let mut local = vec![0u64; bucket_len];
                    for &v in chunk {
                        let idx = get_bucket_index(&bounds_ref, v);
                        local[idx] += 1;
                    }
                    local
                }));
            }
            handles.into_iter().map(|h| h.join().unwrap()).collect()
        });
        if self.counts.len() < bucket_len {
            self.counts.resize(bucket_len, 0);
        }
        for local in partials {
            for i in 0..bucket_len {
                self.counts[i] += local[i];
            }
        }
    }

    pub fn get_result(&self) -> (usize, u64, Vec<u64>) {
        (self.counts.len(), self.base, self.counts.clone())
    }

    pub fn len(&self) -> usize {
        self.counts.len()
    }

    pub fn base(&self) -> u64 {
        self.base
    }

    pub fn counts(&self) -> &[u64] {
        &self.counts
    }

    pub fn reset(&mut self) {
        self.counts.clear();
    }
}
