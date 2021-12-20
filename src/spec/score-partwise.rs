#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct MovementNumber {
    #[serde(rename = "$value")]
    pub content: String,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct MovementTitle {
    #[serde(rename = "$value")]
    pub content: String,
}