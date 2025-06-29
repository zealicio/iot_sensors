use crate::error::AppError;

#[derive(Debug)]
pub enum SensorReading {
    Climate {
        temperature: f32,
        humidity: f32,
        pressure: f32,
    },
    Luminosity {
        lux: f32,
    },
    Magnetic {
        x: f32,
        y: f32,
        z: f32,
    },
    Position {
        latitude: f64,
        longitude: f64,
    },
}

pub trait SensorHandler {
    fn read_data(&mut self) -> Result<SensorReading, AppError>;
}
