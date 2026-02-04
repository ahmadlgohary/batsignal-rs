use std::{
 thread, time::Duration
};
use notify_rust::{
    Notification,
    Hint
};
use crate::{audio::play_notification_sound, config::{BatteryNotification, ChargerNotification}};

pub fn create_notification_id() -> u32 {
    let handle  = Notification::new()
    .show()
    .unwrap();
    let id = handle.id();   
    handle.close();  
    thread::sleep(Duration::from_secs(1));
    id
} 



pub fn send_battery_notification(
        id: u32, 
        battery_level: &i32, 
        notification_information: &BatteryNotification, 
        time: i32
    ){

    Notification::new()
    .id(id)
    .hint(Hint::Transient(true))

    .summary(notification_information.get_message())
    .body(&format!("{battery_level}% of battery remaining"))
    .icon(notification_information.get_icon())
    .urgency(notification_information.get_urgency())
    .timeout(time)
    
    .show()
    .unwrap();
    
    if ! notification_information.get_sound().is_empty() {
        play_notification_sound(notification_information.get_sound());
    } 


}



pub fn send_charger_notification(
        id: u32, 
        charging_state: &str,
        battery_level: &i32,
        notification_information: &ChargerNotification, 
        time: i32
    ) {
    
    if notification_information.get_bool_by_state(charging_state) {
        Notification::new()
        .id(id)
        .hint(Hint::Transient(true))

        .summary(charging_state)
        .body(&format!("{battery_level}% of battery remaining"))
        .icon(notification_information.get_icon_by_state(charging_state))
        .urgency(notification_information.get_urgency())
        .timeout(time) 

        .show()
        .unwrap();
        
        if ! notification_information.get_sound_by_state(charging_state).is_empty() {
            play_notification_sound(notification_information.get_sound_by_state(charging_state));
        }
    }
} 