use base64::{Engine as _, engine::general_purpose};

pub fn encrypt_data(data: &str) -> String {
    general_purpose::STANDARD.encode(data.as_bytes())
}
