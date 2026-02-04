use notify_rust::{Notification, Hint};
use crate::{
    audio::play_notification_sound, 
    config::{
        GetUrgency,
        BatteryNotification, 
        ChargerNotification
    }
};

pub fn send_battery_notification(
        battery_level: &i32, 
        notif_info: &BatteryNotification, 
        time: i32
    ){

    Notification::new()
    .hint(Hint::Transient(true))
    .hint(Hint::Custom("synchronous".into(), "battery_notif".into()))  

    .summary(notif_info.notification_message())
    .body(&format!("{battery_level}% of battery remaining"))
    .icon(notif_info.notification_icon())
    .urgency(notif_info.urgency())
    .timeout(time)

    .show()
    .unwrap();
    
    play_notification_sound(notif_info.notification_sound());
}

pub fn send_charger_notification(
        charging_state: &str,
        battery_level: &i32,
        notif_info: &ChargerNotification, 
        time: i32
    ) {
    
    Notification::new()
    
    // Transient hint means the notification by-passes the server's persistence and is not stored 
    .hint(Hint::Transient(true))
    
    // Used such that new notifications to replace previous notifications without cluttering
    .hint(Hint::Custom("synchronous".into(), "battery_notif".into())) 

    .summary(charging_state)
    .body(&format!("{battery_level}% of battery remaining"))
    .icon(notif_info.icon_for_state(charging_state))
    .urgency(notif_info.urgency())
    .timeout(time) 

    .show()
    .unwrap();
    
    play_notification_sound(notif_info.sound_for_state(charging_state));
} 



// -------- Obsolete --------
// /// This function creates a notification Id such that other notifications use the same id and replace each other. 
// /// The Id is retrieved by sending a notification and closing it. 
// /// This happens fast enough such that the notification does not appear to the user
// /// 
// pub fn create_notification_id() -> u32 {
//     let handle  = Notification::new()
//     .show()
//     .unwrap();
//     let id = handle.id();   
//     handle.close();  
//     id
// } 