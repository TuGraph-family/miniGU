use minigu_common::data_type::LogicalType;
use minigu_common::types::{VectorIndexKey, VectorMetric};
/// Metadata describing a vector index on a property.
///
/// TODO: Attach this metadata to property/catalog entries once vector-index DDL is implemented.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VectorIndexMetadata {
    key: VectorIndexKey,
    dimension: usize,
    metric: VectorMetric,
}

impl VectorIndexMetadata {
    pub fn new(
        key: VectorIndexKey,
        dimension: usize,
        metric: VectorMetric,
    ) -> Result<Self, String> {
        LogicalType::validate_vector_dimension(dimension)?;
        Ok(Self {
            key,
            dimension,
            metric,
        })
    }

    pub fn key(&self) -> &VectorIndexKey {
        &self.key
    }

    pub fn dimension(&self) -> usize {
        self.dimension
    }

    pub fn metric(&self) -> VectorMetric {
        self.metric
    }
}

#[cfg(test)]
mod tests {
    use minigu_common::types::LabelId;

    use super::*;

    fn key(label: u32, property: u32, dimension: usize) -> VectorIndexKey {
        VectorIndexKey::new(LabelId::try_from(label).unwrap(), property, dimension).unwrap()
    }

    #[test]
    fn create_metadata_success() {
        let metadata = VectorIndexMetadata::new(key(1, 1, 128), 128, VectorMetric::L2).unwrap();
        assert_eq!(metadata.dimension(), 128);
        assert_eq!(metadata.metric(), VectorMetric::L2);
    }

    #[test]
    fn create_metadata_invalid_dimension() {
        let err = VectorIndexMetadata::new(key(1, 1, 128), 64, VectorMetric::L2).unwrap_err();
        assert!(err.to_string().contains("vector dimension"));
    }
}
