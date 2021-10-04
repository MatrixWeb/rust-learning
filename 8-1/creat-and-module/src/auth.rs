pub struct User {
    username: String,
    password_hash: u64,
}

fn hash_password<T>(t: &T) -> u64 { 123u64 }


impl User {
    pub fn new(username: &str, passowrd: &str) -> User {
        User {
            username: username.to_string(),
            password_hash: hash_password(&passowrd.to_string()),
        }
    }

    pub fn get_username(&self) -> &String {
        &self.username
    }

    pub fn set_password(&mut self, new_passowrd: &str) {
        self.password_hash = hash_password(&new_passowrd.to_string())
    }
}