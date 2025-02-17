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
        rhs.or(self)
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use crate::{Merge, Patch};

    fn test_merge<T, P>(value: T, patches: impl IntoIterator<Item = P>) -> T
    where
        T: Clone + Patch<P> + Eq + Debug,
        P: Merge<Output = P> + Clone,
    {
        let mut patches = patches.into_iter();

        let mut value_a = value.clone();
        let mut value_b = value;

        let Some(mut combined_patch) = patches.next() else {
            panic!("expected at least one patch");
        };

        value_a.patch(combined_patch.clone());

        for patch in patches {
            value_a.patch(patch.clone());
            combined_patch = combined_patch.merge(patch);
        }

        value_b.patch(combined_patch);

        assert_eq!(value_a, value_b);
        value_a
    }

    #[test]
    fn option_merge() {
        assert_eq!(test_merge(99, [Some(1), Some(2)]), 2);

        assert_eq!(test_merge(99, [None, Some(1), None, None]), 1);

        assert_eq!(test_merge(99, [None, None, None]), 99);
    }
}
