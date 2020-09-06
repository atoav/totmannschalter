// (Full example with detailed comments in examples/01a_quick_example.rs)
//
// This example emonstrates clap's "builder pattern" method of creating arguments
// which the most flexible, but also most verbose.
use clap::App;
use env;
use console::style;


pub mod error;
pub use error::{
    ConfigError,
    MailError
};

pub mod config;
pub use config::{
    Config,
    Mail,
    Endpoint,
    get_config_path
};

pub mod supervisor;
pub use supervisor::Supervisor;




fn main() {
    let _matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .get_matches();

    println!("Starting {} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    // Read config from config directory
    let config = match Config::initialize() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{} {}", style(" Error ").on_red(), e);
            std::process::exit(1);
        }
    };
    println!("Read config from \"{}\"", get_config_path().to_string_lossy());
    println!("{} endpoints found in config", config.endpoint.len());

    // println!("{:#?}", config);
    println!("Scheduling Endpoint Requests");
    let supervisor = Supervisor::from_config(config);
    println!("Running Loop");
    println!("-------------");
    supervisor.run_loop();
}