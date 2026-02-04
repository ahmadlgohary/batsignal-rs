use std::collections::HashSet;
use std::{thread, time::Duration};
use notifications::create_notification_id;

extern crate battery;
mod config;
mod battery_monitor;
mod notifications;
mod audio;

fn main() {
    let _configuration = config::Config::parse_json();
    let _notif_id = create_notification_id();
    let mut sent_levels: HashSet<u8> = HashSet::new();
    
    let manager = match battery_monitor::get_battery_manager() {
        Some(manager) => manager,
        None => return,
    };
    
    let mut battery = match battery_monitor::get_battery(&manager) {
        Some(battery) => battery,
        None => return,
    };

    let mut battery_stats = match battery_monitor::BatteryStats::new(&manager, &mut battery) {
        Some(battery_stats) => battery_stats,
        None => return,
    };
    
    loop {
            battery_stats.get_battery_stats(&manager, &mut battery);
            println!("{:?}", battery_stats);
            battery_stats.handle_charger_notifications(_notif_id, &_configuration);
            battery_stats.handle_battery_state_change(&mut sent_levels);
            battery_stats.handle_battery(_notif_id, &_configuration, &mut sent_levels);
            thread::sleep(Duration::from_secs(1));
        }
}