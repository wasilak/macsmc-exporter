use heck::ToSnakeCase;
use std::fmt::Debug;

pub trait SnakeCaseLabel {
    fn as_label(&self) -> String;
}

impl<T> SnakeCaseLabel for T
where
    T: Debug,
{
    fn as_label(&self) -> String {
        format!("{:?}", self).to_snake_case()
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum TemperatureUnit {
    Celsius,
    // Fahrenheit,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum TypeUnit {
    Proximity,
    Die,
    Graphics,
    SystemAgent,
}
