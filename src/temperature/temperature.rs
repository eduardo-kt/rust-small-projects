// types of temperatures
#[derive(Debug)]
pub enum TemperatureScale {
    Celsius,
    Fahrenheit,
    Kelvin,
}

// temperature errors
#[derive(Debug)]
pub enum TemperatureError {
    BelowAbsoluteZero(String),
}

// temperature structure
#[derive(Debug)]
pub struct Temperature {
    pub scale: TemperatureScale,
    pub value: f64,
}

impl Temperature {
    pub fn new(value: f64, scale: TemperatureScale) -> Result<Temperature,TemperatureError> {
        match scale {
            TemperatureScale::Celsius if value < -273.15 => Err(
                TemperatureError::BelowAbsoluteZero(
                    "Celsius temperatura cannot be below absolute zero.".to_string(),)),
            TemperatureScale::Fahrenheit if value < -459.67 => Err(
                TemperatureError::BelowAbsoluteZero(
                    "Fahrenheit temperatura cannot be below absolute zero.".to_string(),)),
            TemperatureScale::Kelvin if value < 0.00 => Err(
                TemperatureError::BelowAbsoluteZero(
                    "Kelvin temperatura cannot be below absolute zero.".to_string(),)),
            _ => Ok(Temperature {value, scale}),
        }        
    }

    pub fn convert_temperature_to(&self,new_scale: TemperatureScale) -> Temperature {
        let new_temperature = match new_scale {
            TemperatureScale::Celsius => match self.scale {
                TemperatureScale::Celsius => self.value,
                TemperatureScale::Fahrenheit => (self.value - 32.0) * 5.0/9.0,
                TemperatureScale::Kelvin => self.value - 273.15,
            },
            TemperatureScale::Fahrenheit => match self.scale {
                TemperatureScale::Celsius => self.value * 9.0 / 5.0 + 32.0,
                TemperatureScale::Fahrenheit => self.value,
                TemperatureScale::Kelvin => self.value * 9.0 / 5.0 - 459.67,
            },
            TemperatureScale::Kelvin => match self.scale {
                TemperatureScale::Celsius => self.value + 273.15,
                TemperatureScale::Fahrenheit => (self.value + 459.67) * 5.0 / 9.0,
                TemperatureScale::Kelvin => self.value,
            },
        };
        Temperature { scale: new_scale, value: new_temperature }
    }
}