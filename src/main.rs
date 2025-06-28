use crate::sensors::climate_sensor::ClimateSensor;
use std::thread;
use std::time::Duration;

mod sensors;

fn main() {
    let sensor_id = "bme280-gut01".to_string();
    let i2c_bus_path = "/dev/i2c-3";

    match ClimateSensor::new(sensor_id, i2c_bus_path) {
        Ok(mut sensor) => loop {
            thread::sleep(Duration::from_secs(5));
            println!("Reading data...");
            match sensor.read_climate_data() {
                Ok(readings) => {
                    println!("Data Read");
                    println!("{:#?}", readings);
                }
                Err(e) => {
                    eprintln!("Error reading data: {}", e);
                }
            }
        },
        Err(e) => {
            eprintln!("Fatal error: {}", e);
        }
    }
}
