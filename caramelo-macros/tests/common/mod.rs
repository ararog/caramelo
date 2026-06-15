#[derive(Debug)]
pub struct User {
    pub name: String,
    pub age: u32,
    pub city: City,
}

#[derive(Debug, PartialEq)]
pub struct City {
    pub name: String,
    pub state: String,
}

#[allow(dead_code)]
impl User {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn age(&self) -> u32 {
        self.age
    }

    pub fn city(&self) -> &City {
        &self.city
    }
}

#[allow(dead_code)]
impl City {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn state(&self) -> &str {
        &self.state
    }
}

pub fn create_user() -> User {
    User {
        name: "John".to_string(),
        age: 30,
        city: City { name: "New York".to_string(), state: "NY".to_string() },
    }
}
