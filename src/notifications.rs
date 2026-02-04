use rodio;
use std::{
    thread, 
    time::Duration,
};
use notify_rust::{
    Notification,
    Urgency,
    Hint
};
use crate::config::BatteryNotification;

pub fn send_battery_notification(id: u32, battery_level: &i32, notification_information: &BatteryNotification) {
    Notification::new()
    .id(id)
    .hint(Hint::Transient(true))

    .summary(notification_information.get_message())
    .body(&format!("{battery_level}% of battery remaining"))
    .icon(notification_information.get_icon())
    .urgency(notification_information.get_urgency())
    .timeout(5000)
    
    .show()
    .unwrap();
}

pub fn create_notification_id() -> u32 {
    let handle  = Notification::new()
    .show()
    .unwrap();
    let id = handle.id();   
    handle.close();  
    thread::sleep(Duration::from_secs(1));
    id
} 


pub fn testing_notification(id: u32, str: &str) {
    Notification::new()
    .summary(str)
    .body("test body")
    .timeout(5000) 
    .urgency(Urgency::Normal)
    .id(id)
    .hint(Hint::Transient(true))
    .show()
    .unwrap();
} 