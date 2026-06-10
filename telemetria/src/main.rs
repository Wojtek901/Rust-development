fn check_voltage(current_voltage : f64) -> bool {
    current_voltage > 12.0
}

fn calculate_altitude_error(current_altitude : f64, desired_altitude : f64) -> f64{
    let altitude_difference : f64 = desired_altitude - current_altitude;
    altitude_difference
}

fn main(){
    let batter_voltage : f64 = 11.5;
    let current_altitude : f64 = 45.0;
    let target_attitude : f64 = 60.0;

    let is_battery_safe : bool = check_voltage(batter_voltage);
    let altitude_error : f64 = calculate_altitude_error(current_altitude, target_attitude);

    if !is_battery_safe {
        println!("Warning: Too low battery voltage!");
    }

    if altitude_error.abs() > 0.0 {
        println!("Current altitude difference: {}", altitude_error);
    }
}