#![allow(clippy::module_inception)]

/// Type of provider which can be created from provided dependency.
///
/// This trait can be used to emulate extension of self type with dependency type,
/// where the [output](With::Output) is product type consisting of self and provided dependency.
///
/// See [crate] documentation for more.
pub trait With<T>: Sized {
    /// Type of new provider with provided dependency.
    type Output;

    /// Creates new provider from the self and provided dependency.
    ///
    /// # Examples
    ///
    /// ```
    /// use provide::with::With;
    ///
    /// todo!()
    /// ```
    #[must_use]
    fn with(self, dependency: T) -> Self::Output;
}

impl<T> With<T> for () {
    type Output = T;

    fn with(self, dependency: T) -> Self::Output {
        dependency
    }
}
