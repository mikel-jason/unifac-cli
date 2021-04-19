#![feature(test)]

use std::collections::HashMap;
use std::fs;

use serde_derive::{Deserialize, Serialize};
use unifac::{calc, FunctionalGroup, Substance};

use clap::{App, Arg};

#[derive(Debug, Serialize, Deserialize)]
struct YamlBody<T> {
    temperature: f64,
    substances: HashMap<String, T>,
}

#[derive(Debug, Serialize, Deserialize)]
struct YamlBenchBody<T> {
    temperature: f64,
    substances: HashMap<String, T>,
    difftemp: Vec<String>,
    fractions: Vec<Vec<f64>>,
}

#[derive(Debug, Deserialize)]
struct YamlSubstance {
    fraction: f64,
    groups: Vec<String>,
}

#[derive(Debug, Serialize)]
struct YamlOutSubstance {
    fraction: f64,
    groups: Vec<String>,
    gamma: f64,
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
            Arg::with_name("benchmark")
                .short("b")
                .long("benchmark")
                .help("Runs benchmark"),
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
    let output: String;
    if matches.is_present("benchmark") {
        output = match run_benchmark(&filecontent) {
            Ok(o) => o,
            Err(s) => {
                eprintln!("{}\n", s);
                return;
            }
        }
    } else {
        output = match run(&filecontent) {
            Ok(o) => o,
            Err(s) => {
                eprintln!("{}\n", s);
                return;
            }
        };
    }
    if matches.is_present("output") {
        match fs::write(matches.value_of("output").unwrap(), output) {
            Ok(_) => return,
            Err(_) => eprintln!("The file could not be written"),
        };
    } else {
        println!("{}", output);
    }
}

fn run_benchmark(yaml_str: &str) -> Result<String, String> {
    let content: YamlBenchBody<YamlSubstance> = match serde_yaml::from_str(&yaml_str) {
        Ok(c) => c,
        Err(_) => return Err(String::from("Invalid syntax in input file!")),
    };

    let mut result = String::new();
    let mut inputs: Vec<(Vec<Substance>, f64)> = Vec::new();

    for i in 0..10000 {
        let mut substances = Vec::new();
        let mut j = 0;
        for name in content.substances.keys() {
            let groups = content.substances[name]
                .groups
                .iter()
                .map(|group| {
                    let data: Vec<&str> = group.split(":").collect();
                    let id = match str::parse::<u8>(data[0]) {
                        Ok(i) => i,
                        Err(_) => {
                            return Err(format!("Error parsing groups of substance {}", name))
                        }
                    };
                    let count = match str::parse::<f64>(data[1]) {
                        Ok(i) => i,
                        Err(_) => {
                            return Err(format!("Error parsing groups of substance {}", name))
                        }
                    };
                    match FunctionalGroup::from(id, count) {
                        Ok(c) => Ok(c),
                        Err(s) => Err(String::from(s)),
                    }
                })
                .collect::<Result<Vec<FunctionalGroup>, String>>()?;
            substances.push(Substance::from_name(
                name,
                content.fractions[i % 1000][j],
                groups,
            ));

            j += 1;
        }
        let temp = content.temperature + content.difftemp[i % 1000].parse::<f64>().unwrap();
        inputs.push((substances, temp));
    }

    for i in 0..1000 {
        let start = std::time::Instant::now();
        for j in 0..10_000 {
            std::hint::black_box(
                calc(inputs[j].0.clone(), inputs[j].1).expect("Something went terribly wrong!"),
            );
        }
        let elapsed = start.elapsed();
        let timing = elapsed.as_millis();

        result += &format!("{}, {}\n", i, timing);
        println!("{}, {}", i, timing);
    }

    Ok(result)
}

fn run(yaml_str: &str) -> Result<String, String> {
    let content: YamlBody<YamlSubstance> = match serde_yaml::from_str(&yaml_str) {
        Ok(c) => c,
        Err(_) => return Err(String::from("Invalid syntax in input file!")),
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
                        Err(_) => {
                            return Err(format!("Error parsing groups of substance {}", name))
                        }
                    };
                    let count = match str::parse::<f64>(data[1]) {
                        Ok(i) => i,
                        Err(_) => {
                            return Err(format!("Error parsing groups of substance {}", name))
                        }
                    };
                    match FunctionalGroup::from(id, count) {
                        Ok(c) => Ok(c),
                        Err(s) => Err(String::from(s)),
                    }
                })
                .collect::<Result<Vec<FunctionalGroup>, String>>()?;
            Ok(Substance::from_name(name, substance.fraction, groups))
        })
        .collect::<Result<Vec<Substance>, String>>()?;

    let mix = calc(substances, content.temperature)?;

    let substance_map = mix
        .iter()
        .map(|substance| {
            let groups = substance
                .functional_groups
                .iter()
                .map(|group| {
                    let group_str = format!("{}:{}", group.id, group.nu);
                    return String::from(group_str);
                })
                .collect::<Vec<String>>();
            return (
                substance.name.clone(),
                YamlOutSubstance {
                    fraction: substance.fraction,
                    groups,
                    gamma: substance.gamma.unwrap(),
                },
            );
        })
        .collect::<HashMap<String, YamlOutSubstance>>();

    let yamlbody = YamlBody {
        temperature: content.temperature,
        substances: substance_map,
    };

    let yaml_string = match serde_yaml::to_string(&yamlbody) {
        Ok(c) => c,
        Err(_) => return Err(String::from("Could not parse result")),
    };

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
