use std::collections::HashMap;
use std::fs;

use serde_derive::{Deserialize, Serialize};
use unifac::{calc, FunctionalGroup, Substance};

use clap::{App, Arg};

#[derive(Debug, Serialize, Deserialize)]
struct YamlBody {
    temperature: f64,
    substances: HashMap<String, YamlSubstance>,
}

#[derive(Debug, Serialize, Deserialize)]
struct YamlSubstance {
    fraction: f64,
    groups: Vec<String>,
}

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("CLI for the UNIFAC library")
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("Specifies output file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("input")
                .help("Specifies input file")
                .required(true)
                .index(1),
        )
        .get_matches();

    let filecontent = match fs::read_to_string(matches.value_of("input").unwrap()) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Input file could not be read");
            return;
        }
    };
    let output = match run(&filecontent) {
        Ok(o) => o,
        Err(s) => {
            eprintln!("{}\n", s);
            return;
        }
    };
    if matches.is_present("output") {
        match fs::write(matches.value_of("output").unwrap(), output) {
            Ok(_) => return,
            Err(_) => eprintln!("The file could not be written"),
        };
    } else {
        println!("{}", output);
    }
}

fn run(yaml_str: &str) -> Result<String, &'static str> {
    let content: YamlBody = match serde_yaml::from_str(&yaml_str) {
        Ok(c) => c,
        Err(_) => return Err("Invalid syntax in input file!"),
    };

    let substances = content
        .substances
        .iter()
        .map(|(name, substance)| {
            let groups = substance
                .groups
                .iter()
                .map(|group| {
                    let data: Vec<&str> = group.split(":").collect();
                    let id = match str::parse::<u8>(data[0]) {
                        Ok(i) => i,
                        Err(_) => return Err("Invalid syntax in input file!"),
                    };
                    let count = match str::parse::<f64>(data[1]) {
                        Ok(i) => i,
                        Err(_) => return Err("Invalid syntax in input file!"),
                    };
                    FunctionalGroup::from(id, count)
                })
                .collect::<Result<Vec<FunctionalGroup>, &'static str>>()?;
            Ok(Substance::from_name(name, substance.fraction, groups))
        })
        .collect::<Result<Vec<Substance>, &'static str>>()?;

    let mix = calc(substances, content.temperature)?;
    let yaml_string = serde_yaml::to_string(&mix).unwrap();

    return Ok(yaml_string);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_and_calc() {
        let yaml = "---
temperature: 298
substances:
  ethanole:
    fraction: 0.5
    groups:
      - \"1:2\"
      - \"2:1\"
      - \"14:1\"
  benzene:
    fraction: 0.5
    groups:
      - \"9:6\"
        ";

        let res = run(yaml);
        assert!(res.is_ok());
    }
}
