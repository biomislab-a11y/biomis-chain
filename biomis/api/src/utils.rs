pub fn serialize_data<T: serde::Serialize>(data: &T) -> String {
    serde_json::to_string(data).unwrap()
}
