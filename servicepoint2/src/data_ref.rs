/// A trait for getting the underlying raw byte slices of data containers.
///
/// The expectation is that you can create an equal instance with this data given the additional
/// metadata needed.
pub trait DataRef {
    /// Get the underlying bytes writable.
    fn data_ref_mut(&mut self) -> &mut [u8];

    /// Get the underlying bytes read-only.
    fn data_ref(&self) -> &[u8];
}
