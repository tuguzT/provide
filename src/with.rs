/// Type of provider which can be created from provided dependency.
///
/// Type of dependency is determined from the generic type parameter `T`.
///
/// This trait can be used to emulate extension of self type with dependency type,
/// where the result is product type consisting of self and provided dependency.
///
/// See [crate] documentation for more.
pub trait With<T> {
    /// Type of new provider with provided dependency.
    type Output;

    /// Creates new provider from the self and provided dependency.
    ///
    /// # Examples
    ///
    /// ```
    /// use provide::With;
    ///
    /// todo!()
    /// ```
    fn with(self, dependency: T) -> Self::Output;
}

impl<T> With<T> for () {
    type Output = T;

    fn with(self, dependency: T) -> Self::Output {
        dependency
    }
}
