use can_yaml_config_rs::parse_yaml_config_from_file;

mod errors;
mod parser;

fn main() {
    let network = parse_yaml_config_from_file("test.yaml").unwrap();
    println!("{network}");
}
