use crate::libs::metrics::SnakeCaseLabel;
use macsmc::CpuPower;
use macsmc::Smc;
use std::fmt::Debug;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum PowerUnit {
    Watts,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum PowerType {
    Core,
    Dram,
    Gfx,
    Rail,
    Total,
}

impl crate::libs::metrics::Metrics {
    pub fn set_cpu_power(&self, value: &CpuPower) {
        self.power
            .with_label_values(&[&PowerUnit::Watts.as_label(), &PowerType::Core.as_label()])
            .set(Into::<f64>::into(value.core));

        self.power
            .with_label_values(&[&PowerUnit::Watts.as_label(), &PowerType::Dram.as_label()])
            .set(Into::<f64>::into(value.dram));

        self.power
            .with_label_values(&[&PowerUnit::Watts.as_label(), &PowerType::Gfx.as_label()])
            .set(Into::<f64>::into(value.gfx));

        self.power
            .with_label_values(&[&PowerUnit::Watts.as_label(), &PowerType::Rail.as_label()])
            .set(Into::<f64>::into(value.rail));

        self.power
            .with_label_values(&[&PowerUnit::Watts.as_label(), &PowerType::Total.as_label()])
            .set(Into::<f64>::into(value.total));
    }

    pub fn get_cpu_power(
        &self,
        smc: &mut Smc,
        metrics: &crate::libs::metrics::Metrics,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let cpu_power = smc.cpu_power()?;

        tracing::debug!(
            power.core = format!("{:.2}", *cpu_power.core),
            power.dram = format!("{:.2}", *cpu_power.dram),
            power.gfx = format!("{:.2}", *cpu_power.gfx),
            power.rail = format!("{:.2}", *cpu_power.rail),
            power.total = format!("{:.2}", *cpu_power.total),
            "CPU power consumption"
        );

        metrics.set_cpu_power(&cpu_power);
        Ok(())
    }
}
