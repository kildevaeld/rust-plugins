pub extern crate libloading;
pub extern crate uuid;
#[macro_use]
extern crate error_chain;

mod error;
mod macros;
mod types;

pub use error::*;
pub use types::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
