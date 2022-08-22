use std::env;

pub fn no_env_var_msg<'a>(key: &'a str) -> String {
    format!("No env variable \"{}\" was provided", key)
}

pub fn to_telegram_username(username: &str) -> String {
    ["@", username].join("").to_string()
}

pub fn env_var<'a>(key: &'a str) -> String {
    env::var(key).expect(&no_env_var_msg(key))
}
