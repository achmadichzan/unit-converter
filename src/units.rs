use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Category {
    Temperature,
    Length,
}

pub enum Unit {
    Celsius,
    Fahrenheit,
    Kelvin,
    Cm,
    Inch,
    Km,
    Miles,
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Unit::Celsius => write!(f, "celsius"),
            Unit::Fahrenheit => write!(f, "fahrenheit"),
            Unit::Kelvin => write!(f, "kelvin"),
            Unit::Cm => write!(f, "cm"),
            Unit::Inch => write!(f, "inch"),
            Unit::Km => write!(f, "km"),
            Unit::Miles => write!(f, "miles"),
        }
    }
}

impl Unit {
    pub fn category(&self) -> Category {
        match self {
            Unit::Celsius | Unit::Fahrenheit | Unit::Kelvin => Category::Temperature,
            Unit::Cm | Unit::Inch | Unit::Km | Unit::Miles => Category::Length,
        }
    }
}

impl FromStr for Unit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "celsius" => Ok(Unit::Celsius),
            "fahrenheit" => Ok(Unit::Fahrenheit),
            "kelvin" => Ok(Unit::Kelvin),
            "cm" => Ok(Unit::Cm),
            "inch" => Ok(Unit::Inch),
            "km" => Ok(Unit::Km),
            "miles" => Ok(Unit::Miles),
            _ => Err(s.to_string()),
        }
    }
}