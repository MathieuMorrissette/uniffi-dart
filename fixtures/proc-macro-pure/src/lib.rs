// Pure proc-macro example - no UDL file needed!
uniffi::setup_scaffolding!();

#[derive(uniffi::Record)]
pub struct Person {
    pub name: String,
    pub age: u32,
}

#[derive(uniffi::Enum)]
pub enum UserStatus {
    Active,
    Inactive,
    Pending,
}

#[uniffi::export]
pub fn create_person(name: String, age: u32) -> Person {
    Person { name, age }
}

#[uniffi::export]
pub fn greet(person: Person) -> String {
    format!("Hello, {}! You are {} years old.", person.name, person.age)
}

#[uniffi::export]
pub fn status_to_string(user_status: UserStatus) -> String {
    match user_status {
        UserStatus::Active => "Active".to_string(),
        UserStatus::Inactive => "Inactive".to_string(),
        UserStatus::Pending => "Pending".to_string(),
    }
}

#[uniffi::export(default(iterations = 10000, length = 32))]
pub fn hash_data(data: Option<Vec<u8>>, iterations: u32, length: u32) -> Vec<u8> {
    // Simple mock implementation for testing
    let mut result = vec![0u8; length as usize];
    for i in 0..iterations.min(result.len() as u32) {
        if let Some(d) = data.as_ref() {
            if let Some(&b) = d.get(i as usize % d.len()) {
                result[i as usize] = b.wrapping_add(i as u8);
            }
        }
    }
    result
}

use std::sync::Mutex;

#[derive(uniffi::Object)]
pub struct Counter {
    value: Mutex<i32>,
}

#[uniffi::export]
impl Counter {
    #[uniffi::constructor]
    fn new(initial: i32) -> Self {
        Counter {
            value: Mutex::new(initial),
        }
    }

    fn increment(&self) {
        let mut value = self.value.lock().unwrap();
        *value += 1;
    }

    fn get_value(&self) -> i32 {
        *self.value.lock().unwrap()
    }
}
