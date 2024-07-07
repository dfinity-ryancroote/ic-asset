// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
use candid::{self, CandidType, Deserialize};

#[derive(CandidType, Deserialize)]
pub enum DataType {
    #[serde(rename = "new")]
    New,
    #[serde(rename = "append")]
    Append,
}
#[derive(CandidType, Deserialize)]
pub struct HeaderField(pub String, pub String);
#[derive(CandidType, Deserialize)]
pub struct HttpRequest {
    pub url: String,
    pub body: serde_bytes::ByteBuf,
    pub headers: Vec<HeaderField>,
}
#[derive(CandidType, Deserialize)]
pub struct HttpResponse<'a> {
    pub body: std::borrow::Cow<'a, serde_bytes::Bytes>,
    pub headers: Vec<HeaderField>,
    pub status_code: u16,
}
#[derive(CandidType, Deserialize)]
pub struct Item {
    pub key: String,
    pub len: u32,
    pub data_type: DataType,
}
#[derive(CandidType, Deserialize)]
pub struct Metadata {
    pub name: String,
    pub size: candid::Nat,
}
#[derive(CandidType, Deserialize)]
pub struct UploadData {
    pub blob: serde_bytes::ByteBuf,
    pub item: Vec<Item>,
}