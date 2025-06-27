use crate::libs::metrics::SnakeCaseLabel;
use macsmc::Smc;
use std::fmt::Debug;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum CpuCoreTempUnit {
    Celsius,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum CpuCoreTempType {
    Core,
}

impl crate::libs::metrics::Metrics {
    pub fn set_cpu_core_temp(&self, temperature: f64, core_id: usize) {
        let core_id_str = core_id.to_string();

        // Only set metric if temperature is meaningful (> 0.0)
        if temperature > 0.0 {
            self.cpu_core_temperatures
                .with_label_values(&[
                    &CpuCoreTempUnit::Celsius.as_label(),
                    &CpuCoreTempType::Core.as_label(),
                    &core_id_str,
                ])
                .set(temperature);
        }
    }

    pub fn get_cpu_core_temps(
        &self,
        smc: &mut Smc,
        metrics: &crate::libs::metrics::Metrics,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut cpu_cores = smc.cpu_core_temps()?;
        let mut core_id = 0;
        let mut core_temps = Vec::new();

        // Collect all core temperatures
        while let Some(temp_result) = cpu_cores.next() {
            let temperature = temp_result?;
            let temp_celsius = Into::<f64>::into(temperature);

            core_temps.push((core_id, temp_celsius));
            metrics.set_cpu_core_temp(temp_celsius, core_id);
            core_id += 1;
        }

        // Log all core temperatures in a structured way
        if !core_temps.is_empty() {
            let mut debug_fields = vec![];
            for (id, temp) in &core_temps {
                debug_fields.push(format!("core_{} = {:.2}", id, temp));
            }

            tracing::debug!(
                core_count = core_temps.len(),
                temperatures = debug_fields.join(", "),
                "CPU core temperatures"
            );
        }

        Ok(())
    }
}
