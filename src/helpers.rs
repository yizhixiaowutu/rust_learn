pub mod name_helpers {
    pub fn get_full_name(first: &str, last: &str) -> String {
        let full_name = format!("{0}-{1}", first, last);
        full_name
    }
}

pub mod database {
    // some database related functions
}

pub mod private_module {
    pub fn get_age_plus_5(age: u8) -> u8 {
        age + 5
    }
}
