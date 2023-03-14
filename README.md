# icy_hot - A Rust Program To Work With Temperatures

> Note: I am a Rust newb and an intermediate programmer. This is not currently a serious crate but merely a different way to tackle a common introductory Rust program. I am open to all constructive criticism and I am open to a mentor. 

This application was created from a prompt to "Convert temperatures between Fahrenheit to Celsius" at the end of Chapter 3 - Common Programming Concepts in [The Rust Programming Language - by Steve Klabnik and Carol Nichols](https://read.amazon.com/kp/embed?asin=B0B7QTX8LL&preview=newtab&linkCode=kpe&ref_=cm_sw_r_kb_dp_49QFT39CEFZ338ZXA3X4).

## Description
>
> Note: I'm a recovering Java developer so, pleasePardonMyStyleHabit if you see it somewhere in my code: i_am_learning.

My goal for this project is to create a library style application to provide data structures and methods for working with temperatures in Rust, rather than a typical straight forward program. My intent is to iterate on this project as I read more of this book and learn more about Rust in general.

One of my main design considerations was that I don't want to work with the `char` values like `F`, `C`, or `°` and so I used that as an excuse to try my hand at extending the Rust type system to that advantage. My plan was to wrap the `char` values in `enum`s for comparison and representation in code with a `value(&self) -> char` method to return the backing data type in case it's needed. I just didn't want to see this in my code

```rust
{
    match char_value {
        `F` => do_something(),
        `C` => do_something_else(),
    }
}
```

But instead

```rust
{
    match temp_scale {
        TemperatureScale::Fahrenheit => do_something(),
        TemperatureScale::Celsius => do_something_else(),
    }
}
```

The API consists of the following data structures:
> Please note: I don't know how Modules work yet. I was having a hard time with them and decided to move on for now. All code is contained in main.rs for now.

### enum TemperatureSymbols

 `TemperatureSymbols` represent the symbols/signs used when working with/referring to temperatures. This version currently supports 3 symbols:

```rust
{
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
```

#### Degrees - DEGREE_SYMBOL

This `enum` variant wraps and represents the character `°` when `value()`.

#### Asterisk - DEGREE_ASTERISK

This `enum` variant wraps and represents the character `*` when `value()` is called.

#### Space - DEGREE_SPACE_SYMBOL

This `enum` variant wraps and represents the `space` character when `value()` is called.

#### "Degrees" String (TODO)

>Not yet implemented

This `enum` variant *WILL* wrap and represent the String `"Degrees"` when `value()` is called.
*Ex: 76 degrees Fahrenheit*

### enum TemperatureScales

`TemperatureScales` is a data type representing the different temperature scales, or methods for measuring temperature. This version currently supports Fahrenheit and Celsius, with Kelvin coming in the future.

```rust
{
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
}
```

#### Fahrenheit

This `enum` variant represents the Fahrenheit temperature scale and wraps the character `F`, which is returned when `value()` is called.

#### Celsius

This `enum` variant represents the Fahrenheit temperature scale and wraps the character `C`, which is returned when `value()` is called.

#### Kelvin (TODO)

>Not yet implemented

### struct Temperature

This `struct` wraps the `TemperatureScales` and `TemperatureSymbols` `enums` along with an `i32` `value` field to represent the measurement. Currently the `Temperature` API supports:

*Methods described in more detail below*

* **Converting** a `Temperature` between Fahrenheit and Celsius via dedicated methods `to_fahrenheit()` and `to_celsius()` or a more programmatically flexible `convert_to(TemperatureScales)`.

* **Cloning** a `Temperature` to Fahrenheit or Celsius via the `derived` `clone()` method or the `clone_to(TemperatureScales)` method.

* Displaying temperatures in a recognizable and simple format with the `Display` `trait` implementation (*Ex: 76°F*).

In the future I plan to implement methods to do the following:

* Kelvin implementations that work with Fahrenheit and Celsius.
* A builder implementation for `Temperature`
* Parse `Temperatures` from text.
* Arithmetic Operations on `Temperatures`.
* Arithmetic operations on `Temperatures` of different scales.

```rust
{
    #[derive(Debug, Clone, PartialEq, Eq)]
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
}
```

#### value: i32

An `integer` value representing the temperature measurement.

#### scale: TemperatureScales

See TemperatureScales above.

#### symbol: TemperatureSymbols

See TemperatureSymbols above.

### impl Temperature

#### fn convert_to(self, to_scale: TemperatureScales) -> Temperature

* `Temperature::value` is `moved` and converted based on the `to_scale` and the `self.scale`. If `to_scale` matches `self.scale`, `self` is returned.
* `Temperature::scale` is assigned to `to_scale`.
* `Temperature::symbol` the value of `self.symbol` is `moved` into the new `Temperature`.

This method was created to play around with the difference between `moving` and `borrowing` - this being the `moving` implementation. `convert_to(self, TemperatureScale)` will consume the current `Temperature` instance to create a new instance matching the scale of `to_scale` with its `value` converted accordingly.

This method is a wrapper for `to_fahrenheit()` and `to_celsius()` and was envisioned as a more programmatically friendly way to dynamically convert temperatures without having to `match` a scale and call the respective method to convert - `convert_to` does that for you!

#### fn to_fahrentheit(self) -> Temperature

* `Temperature::value` is `moved` and converted based on `self.scale` to `Fahrenheit`.
* `Temperature::scale` the value of `self.scale` is `moved` into the new `Temperature`.
* `Temperature::symbol` the value of `self.symbol` is `moved` into the new `Temperature`.

This method consumes the current Temperature instance and `moves` its data into a new `Temperature` instance with a `value` appropriately converted to `Fahrenheit`.

#### fn to_celsius(self) -> Temperature

* `Temperature::value` is `moved` and converted based on `self.scale` to `Celsius`.
* `Temperature::scale` the value of `self.scale` is `moved` into the new `Temperature`.
* `Temperature::symbol` the value of `self.symbol` is `moved` into the new `Temperature`.

This method consumes the current Temperature instance and `moves` its data into a new `Temperature` instance with a `value` appropriately converted to `Celsius`.

#### fn clone_to(&self, to_scale: TemperatureScales) -> Temperature

* All data is cloned using the `Clone` trait implementation.

This method was created to play around with the `move` and `borrow` systems where this implementation is `borrow`. The intent of this method is to be able to clone a current `Temperature` instance and convert it to the `to_scale` leaving the original instance available for use.

This method uses `clone()` to create a clone instance and then `to_{scale}()` to consume the clone instance and make a new one converted to the `to_scale`. If `self.scale` is equal to `to_scale`, `self.clone()` is returned.

#### fn default() -> Self

This method implements a default `Temperature` instance of:

```rust
{
    value: 0,
    scale: TemperatureScales::Celsius,
    symbol: TemperatureSymbols::Degrees,
}
```

#### Display impl

This `Display` implementation for the `Temperature` struct results in an easily readable and recognizable format of `{value}{symbol}{scale}` which looks like `76°F`.

## Halp

As a Rust newb and a learning Software Engineer, I accept all help offered. If you are a mentor and interested in mentee who is a fast learner and very interested in Rust, please reach out!

## Authors

Cory Robinson

cearobinson@gmail.com

## Version History

* 0.1: This version meets the requirements laid out by the challenge prompt of converting temperatures between Fahrenheit and Celsius.

## Acknowledgments

Inspiration, code snippets, etc.

* [The Rust Book](https://doc.rust-lang.org/book/title-page.html)
* [The Rust Programming Language - by Steve Klabnik and Carol Nichols](https://read.amazon.com/kp/embed?asin=B0B7QTX8LL&preview=newtab&linkCode=kpe&ref_=cm_sw_r_kb_dp_49QFT39CEFZ338ZXA3X4)

* [Markdown Cheat Sheet](https://www.markdownguide.org/cheat-sheet/)
