mod system;

use std::io::BufRead;

use system::System;

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

            if let Err(e) = serde_json::from_str::<System>(&l[range]) {
                eprintln!("{}", &l);
                eprintln!("Line {}: {}", idx, e);
                std::process::exit(1);
            }
        });
}
