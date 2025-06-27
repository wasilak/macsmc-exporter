use heck::ToSnakeCase;
use prometheus::GaugeVec;
use prometheus::register_gauge_vec;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Metrics {
    pub temperatures: GaugeVec,
}

impl Metrics {
    pub fn new() -> Self {
        let temperatures = register_gauge_vec!(
            "cpu_temperature",
            "Metric for CPU temperatures",
            &["unit", "temp_type"]
        )
        .unwrap();
        Self { temperatures }
    }
}

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
