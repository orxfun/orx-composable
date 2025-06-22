use std::marker::PhantomData;

pub enum Never {}

pub trait TypeSequence: Default {
    type ComposeWith<X>: TypeSequence;

    type SplitLeft;

    type SplitRight: TypeSequence;
}

pub trait TypeSequenceEnd {}

#[derive(Default)]
pub struct End;

impl TypeSequence for End {
    type ComposeWith<X> = One<X>;

    type SplitLeft = Never;

    type SplitRight = Self;
}

impl TypeSequenceEnd for End {}

#[derive(Debug)]
pub struct One<T>(PhantomData<T>);

impl<T> TypeSequence for One<T> {
    type ComposeWith<X> = Many<T, One<X>>;

    type SplitLeft = T;

    type SplitRight = End;
}

impl<T> Default for One<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

pub struct Many<L, R>(PhantomData<(L, R)>)
where
    R: TypeSequence;

impl<L, R> TypeSequence for Many<L, R>
where
    R: TypeSequence,
{
    type ComposeWith<X> = Many<L, R::ComposeWith<X>>;

    type SplitLeft = L;

    type SplitRight = R;
}

impl<L, R> Default for Many<L, R>
where
    R: TypeSequence,
{
    fn default() -> Self {
        Self(Default::default())
    }
}

pub trait InputBuilder: Sized {
    type Left;

    type Right: TypeSequence;

    type X;

    type ComposedWith<I>: InputBuilder;

    fn compose(self, value: Self::Left) -> Self::ComposedWith<Self::Left> {
        todo!()
    }

    fn value(self) -> Self::X;
}

pub struct InputBuilder0<Left, Right>(PhantomData<(Left, Right)>)
where
    Right: TypeSequence;

impl<Left, Right> InputBuilder for InputBuilder0<Left, Right>
where
    Right: TypeSequence,
{
    type Left = Left;

    type Right = Right;

    type X = ();

    type ComposedWith<I> = InputBuilder1<
        <Self::Right as TypeSequence>::SplitLeft,
        <Self::Right as TypeSequence>::SplitRight,
        I,
    >;

    fn compose(self, value: Self::Left) -> Self::ComposedWith<Self::Left> {
        InputBuilder1(PhantomData, value)
    }

    fn value(self) -> Self::X {
        ()
    }
}

pub struct InputBuilder1<Left, Right, X>(PhantomData<(Left, Right)>, X)
where
    Right: TypeSequence;

impl<Left, Right, X> InputBuilder for InputBuilder1<Left, Right, X>
where
    Right: TypeSequence,
{
    type Left = Left;

    type Right = Right;

    type X = X;

    type ComposedWith<I> = InputBuilder2<
        <Self::Right as TypeSequence>::SplitLeft,
        <Self::Right as TypeSequence>::SplitRight,
        Self::X,
        I,
    >;

    fn compose(self, value: Self::Left) -> Self::ComposedWith<Self::Left> {
        InputBuilder2(PhantomData, self.1, value)
    }

    fn value(self) -> Self::X {
        self.1
    }
}

pub struct InputBuilder2<Left, Right, X1, X2>(PhantomData<(Left, Right)>, X1, X2)
where
    Right: TypeSequence;

impl<Left, Right, X1, X2> InputBuilder for InputBuilder2<Left, Right, X1, X2>
where
    Right: TypeSequence,
{
    type Left = Left;

    type Right = Right;

    type X = (X1, X2);

    type ComposedWith<I> = InputBuilder2<
        <Self::Right as TypeSequence>::SplitLeft,
        <Self::Right as TypeSequence>::SplitRight,
        Self::X,
        I,
    >;

    fn compose(self, value: Self::Left) -> Self::ComposedWith<Self::Left> {
        InputBuilder2(PhantomData, (self.1, self.2), value)
    }

    fn value(self) -> Self::X {
        (self.1, self.2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abc() {
        let x = End;
        let y = compose_with::<_, u32>(x);
        let z = compose_with::<_, char>(y);
        let w = compose_with::<_, bool>(z);
        let t = compose_with::<_, String>(w);

        type X = Many<One<u32>, Many<One<char>, Many<One<bool>, One<String>>>>;

        type TypeSeq = Many<u32, Many<char, Many<bool, One<String>>>>;
        type Left = <TypeSeq as TypeSequence>::SplitLeft;
        type Right = <TypeSeq as TypeSequence>::SplitRight;

        let builder_x = InputBuilder0::<Left, Right>(PhantomData);
        let builder_y = builder_x.compose(42);
        let builder_z = builder_y.compose('x');
        let builder_w = builder_z.compose(true);
        let builder_t = builder_w.compose(String::from("abc"));
        let result = builder_t.value();
        dbg!(result);

        assert_eq!(12, 33);
    }

    fn compose_with<S, Y>(_: S) -> <S as TypeSequence>::ComposeWith<Y>
    where
        S: TypeSequence,
    {
        Default::default()
    }
}
