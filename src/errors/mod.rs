pub mod authorization;
pub mod database;
pub mod http;
pub mod validation;

pub struct Error<'a> {
    pub code: &'a str,
    pub message: &'a str,
}
