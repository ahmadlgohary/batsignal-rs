use notify_rust::{Hint, Notification, Urgency};
use crate::audio::play_notification_sound;


pub fn send_battery_notification(
        battery_level: &i32, 
        notif_message: &str,
        notif_icon: &str,
        notif_urgency: Urgency,
        notif_sound: &str,
        time: i32
    ){

    Notification::new()
    .hint(Hint::Transient(true))
    .hint(Hint::Custom("synchronous".into(), "battery_notif".into()))  

    .summary(notif_message)
    .body(&format!("{battery_level}% of battery remaining"))
    .icon(notif_icon)
    .urgency(notif_urgency)
    .timeout(time)

    .show()
    .unwrap();
    
    play_notification_sound(notif_sound);
}

pub fn send_charger_notification(
        charging_state: &str,
        battery_level: &i32,
        notif_icon: &str, 
        notif_urgency: Urgency,
        notif_sound: &str,
        time: i32
    ) {
    
    Notification::new()
    
    // Transient hint means the notification by-passes the server's persistence and is not stored 
    .hint(Hint::Transient(true))
    
    // Used such that new notifications to replace previous notifications without cluttering
    .hint(Hint::Custom("synchronous".into(), "battery_notif".into())) 

    .summary(charging_state)
    .body(&format!("{battery_level}% of battery remaining"))
    .icon(notif_icon)
    .urgency(notif_urgency)
    .timeout(time) 

    .show()
    .unwrap();
    
    play_notification_sound(notif_sound);
} 


