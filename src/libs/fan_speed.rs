use crate::libs::metrics::SnakeCaseLabel;
use macsmc::Smc;
use std::fmt::Debug;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum FanSpeedUnit {
    Rpm,
    Mode,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum FanSpeedType {
    Actual,
    Min,
    Max,
    Target,
    Safe,
    Mode,
}

impl crate::libs::metrics::Metrics {
    pub fn set_fan_speed(&self, fan_speed: &macsmc::FanSpeed, fan_id: usize) {
        let fan_id_str = fan_id.to_string();

        // actual (Rpm) - if available
        if *fan_speed.actual > 0.0 {
            self.fan_speed
                .with_label_values(&[
                    &FanSpeedUnit::Rpm.as_label(),
                    &FanSpeedType::Actual.as_label(),
                    &fan_id_str,
                ])
                .set(Into::<f64>::into(fan_speed.actual));
        }

        // min (Rpm) - if available
        if *fan_speed.min > 0.0 {
            self.fan_speed
                .with_label_values(&[
                    &FanSpeedUnit::Rpm.as_label(),
                    &FanSpeedType::Min.as_label(),
                    &fan_id_str,
                ])
                .set(Into::<f64>::into(fan_speed.min));
        }

        // max (Rpm) - if available
        if *fan_speed.max > 0.0 {
            self.fan_speed
                .with_label_values(&[
                    &FanSpeedUnit::Rpm.as_label(),
                    &FanSpeedType::Max.as_label(),
                    &fan_id_str,
                ])
                .set(Into::<f64>::into(fan_speed.max));
        }

        // target (Rpm) - if available
        if *fan_speed.target > 0.0 {
            self.fan_speed
                .with_label_values(&[
                    &FanSpeedUnit::Rpm.as_label(),
                    &FanSpeedType::Target.as_label(),
                    &fan_id_str,
                ])
                .set(Into::<f64>::into(fan_speed.target));
        }

        // safe (Rpm) - if available
        if *fan_speed.safe > 0.0 {
            self.fan_speed
                .with_label_values(&[
                    &FanSpeedUnit::Rpm.as_label(),
                    &FanSpeedType::Safe.as_label(),
                    &fan_id_str,
                ])
                .set(Into::<f64>::into(fan_speed.safe));
        }

        // mode (FanMode as numeric value)
        self.fan_speed
            .with_label_values(&[
                &FanSpeedUnit::Mode.as_label(),
                &FanSpeedType::Mode.as_label(),
                &fan_id_str,
            ])
            .set(match fan_speed.mode {
                macsmc::FanMode::Auto => 0.0,
                macsmc::FanMode::Forced => 1.0,
            });
    }

    pub fn get_fan_speeds(
        &self,
        smc: &mut Smc,
        metrics: &crate::libs::metrics::Metrics,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut fans = smc.fans()?;
        let mut fan_id = 0;

        while let Some(fan_speed_result) = fans.next() {
            let fan_speed = fan_speed_result?;
            tracing::debug!(
                fan_id = fan_id,
                actual = format!("{:.0}", *fan_speed.actual),
                min = format!("{:.0}", *fan_speed.min),
                max = format!("{:.0}", *fan_speed.max),
                target = format!("{:.0}", *fan_speed.target),
                safe = format!("{:.0}", *fan_speed.safe),
                mode = format!("{:?}", fan_speed.mode),
                "Fan speed"
            );

            metrics.set_fan_speed(&fan_speed, fan_id);
            fan_id += 1;
        }

        Ok(())
    }
}
