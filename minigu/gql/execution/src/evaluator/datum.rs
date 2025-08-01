use arrow::array::{Array, ArrayRef, Datum};

#[derive(Debug, Clone)]
pub struct DatumRef {
    array: ArrayRef,
    is_scalar: bool,
}

impl DatumRef {
    pub fn new(array: ArrayRef, is_scalar: bool) -> Self {
        Self { array, is_scalar }
    }

    #[inline]
    pub fn as_array(&self) -> &ArrayRef {
        &self.array
    }

    #[inline]
    pub fn is_scalar(&self) -> bool {
        self.is_scalar
    }

    #[inline]
    pub fn into_array(self) -> ArrayRef {
        self.array
    }
}

impl Datum for DatumRef {
    #[inline]
    fn get(&self) -> (&dyn Array, bool) {
        (self.array.as_ref(), self.is_scalar)
    }
}
