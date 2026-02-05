use std::collections::HashSet;
use std::{thread, time::Duration};

extern crate battery;
mod config;
mod battery_monitor;
mod notifications;
mod audio;

fn main() {
    let configuration = config::Config::parse_toml();
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