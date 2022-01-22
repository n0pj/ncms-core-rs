pub mod authentications;
pub mod db;
pub mod errors;
pub mod http;
pub mod models;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
