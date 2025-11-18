//! BlockStatistic: bucket_ids.len() == prefix.len(); res_vec.len() = max(bucket_ids) + 1.
//! Each bucket_id indexes into res_vec for that bucket's max remainder (余向量最大值).

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::procedures::gcard_query::catalog::CompressedDegreeSeq;
use crate::procedures::gcard_query::degreepiecewise::fast_compressor::{build_bounds, get_bucket_index};
use crate::procedures::gcard_query::error::{GCardError, GCardResult};

const BUCKET_BASE: u64 = 2;

/// 前 8 bit 有效值 + 余量；bucket 作为 shift，余量 = value & ((1<<shift)-1)，shift 上限 56。
#[inline]
fn prefix_and_remainder(value: u64, bucket_id: u8) -> (u8, u64) {
    if value == 0 {
        return (0, 0);
    }
    let shift = (bucket_id as usize).min(56);
    let prefix = (value >> shift) as u8;
    let remainder = value & ((1u64 << shift).saturating_sub(1));
    (prefix, remainder)
}

#[derive(Debug, Clone, Default)]
pub struct BlockStatistic {
    /// 每个条目的桶号，与 prefix 等长
    pub bucket_ids: Vec<u8>,
    /// 每个条目的前 8 bit 有效值
    pub prefix: Vec<u8>,
    /// 按 bucket_id 索引的余向量最大值，长度 = max(bucket_ids) + 1
    pub res_vec: Vec<u64>,
}

impl BlockStatistic {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_u64_sequence(values: &[u64]) -> GCardResult<Self> {
        if values.is_empty() {
            return Ok(BlockStatistic::default());
        }
        let max_val = values.iter().copied().max().unwrap_or(0);
        let bounds = build_bounds(BUCKET_BASE, max_val);
        let mut bucket_ids: Vec<u8> = Vec::with_capacity(values.len());
        let mut prefix: Vec<u8> = Vec::with_capacity(values.len());
        let mut res_max: Vec<u64> = Vec::new(); // res_max[b] = max remainder for bucket b
        for &v in values {
            let b = get_bucket_index(&bounds, v);
            let b_u8 = b.min(u8::MAX as usize) as u8;
            let (p, r) = prefix_and_remainder(v, b_u8);
            bucket_ids.push(b_u8);
            prefix.push(p);
            let idx = b.min(u8::MAX as usize);
            if res_max.len() <= idx {
                res_max.resize(idx + 1, 0);
            }
            res_max[idx] = res_max[idx].max(r);
        }
        let max_b = bucket_ids.iter().copied().max().unwrap_or(0) as usize;
        res_max.truncate(max_b + 1);
        Ok(BlockStatistic {
            bucket_ids,
            prefix,
            res_vec: res_max,
        })
    }

    pub fn get_by_rank(&self, rank: usize) -> Option<(u8, u8, u64)> {
        let b = *self.bucket_ids.get(rank)?;
        let p = *self.prefix.get(rank)?;
        let r = self.res_vec.get(b as usize).copied().unwrap_or(0);
        Some((b, p, r))
    }

    pub fn bucket_ids(&self) -> &[u8] {
        &self.bucket_ids
    }

    pub fn recover_upper_bound_at_rank(&self, rank: usize) -> Option<u64> {
        let (b, p, r) = self.get_by_rank(rank)?;
        let shift = (b as usize).min(56);
        Some(((p as u64) << shift) | r)
    }

    pub fn update_at_rank(&mut self, rank: usize, new_value: u64) {
        if rank >= self.bucket_ids.len() {
            return;
        }
        let bounds = build_bounds(BUCKET_BASE, new_value.max(1));
        let new_b = get_bucket_index(&bounds, new_value).min(u8::MAX as usize);
        let new_b_u8 = new_b as u8;
        let (new_p, new_r) = prefix_and_remainder(new_value, new_b_u8);

        self.bucket_ids[rank] = new_b_u8;
        self.prefix[rank] = new_p;

        if self.res_vec.len() <= new_b {
            self.res_vec.resize(new_b + 1, 0);
        }
        self.res_vec[new_b] = self.res_vec[new_b].max(new_r);
    }

    pub fn remove_at_rank(&mut self, rank: usize) {
        if rank < self.bucket_ids.len() {
            self.bucket_ids.remove(rank);
            self.prefix.remove(rank);
        }
    }

    pub fn upper_limit_ratio(&self) -> f64 {
        if self.bucket_ids.is_empty() {
            return 0.0;
        }
        let at_limit = self
            .bucket_ids
            .iter()
            .filter(|&&b| {
                let rem_bits = (b as usize).saturating_sub(8).min(56);
                let max_r = (1u64 << rem_bits).saturating_sub(1);
                self.res_vec.get(b as usize).copied().unwrap_or(0) >= max_r
            })
            .count();
        at_limit as f64 / self.bucket_ids.len() as f64
    }

