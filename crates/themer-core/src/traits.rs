use std::hash::Hash;

pub trait Theme: Clone + PartialEq + Sized + 'static {}

pub trait ThemeKey: Clone + Copy + PartialEq + Hash + 'static {
    type Theme: Theme + 'static;
    fn theme(&self) -> &'static Self::Theme;
}
