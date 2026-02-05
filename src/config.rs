use serde::Deserialize;
use notify_rust::Urgency;
use std::{collections::BTreeMap, fs};

// ----------------------------------------------------------------
// Configuration Struct and Implementation
// ----------------------------------------------------------------
#[derive(Debug, Deserialize)]
pub struct Config {
    notification_time: Option<i32>,
    pub(crate) high_battery_levels: Option<BTreeMap<u8, BatteryNotification>>,
    pub(crate) low_battery_levels: Option<BTreeMap<u8, BatteryNotification>>,
    pub(crate) charger_notifications: Option<ChargerNotification>
}

/// This function reads a json config file and parses it into the Config Struct
impl Config {
    pub fn parse_toml() -> Self {
        // TODO: 
        //  - make it take command line arguments for the path 
        fs::read_to_string("config.toml")
            .ok()
            .and_then(|file| { toml::from_str(&file).ok()})
            .unwrap_or_else(|| { 
                eprintln!("Failed to load config.toml, using defaults");
                Self::default()})
    }

    /// This function creates a default config
    fn default() -> Self {
        let low_battery_map = Some(BTreeMap::from([(20, 
            BatteryNotification { 
                message: "Battery Low".to_string(), 
                notification_icon: None, 
                notification_sound: None, 
                urgent_level: None 
            })]));
            
        Self { 
            notification_time: None, 
            high_battery_levels: None, 
            low_battery_levels: low_battery_map, 
            charger_notifications: None 
        }
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
#[derive(Debug, Deserialize)]
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