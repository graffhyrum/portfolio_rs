pub(crate) fn get_version() -> String {
    // let toml_string = fs::read_to_string("Cargo.toml")
    //     .expect("Something went wrong reading the file");
    // let table = toml_string.parse::<Table>().expect("Failed to parse TOML");
    // let version = table["package"]["version"].as_str().expect("Failed to get version");
    // version.to_string()
    env!("CARGO_PKG_VERSION").to_string()
}