use crate::libs::metrics::SnakeCaseLabel;
use macsmc::Smc;
use std::fmt::Debug;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum BatteryDetailUnit {
    Cycles,
    MilliAmpereHours,
    MilliAmpere,
    Volts,
    Watts,
    Percent,
    Seconds,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum BatteryDetailType {
    Cycles,
    CurrentCapacity,
    FullCapacity,
    Amperage,
    Voltage,
    Power,
    Percentage,
    TimeRemaining,
    TimeUntilFull,
}

impl crate::libs::metrics::Metrics {
    pub fn set_battery_detail(&self, battery_detail: &macsmc::BatteryDetail, battery_id: usize) {
        let battery_id_str = battery_id.to_string();

        // cycles (u32)
        self.battery_detail
            .with_label_values(&[
                &BatteryDetailUnit::Cycles.as_label(),
                &BatteryDetailType::Cycles.as_label(),
                &battery_id_str,
            ])
            .set(battery_detail.cycles as f64);

        // current_capacity (MilliAmpereHours)
        self.battery_detail
            .with_label_values(&[
                &BatteryDetailUnit::MilliAmpereHours.as_label(),
                &BatteryDetailType::CurrentCapacity.as_label(),
                &battery_id_str,
            ])
            .set(Into::<f64>::into(*battery_detail.current_capacity));

        // full_capacity (MilliAmpereHours)
        self.battery_detail
            .with_label_values(&[
                &BatteryDetailUnit::MilliAmpereHours.as_label(),
                &BatteryDetailType::FullCapacity.as_label(),
                &battery_id_str,
            ])
            .set(Into::<f64>::into(*battery_detail.full_capacity));

        // amperage (MilliAmpere)
        self.battery_detail
            .with_label_values(&[
                &BatteryDetailUnit::MilliAmpere.as_label(),
                &BatteryDetailType::Amperage.as_label(),
                &battery_id_str,
            ])
            .set(Into::<f64>::into(*battery_detail.amperage));

        // voltage (Volt)
        self.battery_detail
            .with_label_values(&[
                &BatteryDetailUnit::Volts.as_label(),
                &BatteryDetailType::Voltage.as_label(),
                &battery_id_str,
            ])
            .set(Into::<f64>::into(*battery_detail.voltage));

        // power (Watt)
        self.battery_detail
            .with_label_values(&[
                &BatteryDetailUnit::Watts.as_label(),
                &BatteryDetailType::Power.as_label(),
                &battery_id_str,
            ])
            .set(Into::<f64>::into(*battery_detail.power));

        // percentage (calculated method)
        self.battery_detail
            .with_label_values(&[
                &BatteryDetailUnit::Percent.as_label(),
                &BatteryDetailType::Percentage.as_label(),
                &battery_id_str,
            ])
            .set(battery_detail.percentage() as f64);

        // time_remaining (calculated method) - if available
        if let Some(time_remaining) = battery_detail.time_remaining() {
            self.battery_detail
                .with_label_values(&[
                    &BatteryDetailUnit::Seconds.as_label(),
                    &BatteryDetailType::TimeRemaining.as_label(),
                    &battery_id_str,
                ])
                .set(time_remaining.as_secs() as f64);
        }

        // time_until_full (calculated method) - if available
        if let Some(time_until_full) = battery_detail.time_until_full() {
            self.battery_detail
                .with_label_values(&[
                    &BatteryDetailUnit::Seconds.as_label(),
                    &BatteryDetailType::TimeUntilFull.as_label(),
                    &battery_id_str,
                ])
                .set(time_until_full.as_secs() as f64);
        }
    }

    pub fn get_battery_detail(
        &self,
        smc: &mut Smc,
        metrics: &crate::libs::metrics::Metrics,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Get battery details iterator and process all batteries
        let mut battery_details = smc.battery_details()?;
        let mut battery_id = 0;
        let mut battery_info = Vec::new();

        // Collect all battery details
        while let Some(battery_detail_result) = battery_details.next() {
            let battery_detail = battery_detail_result?;

            battery_info.push((
                battery_id,
                battery_detail.cycles,
                *battery_detail.current_capacity,
                *battery_detail.full_capacity,
                *battery_detail.amperage,
                *battery_detail.voltage,
                *battery_detail.power,
                battery_detail.percentage(),
            ));

            metrics.set_battery_detail(&battery_detail, battery_id);
            battery_id += 1;
        }

        // Log all battery details in a structured way
        if !battery_info.is_empty() {
            let mut debug_fields = vec![];
            for (id, cycles, current_cap, full_cap, amperage, voltage, power, percentage) in
                &battery_info
            {
                debug_fields.push(format!(
                    "battery_{} = {{cycles: {}, current_cap: {:.0}, full_cap: {:.0}, amperage: {:.0}, voltage: {:.2}, power: {:.2}, percentage: {:.2}}}",
                    id, cycles, current_cap, full_cap, amperage, voltage, power, percentage
                ));
            }

            tracing::debug!(
                battery_count = battery_info.len(),
                batteries = debug_fields.join(", "),
                "Battery details"
            );
        }

        Ok(())
    }
}
