use macsmc::CpuTemperatures;
use prometheus::GaugeVec;

use crate::libs::cpu_temperature::{SnakeCaseLabel, TemperatureUnit, TypeUnit};

#[derive(Debug, Clone)]
pub struct Metrics {
    pub temperatures: GaugeVec,
}

impl Metrics {
    pub fn set_cpu_temp(&self, value: &CpuTemperatures) {
        self.temperatures
            .with_label_values(&[
                TemperatureUnit::Celsius.as_label(),
                TypeUnit::Proximity.as_label(),
            ])
            .set(Into::<f64>::into(value.proximity));

        self.temperatures
            .with_label_values(&[
                TemperatureUnit::Celsius.as_label(),
                TypeUnit::Die.as_label(),
            ])
            .set(Into::<f64>::into(value.die));

        self.temperatures
            .with_label_values(&[
                TemperatureUnit::Celsius.as_label(),
                TypeUnit::Graphics.as_label(),
            ])
            .set(Into::<f64>::into(value.graphics));

        self.temperatures
            .with_label_values(&[
                TemperatureUnit::Celsius.as_label(),
                TypeUnit::SystemAgent.as_label(),
            ])
            .set(Into::<f64>::into(value.system_agent));
    }
}
