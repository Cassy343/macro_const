#![no_std]

/*!
Creates corresponding macro definitions for constants, allowing the value of the constants
to be used in the context of other macros.

# Examples

```ignore
macro_const! {
    const FILE: &str = "message.txt";
}

println!("Contents: {}", include_str!(FILE!()));
println!("File: {}", FILE);
```

Doc comments can be added as well. The documentation for the constant will be added to the
generated macro verbatim. To export the generated macro, simply add the `#[macro_export]`
attribute to the constant definition. 

```
# #[macro_use]
# extern crate macro_const;
# fn main() {
macro_const! {
    /// The API base URL.
    #[macro_export]
    pub const BASE_URL: &str = "https://myapi.io/";

    /// The current supported API version.
    pub const API_VERSION: &str = "v1";
}

assert_eq!("https://myapi.io/v1", concat!(BASE_URL!(), API_VERSION!()));
# }
```
*/

/// Generates corresponding macros for constants that evaluate to the same values as the constants.
#[macro_export]
macro_rules! macro_const {
    ($($(#[$attr:meta])* $vis:vis const $name:ident : $type:ty = $value:expr ;)+) => {
        $(
            #[allow(unused_attributes)]
            $( #[$attr] )* $vis const $name : $type = $value;

            #[allow(unused_attributes)]
            $( #[$attr] )*
            macro_rules! $name {
                () => {
                    $value
                }
            }
        )+
    };
}
