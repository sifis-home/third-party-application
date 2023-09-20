use sifis_api::Sifis;

#[tokio::main]
async fn main() -> Result<(), sifis_api::Error> {
    // Creation of SIFIS-HOME API context
    let sifis = Sifis::new().await?;
    // Look for every sinks in an environment
    let sinks = sifis.sinks().await?;

    for sink in sinks {
        let flow = sink.get_flow().await?;
        let temperature = sink.get_temperature().await?;
        println!("{:<15} {:<7} {:<5} ", sink.id, flow, temperature);
        sink.open_drain().await?;
        sink.set_flow(0).await?;
    }

    Ok(())
}
