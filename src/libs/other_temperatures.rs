use crate::libs::metrics::SnakeCaseLabel;
use macsmc::Smc;
use std::fmt::Debug;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum OtherTempUnit {
    Celsius,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum OtherTempType {
    MemoryBankProximity,
    MainboardProximity,
    PlatformControllerHubDie,
    Airport,
    AirflowLeft,
    AirflowRight,
    ThunderboltLeft,
    ThunderboltRight,
    Heatpipe1,
    Heatpipe2,
    PalmRest1,
    PalmRest2,
}

impl crate::libs::metrics::Metrics {
    pub fn set_other_temp(&self, value: &macsmc::OtherTemperatures) {
        // memory_bank_proximity
        if *value.memory_bank_proximity > 0.0 {
            self.other_temperatures
                .with_label_values(&[
                    &OtherTempUnit::Celsius.as_label(),
                    &OtherTempType::MemoryBankProximity.as_label(),
                ])
                .set(Into::<f64>::into(value.memory_bank_proximity));
        }

        // mainboard_proximity
        if *value.mainboard_proximity > 0.0 {
            self.other_temperatures
                .with_label_values(&[
                    &OtherTempUnit::Celsius.as_label(),
                    &OtherTempType::MainboardProximity.as_label(),
                ])
                .set(Into::<f64>::into(value.mainboard_proximity));
        }

        // platform_controller_hub_die
        if *value.platform_controller_hub_die > 0.0 {
            self.other_temperatures
                .with_label_values(&[
                    &OtherTempUnit::Celsius.as_label(),
                    &OtherTempType::PlatformControllerHubDie.as_label(),
                ])
                .set(Into::<f64>::into(value.platform_controller_hub_die));
        }

        // airport
        if *value.airport > 0.0 {
            self.other_temperatures
                .with_label_values(&[
                    &OtherTempUnit::Celsius.as_label(),
                    &OtherTempType::Airport.as_label(),
                ])
                .set(Into::<f64>::into(value.airport));
        }

        // airflow_left
        if *value.airflow_left > 0.0 {
            self.other_temperatures
                .with_label_values(&[
                    &OtherTempUnit::Celsius.as_label(),
                    &OtherTempType::AirflowLeft.as_label(),
                ])
                .set(Into::<f64>::into(value.airflow_left));
        }

        // airflow_right
        if *value.airflow_right > 0.0 {
            self.other_temperatures
                .with_label_values(&[
                    &OtherTempUnit::Celsius.as_label(),
                    &OtherTempType::AirflowRight.as_label(),
                ])
                .set(Into::<f64>::into(value.airflow_right));
        }

        // thunderbolt_left
        if *value.thunderbolt_left > 0.0 {
            self.other_temperatures
                .with_label_values(&[
                    &OtherTempUnit::Celsius.as_label(),
                    &OtherTempType::ThunderboltLeft.as_label(),
                ])
                .set(Into::<f64>::into(value.thunderbolt_left));
        }

        // thunderbolt_right
        if *value.thunderbolt_right > 0.0 {
            self.other_temperatures
                .with_label_values(&[
                    &OtherTempUnit::Celsius.as_label(),
                    &OtherTempType::ThunderboltRight.as_label(),
                ])
                .set(Into::<f64>::into(value.thunderbolt_right));
        }

        // heatpipe_1
        if *value.heatpipe_1 > 0.0 {
            self.other_temperatures
                .with_label_values(&[
                    &OtherTempUnit::Celsius.as_label(),
                    &OtherTempType::Heatpipe1.as_label(),
                ])
                .set(Into::<f64>::into(value.heatpipe_1));
        }

        // heatpipe_2
        if *value.heatpipe_2 > 0.0 {
            self.other_temperatures
                .with_label_values(&[
                    &OtherTempUnit::Celsius.as_label(),
                    &OtherTempType::Heatpipe2.as_label(),
                ])
                .set(Into::<f64>::into(value.heatpipe_2));
        }

        // palm_rest_1
        if *value.palm_rest_1 > 0.0 {
            self.other_temperatures
                .with_label_values(&[
                    &OtherTempUnit::Celsius.as_label(),
                    &OtherTempType::PalmRest1.as_label(),
                ])
                .set(Into::<f64>::into(value.palm_rest_1));
        }

        // palm_rest_2
        if *value.palm_rest_2 > 0.0 {
            self.other_temperatures
                .with_label_values(&[
                    &OtherTempUnit::Celsius.as_label(),
                    &OtherTempType::PalmRest2.as_label(),
                ])
                .set(Into::<f64>::into(value.palm_rest_2));
        }
    }

    pub fn get_other_temp(
        &self,
        smc: &mut Smc,
        metrics: &crate::libs::metrics::Metrics,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let other_temps = smc.other_temperatures()?;

        tracing::debug!(
            memory_bank_proximity = format!("{:.2}", *other_temps.memory_bank_proximity),
            mainboard_proximity = format!("{:.2}", *other_temps.mainboard_proximity),
            platform_controller_hub_die =
                format!("{:.2}", *other_temps.platform_controller_hub_die),
            airport = format!("{:.2}", *other_temps.airport),
            airflow_left = format!("{:.2}", *other_temps.airflow_left),
            airflow_right = format!("{:.2}", *other_temps.airflow_right),
            thunderbolt_left = format!("{:.2}", *other_temps.thunderbolt_left),
            thunderbolt_right = format!("{:.2}", *other_temps.thunderbolt_right),
            heatpipe_1 = format!("{:.2}", *other_temps.heatpipe_1),
            heatpipe_2 = format!("{:.2}", *other_temps.heatpipe_2),
            palm_rest_1 = format!("{:.2}", *other_temps.palm_rest_1),
            palm_rest_2 = format!("{:.2}", *other_temps.palm_rest_2),
            "Other temperatures"
        );

        metrics.set_other_temp(&other_temps);
        Ok(())
    }
}
