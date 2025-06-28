use bme280::i2c::BME280;
use linux_embedded_hal::{Delay, I2cdev};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct ClimateSensorReadings {
    pub sensor_id: String,
    pub timestamp: u64,
    pub temperature: f32,
    pub humidity: f32,
    pub pressure: f32,
}

pub struct ClimateSensor {
    driver: BME280<I2cdev>,
    delay: Delay,
    id: String,
}

impl ClimateSensor {
    pub fn new(id: String, i2c_bus_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let i2c_bus = I2cdev::new(i2c_bus_path)?;
        let mut driver = BME280::new_primary(i2c_bus);
        let mut delay = Delay;

        match driver.init(&mut delay) {
            Ok(_) => {}
            Err(e) => {
                return Err(format!("Error initializing sensor: {:?}", e).into());
            }
        }

        Ok(ClimateSensor { driver, delay, id })
    }

    pub fn read_climate_data(
        &mut self,
    ) -> Result<ClimateSensorReadings, Box<dyn std::error::Error>> {
        let measures = match self.driver.measure(&mut self.delay) {
            Ok(m) => m,
            Err(e) => {
                return Err(format!("Error reading data: {:?}", e).into());
            }
        };
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        Ok(ClimateSensorReadings {
            sensor_id: self.id.clone(),
            timestamp,
            temperature: measures.temperature,
            humidity: measures.humidity,
            pressure: measures.pressure / 100.0,
        })
    }
}
