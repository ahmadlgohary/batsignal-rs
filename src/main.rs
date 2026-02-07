use std::collections::HashSet;
use std::{thread, time::Duration};

use crate::cli_args::{parse_cli};
use crate::config_file_manager::{create_default_config_file, handle_config_file, print_config_toml_with_message};

extern crate battery;
mod config;
mod config_file_manager;
mod cli_args;
mod battery_monitor;
mod notifications;
mod audio;
mod tests;

/*
 * batsignal-rs [flags] [args]
 * -h, --help
 * -V, --version
 *
 * -c, --config <file>
 *   , --print-config
 *   , --print-config-template
 *   , --create-config creates a config template to ~/.config/batsignal-rs
 *
 * if no flags are passed 
 *      the config file in ~/.config/batsignal-rs is used
 * if no config file is found
 *       a default config is used
 *
*/
fn main() {
    let cli_arguments = parse_cli();   
    
    if cli_arguments.print_config_template {
        print_config_toml_with_message(&config::Config::default(),
            "Printing Config File Template").ok();
        return;
    }
    
    if cli_arguments.create_config {
        create_default_config_file(&cli_arguments.config_path);
        return;
    }
    
    let configuration = handle_config_file(&cli_arguments.config_path);
    
    if cli_arguments.print_config {
        print_config_toml_with_message(&configuration,
            "Printing Current Config File").ok();
        return;
    }

    let mut battery_notif_sent: HashSet<u8> = HashSet::new();
    let notif_time = configuration.time();
    let charger_notif = &configuration.charger_notifications;
    let low_level_notifs = &configuration.low_battery_levels;
    let high_level_notifs = &configuration.high_battery_levels;

    let manager = match battery_monitor::init_battery_manager() {
        Some(manager) => manager,
        None => return,
    };
    
    let mut battery = match battery_monitor::init_battery(&manager) {
        Some(battery) => battery,
        None => return,
    };

    let mut battery_stats = match battery_monitor::BatteryStats::new(&manager, &mut battery) {
        Some(battery_stats) => battery_stats,
        None => return,
    };
    
    loop {
            battery_stats.update_battery_stats(&manager, &mut battery);
            println!("{:?}", battery_stats);
            battery_stats.handle_charger_notifications(charger_notif, notif_time);
            battery_stats.handle_battery_state_change(&mut battery_notif_sent);
            battery_stats.handle_battery(low_level_notifs, high_level_notifs, notif_time, &mut battery_notif_sent);
            thread::sleep(Duration::from_secs(1));
    }
}