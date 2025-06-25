/// A reduction which might or might not use closed over data.
///
/// # Examples
///
/// Some common non-capturing reductions are:
/// * addition or product of numbers
/// * and or or of booleans
/// * concatenations of lists
///
/// The following is an example implementation:
///
/// ```
/// use orx_composable::*;
///
/// struct Add;
///
/// impl Reduction for Add {
///     type Unit = u32;
///
///     fn reduce(&self, a: Self::Unit, b: Self::Unit) -> Self::Unit {
///         a + b
///     }
/// }
///
/// let add = Add;
///
/// let result = add.reduce(1, add.reduce(2, 3));
/// assert_eq!(result, 6);
/// ```
///
/// The following example, on the other hand, demonstrates a reduction which
/// uses captured data.
///
/// Notice that the captured data is used immutable, and hence, the reduction
/// is expected to be pure.
///
/// ```
/// use orx_composable::*;
///
/// struct MaxBelow42 {
///     upper_bound: u32,
/// }
///
/// impl Reduction for MaxBelow42 {
///     type Unit = u32;
///
///     fn reduce(&self, a: Self::Unit, b: Self::Unit) -> Self::Unit {
///         (a.max(b)).min(self.upper_bound)
///     }
/// }
///
/// let max = MaxBelow42 { upper_bound: 42 };
///
/// assert_eq!(max.reduce(3, 5), 5);
/// assert_eq!(max.reduce(50, 7), 42);
/// ```
pub trait Reduction {
    /// Unit of reduction.
    type Unit;

    /// Reduces two arguments of `a` and `b` into a single value.
    fn reduce(&self, a: Self::Unit, b: Self::Unit) -> Self::Unit;
}
