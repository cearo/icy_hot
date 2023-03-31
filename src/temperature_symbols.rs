
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
} impl<T> Display for TemperatureSymbol<T> where T: Copy  + Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.symbol)
    }
} impl<T> From<T> for TemperatureSymbol<T> where T: Display {
    fn from(value: T) -> Self {
        TemperatureSymbol { symbol: value }
    }
} impl From<TemperatureSymbol<char>> for char {
    fn from(value: TemperatureSymbol<char>) -> Self {
        value.symbol
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemperatureSymbols<'a> {
    Kelvin(TemperatureSymbol<&'a str>),
    DegreesChar(TemperatureSymbol<char>),
    DegreesStr(TemperatureSymbol<&'a str>),
    Asterisk(TemperatureSymbol<char>),
    Space(TemperatureSymbol<char>),
    CustomChar(TemperatureSymbol<char>),
    CustomString(TemperatureSymbol<&'a str>),
} impl<'a> TemperatureSymbols<'a> {
    const DEGREES_CHAR: char = '°';
    const DEGREES_STR: &str = "degrees";
    const ASTERISK: char = '*';
    const SPACE: char = ' ';
    const KELVIN: &str = "kelvin";
} impl<'a> From<char> for TemperatureSymbols<'a> {
    fn from(value: char) -> Self {
        match value {
            val @ Self::DEGREES_CHAR => Self::DegreesChar(TemperatureSymbol{symbol: val}),
            val @ Self::ASTERISK => Self::Asterisk(TemperatureSymbol{symbol: val}),
            val @ Self::SPACE => Self::Space(TemperatureSymbol { symbol: val }),
            val => Self::CustomChar(TemperatureSymbol{symbol: val}),
        }
    }
} impl<'a> From<&str> for TemperatureSymbols<'a> {
    fn from(value: &str) -> Self {
        if let Some(val) = value.chars().next() {
            return Self::from(val)
        }
        match value {
            val @Self::DEGREES_STR => Self::DegreesStr(TemperatureSymbol { symbol: val }),
            val @Self::KELVIN => Self::Kelvin(TemperatureSymbol { symbol: val }),
            val => Self::CustomString(TemperatureSymbol{ symbol: val }),
        }
    }
}
