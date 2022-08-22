use crate::app::models::Credentials;

pub fn check_login<'a>(creds: &Credentials, (username, token): (&'a str, &'a str)) -> bool {
  creds.username() == username && creds.token() == token
}
