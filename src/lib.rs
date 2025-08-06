use std::str::FromStr;
use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Scale {
    Celsius,
    Fahrenheit,
    Kelvin,
    Rankine,
    Reaumur,
}

impl FromStr for Scale {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "c" | "celsius" => Ok(Scale::Celsius),
            "f" | "fahrenheit" => Ok(Scale::Fahrenheit),
            "k" | "kelvin" => Ok(Scale::Kelvin),
            "r" | "rankine" => Ok(Scale::Rankine),
            "re" | "reaumur" => Ok(Scale::Reaumur),
            _ => Err(format!("'{}' is not a valid scale. Use 'celsius', 'fahrenheit', 'kelvin', 'rankine', or 'reaumur'.", s)),
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
            Scale::Reaumur => write!(f, "Reaumur"),
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
            Scale::Reaumur => self.value * 5.0 / 4.0,
        };
        Temperature::new(value, Scale::Celsius)
    }

    pub fn to_fahrenheit(&self) -> Temperature {
        if self.scale == Scale::Fahrenheit {
            return *self;
        }
        let celsius = self.to_celsius().value;
        Temperature::new(celsius * 9.0 / 5.0 + 32.0, Scale::Fahrenheit)
    }

    pub fn to_kelvin(&self) -> Temperature {
        if self.scale == Scale::Kelvin {
            return *self;
        }
        let celsius = self.to_celsius().value;
        Temperature::new(celsius + 273.15, Scale::Kelvin)
    }

    pub fn to_rankine(&self) -> Temperature {
        if self.scale == Scale::Rankine {
            return *self;
        }
        let celsius = self.to_celsius().value;
        Temperature::new((celsius + 273.15) * 9.0 / 5.0, Scale::Rankine)
    }

    pub fn to_reaumur(&self) -> Temperature {
        let celsius = self.to_celsius().value;
        Temperature::new(celsius * 4.0 / 5.0, Scale::Reaumur)
    }
}

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2} °{}", self.value, self.scale.to_string().chars().next().unwrap())
    }
}
