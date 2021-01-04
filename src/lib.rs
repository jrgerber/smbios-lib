mod fields;
pub use fields::*;
pub mod windows;
pub mod structs;
pub mod display;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
