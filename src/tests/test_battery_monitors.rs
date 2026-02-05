use super::*;

#[rstest]
fn charging_sends_notification_once(_clear_calls:()) {
    let battery_str = r#"
        prev_state = "Charging"
        last_notified_state = "Charging"
        current_state = "Discharging"
        percentage = 17
    "#;

    let config_str= r#"
    [charger_notifications]
    charging = true
    charging_icon = "battery-charging"
    plugged_sound = "battery_charging.ogg"

    discharging = true
    discharging_icon = "battery-discharging"
    unplugged_sound = "battery_discharging.mp3"

    urgent_level = "Normal"
    "#;

    let mut battery = BatteryStats::parse_toml_from_str(battery_str).ok().unwrap();
    let charger= Config::parse_toml_from_str(config_str).unwrap().charger_notifications;

    battery.handle_charger_notifications(&charger, 5000);
    battery.handle_charger_notifications(&charger, 5000);
    let calls = get_calls();


    println!("{:?}",calls);
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0], "Discharging");
}

#[rstest]
fn discharging_sends_notification_once(_clear_calls:()) {
    let battery_str = r#"
        prev_state = "Discharging"
        last_notified_state = "Discharging"
        current_state = "Charging"
        percentage = 17
    "#;

    let config_str= r#"
    [charger_notifications]
    charging = true
    charging_icon = "battery-charging"
    plugged_sound = "battery_charging.ogg"

    discharging = true
    discharging_icon = "battery-discharging"
    unplugged_sound = "battery_discharging.mp3"

    urgent_level = "Normal"
    "#;

    let mut battery = BatteryStats::parse_toml_from_str(battery_str).ok().unwrap();
    let charger= Config::parse_toml_from_str(config_str).unwrap().charger_notifications;

    battery.handle_charger_notifications(&charger, 5000);
    battery.handle_charger_notifications(&charger, 5000);
    let calls = get_calls();


    println!("{:?}",calls);
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0], "Charging");
}


// --------------------------------------------
// Test Template
// --------------------------------------------
/*
// clear_calls must be passed to 
// every test that tests if notifications are sent
fn test_template(_clear_calls: ()) {

    // Test Code goes here
    // Test Code goes here
    // Test Code goes here
    
    let calls = get_calls();
    
    assert_eq!(, );
    ...
}
*/