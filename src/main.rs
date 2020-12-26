use std::collections::HashMap;
use std::fs;

use serde_derive::{Serialize, Deserialize};

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

fn main()  {
    let filecontent = fs::read_to_string("assets/demo.yaml").unwrap();
    let content: YamlBody = serde_yaml::from_str(&filecontent).unwrap();
    println!("Temperature: {:?}", content.temperature);
    for (name, substance) in content.substances {
        println!("{} with {} groups, fraction: {}", name, substance.groups.len(), substance.fraction);
        for group in substance.groups {
            let g: Vec<&str> = group.split(":").collect();
            let id = str::parse::<usize>(g[0]).unwrap();
            let count = str::parse::<usize>(g[1]).unwrap();
            println!("\tGroup {:?}: {:?} times", id, count);
        }
    }
}