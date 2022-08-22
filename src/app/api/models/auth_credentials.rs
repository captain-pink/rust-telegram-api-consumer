#[derive(Clone, Debug)]
pub struct AuthCredentials {
    username: String,
    password: String,
}

impl AuthCredentials {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }

    pub fn read(&self) -> (String, String) {
        (self.username.clone(), self.password.clone())
    }
}
