use clap::App;

const SCHEMA_DIR: &'static str = "cli_yaml.yml";

#[derive(Debug)]
pub struct CLI {
    matches: clap::ArgMatches<'_>
}

impl CLI {
    pub fn new() -> Self {
        Self {
            matches: App::from_yaml(load_yaml!(SCHEMA_DIR)).get_matches()
        }
    }
}