use crate::{Computation, ComputeReduce, Reduction, compute_reduce::ComputeReduceEmpty};

/// A reduction `R` and a reducible over `R` computation `C`, providing [`compose`] and [`compute`]
/// methods.
///
/// # Examples
///
/// The following example demonstrates creating a composition over the reduction `Add`.
/// Then, this computation is composed with different computations requiring different
/// inputs, while all returning a number.
///
/// ```
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
/// let c = Composition::new(Add);
/// assert_eq!(c.compute(()), 0);
///
/// // compose 1
/// let c = Composition::new(Add).compose(StrLen);
/// assert_eq!(c.compute("xyz"), 3);
///
/// // compose 2
/// let c = Composition::new(Add).compose(StrLen).compose(SliceLen);
/// assert_eq!(c.compute(("xyz", &[true, false])), 5);
///
/// // compose 3
/// let c = Composition::new(Add)
///     .compose(StrLen)
///     .compose(SliceLen)
///     .compose(NumEvens);
/// assert_eq!(c.compute((("xyz", &[true, false]), vec![1, 2, 3, 4, 5])), 7);
/// ```
///
/// [`compose`]: Composition::compose
/// [`compute`]: Computation::compute
pub struct Composition<R, C>
where
    R: Reduction,
    C: ComputeReduce<R = R>,
{
    reduction: R,
    computation: C,
}

impl<R> Composition<R, ComputeReduceEmpty<R>>
where
    R: Reduction,
{
    /// Creates a new composition over the `reduction` without yet any computation.
    pub fn new(reduction: R) -> Self {
        Self {
            reduction,
            computation: Default::default(),
        }
    }
}

impl<R, C> Composition<R, C>
where
    R: Reduction,
    C: ComputeReduce<R = R>,
{
    /// Composes this computation with the `other` over the reduction `R` and returns
    /// the resulting computation.
    pub fn compose<C2>(self, other: C2) -> Composition<R, C::Compose<C2>>
    where
        C2: Computation<Out = R::Unit>,
    {
        Composition {
            reduction: self.reduction,
            computation: self.computation.compose(other),
        }
    }
}

impl<R, C> Computation for Composition<R, C>
where
    R: Reduction,
    C: ComputeReduce<R = R>,
{
    type In<'i> = C::In<'i>;

    type Out = R::Unit;

    /// Computes output for the given `input`.
    ///
    /// # Examples
    ///
    /// The following example demonstrates creating a composition over the reduction `Add`.
    /// Then, this computation is composed with different computations requiring different
    /// inputs, while all returning a number.
    ///
    /// ```
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
    /// let c = Composition::new(Add);
    /// assert_eq!(c.compute(()), 0);
    ///
    /// // compose 1
    /// let c = Composition::new(Add).compose(StrLen);
    /// assert_eq!(c.compute("xyz"), 3);
    ///
    /// // compose 2
    /// let c = Composition::new(Add).compose(StrLen).compose(SliceLen);
    /// assert_eq!(c.compute(("xyz", &[true, false])), 5);
    ///
    /// // compose 3
    /// let c = Composition::new(Add)
    ///     .compose(StrLen)
    ///     .compose(SliceLen)
    ///     .compose(NumEvens);
    /// assert_eq!(c.compute((("xyz", &[true, false]), vec![1, 2, 3, 4, 5])), 7);
    /// ```
    #[inline(always)]
    fn compute(&self, input: Self::In<'_>) -> Self::Out {
        self.computation.compute_reduce(&self.reduction, input)
    }
}
