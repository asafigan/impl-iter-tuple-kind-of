trait Identity<Z> {
    fn identity(value: Z) -> Self;
}

impl<T: Sized> Identity<T> for T {
    fn identity(value: Self) -> Self {
        value
    }
}
trait RefIdentity<Z: ?Sized> {
    fn ref_identity(value: &Z) -> &Self;
}

impl<T: ?Sized> RefIdentity<T> for T {
    fn ref_identity(value: &Self) -> &Self {
        value
    }
}

trait ToArray<'b, Z, const N: usize> {
    fn to_array<'a: 'b, T: Identity<Z>>(&'a self) -> [T; N];
}

impl<'b, A: 'b, B: 'b, Z> ToArray<'b, Z, 2> for (A, B)
where
    Z: From<&'b A> + From<&'b B> + 'b,
{
    fn to_array<'a: 'b, T: Identity<Z>>(&'a self) -> [T; 2] {
        [T::identity(Z::from(&self.0)), T::identity(Z::from(&self.1))]
    }
}

trait ToIter<'b, Z> {
    type Iter<'a, T>: Iterator<Item = T>
    where
        Self: 'a + 'b,
        T: 'a,
        'b: 'a,
        'a: 'b;
    fn to_iter<'a: 'b, T: Identity<Z> + 'a>(&'a self) -> Self::Iter<'a, T>
    where
        'b: 'a;
}

impl<'b, A: 'b, B: 'b, Z> ToIter<'b, Z> for (A, B)
where
    Z: From<&'b A> + From<&'b B> + 'b,
{
    type Iter<'a, T> = core::array::IntoIter<T, 2>
    where
        Self: 'a + 'b,
        T: 'a,
        'b: 'a,
        'a: 'b;
    fn to_iter<'a: 'b, T: Identity<Z> + 'a>(&'a self) -> Self::Iter<'a, T>
    where
        'b: 'a,
    {
        [T::identity(Z::from(&self.0)), T::identity(Z::from(&self.1))].into_iter()
    }
}

trait AsIter<'b, Z: ?Sized> {
    type Iter<'a, T: ?Sized>: Iterator<Item = &'a T>
    where
        Self: 'a + 'b,
        T: 'a,
        'a: 'b,
        'b: 'a;

    fn iter<'a: 'b, T: RefIdentity<Z> + ?Sized>(&'a self) -> Self::Iter<'a, T>
    where
        'b: 'a;
}

impl<'b, A: 'b, B: 'b, Z: ?Sized> AsIter<'b, Z> for (A, B)
where
    &'b Z: From<&'b A> + From<&'b B> + 'b,
{
    type Iter<'a, T: ?Sized> = core::array::IntoIter<&'a T, 2>
    where
        Self: 'a + 'b,
        T: 'a,
        'a: 'b,
        'b: 'a;

    fn iter<'a: 'b, T: RefIdentity<Z> + ?Sized>(&'a self) -> Self::Iter<'a, T>
    where
        'b: 'a,
    {
        [
            T::ref_identity(<&'a Z>::from(&self.0)),
            T::ref_identity(<&'a Z>::from(&self.1)),
        ]
        .into_iter()
    }
}

trait ToArrayMut<'b, Z, const N: usize> {
    fn to_array_mut<'a: 'b, T: Identity<Z>>(&'a mut self) -> [T; N];
}

impl<'b, A: 'b, B: 'b, Z> ToArrayMut<'b, Z, 2> for (A, B)
where
    Z: From<&'b mut A> + From<&'b mut B> + 'b,
{
    fn to_array_mut<'a: 'b, T: Identity<Z>>(&'a mut self) -> [T; 2] {
        [
            T::identity(Z::from(&mut self.0)),
            T::identity(Z::from(&mut self.1)),
        ]
    }
}

trait ToArrayOwned<Z, const N: usize> {
    fn to_array_owned<T: Identity<Z>>(self) -> [T; N];
}

impl<'b, A: 'b, B: 'b, Z> ToArrayOwned<Z, 2> for (A, B)
where
    Z: From<A> + From<B> + 'b,
{
    fn to_array_owned<T: Identity<Z>>(self) -> [T; 2] {
        [T::identity(Z::from(self.0)), T::identity(Z::from(self.1))]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn to_array() {
        for x in (1_u8, 2_i8).to_array_mut::<&mut dyn Example>() {
            dbg!(x.example());
        }

        for x in (1_u8, 2_i8).to_array_owned::<Box<dyn Example>>() {
            dbg!(x.example());
        }

        let values: Vec<_> = (1_u8, 2_i8)
            .to_array::<&dyn Example>()
            .into_iter()
            .map(|x| x.example())
            .collect();

        assert_eq!(values, [1, 2]);
    }

    #[test]
    fn as_iter() {
        let values: Vec<_> = (1_u8, 2_i8)
            .iter::<dyn Example>()
            .map(|x| x.example())
            .collect();

        assert_eq!(values, [1, 2]);
    }

    #[test]
    fn to_iter() {
        let values: Vec<_> = (1_u8, 2_i8)
            .to_iter::<&dyn Example>()
            .map(|x| x.example())
            .collect();

        assert_eq!(values, [1, 2]);
    }

    #[test]
    fn example_list() {
        let values: Vec<_> = (1_u8, 2_i8).iter_examples().map(|x| x.example()).collect();

        assert_eq!(values, [1, 2]);
    }

    trait Example {
        fn example(&self) -> i32;
    }

    impl<'a, T: Example> From<&'a T> for &'a dyn Example {
        fn from(value: &'a T) -> Self {
            value
        }
    }

    impl<'a, T: Example> From<&'a mut T> for &'a mut dyn Example {
        fn from(value: &'a mut T) -> Self {
            value
        }
    }

    impl<T: Example + 'static> From<T> for Box<dyn Example> {
        fn from(value: T) -> Self {
            Box::new(value)
        }
    }

    trait ExampleList<'a> {
        type Item: Example + ?Sized;
        type IntoIter: Iterator<Item = &'a Self::Item>
        where
            Self: 'a;

        fn iter_examples(&'a self) -> Self::IntoIter;
    }

    impl<'a, T> ExampleList<'a> for T
    where
        T: ToIter<'a, &'a dyn Example>,
    {
        type Item = dyn Example + 'a;

        type IntoIter = T::Iter<'a, &'a (dyn Example + 'a)>
        where
            Self: 'a;

        fn iter_examples(&'a self) -> Self::IntoIter {
            self.to_iter()
        }
    }

    impl Example for u8 {
        fn example(&self) -> i32 {
            *self as i32
        }
    }

    impl Example for i8 {
        fn example(&self) -> i32 {
            *self as i32
        }
    }
}
