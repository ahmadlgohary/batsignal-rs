use crate::config::{BatteryNotification, ChargerNotification, GetUrgency};
use std::{collections::{BTreeMap, HashSet}, io};

use crate::notifications::{send_battery_notification, send_charger_notification};


//  ----------------------------------------------------
//  BatteryStats Struct and Implementation
//  ----------------------------------------------------
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
        let current_state = battery_state_to_string(battery.state());
        let percentage = (battery.state_of_charge().value * 100.0) as i32;
        let previous_state = if current_state == "Discharging" {"Charging"} else {"Discharging"};

        Some(
            BatteryStats { 
            prev_state: previous_state.to_string(), 
            last_notified_state: current_state.clone(),
            current_state,
            percentage
        })
    } 
    
    pub fn update_battery_stats(&mut self, battery_manager: &battery::Manager,  battery: &mut battery::Battery) {
        if let Ok(()) = battery_manager.refresh(battery) {

            self.current_state = battery_state_to_string(battery.state());
            self.percentage = ((battery.state_of_charge()).value * 100.0) as i32;
        }
    } 
    
    pub fn handle_charger_notifications(
        &mut self, 
        charger_notif: &Option<ChargerNotification>, 
        notif_time: i32 
    ) {
        
        let inferred_state: &str = if self.current_state == "Unknown" {
            // Transition edge: infer the *new* state
            if self.prev_state == "Discharging" { "Charging"} 
            else { "Discharging" }
        } 
        else {
            // Normal case: use current state
            self.current_state.as_str()
        };

        // Notify only once per inferred state change
        if inferred_state != self.last_notified_state {
            
            self.last_notified_state = inferred_state.to_string();
            
            if let Some(charger_notifications) = charger_notif
                && charger_notifications.should_notify_for_state(inferred_state) {
                    send_charger_notification(
                        inferred_state, 
                        &self.percentage,
                        charger_notifications.icon_for_state(inferred_state),
                        charger_notifications.urgency(),
                        charger_notifications.sound_for_state(inferred_state),
                        notif_time
                    );
            }
        }
    }

    pub fn handle_battery_state_change(&mut self, battery_notif_sent: &mut HashSet<u8>){
        // This means we switched states 
        if self.prev_state != self.current_state {
            // previous state should never be unknown
            if self.current_state != "Unknown" {
                self.prev_state = self.current_state.clone();
            }
            battery_notif_sent.clear();
        }
    }

    pub fn handle_battery(
        &mut self, 
        low_level_notifs: &Option<BTreeMap<u8, BatteryNotification>>, 
        high_level_notifs: &Option<BTreeMap<u8, BatteryNotification>>, 
        notif_time: i32,
        battery_notif_sent: &mut HashSet<u8>
    ) {
            

        if self.current_state == "Discharging" && let Some(low_charges) = low_level_notifs {
                
            for (battery_level, notification_info)  in low_charges.iter().rev(){
                if self.percentage <= *battery_level as i32 && !(battery_notif_sent).contains(battery_level) {

                    battery_notif_sent.insert(*battery_level);
                    
                    send_battery_notification(
                        &self.percentage, 
                        notification_info.notification_message(), 
                        notification_info.notification_icon(), 
                        notification_info.urgency(), 
                        notification_info.notification_sound(), 
                        notif_time
                    );
                }
            }
            
        }
        else if self.current_state == "Charging" && let Some(high_charges) = high_level_notifs {

            for (battery_level, notification_info)  in high_charges.iter(){
                if self.percentage >= *battery_level as i32 && !(battery_notif_sent).contains(battery_level) {
                    
                    battery_notif_sent.insert(*battery_level);
                    
                    send_battery_notification(
                        &self.percentage, 
                        notification_info.notification_message(), 
                        notification_info.notification_icon(), 
                        notification_info.urgency(), 
                        notification_info.notification_sound(), 
                        notif_time
                    );
                }
            }
        }
    }
}


//  ----------------------------------------------------
//  Battery Helper Functions
//  ----------------------------------------------------
pub fn init_battery_manager () -> Option<battery::Manager> {
    match battery::Manager::new(){
        Ok(manager) => Some(manager),
        Err(error) => {           
            eprintln!("Unable to get battery manager\n\n\n {error}");
            None
        }
    }
}

pub fn init_battery(battery_manager: &battery::Manager) -> Option<battery::Battery> {
    let mut batteries_iterator = battery_manager.batteries().ok()?;

    match batteries_iterator.next() {
        Some(Ok(battery)) => Some(battery),
        Some(Err(error)) => {
            eprintln!("Unable to access battery information\n\n\n {error}");
            None
        }
        None => {
            eprintln!(
                "Unable to find any batteries\n\n\n {}",
                io::Error::from(io::ErrorKind::NotFound)
            );
            None
        }
    }
}

fn battery_state_to_string(state: battery::State) -> String {
    // Using a match statement rather than the to_string() implementation of the battery crate
    // because it returns the strings in lowercase and they are needed in Title case for better formatting
    // and 'Full' and 'Empty' states add extra complexity that can be simply avoided by having 3 states 
    match state {
            battery::State::Charging    |
            battery::State::Full        => "Charging",
            battery::State::Discharging |
            battery::State::Empty       => "Discharging",
            battery::State::Unknown     => "Unknown", 
            _                           => "Unknown"   // making clippy happy :)
    }.to_string()
}
