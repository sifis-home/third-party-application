use sifis_api::Sifis;

#[tokio::main]
async fn main() -> Result<(), sifis_api::Error> {
    // Creation of SIFIS-HOME API context
    let sifis = Sifis::new().await?;
    // Look for every fridges in an environment
    let fridges = sifis.fridges().await?;

    for fridge in fridges {
        let is_open: &str = if fridge.is_open().await? {
            "Open"
        } else {
            "Closed"
        };
        let temperature = fridge.temperature().await?;
        let target_temperature = fridge.target_temperature().await?;
        println!(
            "{:<15} {:<8} {:<5} {:<5} ",
            fridge.id, is_open, temperature, target_temperature
        );
        fridge.set_target_temperature(5).await?;
    }

    Ok(())
}
