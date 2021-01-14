#![warn(missing_docs)]

mod fields;
pub use fields::*;
pub mod display;
pub mod structs;
pub mod windows;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
