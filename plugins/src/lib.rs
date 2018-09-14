pub extern crate libloading;
pub extern crate uuid;
#[macro_use]
extern crate error_chain;

pub mod error;
pub mod macros;
pub mod types;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
