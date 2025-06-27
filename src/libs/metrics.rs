use heck::ToSnakeCase;
use prometheus::GaugeVec;
use prometheus::register_gauge_vec;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Metrics {
    pub temperatures: GaugeVec,
    pub power: GaugeVec,
    pub battery_detail: GaugeVec,
    pub battery_info: GaugeVec,
    pub fan_speed: GaugeVec,
    pub gpu_temperatures: GaugeVec,
    pub other_temperatures: GaugeVec,
    pub cpu_core_temperatures: GaugeVec,
}

impl Metrics {
    pub fn new() -> Self {
        let temperatures = register_gauge_vec!(
            "cpu_temperature",
            "Metric for CPU temperatures",
            &["unit", "temp_type"]
        )
        .unwrap();

        let power = register_gauge_vec!(
            "cpu_power",
            "Metric for CPU power consumption",
            &["unit", "power_type"]
        )
        .unwrap();

        let battery_detail = register_gauge_vec!(
            "battery_detail",
            "Metric for battery detailed information",
            &["unit", "detail_type", "battery_id"]
        )
        .unwrap();

        let battery_info = register_gauge_vec!(
            "battery_info",
            "Metric for battery general information",
            &["unit", "info_type", "battery_id"]
        )
        .unwrap();

        let fan_speed = register_gauge_vec!(
            "fan_speed",
            "Metric for fan speed information",
            &["unit", "speed_type", "fan_id"]
        )
        .unwrap();

        let gpu_temperatures = register_gauge_vec!(
            "gpu_temperature",
            "Metric for GPU temperatures",
            &["unit", "temp_type"]
        )
        .unwrap();

        let other_temperatures = register_gauge_vec!(
            "other_temperature",
            "Metric for other system temperatures",
            &["unit", "temp_type"]
        )
        .unwrap();

        let cpu_core_temperatures = register_gauge_vec!(
            "cpu_core_temperature",
            "Metric for individual CPU core temperatures",
            &["unit", "temp_type", "core_id"]
        )
        .unwrap();

        Self {
            temperatures,
            power,
            battery_detail,
            battery_info,
            fan_speed,
            gpu_temperatures,
            other_temperatures,
            cpu_core_temperatures,
        }
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
