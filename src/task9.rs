use anyhow::{anyhow, Result};
use std::{fs::File, io::BufReader};
use xml_parser::parse_duration;

use xml::{reader::XmlEvent, EventReader};

fn main() -> Result<()> {
    let mut duration_sum = 0.;
    let mut duration_count = 0;

    let file = File::open("rezepte.xml")?;
    let file = BufReader::new(file);

    let parser = EventReader::new(file);
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement {
                name, attributes, ..
            }) => {
                if name.local_name != "zubereitung" {
                    continue;
                }

                let Ok(duration): Result<f64> = attributes
                    .iter()
                    .filter(|x| x.name.local_name == "dauer")
                    .map(|x| parse_duration(&x.value))
                    .sum()
                else {
                    continue;
                };

                duration_sum += duration;
                duration_count += 1;
            }
            Err(e) => {
                Err(anyhow!("Error: {e}"))?;
            }
            _ => (),
        }
    }

    println!(
        "average duration {} s",
        duration_sum / duration_count as f64
    );

    Ok(())
}
