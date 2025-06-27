use crate::libs::metrics::SnakeCaseLabel;
use macsmc::Smc;
use std::fmt::Debug;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum BatteryInfoUnit {
    Boolean,
    Celsius,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum BatteryInfoType {
    BatteryPowered,
    Charging,
    AcPresent,
    HealthOk,
    TemperatureMax,
    Temperature1,
    Temperature2,
}

impl crate::libs::metrics::Metrics {
    pub fn set_battery_info(&self, battery_info: &macsmc::BatteryInfo) {
        // Since battery_info() returns a single struct (not an iterator), we use battery_id = 0
        let battery_id_str = 0.to_string();

        // battery_powered (bool)
        self.battery_info
            .with_label_values(&[
                &BatteryInfoUnit::Boolean.as_label(),
                &BatteryInfoType::BatteryPowered.as_label(),
                &battery_id_str,
            ])
            .set(if battery_info.battery_powered {
                1.0
            } else {
                0.0
            });

        // charging (bool)
        self.battery_info
            .with_label_values(&[
                &BatteryInfoUnit::Boolean.as_label(),
                &BatteryInfoType::Charging.as_label(),
                &battery_id_str,
            ])
            .set(if battery_info.charging { 1.0 } else { 0.0 });

        // ac_present (bool)
        self.battery_info
            .with_label_values(&[
                &BatteryInfoUnit::Boolean.as_label(),
                &BatteryInfoType::AcPresent.as_label(),
                &battery_id_str,
            ])
            .set(if battery_info.ac_present { 1.0 } else { 0.0 });

        // health_ok (bool)
        self.battery_info
            .with_label_values(&[
                &BatteryInfoUnit::Boolean.as_label(),
                &BatteryInfoType::HealthOk.as_label(),
                &battery_id_str,
            ])
            .set(if battery_info.health_ok { 1.0 } else { 0.0 });

        // temperature_max (Celsius)
        if *battery_info.temperature_max > 0.0 {
            self.battery_info
                .with_label_values(&[
                    &BatteryInfoUnit::Celsius.as_label(),
                    &BatteryInfoType::TemperatureMax.as_label(),
                    &battery_id_str,
                ])
                .set(Into::<f64>::into(battery_info.temperature_max));
        }

        // temperature_1 (Celsius)
        if *battery_info.temperature_1 > 0.0 {
            self.battery_info
                .with_label_values(&[
                    &BatteryInfoUnit::Celsius.as_label(),
                    &BatteryInfoType::Temperature1.as_label(),
                    &battery_id_str,
                ])
                .set(Into::<f64>::into(battery_info.temperature_1));
        }

        // temperature_2 (Celsius)
        if *battery_info.temperature_2 > 0.0 {
            self.battery_info
                .with_label_values(&[
                    &BatteryInfoUnit::Celsius.as_label(),
                    &BatteryInfoType::Temperature2.as_label(),
                    &battery_id_str,
                ])
                .set(Into::<f64>::into(battery_info.temperature_2));
        }
    }

    pub fn get_battery_info(
        &self,
        smc: &mut Smc,
        metrics: &crate::libs::metrics::Metrics,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let battery_info = smc.battery_info()?;

        tracing::debug!(
            battery_powered = battery_info.battery_powered,
            charging = battery_info.charging,
            ac_present = battery_info.ac_present,
            health_ok = battery_info.health_ok,
            temperature_max = format!("{:.2}", *battery_info.temperature_max),
            temperature_1 = format!("{:.2}", *battery_info.temperature_1),
            temperature_2 = format!("{:.2}", *battery_info.temperature_2),
            "Battery info"
        );

        metrics.set_battery_info(&battery_info);
        Ok(())
    }
}
