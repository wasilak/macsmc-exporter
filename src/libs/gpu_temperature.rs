use crate::libs::metrics::SnakeCaseLabel;
use macsmc::Smc;
use std::fmt::Debug;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum GpuTempUnit {
    Celsius,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum GpuTempType {
    Proximity,
    Die,
}

impl crate::libs::metrics::Metrics {
    pub fn set_gpu_temp(&self, value: &macsmc::GpuTemperatures) {
        // proximity temperature
        if *value.proximity > 0.0 {
            self.gpu_temperatures
                .with_label_values(&[
                    &GpuTempUnit::Celsius.as_label(),
                    &GpuTempType::Proximity.as_label(),
                ])
                .set(Into::<f64>::into(value.proximity));
        }

        // die temperature
        if *value.die > 0.0 {
            self.gpu_temperatures
                .with_label_values(&[
                    &GpuTempUnit::Celsius.as_label(),
                    &GpuTempType::Die.as_label(),
                ])
                .set(Into::<f64>::into(value.die));
        }
    }

    pub fn get_gpu_temp(
        &self,
        smc: &mut Smc,
        metrics: &crate::libs::metrics::Metrics,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let gpu_temps = smc.gpu_temperature()?;

        tracing::debug!(
            proximity = format!("{:.2}", *gpu_temps.proximity),
            die = format!("{:.2}", *gpu_temps.die),
            "GPU temperatures"
        );

        metrics.set_gpu_temp(&gpu_temps);
        Ok(())
    }
}
