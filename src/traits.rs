use std::hash::Hash;

pub trait Theme: Clone + PartialEq + Sized {}

pub trait ThemeKey<T>: Clone + PartialEq + Hash
where
    T: Theme,
{
    fn theme() -> &'static T;
}
