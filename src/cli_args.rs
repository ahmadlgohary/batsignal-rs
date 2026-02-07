use clap::{Arg, Command};

use crate::config_file_manager::get_config_dir;

#[derive(Debug)]
pub struct CliArgs {
    pub config_path: String,
    pub print_config: bool,
    pub print_config_template: bool,
    pub create_config: bool,
}

pub fn parse_cli() -> CliArgs {
    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about("A lightweight battery notification daemon in rust")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .num_args(1)  
                .help("Path to config file"),
        )
        .arg(
            Arg::new("print-config")
                .long("print-config")
                .help("Print the resolved configuration and exit")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
           Arg::new("print-config-template")
                .long("print-config-template")
                .help("Print the config template")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
           Arg::new("create-config")
                .long("create-config")
                .help("Create a config file at '~/.config/batsignal-rs', if one does not exit")
                .action(clap::ArgAction::SetTrue)
        )
        .get_matches();

    CliArgs {
        config_path: matches.get_one::<String>("config").cloned().unwrap_or(get_config_dir()),
        print_config: *matches.get_one::<bool>("print-config").unwrap_or(&false),
        print_config_template: *matches.get_one::<bool>("print-config-template").unwrap_or(&false),
        create_config: *matches.get_one::<bool>("create-config").unwrap_or(&false),
    }
}