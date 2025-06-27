use macsmc::Smc;

mod libs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    libs::logger::Logger::init("text".to_string());

    let mut smc = Smc::connect()?;

    let metrics = libs::prometheus::init().await?;

    libs::webserver::init().await?;

    metrics.get_cpu_temp(&mut smc, &metrics)?;

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        metrics.get_cpu_temp(&mut smc, &metrics)?;
    }
}
