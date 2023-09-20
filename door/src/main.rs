use sifis_api::Sifis;

#[tokio::main]
async fn main() -> Result<(), sifis_api::Error> {
    // Creation of SIFIS-HOME API context
    let sifis = Sifis::new().await?;
    let doors = sifis.doors().await?;

    for door in doors {
        let is_open = door.is_open().await?;
        let jammed = door.lock_status().await?;
        println!("{:<15} {:<7} {:<5} ", door.id, is_open, jammed);
    }

    Ok(())
}
