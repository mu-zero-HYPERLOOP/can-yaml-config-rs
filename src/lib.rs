use std::{rc::Rc, sync::Arc};

use can_config_rs::config::{Network, NetworkBuilder};
use errors::Result;

mod errors;
mod parser;


pub fn parse_yaml_config_from_file(path : &str) -> Result<Arc<Network>> {
    let src = std::fs::read_to_string(path)?;
    let network = parse_yaml_config(&src)?;
    Ok(network)
}

pub fn parse_yaml_config(src : &str) -> Result<Arc<Network>> {
    let mut network_builder = NetworkBuilder::new();

    let docs = yaml_rust::yaml::YamlLoader::load_from_str(src)?;
    let doc = &docs[0];
    println!("{doc:?}");
    parser::parse_top_level(doc, &mut network_builder)?;


    let network = network_builder.build()?;
    Ok(network)
}
