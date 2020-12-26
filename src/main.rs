use std::collections::HashMap;
use std::fs;

use serde_derive::{Serialize, Deserialize};
use unifac::{calc, FunctionalGroup, Substance};

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
    run();
}

fn run() -> Result<(), &'static str> {
    let filecontent = fs::read_to_string("assets/demo.yaml").unwrap();
    let content: YamlBody = serde_yaml::from_str(&filecontent).unwrap();

    let substances = content.substances.iter().map(|(n, s)| {
        let g = s.groups.iter().map(|g| {
            let data: Vec<&str> = g.split(":").collect();
            let id = str::parse::<u8>(data[0]).unwrap();
            let count = str::parse::<f64>(data[1]).unwrap();
            FunctionalGroup::from(id, count)
        }).collect::<Result<Vec<FunctionalGroup>, &'static str>>()?;
        Ok(Substance {
            fraction: s.fraction,
            functional_groups: g,
            gamma: None,
        })
    }).collect::<Result<Vec<Substance>, &'static str>>()?;

    let mix = calc(substances, content.temperature)?;
    println!("{:?}", mix);
    
    return Ok(());
}