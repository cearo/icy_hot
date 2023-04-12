pub mod temperatures {
    use crate::temperature_symbols::TemperatureSymbols;

    // TODO
    // Implement From Trait (and the like) to the TemperatureScale rather than Temperature having to_{scale}()
    /// Leave convert_to() as wrapper in Temperature to call TemperatureScale::from()

    // Scales
    const FAHRENHEIT_SCALE_SYMBOL: char = 'F';
    const CELSIUS_SCALE_SYMBOL: char = 'C';

    // Data type to represent temperature scales
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum TemperatureScales {
        Fahrenheit,
        Celsius,
    }
    impl TemperatureScales {
        pub fn value(&self) -> char {
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
    // #[derive(Debug, Clone, PartialEq, Eq)]
    // enum TemperatureSymbols {
    //     Degrees,
    //     Asterisk,
    //     Space,
    // }
    // impl TemperatureSymbols {
    //     fn value(&self) -> char {
    //         match self {
    //             TemperatureSymbols::Degrees => DEGREE_SYMBOL,
    //             TemperatureSymbols::Asterisk => DEGREE_ASTERISK_SYMBOL,
    //             TemperatureSymbols::Space => DEGREE_SPACE_SYMBOL,
    //         }
    //     }
    // }

    // Math
    const FAHRENHEIT_WATER_FREEZE_TEMP: f64 = 32.0;
    const CELSIUS_TO_FAHRENHEIT_RATIO: f64 = 9.0 / 5.0;
    const FAHRENHEIT_TO_CELSIUS_RATIO: f64 = 5.0 / 9.0;

    // Data type to represent temperatures
    // Implement builder pattern: Temperature.value(55).asterisk().to_celsius()
    // Eventually Temperature.add(Temperature(55).to_celsius(), Temperature(100).to_fahrenheit()).to_celsius()
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Temperature {
        pub value: i32,
        pub scale: TemperatureScales,
        pub symbol: TemperatureSymbols,
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
            match self.scale {
                TemperatureScales::Fahrenheit => self,
                TemperatureScales::Celsius => Temperature {
                    value: (f64::from(self.value) * CELSIUS_TO_FAHRENHEIT_RATIO
                        + FAHRENHEIT_WATER_FREEZE_TEMP) as i32,
                    scale: self.scale,
                    symbol: self.symbol,
                },
            }
        }

        pub fn to_celsius(self) -> Temperature {
            match self.scale {
                TemperatureScales::Fahrenheit => Temperature {
                    value: ((f64::from(self.value) - FAHRENHEIT_WATER_FREEZE_TEMP)
                        * FAHRENHEIT_TO_CELSIUS_RATIO) as i32,
                    scale: self.scale,
                    symbol: self.symbol,
                },
                TemperatureScales::Celsius => self,
            }
        }

        pub fn clone_to(&self, to_scale: TemperatureScales) -> Temperature {
            if self.scale == to_scale {
                return self.clone();
            }

            match to_scale {
                TemperatureScales::Fahrenheit => self.clone().to_fahrenheit(),
                TemperatureScales::Celsius => self.clone().to_celsius(),
            }
        }
    }
    impl Default for Temperature {
        fn default() -> Self {
            Self {
                value: 0,
                scale: TemperatureScales::Celsius,
                symbol: TemperatureSymbols::DegreesChar("°".to_owned()),
            }
        }
    }
    // impl std::fmt::Display for Temperature {
    //     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    //         let symbol = TemperatureSymbols::from('°');
    //         write!(
    //             f,
    //             "{}{}{}",
    //             self.value,
    //             symbol,
    //             self.scale.value()
    //         )
    //     }
    //}
}
