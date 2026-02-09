use macsmc::Smc;

mod libs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    libs::logger::Logger::init("text".to_string());

    let mut smc = Smc::connect()?;

    let metrics = libs::prometheus::init().await?;

    libs::webserver::init().await?;

    loop {
        metrics.get_cpu_temp(&mut smc, &metrics)?;
        metrics.get_cpu_power(&mut smc, &metrics)?;
        metrics.get_cpu_core_temps(&mut smc, &metrics)?;
        metrics.get_battery_detail(&mut smc, &metrics)?;
        metrics.get_battery_info(&mut smc, &metrics)?;
        metrics.get_fan_speeds(&mut smc, &metrics)?;
        metrics.get_gpu_temp(&mut smc, &metrics)?;
        metrics.get_other_temp(&mut smc, &metrics)?;

        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }
}
