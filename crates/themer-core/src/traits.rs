use std::hash::Hash;

pub trait Theme: Clone + PartialEq + Sized + 'static {}

pub trait ThemeKey<T>: Clone + PartialEq + Hash + 'static
where
    T: Theme,
{
    fn theme(&self) -> &'static T;
}
