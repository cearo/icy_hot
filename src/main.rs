
use std::fmt;

// #TODO
// Implement Temperature::parse(File) -> Vec<Temperature>

fn main() {
    let temp_f = Temperature {value: 0, scale: TemperatureScales::Celsius, symbol: TemperatureSymbols::Degrees};
    // Want to add an overload: convert_to(TemperatureScale, TemperatureSymbol)
    print!("{:#?}", &temp_f);
    let new_temp = temp_f.convert_to(TemperatureScales::Fahrenheit);
    let compare_temp = Temperature { value: 32, scale: TemperatureScales::Fahrenheit, symbol: TemperatureSymbols::Degrees};
    let eq_test = new_temp == compare_temp;
    let clone_test = new_temp.clone();
    println!("New Temp Debug: {:#?}", &new_temp);
    println!("New Temp Display: {}", new_temp);
    println!("Clone Test: {:#?}", clone_test);
    println!("Equality Test: {}", eq_test);
}

// Scales
const FAHRENHEIT_SCALE_SYMBOL: char = 'F';
const CELSIUS_SCALE_SYMBOL: char = 'C';

// Data type to represent temperature scales
#[derive(Debug, Clone, PartialEq, Eq)]
enum TemperatureScales {
    Fahrenheit,
    Celsius,
} impl TemperatureScales {
    fn value(&self) -> char {
        match self {
            Self::Fahrenheit => FAHRENHEIT_SCALE_SYMBOL,
            Self::Celsius => CELSIUS_SCALE_SYMBOL,
        }
    }
}

// Char symbols
const DEGREE_SYMBOL: char = '\u{00B0}';
const DEGREE_ASTERISK_SYMBOL: char = '*';
const DEGREE_SPACE_SYMBOL: char = ' ';

// Data type to represent temperature unit symbols
#[derive(Debug, Clone, PartialEq, Eq)]
enum TemperatureSymbols {
    Degrees,
    Asterisk,
    Space,
} impl TemperatureSymbols {
    fn value(&self) -> char {
        match self {
            TemperatureSymbols::Degrees => DEGREE_SYMBOL,
            TemperatureSymbols::Asterisk => DEGREE_ASTERISK_SYMBOL,
            TemperatureSymbols::Space => DEGREE_SPACE_SYMBOL,
        }
    }
}

// Math
const FAHRENHEIT_WATER_FREEZE_TEMP: f64 = 32.0;
const CELSIUS_TO_FAHRENHEIT_RATIO: f64 = 9.0 / 5.0;
const FAHRENHEIT_TO_CELSIUS_RATIO: f64 = 5.0 / 9.0;

#[derive(Debug, Clone, PartialEq, Eq)]
// Data type to represent temperatures
// Implement builder pattern: Temperature.value(55).asterisk().to_celsius()
// Eventually Temperature.add(Temperature(55).to_celsius(), Temperature(100).to_fahrenheit()).to_celsius()
struct Temperature {
    value: i32,
    scale: TemperatureScales,
    symbol: TemperatureSymbols
}
 impl Temperature {
    
    pub fn convert_to(self, to_scale: TemperatureScales) -> Temperature {
        if self.scale == to_scale {
            return self;
        }

        match to_scale {
            TemperatureScales::Fahrenheit => self.to_fahrenheit(),
            TemperatureScales::Celsius => self.to_celsius(),
        }
    }

    pub fn to_fahrenheit(self) -> Temperature {
        
        const FAHRENHEIT_WATER_FREEZE_TEMP: f64 = 32.0;
        match self.scale {
            TemperatureScales::Fahrenheit => self,
            TemperatureScales::Celsius => Temperature { 
                value: (f64::from(self.value) * CELSIUS_TO_FAHRENHEIT_RATIO + FAHRENHEIT_WATER_FREEZE_TEMP) as i32, 
                scale: self.scale, 
                symbol: self.symbol 
            },
        }
    }

    pub fn to_celsius(self) -> Temperature {
        match self.scale {
            TemperatureScales::Fahrenheit => Temperature { 
                value: ((f64::from(self.value) - FAHRENHEIT_WATER_FREEZE_TEMP) * FAHRENHEIT_TO_CELSIUS_RATIO) as i32, 
                scale: self.scale, 
                symbol: self.symbol,
            },
            TemperatureScales::Celsius => self,
        }
    }

    pub fn clone_to(&self, to_scale: TemperatureScales) -> Temperature {
        match to_scale {
            TemperatureScales::Fahrenheit => self.clone().to_fahrenheit(),
            TemperatureScales::Celsius => self.clone().to_celsius(),
        }
    }
} impl Default for Temperature {
    fn default() -> Self {
        Self { value: 0, scale: TemperatureScales::Celsius, symbol: TemperatureSymbols::Degrees }
    }
} impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}", self.value, self.symbol.value(), self.scale.value())
    }
}