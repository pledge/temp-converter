use std::str::FromStr;
use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Scale {
    Celsius,
    Fahrenheit,
    Kelvin,
    Rankine,
}

impl FromStr for Scale {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "c" | "celsius" => Ok(Scale::Celsius),
            "f" | "fahrenheit" => Ok(Scale::Fahrenheit),
            "k" | "kelvin" => Ok(Scale::Kelvin),
            "r" | "rankine" => Ok(Scale::Rankine),
            _ => Err(format!("'{}' is not a valid scale. Use 'celsius', 'fahrenheit', 'kelvin', or 'rankine'.", s)),
        }
    }
}

impl fmt::Display for Scale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Scale::Celsius => write!(f, "Celsius"),
            Scale::Fahrenheit => write!(f, "Fahrenheit"),
            Scale::Kelvin => write!(f, "Kelvin"),
            Scale::Rankine => write!(f, "Rankine"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Temperature {
    pub value: f64,
    pub scale: Scale,
}

impl Temperature {
    pub fn new(value: f64, scale: Scale) -> Self {
        Temperature { value, scale }
    }

    pub fn to_celsius(&self) -> Temperature {
        let value = match self.scale {
            Scale::Celsius => self.value,
            Scale::Fahrenheit => (self.value - 32.0) * 5.0 / 9.0,
            Scale::Kelvin => self.value - 273.15,
            Scale::Rankine => (self.value - 491.67) * 5.0 / 9.0,
        };
        Temperature::new(value, Scale::Celsius)
    }

    pub fn to_fahrenheit(&self) -> Temperature {
        let value = match self.scale {
            Scale::Celsius => self.value * 9.0 / 5.0 + 32.0,
            Scale::Fahrenheit => self.value,
            Scale::Kelvin => (self.value - 273.15) * 9.0 / 5.0 + 32.0,
            Scale::Rankine => self.value - 459.67,
        };
        Temperature::new(value, Scale::Fahrenheit)
    }

    pub fn to_kelvin(&self) -> Temperature {
        let value = match self.scale {
            Scale::Celsius => self.value + 273.15,
            Scale::Fahrenheit => (self.value - 32.0) * 5.0 / 9.0 + 273.15,
            Scale::Kelvin => self.value,
            Scale::Rankine => self.value * 5.0 / 9.0,
        };
        Temperature::new(value, Scale::Kelvin)
    }

    pub fn to_rankine(&self) -> Temperature {
        let value = match self.scale {
            Scale::Celsius => (self.value + 273.15) * 9.0 / 5.0,
            Scale::Fahrenheit => self.value + 459.67,
            Scale::Kelvin => self.value * 9.0 / 5.0,
            Scale::Rankine => self.value,
        };
        Temperature::new(value, Scale::Rankine)
    }
}

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2} °{}", self.value, self.scale.to_string().chars().next().unwrap())
    }
}
