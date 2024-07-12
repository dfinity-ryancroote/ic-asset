// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
#![allow(dead_code, unused_imports)]
use candid::{self, CandidType, Deserialize, Principal};
use ic_cdk::api::call::CallResult as Result;

#[derive(CandidType, Deserialize)]
pub enum DataType {
    #[serde(rename = "new")]
    New,
    #[serde(rename = "delete")]
    Delete,
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
    pub content_type: String,
    pub timestamp: candid::Nat,
    pub data_type: DataType,
}
#[derive(CandidType, Deserialize)]
pub struct Metadata {
    pub name: String,
    pub size: candid::Nat,
    pub timestamp: candid::Nat,
}
#[derive(CandidType, Deserialize)]
pub struct UploadData {
    pub blob: serde_bytes::ByteBuf,
    pub item: Vec<Item>,
}
