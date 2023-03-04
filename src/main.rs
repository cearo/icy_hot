
// Math
const FAHRENHEIT_WATER_FREEZE_TEMP: f64 = 32.0;
const FAHRENHEIT_TO_CELSIUS_RATIO: f64 = 5.0 / 9.0;
const CELSIUS_TO_FAHRENHEIT_RATIO: f64 = 9.0 / 5.0;

fn main() {
    // Will loop for user input and handle invalid input cases
    // Want to be able to suggest if they are using an unsupported TemperatureScale or TemperatureSymbol
    let temp_f = Temperature {value: 32, scale: TemperatureScales::Fahrenheit, symbol: TemperatureSymbols::Space};
    // Want to add an overload: convert_to(TemperatureScale, TemperatureSymbol)
    let new_temp = temp_f.convert_to(TemperatureScales::Celsius);
    println!("{:#?}", new_temp);
    println!("New Temperature is: {new_temp_val}{new_deg_sym}{new_deg_scale}",
        new_temp_val = new_temp.value, 
        new_deg_sym = new_temp.symbol.value(), 
        new_deg_scale = new_temp.scale.value()
    );
}

// Data type to represent temperature scales
// Scales
const FAHRENHEIT_SCALE_SYMBOL: char = 'F';
const CELSIUS_SCALE_SYMBOL: char = 'C';
#[derive(Debug)]
enum TemperatureScales {
    Fahrenheit,
    Celsius,
} impl TemperatureScales {
    fn value(self) -> char {
        match self {
            Self::Fahrenheit => FAHRENHEIT_SCALE_SYMBOL,
            Self::Celsius => CELSIUS_SCALE_SYMBOL
        }
    }
}

// Data type to represent temperature unit symbols
// Symbols
const DEGREE_SYMBOL: char = '\u{00B0}';
const DEGREE_ASTERISK_SYMBOL: char = '*';
const DEGREE_SPACE_SYMBOL: char = ' ';

#[derive(Debug)]
enum TemperatureSymbols {
    Degrees,
    Asterisk,
    Space,
} impl TemperatureSymbols {
    fn value(self) -> char {
        match self {
            TemperatureSymbols::Degrees => DEGREE_SYMBOL,
            TemperatureSymbols::Asterisk => DEGREE_ASTERISK_SYMBOL,
            TemperatureSymbols::Space => DEGREE_SPACE_SYMBOL,
        }
    }
}

#[derive(Debug)]
// Data type to represent temperatures
struct Temperature {
    value: i32,
    scale: TemperatureScales,
    symbol: TemperatureSymbols
} impl Temperature {
    // So ugly. I need to find a way to check if self.scale is to_scale but I can't find a way to do that in the match statement.
    // I want the logic to be:
    // If -for some dumb reason- you're asking to convert Temperature to the same TemperatureScale it already is: return self
    // else, do F -> C or C -> F conversions.
    fn convert_to(self, to_scale: TemperatureScales) -> Temperature {
        // The TemperatureScale we are going to
        match to_scale {
            TemperatureScales::Fahrenheit => {
                // The TemperatureScale we are coming from
                match self.scale {
                    // The TemperatureScale we are going to is the same as self
                    TemperatureScales::Fahrenheit => self,
                    TemperatureScales::Celsius => Temperature { 
                        value: (f64::from(self.value) * CELSIUS_TO_FAHRENHEIT_RATIO + FAHRENHEIT_WATER_FREEZE_TEMP) as i32, 
                        scale: TemperatureScales::Celsius, 
                        symbol: self.symbol 
                    },
                }
            }
            TemperatureScales::Celsius => {
                // The reverse of all of that ^^^
                match self.scale {
                    TemperatureScales::Fahrenheit => Temperature { 
                        value: ((f64::from(self.value) - FAHRENHEIT_WATER_FREEZE_TEMP) * FAHRENHEIT_TO_CELSIUS_RATIO) as i32, 
                        scale: TemperatureScales::Celsius, 
                        symbol: self.symbol, 
                    },
                    // The TemperatureScale we are going to is the same as self
                    TemperatureScales::Celsius => self,
                }
            },
        }
    }
}
