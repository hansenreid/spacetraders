pub mod register;
pub mod status;

pub struct ApiConfig {
    pub(crate) base_url: String,
}

impl ApiConfig {
    pub fn new(url: String) -> Self {
        Self { base_url: url }
    }
}
