#![recursion_limit="256"]
#[macro_use]
extern crate diesel;

pub mod schema;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
