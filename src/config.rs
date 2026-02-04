use std::{collections::BTreeMap, fs};
use serde_json;
use serde::Deserialize;
use notify_rust::Urgency;

#[derive(Debug, Deserialize)]
pub struct Config {
    notification_time: Option<u64>,

    pub(crate) high_battery_levels: Option<BTreeMap<u8, BatteryNotification>>,

    pub(crate) low_battery_levels: Option<BTreeMap<u8, BatteryNotification>>,
    
    charger_notifications: Option<ChargerNotification>
}

#[derive(Debug, Deserialize)]
pub struct BatteryNotification{
    message: String,
    notification_icon: Option<String>,
    notification_sound: Option<String>,
    urgent_level: Option<String>
} 

#[derive(Debug, Deserialize)]
pub struct ChargerNotification {
    charging: Option<bool>,
    plugged_sound: Option<String>,
    charging_icon: Option<String>,
    discharging: Option<bool>,
    unplugged_sound: Option<String>,
    discharging_icon: Option<String>,
    urgent_level: Option<String>
}

impl Config {
    pub fn parse_json() -> Config {
        let file: String = fs::read_to_string("config.json").expect("Failed to open file");
        let config: Config = serde_json::from_str(&file).unwrap();
        config
    }
}

impl BatteryNotification {
    pub fn get_message(self: &Self) -> &str {
        self.message.as_str()
    }
    
    pub fn get_icon(self: &Self) -> &str {
        if let Some(icon) = &self.notification_icon {
            return icon.as_str();
        }
        return "";
    }
    
    pub fn get_sound(self: &Self) -> &str {
        if let Some(sound) = &self.notification_sound {
            return sound.as_str();
        }
        return "";
    }
    
    pub fn get_urgency(self: &Self) -> Urgency {
        if let Some(urgent_level) = &self.urgent_level {
           match urgent_level.as_str().trim().try_into() {
                Ok(urgent) => return urgent,
                Err(_) => { 
                    eprint!("Unsupported Urgency Level.\t Resorting to Normal");
                    return Urgency::Normal;
                }
           }
        }
        return Urgency::Normal;
    }
}
