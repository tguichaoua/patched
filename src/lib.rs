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

/// An operator that merges two patch values.
///
/// This operation must be coherent with [`Patch`] so that the two following codes are equivalent.
///
/// ```no_run
/// # use patched::{Patch, Merge};
/// # #[derive(Patch)] struct Foo;
/// # let mut value: Foo = loop {}
/// # let patch_1: FooPatch = loop {}
/// # let patch_2: FooPatch = loop {}
/// // 1
/// value.patch(patch_1);
/// value.patch(patch_2);
///
/// // 2
/// value.patch(patch_1.merge(patch_2))
/// ```
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
