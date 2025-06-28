use crate::{
    Computation, InputBuilder, ReducibleComputation, Reduction,
    compute_reduce::ReducibleComputationEmpty,
};

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
    C: ReducibleComputation<R = R>,
{
    reduction: R,
    computation: C,
}

impl<R> Composition<R, ReducibleComputationEmpty<R>>
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
    C: ReducibleComputation<R = R>,
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

    /// Creates a type-safe input builder to build the inputs of a computation by adding
    /// input of each computation one by one.
    ///
    /// Once all input pieces are added `value` can be called to obtained the
    /// composed input. Note that `value` cannot be called beforehand.
    ///
    /// Consider for instance, a composition of four computations over a particular
    /// reduction. Assume the inputs of these computations are 'a: A', 'b: B', 'c: C'
    /// and 'd: D' in the order they are composed. Then, input of the composed computation
    /// is represented as recursively composed tuples, `(((a, b), c), d)` in this example.
    /// Typing out this type becomes more and more complicated as we keep composing
    /// more computations.
    ///
    /// `InputBuilder` allows to avoid this complexity and allows to the create the input type
    /// by adding the inputs one after the other:
    ///
    /// ```ignore
    /// let input: (((A, B), C), D) = computation
    ///     .input_builder()
    ///     .add(a)
    ///     .add(b)
    ///     .add(c)
    ///     .add(d)
    ///     .value();
    ///
    /// let result = computation.compute(input);
    /// ```
    ///
    /// Note that the input builder is type-safe due to the following:
    /// * `add` calls cannot be made with wrong types,
    /// * `add` calls cannot be made in wrong order,
    /// * `value` cannot be called before making exactly the required sequence of `add` calls.
    ///
    /// # Examples
    ///
    /// The following example demonstrates composition of three computations over
    /// the reduction `Add`, and use of input builder to call the composed computation with
    /// different inputs.
    ///
    /// ```
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
    /// let c = Composition::new(Add)
    ///     .compose(StrLen)
    ///     .compose(SliceLen)
    ///     .compose(NumEvens);
    ///
    /// let input = c
    ///     .input_builder()
    ///     .add("abc")
    ///     .add(&[true, false])
    ///     .add(vec![1, 2, 4, 6])
    ///     .value();
    /// assert_eq!(c.compute(input), 8);
    ///
    /// let input = c
    ///     .input_builder()
    ///     .add("x")
    ///     .add(&[false])
    ///     .add(vec![1])
    ///     .value();
    /// assert_eq!(c.compute(input), 2);
    /// ```
    pub fn input_builder(&self) -> InputBuilder<C::InQueue<'_>> {
        Default::default()
    }
}

impl<R, C> Computation for Composition<R, C>
where
    R: Reduction,
    C: ReducibleComputation<R = R>,
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
