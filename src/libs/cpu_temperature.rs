use crate::libs::metrics::SnakeCaseLabel;
use macsmc::CpuTemperatures;
use macsmc::Smc;
use std::fmt::Debug;

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

impl crate::libs::metrics::Metrics {
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

    pub fn get_cpu_temp(
        &self,
        smc: &mut Smc,
        metrics: &crate::libs::metrics::Metrics,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let cpu_temp = smc.cpu_temperature()?;

        tracing::debug!(
            temperature.proximity = format!("{:.2}", *cpu_temp.proximity),
            temperature.die = format!("{:.2}", *cpu_temp.die),
            temperature.graphics = format!("{:.2}", *cpu_temp.graphics),
            temperature.system_agent = format!("{:.2}", *cpu_temp.system_agent),
            "CPU temperatures"
        );

        metrics.set_cpu_temp(&cpu_temp);
        Ok(())
    }
}
