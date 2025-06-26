use macsmc::Smc;

mod libs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    libs::logger::Logger::init("text".to_string());

    let mut smc = Smc::connect()?;

    let metrics = libs::prometheus::init().await?;

    libs::webserver::init().await?;

    get_temp(&mut smc, &metrics)?;

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        get_temp(&mut smc, &metrics)?;
    }
}

fn get_temp(
    smc: &mut Smc,
    metrics: &libs::metrics::Metrics,
) -> Result<(), Box<dyn std::error::Error>> {
    let cpu_temp = smc.cpu_temperature()?;
    // tracing::debug!("temperature: {:#?}", cpu_temp);

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
