use sifis_api::{Lamp, Sifis};

async fn check_lamp(lamp: &Lamp<'_>) -> Result<(), sifis_api::Error> {
    // Get lamp state, whether it is on or off
    let on_off: &str = if lamp.get_on_off().await? {
        "On"
    } else {
        "Off"
    };
    // Get brightness
    let brightness: u8 = lamp.get_brightness().await?;
    // Print lamp identifier, on/off state, and brightness
    println!(
        "{:<15} Status {:<7} Brightness {:<5} ",
        lamp.id, on_off, brightness
    );
    // Turn on the lamp
    lamp.turn_on().await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), sifis_api::Error> {
    // Creation of SIFIS-HOME API context
    let sifis = Sifis::new().await?;
    // Look for every lamps in an environment
    let lamps = sifis.lamps().await?;

    for lamp in lamps {
        check_lamp(&lamp)
            .await
            .unwrap_or_else(|_| println!("{lamp} Access Not Allowed"));
    }

    Ok(())
}
