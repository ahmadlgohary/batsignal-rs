use serde::{Deserialize, Serialize};
use notify_rust::Urgency;
use std::{collections::BTreeMap};

// ----------------------------------------------------------------
// Configuration Struct and Implementation
// ----------------------------------------------------------------
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    notification_time: Option<i32>,
    pub high_battery_levels: Option<BTreeMap<u8, BatteryNotification>>,
    pub low_battery_levels: Option<BTreeMap<u8, BatteryNotification>>,
    pub charger_notifications: Option<ChargerNotification>
}

/// This function reads a json config file and parses it into the Config Struct
impl Config {

    /// This function creates a default config
    pub fn default() -> Self {
        // The default config, sends a notification only at 20% battery
        let notification_time = Some(5000);
        let low_battery_levels = Some(BTreeMap::from([(20, 
            BatteryNotification { 
                message: "Battery Low".to_string(), 
                notification_icon: Some("".to_string()), 
                notification_sound: Some("".to_string()), 
                urgent_level: Some("".to_string()) 
            })]));

        let high_battery_levels = Some(BTreeMap::from([(100, 
            BatteryNotification { 
                message: "".to_string(), 
                notification_icon: Some("".to_string()), 
                notification_sound: Some("".to_string()), 
                urgent_level: Some("".to_string()) 
            })]));
        
        let charger_notifications = Some (ChargerNotification {
            charging: Some(false),
            plugged_sound: Some("".to_string()),
            charging_icon: Some("".to_string()),
            discharging: Some(false),
            unplugged_sound: Some("".to_string()),
            discharging_icon: Some("".to_string()),
            urgent_level: Some("".to_string())
            });

        Self { notification_time,high_battery_levels, low_battery_levels, charger_notifications }
    }

    /// Getter function to return the time specified in the configuration file
    /// Defaults to 5000 ms (5 seconds)
    pub fn time(&self) -> i32 {
        // default to 5000 ms (5 seconds)
        self.notification_time.unwrap_or(5000)
    }
}

// ----------------------------------------------------------------
// Battery Notification Struct and Implementation
// ----------------------------------------------------------------
#[derive(Debug, Deserialize, Serialize)]
pub struct BatteryNotification{
    message: String,
    notification_icon: Option<String>,
    notification_sound: Option<String>,
    urgent_level: Option<String>
}

/// This implementation defines Getter functions for the fields in the above struct
/// Also deals with option<> types by returning default values
impl BatteryNotification {
    pub fn notification_message(&self) -> &str {
        self.message.as_str()
    }
    
    pub fn notification_icon(&self) -> &str {
        // Defaults to an empty string 
        self.notification_icon.as_deref().unwrap_or("")
    }
    
    pub fn notification_sound(&self) -> &str {
        // Defaults to an empty string 
        self.notification_sound.as_deref().unwrap_or("")
    }
}

// ----------------------------------------------------------------
// Charger Notification Struct and Implementation
// ----------------------------------------------------------------
#[derive(Debug, Deserialize, Serialize)]
pub struct ChargerNotification {
    charging: Option<bool>,
    plugged_sound: Option<String>,
    charging_icon: Option<String>,
    discharging: Option<bool>,
    unplugged_sound: Option<String>,
    discharging_icon: Option<String>,
    urgent_level: Option<String>
}

/// This implementation defines Getter functions for the fields in the above struct
/// Also deals with option<> types by returning default values
impl ChargerNotification {

    pub fn should_notify_for_state(&self, state: &str) -> bool {
         match state {
            "Charging" => self.charging.unwrap_or(false),
            "Discharging" => self.discharging.unwrap_or(false),
            _ => false
        }
    }

    pub fn icon_for_state(&self, state: &str) -> &str {
        match state {
            "Charging" => self.charging_icon.as_deref().unwrap_or(""),
            "Discharging" => self.discharging_icon.as_deref().unwrap_or(""),
            _ => ""
        }
    }

    pub fn sound_for_state(&self, state: &str) -> &str {
         match state {
            "Charging" => self.plugged_sound.as_deref().unwrap_or(""),
            "Discharging" => self.unplugged_sound.as_deref().unwrap_or(""),
            _ => ""
        }
    }
}

// ----------------------------------------------------------------
// Trait for Battery Notification and Charger Notification 
// ----------------------------------------------------------------
pub trait GetUrgency {
    fn urgent_level(&self) -> &str;

    fn urgency(&self) -> Urgency {
        match self.urgent_level().try_into(){
            Ok(urgency) => {return urgency;},
            Err(_) => {
                eprintln!("Unsupported urgency level, defaulting to Normal");
            }
        };
        Urgency::Normal
    }
}

impl GetUrgency for BatteryNotification {
    fn urgent_level(&self) -> &str {
        self.urgent_level.as_deref().unwrap_or("Normal").trim()
    }
}

impl GetUrgency for ChargerNotification {
    fn urgent_level(&self) -> &str {
        self.urgent_level.as_deref().unwrap_or("Normal").trim()
    }
}