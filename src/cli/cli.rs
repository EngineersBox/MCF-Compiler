use clap::App;

const SCHEMA_DIR: &'static str = "cli_schema.yml";

#[derive(Debug)]
pub struct CLI {
    matches: clap::ArgMatches<'static>
}

impl CLI {
    pub fn new() -> Self {
        Self {
            matches: App::from_yaml(load_yaml!("cli_schema.yml")).get_matches()
        }
    }
}