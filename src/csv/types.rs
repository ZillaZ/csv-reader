#[derive(Debug)]
pub struct User {
    pub email: String,
    pub password: String
}

impl User {
    pub fn new(email: &str, password: &str) -> Self {
        Self {
            email: email.into(),
            password: password.into()
        }
    }
}
