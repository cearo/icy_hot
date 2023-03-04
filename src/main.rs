
// Math
const FAHRENHEIT_WATER_FREEZE_TEMP: f64 = 32.0;
const FAHRENHEIT_TO_CELSIUS_RATIO: f64 = 5.0 / 9.0;
const CELSIUS_TO_FAHRENHEIT_RATIO: f64 = 9.0 / 5.0;

fn main() {
    let tempF = Temperature {value: 0, scale: TemperatureScales::Celsius, symbol: TemperatureSymbols::Degrees};
    let newTemp = tempF.convertTo(TemperatureScales::Fahrenheit);
    println!("{:#?}", newTemp);
    println!("{new_temp_val}{new_deg_sym}{new_deg_scale}",
    new_temp_val = newTemp.value, new_deg_sym = newTemp.symbol.value(), new_deg_scale = newTemp.scale.value());
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
    fn convertTo(self, toScale: TemperatureScales) -> Temperature {
        match toScale {
            TemperatureScales::Fahrenheit => {
                match self.scale {
                    TemperatureScales::Fahrenheit => self,
                    TemperatureScales::Celsius => Temperature { 
                        value: (f64::from(self.value) * CELSIUS_TO_FAHRENHEIT_RATIO + FAHRENHEIT_WATER_FREEZE_TEMP) as i32, 
                        scale: TemperatureScales::Celsius, 
                        symbol: self.symbol },
                }
            }
            TemperatureScales::Celsius => todo!(),
        }
    }
}
// fn to convert between temperatures
// fn to convert from F -> C
// fn to convert from C -> 