    pub fn get_compressed_degree_seq(&self) -> GCardResult<Option<CompressedDegreeSeq>> {
        if self.bucket_ids.is_empty() {
            return Ok(None);
        }
        let n_buckets = self.res_vec.len();
        let mut counts = vec![0u64; n_buckets];
        for &b in &self.bucket_ids {
            if (b as usize) < n_buckets {
                counts[b as usize] += 1;
            }
        }
        Ok(Some(CompressedDegreeSeq::FastCompressor {
            len: n_buckets,
            base: BUCKET_BASE,
            counts,
        }))
    }
    
    pub fn disk_size(&self) -> usize {
        self.serialize().len()
    }

    pub fn serialize(&self) -> Vec<u8> {
        let n = self.bucket_ids.len();
        let rlen = self.res_vec.len();
        let mut out = Vec::with_capacity(n + n + 8 + rlen * 8);
        out.extend_from_slice(&self.bucket_ids);
        out.extend_from_slice(&self.prefix);
        out.extend_from_slice(&(rlen as u64).to_le_bytes());
        for &v in &self.res_vec {
            out.extend_from_slice(&v.to_le_bytes());
        }
        out
    }

    pub fn deserialize(bytes: &[u8], entry_count: usize) -> GCardResult<Self> {
        let need = entry_count.checked_mul(2).ok_or_else(|| GCardError::InvalidData("entry_count overflow".into()))?;
        if bytes.len() < need + 8 {
            return Err(GCardError::InvalidData("block too short for bucket_ids and prefix".into()));
        }
        let bucket_ids = bytes[0..entry_count].to_vec();
        let prefix = bytes[entry_count..entry_count + entry_count].to_vec();
        let rlen = u64::from_le_bytes(bytes[need..need + 8].try_into().map_err(|_| GCardError::InvalidData("res_vec len".into()))?) as usize;
        let res_start = need + 8;
        if bytes.len() < res_start + rlen * 8 {
            return Err(GCardError::InvalidData("block too short for res_vec".into()));
        }
        let mut res_vec = Vec::with_capacity(rlen);
        for i in 0..rlen {
            let start = res_start + i * 8;
            let v = u64::from_le_bytes(bytes[start..start + 8].try_into().map_err(|_| GCardError::InvalidData("res_vec u64".into()))?);
            res_vec.push(v);
        }
        Ok(BlockStatistic {
            bucket_ids,
            prefix,
            res_vec,
        })
    }
}

impl Serialize for BlockStatistic {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let n = self.bucket_ids.len() as u64;
        let mut bytes = n.to_le_bytes().to_vec();
        bytes.extend(self.serialize());
        s.serialize_bytes(&bytes)
    }
}

impl<'de> Deserialize<'de> for BlockStatistic {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let bytes = Vec::<u8>::deserialize(d)?;
        if bytes.len() < 8 {
            return Err(serde::de::Error::custom("block too short for entry_count"));
        }
        let entry_count = u64::from_le_bytes(bytes[0..8].try_into().map_err(serde::de::Error::custom)?) as usize;
        Self::deserialize(&bytes[8..], entry_count).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_u64_sequence_empty() {
        let bs = BlockStatistic::from_u64_sequence(&[]).unwrap();
        assert!(bs.bucket_ids.is_empty());
        assert!(bs.prefix.is_empty());
        assert!(bs.res_vec.is_empty());
    }

    #[test]
    fn test_from_u64_sequence_and_get() {
        let values: Vec<u64> = vec![0, 1, 2, 3, 4, 100, 1000];
        let bs = BlockStatistic::from_u64_sequence(&values).unwrap();
        assert_eq!(bs.bucket_ids.len(), values.len());
        assert_eq!(bs.prefix.len(), values.len());
        assert!(bs.res_vec.len() <= bs.bucket_ids.iter().copied().max().unwrap_or(0) as usize + 1);

        for (rank, &v) in values.iter().enumerate() {
            let (b, p, r) = bs.get_by_rank(rank).unwrap();
            let shift = (b as usize).min(56);
            let expected_prefix = if v == 0 { 0 } else { (v >> shift) as u8 };
            let expected_remainder = v & ((1u64 << shift).saturating_sub(1));
            assert_eq!(p, expected_prefix, "rank {} prefix", rank);
            assert!(r >= expected_remainder, "rank {} res_vec upper bound", rank);
        }
    }

    #[test]
    fn test_bucket_ids() {
        let values: Vec<u64> = vec![1, 2, 10, 100];
        let bs = BlockStatistic::from_u64_sequence(&values).unwrap();
        let ids = bs.bucket_ids();
        assert_eq!(ids.len(), 4);
        assert_eq!(ids, bs.bucket_ids.as_slice());
    }

