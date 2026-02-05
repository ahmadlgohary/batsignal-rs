/* *
 * IMPORTANT
 * Tests should run in a single thread!
 * After a few hours of debugging race conditions and adding a global lock,
 * I realised this was just a more complex way of doing single threading.
 * Majority of the tests will be testing if notifiations are sent correctly,
 * a global lock would result in them running sequentially anyways 
 * */

#![cfg(test)]
use std::sync::Mutex;

use notify_rust::Urgency;
use rstest::{fixture, rstest};

use crate::{battery_monitor::BatteryStats, config::Config};

mod test_battery_monitors;


/* *
 * This would simulate a notification server recieving notifications    
 *
 * Rust forbids mut statics, since they can result in undefined behavior
 * and the compiler cannot guarantee that two references to it are never active at the same time
 * Wrapping it in a mutex ensures only 1 mut ref exits at a time 
 * */
static TEST_CALLS: Mutex<Vec<String>> = Mutex::new(Vec::new());

#[fixture]
fn clear_calls() {
    TEST_CALLS.lock().unwrap().clear();
}

fn get_calls() -> Vec<String> {
    TEST_CALLS.lock().unwrap().clone()
}

impl Config {
    pub fn parse_toml_from_str(config_str: &str) -> Result<Self, toml::de::Error>{
        toml::from_str(&config_str)
    }
}

impl BatteryStats {
    fn parse_toml_from_str(battery_str: &str) -> Result<Self, toml::de::Error>{
        toml::from_str(&battery_str)
    }
}



pub fn send_battery_notification(
        battery_level: &i32, 
        _notif_message: &str,
        _notif_icon: &str,
        _notif_urgency: Urgency,
        _notif_sound: &str,
        _time: i32
    ){
       TEST_CALLS
            .lock()
            .unwrap()
            // make sure we send at the correct battery level
            .push(format!("{battery_level}" ));
}


pub fn send_charger_notification(
        charging_state: &str,
        _battery_level: &i32,
        _notif_icon: &str, 
        _notif_urgency: Urgency,
        _notif_sound: &str,
        _time: i32
    ) {
       TEST_CALLS
            .lock()
            .unwrap()
            // make sure we send at the correct battery state 
            .push(format!("{charging_state}"));
} 

