use std::sync::Arc;

use can_config_rs::{config::Network, builder::NetworkBuilder};
use errors::Result;

pub mod errors;
mod parser;


pub fn parse_yaml_config_from_file(path : &str) -> Result<Arc<Network>> {
    let src = std::fs::read_to_string(path)?;
    let network = parse_yaml_config(&src)?;
    Ok(network)
}

pub fn parse_yaml_config(src : &str) -> Result<Arc<Network>> {
    println!("[CANZERO-YAML] Begin parsing");
    let mut network_builder = NetworkBuilder::new();

    let docs = yaml_rust::yaml::YamlLoader::load_from_str(src)?;
    let doc = &docs[0];
    //println!("{doc:?}");
    parser::parse_top_level(doc, &mut network_builder)?;
    println!("[CANZERO-YAML] Finished parsing");

    let network = network_builder.build()?;
    Ok(network)
}
