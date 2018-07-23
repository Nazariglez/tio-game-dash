#[macro_use] extern crate lazy_static;
extern crate regex;

use regex::Regex;

lazy_static! {
    //https://www.w3.org/TR/html51/sec-forms.html#valid-e-mail-address
    static ref EMAIL_REGEX: Regex = Regex::new("^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$").unwrap();
}

pub fn is_valid_email(email: &str) -> bool {
    EMAIL_REGEX.is_match(email)
}

