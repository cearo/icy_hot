pub mod icy;
pub mod temperature_symbols;
use crate::icy::temperatures::*;
use crate::temperature_symbols::TemperatureSymbols;

// #TODO
// Implement Temperature::parse(File) -> Vec<Temperature>

fn main() {
    let temp_f = Temperature {
        value: 0,
        scale: TemperatureScales::Celsius,
        symbol: TemperatureSymbols::DegreesChar,
    };
    // Want to add an overload: convert_to(TemperatureScale, TemperatureSymbol)
    print!("{:#?}", &temp_f);
    let new_temp = temp_f.convert_to(TemperatureScales::Fahrenheit);
    let compare_temp = Temperature {
        value: 32,
        scale: TemperatureScales::Fahrenheit,
        symbol: TemperatureSymbols::DegreesChar,
    };
    let eq_test = new_temp == compare_temp;
    let clone_test = new_temp.clone();
    println!("New Temp Debug: {:#?}", &new_temp);
    //println!("New Temp Display: {}", new_temp);
    println!("Clone Test: {:#?}", clone_test);
    println!("Equality Test: {}", eq_test);
    println!("Char to Symbol enum: {:#?}", TemperatureSymbols::from("°"));
}
