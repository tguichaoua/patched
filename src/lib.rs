#[cfg(feature = "macros")]
pub use ::patched_macros::Patch;

/// Modify partially or totally the value of `self` from a patch value.
pub trait Patch<P> {
    /// Modify partially or totally the value of `self` from a patch value.
    fn patch(&mut self, patch: P);

    /// Consumes `self` and returns a patched version.
    fn with_patch(mut self, patch: P) -> Self
    where
        Self: Sized,
    {
        self.patch(patch);
        self
    }
}

impl<T> Patch<Option<T>> for T {
    /// Sets the value of `self` if `patch` is `Some`.
    fn patch(&mut self, patch: Option<T>) {
        if let Some(value) = patch {
            *self = value;
        }
    }
}

/// An operator that merges two values.
pub trait Merge<Rhs = Self> {
    /// The result of the merge operation.
    type Output;

    /// Merges `self` and `rhs`.
    fn merge(self, rhs: Rhs) -> Self::Output;
}

impl<T> Merge for Option<T> {
    type Output = Self;

    #[inline]
    fn merge(self, rhs: Self) -> Self::Output {
        self.or(rhs)
    }
}
