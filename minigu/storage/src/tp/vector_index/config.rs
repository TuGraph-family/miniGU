#[derive(Debug, Clone, Copy)]
pub struct VectorIndexConfig {
    pub dimension: usize,
    pub max_points: usize,
}

pub fn create_vector_index_config(dimension: usize, vector_count: usize) -> VectorIndexConfig {
    let max_points = vector_count.min(u32::MAX as usize);
    VectorIndexConfig {
        dimension,
        max_points,
    }
}
