pub struct Api {
    pub api_key: String,
}

impl Api {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }

    pub fn get_api_key(&self) -> &str {
        &self.api_key
    }
}