    #[test]
    fn test_serialize_deserialize() {
        let values: Vec<u64> = vec![1, 100, 1000];
        let bs = BlockStatistic::from_u64_sequence(&values).unwrap();
        let entry_count = bs.bucket_ids.len();
        let bytes = bs.serialize();
        let restored = BlockStatistic::deserialize(&bytes, entry_count).unwrap();
        assert_eq!(restored.bucket_ids, bs.bucket_ids);
        assert_eq!(restored.prefix, bs.prefix);
        assert_eq!(restored.res_vec, bs.res_vec);
    }

    // ── upper_limit_ratio 测试 ────────────────────────────────────────────────

    /// 空序列返回 0.0。
    #[test]
    fn test_upper_limit_ratio_empty() {
        let bs = BlockStatistic::new();
        assert_eq!(bs.upper_limit_ratio(), 0.0);
    }

    /// bucket b < 8 时 rem_bits = 0，max_r = 0，res_vec[b] >= 0 恒成立，
    /// 所有条目均触限，返回 1.0。
    #[test]
    fn test_upper_limit_ratio_small_bucket_always_at_limit() {
        // 1→bucket 0, 3→bucket 2, 5→bucket 3，均 b < 8
        let bs = BlockStatistic::from_u64_sequence(&[1, 3, 5]).unwrap();
        assert!(bs.bucket_ids.iter().all(|&b| b < 8));
        assert_eq!(bs.upper_limit_ratio(), 1.0);
    }

    /// value = 2^20（精确 2 的幂）落在 bucket 20，remainder = 0。
    /// rem_bits = 20 - 8 = 12，max_r = (1<<12)-1 = 4095。
    /// res_vec[20] = 0 < 4095 → 不触限，返回 0.0。
    #[test]
    fn test_upper_limit_ratio_exact_power_not_at_limit() {
        let v = 1u64 << 20; // 1_048_576
        let bs = BlockStatistic::from_u64_sequence(&[v]).unwrap();
        assert_eq!(bs.bucket_ids[0], 20, "应落在 bucket 20");
        assert_eq!(bs.res_vec[20], 0, "remainder 应为 0");
        assert_eq!(bs.upper_limit_ratio(), 0.0);
    }

    /// bucket 20 的普通值（remainder >> max_r），应触限，返回 1.0。
    /// 2^19 + 1 = 524_289 落在 bucket 20，remainder = 524_289 >> (1<<12)-1。
    #[test]
    fn test_upper_limit_ratio_large_bucket_at_limit() {
        let v = (1u64 << 19) + 1; // 524_289，落在 bucket 20
        let bs = BlockStatistic::from_u64_sequence(&[v]).unwrap();
        assert_eq!(bs.bucket_ids[0], 20);
        let rem_bits: usize = 20usize.saturating_sub(8);
        let max_r = (1u64 << rem_bits) - 1; // 4095
        assert!(bs.res_vec[20] >= max_r, "remainder 应触及余量上界");
        assert_eq!(bs.upper_limit_ratio(), 1.0);
    }

    /// 混合：1<<20（bucket 20，不触限）+ 5（bucket 3，b<8 恒触限）→ 0.5。
    #[test]
    fn test_upper_limit_ratio_mixed() {
        let bs = BlockStatistic::from_u64_sequence(&[1u64 << 20, 5]).unwrap();
        let ratio = bs.upper_limit_ratio();
        assert!((ratio - 0.5).abs() < f64::EPSILON, "期望 0.5，实际 {ratio}");
    }

    /// 同桶感染：bucket 12 内两个条目——4096（remainder=0）和 2049（remainder=2049）。
    /// rem_bits = 4，max_r = 15。res_vec[12] = max(0, 2049) = 2049 >= 15，
    /// 导致 bucket 12 的所有条目（含 4096）均被计为触限，返回 1.0。
    #[test]
    fn test_upper_limit_ratio_bucket_infection() {
        // 4096 = 2^12 → bucket 12，remainder = 0（本身不触限）
        // 2049 → bucket 12，remainder = 2049（触限）
        // 两者共享 res_vec[12]，res_vec[12] 被拉高后连带 4096 也触限
        let bs = BlockStatistic::from_u64_sequence(&[4096u64, 2049u64]).unwrap();
        assert_eq!(bs.bucket_ids[0], 12);
        assert_eq!(bs.bucket_ids[1], 12);
        let max_r: u64 = (1u64 << (12usize.saturating_sub(8))) - 1; // 15
        assert!(bs.res_vec[12] >= max_r);
        assert_eq!(bs.upper_limit_ratio(), 1.0);
    }
}
