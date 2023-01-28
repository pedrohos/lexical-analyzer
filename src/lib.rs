pub mod common;
pub mod core;

use std::error::Error;

use common::config;
use common::helpers;

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    let data = helpers::read_from_file_path(&config.file_path);
    Ok(())
}