/// Type of provider which provides dependency by *value*.
///
/// This trait can be interpreted as an extension of [`Into`] trait
/// but with the ability to return remaining part of the provider to be used later
/// or in chain to retrieve more dependencies.
///
/// See [crate] documentation for more.
pub trait Provide<T> {
    /// Remaining part of the provider after providing dependency by value.
    type Remainder;

    /// Provides dependency by *value*, also returning
    /// [remaining part](Provide::Remainder) of the provider.
    ///
    /// # Examples
    ///
    /// ```
    /// use provide::Provide;
    ///
    /// todo!()
    /// ```
    fn provide(self) -> (T, Self::Remainder);
}

impl<T, U> Provide<T> for U
where
    U: Into<T>,
{
    type Remainder = ();

    fn provide(self) -> (T, Self::Remainder) {
        let dependency = self.into();
        (dependency, ())
    }
}
