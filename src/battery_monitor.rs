use std::{collections::HashSet, io};

use crate::{config::Config, notifications::{send_battery_notification, send_charger_notification}};


pub fn get_battery_manager () -> Option<battery::Manager> {
    let manager = match battery::Manager::new(){
        Ok(manager) => Some(manager),
        Err(error) => {
            
            eprintln!("Unable to get battery manager\n\n\n {error}");
            
            return None;
        },
    };
    manager
}


pub fn get_battery(battery_manager: & battery::Manager) -> Option<battery::Battery> {
    let mut batteries_iterator = battery_manager.batteries().ok()?;

    let battery = match batteries_iterator.next() {
        Some(Ok(battery)) => Some(battery),
        Some(Err(error)) => {
            eprintln!("Unable to access battery information\n\n\n {error}");
            return None;
        }
        None => {
            eprintln!(
                "Unable to find any batteries\n\n\n {}",
                io::Error::from(io::ErrorKind::NotFound)
                );

            return None;
        }
    };
    battery
}


#[derive(Debug)]
pub struct BatteryStats {
    prev_state: String,
    last_notified_state: String,
    current_state: String, 
    percentage: i32, 
}

impl BatteryStats {

    pub fn new(battery_manager: &battery::Manager,  battery: &mut battery::Battery) -> Option<Self> {

        battery_manager.refresh(battery).ok()?;
        let current_state = match battery.state() {
            battery::State::Charging => "Charging",
            battery::State::Full => "Charging",
            battery::State::Discharging => "Discharging",
            battery::State::Empty => "Discharging",
            battery::State::Unknown=> "Unknown",
            _ => "Unknown" 
        };

        let percentage = (battery.state_of_charge().value * 100.0) as i32;
        let previous_state = if current_state == "Discharging" {"Charging"} else {"Discharging"};

        Some(
            BatteryStats { 
            prev_state: previous_state.to_string(), 
            last_notified_state: current_state.to_string(),
            current_state: current_state.to_string(),
            percentage: percentage
        })
    } 
    
    pub fn get_battery_stats(self: &mut Self, battery_manager: &battery::Manager,  battery: &mut battery::Battery) {

        match battery_manager.refresh(battery) {
            Ok(()) => {
                let current_state = match battery.state() {
                    battery::State::Charging => "Charging",
                    battery::State::Full => "Charging",
                    battery::State::Discharging => "Discharging",
                    battery::State::Empty => "Discharging",
                    battery::State::Unknown=> "Unknown",
                    _ => "Unknown" 
                };
                self.current_state = current_state.to_string();
                self.percentage = ((battery.state_of_charge()).value * 100.0) as i32;
            },
            Err(_) => ()  
        }
        ()
    }

    pub fn handle_charger_notifications(self: &mut Self, notification_id: u32, configuration: &Config ) {
        
        let inferred_state: &str;

        if self.current_state == "Unknown" {
            // Transition edge: infer the *new* state
            inferred_state = if self.prev_state == "Discharging" { "Charging"} else { "Discharging" };
        } 
        else {
            // Normal case: use current state
            inferred_state = self.current_state.as_str();
        }

        // Notify only once per inferred state change
        if inferred_state != self.last_notified_state {
            if let Some(charger_notifications) = &configuration.charger_notifications {
                send_charger_notification(
                    notification_id, 
                    inferred_state, 
                    &self.percentage,
                    &charger_notifications,
                    configuration.get_time()
                    );
            }
            self.last_notified_state = inferred_state.to_string();
        }
    }

    pub fn handle_battery_state_change(self: &mut Self, notifications_sent: &mut HashSet<u8>){
        // This means we switched states 
        if self.prev_state != self.current_state {
            // previous state should never be unknown
            if self.current_state != "unknown" {
                self.prev_state = self.current_state.clone();
            }
            notifications_sent.clear();
        }
    }

    pub fn handle_battery(self: &mut Self, notification_id: u32, _configuration: &Config, notifications_sent: &mut HashSet<u8>) {
        let notification_time = _configuration.get_time();
        if self.current_state == "Discharging" {
            
            if let Some(low_charges) = &_configuration.low_battery_levels{
                
                for (battery_level, notification_info)  in low_charges.iter().rev(){
                    if self.percentage <= *battery_level as i32 {
                        if !(notifications_sent).contains(battery_level) {
                            notifications_sent.insert(*battery_level);
                            send_battery_notification(notification_id, &self.percentage, &notification_info, notification_time);
                        }
                    }
                }
            }
            
        }
        else if self.current_state == "Charging" {


            if let Some(high_charges) = &_configuration.high_battery_levels{
                for (battery_level, notification_info)  in high_charges.iter(){
                    if self.percentage >= *battery_level as i32 {
                        if !(notifications_sent).contains(battery_level) {
                            notifications_sent.insert(*battery_level);
                            send_battery_notification(notification_id, &self.percentage, &notification_info, notification_time);
                        }
                    }
                }
            }
        }
    }
}