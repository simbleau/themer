pub trait Theme: Clone + PartialEq + Sized + 'static {}

pub trait ThemeKey:
    Clone + Copy + PartialEq + std::hash::Hash + 'static
{
    type Theme: Theme + 'static;
    fn theme(&self) -> &'static Self::Theme;
}
