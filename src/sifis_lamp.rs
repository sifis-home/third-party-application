use sifis_api::Sifis;

// A context for Sifis lamps in an environment
pub struct Lamps {
    // Sifis-Home context
    context: Sifis,
    // A vector of lamps
    lamps: Vec<Lamp>,
}

// Lamps information contained in Things
pub struct Lamp {
    lamp: sifis_api::Lamp<'static>,
    on_off: bool,
    brightness: u8,
}

impl Lamps {
    // Returns all Sifis-Home lamps in an environment
    pub async fn lamps() -> Result<Self, sifis_api::Error> {
        // Creation of Sifis-Home API context
        let sifis = Sifis::new().await?;
        // Look for every lamps in an environment
        let lamps = sifis.lamps().await?;

        // Create a vector of Sifis-Home lamps
        let mut sifis_lamps = Vec::new();

        // Iterate over each Sifis-Home lamp contained in an environment
        for lamp in lamps {
            // Get a lamp state, whether it is on or off
            let on_off: bool = lamp.get_on_off().await?;
            // Get brightness
            let brightness: u8 = lamp.get_brightness().await?;
            // Save information which are going to be shown in a GUI
            sifis_lamps.push(Lamp {
                lamp,
                on_off,
                brightness,
            });
        }

        Ok(Self {
            context: sifis,
            lamps: sifis_lamps,
        })
    }

    // Turn on a specific lamp, identified by its index
    pub async fn turn_on(&mut self, index: usize) -> Result<bool, sifis_api::Error> {
        // Turn on a lamp
        self.lamps[index].lamp.turn_on().await
    }

    // Turn off a specific lamp, identified by its index
    pub async fn turn_off(&mut self, index: usize) -> Result<bool, sifis_api::Error> {
        // Turn off a lamp
        self.lamps[index].lamp.turn_off().await
    }
}
