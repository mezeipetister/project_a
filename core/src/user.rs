pub struct User {
    id: Option<String>,
    name: Option<String>,
    email: Option<String>,
    address: Option<String>,
    password: Option<String>,
}

impl User {
    fn new() -> Self {
        User {
            id: None,
            name: None,
            email: None,
            address: None,
            password: None,
        }
    }
}

pub trait TUser {
    fn get_id(&self) -> String;
}
