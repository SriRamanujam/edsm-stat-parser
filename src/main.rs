mod system;

use std::io::BufRead;

use system::{body::planet::PlanetType, body::Body, System};

use rayon::prelude::*;

fn main() {
    let f = std::fs::OpenOptions::new()
        .read(true)
        .write(false)
        .truncate(false)
        .open("../galaxy.json")
        .unwrap();

    let reader = std::io::BufReader::new(f);

    // skip the first line
    reader
        .lines()
        .skip(1)
        .take_while(|v| v.as_ref().unwrap().len() > 1)
        .enumerate()
        .par_bridge()
        .for_each(|(idx, line)| {
            let l: String = line.unwrap();

            // we are speculatively parsing the file to find the easy erorrs. give it a slice of all but the last trailing comma
            // to prevent parse errors. Note that the last element will not have a trailing comma, so we have to adjust
            // the range accordingly.
            let range = if l.chars().last().unwrap() == ',' {
                ..l.len() - 1
            } else {
                ..l.len()
            };

            let system = match serde_json::from_str::<System>(&l[range]) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("{}", &l);
                    eprintln!("Line {}: {}", idx, e);
                    std::process::exit(1);
                }
            };

            for body in &system.bodies {
                if let Body::Planet(p) = body {
                    // our conditions:
                    // 1. Gravity between 0.85 and 1.15 G's
                    // 2. Surface temperature between 283 and 293 K
                    // 3. Atmosphere pressure between 0.85 and 1.15 atm
                    // 4. Type is "Earth Like world"

                    match p.r#type {
                        PlanetType::EarthLikeWorld => {
                            if let Some(surface_temp) = p.common.surfaceTemperature {
                                if 283.0 <= surface_temp && surface_temp <= 293.0 {
                                    if let Some(pressure) = p.surfacePressure {
                                        if 0.85 <= pressure && pressure <= 1.15 {
                                            if 0.85 <= p.gravity && p.gravity <= 1.15 {
                                                println!(
                                                    "FOUND: {} in {} system",
                                                    p.common.name, system.name
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        _ => continue,
                    }
                }
            }
        });
}
