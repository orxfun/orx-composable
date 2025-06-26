use crate::{Computation, Reduction};
use orx_meta_queue::MetaQueue;

/// A computation and a reduction, which can be composed since the output
/// of the computation is the unit of the reduction.
///
/// # Examples
///
/// The following demonstrates composition of three computations returning
/// `usize` over `Add` used as the reduction.
///
/// ```
/// use orx_composable::compute_reduce::ReducibleComputationEmpty;
/// use orx_composable::*;
///
/// struct Add;
///
/// impl Reduction for Add {
///     type Unit = usize;
///
///     fn identity(&self) -> Self::Unit {
///         0
///     }
///
///     fn reduce(&self, a: Self::Unit, b: Self::Unit) -> Self::Unit {
///         a + b
///     }
/// }
///
/// struct StrLen;
///
/// impl Computation for StrLen {
///     type In<'i> = &'i str;
///
///     type Out = usize;
///
///     fn compute(&self, str: Self::In<'_>) -> Self::Out {
///         str.len()
///     }
/// }
///
/// struct SliceLen;
///
/// impl Computation for SliceLen {
///     type In<'i> = &'i [bool];
///
///     type Out = usize;
///
///     fn compute(&self, slice: Self::In<'_>) -> Self::Out {
///         slice.len()
///     }
/// }
///
/// struct NumEvens;
///
/// impl Computation for NumEvens {
///     type In<'i> = Vec<u64>;
///
///     type Out = usize;
///
///     fn compute(&self, vec: Self::In<'_>) -> Self::Out {
///         vec.iter().filter(|x| *x % 2 == 0).count()
///     }
/// }
///
/// let c = ReducibleComputationEmpty::<Add>::new();
/// let c = c.compose(StrLen);
/// let c = c.compose(SliceLen);
/// let c = c.compose(NumEvens);
///
/// assert_eq!(
///     c.compute_reduce(&Add, (("xyz", &[true, false]), vec![1, 2, 4, 4])),
///     8
/// );
/// ```
pub trait ReducibleComputation {
    /// Input of the computation.
    ///
    /// Note that output of the computation is equal to the unit of the
    /// Reduction used to compose multiple computations over their results.
    type R: Reduction;

    /// reduction `Self::R`.
    type In<'i>;

    type InQueue<'i>: MetaQueue;

    /// Type obtained by composing this [`ReducibleComputation`] with the computation
    /// `C` having the same output type.
    type Compose<C>: ReducibleComputation<R = Self::R>
    where
        C: Computation<Out = <Self::R as Reduction>::Unit>;

    /// Composes this [`ReducibleComputation`] with the computation `C` having the same
    /// output type over reduction `R`.
    fn compose<C>(self, other: C) -> Self::Compose<C>
    where
        C: Computation<Out = <Self::R as Reduction>::Unit>;

    /// Computes-reduces and returns the result.
    ///
    /// * If the compute-reduce has no composed computation (identity), then it
    ///   returns `reduction.identity()`.
    /// * If there is only one composed computation, then it directly returns the
    ///   result of this computation with the given `input`.
    /// * If there exist more than one composed computation, the `input` is the
    ///   tuple of all all composed computations. Then, outputs of all computations
    ///   with their corresponding inputs are computed and outputs are reduced
    ///   to a single value using the provided `reduction`.
    ///
    /// # Examples
    ///
    /// The following example demonstrates composing more and more computations
    /// over addition as the reduction.
    ///
    /// ```
    /// use orx_composable::compute_reduce::ReducibleComputationEmpty;
    /// use orx_composable::*;
    ///
    /// struct Add;
    ///
    /// impl Reduction for Add {
    ///     type Unit = usize;
    ///
    ///     fn identity(&self) -> Self::Unit {
    ///         0
    ///     }
    ///
    ///     fn reduce(&self, a: Self::Unit, b: Self::Unit) -> Self::Unit {
    ///         a + b
    ///     }
    /// }
    ///
    /// struct StrLen;
    ///
    /// impl Computation for StrLen {
    ///     type In<'i> = &'i str;
    ///
    ///     type Out = usize;
    ///
    ///     fn compute(&self, str: Self::In<'_>) -> Self::Out {
    ///         str.len()
    ///     }
    /// }
    ///
    /// struct SliceLen;
    ///
    /// impl Computation for SliceLen {
    ///     type In<'i> = &'i [bool];
    ///
    ///     type Out = usize;
    ///
    ///     fn compute(&self, slice: Self::In<'_>) -> Self::Out {
    ///         slice.len()
    ///     }
    /// }
    ///
    /// struct NumEvens;
    ///
    /// impl Computation for NumEvens {
    ///     type In<'i> = Vec<u64>;
    ///
    ///     type Out = usize;
    ///
    ///     fn compute(&self, vec: Self::In<'_>) -> Self::Out {
    ///         vec.iter().filter(|x| *x % 2 == 0).count()
    ///     }
    /// }
    ///
    /// // compose 0
    /// let c = ReducibleComputationEmpty::<Add>::new();
    /// assert_eq!(c.compute_reduce(&Add, ()), 0);
    ///
    /// // compose 1
    /// let c = ReducibleComputationEmpty::<Add>::new().compose(StrLen);
    /// assert_eq!(c.compute_reduce(&Add, "xyz"), 3);
    ///
    /// // compose 2
    /// let c = ReducibleComputationEmpty::<Add>::new()
    ///     .compose(StrLen)
    ///     .compose(SliceLen);
    /// assert_eq!(c.compute_reduce(&Add, ("xyz", &[true, false])), 5);
    ///
    /// // compose 3
    /// let c = ReducibleComputationEmpty::<Add>::new()
    ///     .compose(StrLen)
    ///     .compose(SliceLen)
    ///     .compose(NumEvens);
    /// assert_eq!(
    ///     c.compute_reduce(&Add, (("xyz", &[true, false]), vec![1, 2, 4, 4])),
    ///     8
    /// );
    /// ```
    fn compute_reduce<'i>(
        &self,
        reduction: &Self::R,
        input: Self::In<'i>,
    ) -> <Self::R as Reduction>::Unit;
}
