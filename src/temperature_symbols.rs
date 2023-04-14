
use std::{convert::From,convert::TryFrom, fmt::Display};

//Leaving this stuff for now as we would ideally like to build an extensible interface
// trait ReturnValue<'a, T> {
//     fn value(&self) -> T;
// }

// #[derive(Debug, Clone, PartialEq, Eq)]
// struct TemperatureSymbol<T> where T: Display {
//     symbol: T
// } impl<'a, T> ReturnValue<'a, T> for TemperatureSymbol<T> where T: Display {
//     fn value(&self) -> T {
//         return self.symbol;
//     }
// } impl<T> Display for TemperatureSymbol<T> where T: Display {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.symbol)
//     }
// } impl<T> From<T> for TemperatureSymbol<T> where T: Display {
//     fn from(value: T) -> Self {
//         TemperatureSymbol { symbol: value }
//     }
// }

const DEGREES_CHAR: &str = "°";
const DEGREES_STR: &str = "degrees";
const ASTERISK: &str = "*";
const SPACE: &str = " ";
const KELVIN: &str = "kelvin";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemperatureSymbols {
    Kelvin(String),
    DegreesChar(String),
    DegreesStr(String),
    Asterisk(String),
    Space(String),
    CustomChar(String),
    CustomString(String),
} impl From<char> for TemperatureSymbols {
    fn from(value: char) -> Self {
        match value.to_string().as_str() {
            val @ DEGREES_CHAR => Self::DegreesChar(val.to_string()),
            val @ ASTERISK => Self::Asterisk(val.to_string()),
            val @ SPACE => Self::Space(val.to_string()),
            val => Self::CustomChar(val.to_string()),
        }
    }
} impl TryFrom<TemperatureSymbols> for char {
    type Error = ();
    fn try_from(value: TemperatureSymbols) -> Result<char,Self::Error> {
        match value {
            TemperatureSymbols::DegreesChar(c) => Ok(c.chars().next().unwrap()),
            TemperatureSymbols::Asterisk(c) => Ok(c.chars().next().unwrap()),
            TemperatureSymbols::Space(c) => Ok(c.chars().next().unwrap()),
            TemperatureSymbols::CustomChar(c) => Ok(c.chars().next().unwrap()),
            _ => Err(())
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
            val @ DEGREES_STR => Self::DegreesStr(val.to_string()),
            val @ KELVIN => Self::Kelvin(val.to_string()),
            val => Self::CustomString(val.to_string()),
        }
    }
} impl From<TemperatureSymbols> for String {
    fn from(value: TemperatureSymbols) -> Self {
        match value {
            TemperatureSymbols::Kelvin(val) => val,
            TemperatureSymbols::DegreesChar(val) => val,
            TemperatureSymbols::DegreesStr(val) => val,
            TemperatureSymbols::Asterisk(val) => val,
            TemperatureSymbols::Space(val) => val,
            TemperatureSymbols::CustomChar(val) => val,
            TemperatureSymbols::CustomString(val) => val,
        }
    }
} impl Display for TemperatureSymbols {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self.to_owned()))
    }
} impl PartialEq<&str> for TemperatureSymbols {
    fn eq(&self, other: &&str) -> bool {
        match self {
            TemperatureSymbols::Kelvin(val) => **val == **other,
            TemperatureSymbols::DegreesChar(val) => **val == **other,
            TemperatureSymbols::DegreesStr(val) => **val == **other,
            TemperatureSymbols::Asterisk(val) => **val == **other,
            TemperatureSymbols::Space(val) => **val == **other,
            TemperatureSymbols::CustomChar(val) => **val == **other,
            TemperatureSymbols::CustomString(val) => **val == **other,
        }
    }
} impl PartialEq<TemperatureSymbols> for &str {
    fn eq(&self, other: &TemperatureSymbols) -> bool {
        match other {
            TemperatureSymbols::Kelvin(val) => **val == **self,
            TemperatureSymbols::DegreesChar(val) => **val == **self,
            TemperatureSymbols::DegreesStr(val) => **val == **self,
            TemperatureSymbols::Asterisk(val) => **val == **self,
            TemperatureSymbols::Space(val) => **val == **self,
            TemperatureSymbols::CustomChar(val) => **val == **self,
            TemperatureSymbols::CustomString(val) => **val == **self,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success_convert_str_to_temperature_symbol() {
        let degrees_char: &str = "°";

        assert_eq!(TemperatureSymbols::DegreesChar("°".to_owned()), TemperatureSymbols::from(degrees_char))
    }

    #[test]
    fn success_convert_temperature_symbol_to_str() {
        let degrees_char: &str = "°";

        assert_eq!(degrees_char.to_owned(), String::from(TemperatureSymbols::DegreesChar(degrees_char.to_owned())))
    }

    #[test]
    fn success_convert_char_to_temperature_symbol() {
        let degrees_char: char = '°';

        assert_eq!(TemperatureSymbols::DegreesChar(degrees_char.into()), TemperatureSymbols::from(degrees_char))
    }

    #[test]
    fn success_convert_temperature_symbol_to_char() {
        let degrees_enum = TemperatureSymbols::DegreesChar("°".to_owned());

        assert_eq!('°', char::try_from(degrees_enum).unwrap());
    }

    #[test]
    fn error_convert_temperature_symbol_to_char() {
        let degrees_string_enum = TemperatureSymbols::DegreesStr("degrees".to_owned());

        assert_eq!(Result::Err(()), char::try_from(degrees_string_enum));
    }
    
    #[test]
    fn success_temperature_symbol_left_equals_str() {
        let degrees_char: &str = "°";
        let degrees_string_enum = TemperatureSymbols::DegreesStr(degrees_char.to_owned());
        
        assert!(degrees_string_enum == degrees_char);
    }

    #[test]
    fn success_temperature_symbol_left_not_equals_str() {
        let degrees_char: &str = "°";
        let degrees_string_enum = TemperatureSymbols::Asterisk("*".to_owned());
        
        assert!(degrees_string_enum != degrees_char);
    }

    #[test]
    fn success_temperature_symbol_left_equals_emoji() {
        let smiley_face_emoji: &str = "😊";
        let smiley_temperature_symbol = TemperatureSymbols::CustomChar(smiley_face_emoji.to_owned());

        assert!(smiley_temperature_symbol == smiley_face_emoji);
    }

    #[test]
    fn success_temperature_symbol_right_equals_emoji() {
        let smiley_face_emoji: &str = "😊";
        let smiley_temperature_symbol = TemperatureSymbols::CustomChar(smiley_face_emoji.to_owned());

        assert!(smiley_face_emoji == smiley_temperature_symbol);
    }
}
