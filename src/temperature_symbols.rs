
use std::{convert::From, fmt::Display};

trait ReturnValue<'a, T> {
    fn value(&self) -> T;
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TemperatureSymbol<T> where T: Display {
    symbol: T
} impl<'a, T> ReturnValue<'a, T> for TemperatureSymbol<T> where T: Display {
    fn value(&self) -> T {
        return self.symbol;
    }
} impl<T> Display for TemperatureSymbol<T> where T: Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.symbol)
    }
} impl<T> From<T> for TemperatureSymbol<T> where T: Display {
    fn from(value: T) -> Self {
        TemperatureSymbol { symbol: value }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemperatureSymbols {
    Kelvin(TemperatureSymbol<&'static str>),
    DegreesChar(TemperatureSymbol<&'static str>),
    DegreesStr(TemperatureSymbol<&'static str>),
    Asterisk(TemperatureSymbol<&'static str>),
    Space(TemperatureSymbol<&'static str>),
    CustomChar(TemperatureSymbol<&'static str>),
    CustomString(TemperatureSymbol<&'static str>),
} impl TemperatureSymbols {
    const DEGREES_CHAR: &str = "°";
    const DEGREES_STR: &str = "degrees";
    const ASTERISK: &str = "*";
    const SPACE: &str = " ";
    const KELVIN: &str = "kelvin";
} impl From<char> for TemperatureSymbols {
    fn from(value: char) -> Self {
        let value: &str = value.to_string().as_str();
        match value {
            val @ Self::DEGREES_CHAR => Self::DegreesChar(TemperatureSymbol{symbol: val}),
            val @ Self::ASTERISK => Self::Asterisk(TemperatureSymbol{symbol: val}),
            val @ Self::SPACE => Self::Space(TemperatureSymbol { symbol: val }),
            val => Self::CustomChar(TemperatureSymbol{symbol: val}),
        }
    }
} impl From<&str> for TemperatureSymbols {
    fn from(value: &str) -> Self {
        let chars_peek = value.chars().peekable();
        if chars_peek.count() == 1 {
            return Self::from(value.chars()
            .next()
            .expect("I'm not sure how there's no char here... hepl"))
        }
            
        match value {
            val @Self::DEGREES_STR => Self::DegreesStr(TemperatureSymbol { symbol: val }),
            val @Self::KELVIN => Self::Kelvin(TemperatureSymbol { symbol: val }),
            val => Self::CustomString(TemperatureSymbol{ symbol: val }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success_convert_str_to_temperature_symbol() {
        let degrees_char: &str = "°";

        assert_eq!(TemperatureSymbol{symbol: "°"}, TemperatureSymbol::from(degrees_char))
    }
}
