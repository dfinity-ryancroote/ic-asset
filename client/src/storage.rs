// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
#![allow(dead_code, unused_imports)]
use candid::{self, CandidType, Decode, Deserialize, Encode, Principal};
type Result<T> = std::result::Result<T, ic_agent::AgentError>;

#[derive(CandidType, Deserialize)]
pub struct Metadata {
    pub name: String,
    pub size: u128,
    pub timestamp: u128,
}
#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum DataType {
    #[serde(rename = "new")]
    New,
    #[serde(rename = "delete")]
    Delete,
    #[serde(rename = "append")]
    Append,
}
#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct Item {
    pub key: String,
    pub len: u32,
    pub timestamp: u128,
    pub data_type: DataType,
}
#[derive(CandidType, Deserialize)]
pub struct UploadData {
    pub blob: serde_bytes::ByteBuf,
    pub item: Vec<Item>,
}

pub struct Service<'a>(pub Principal, pub &'a ic_agent::Agent);
impl<'a> Service<'a> {
    pub async fn commit(&self) -> Result<()> {
        let args = Encode!()?;
        let bytes = self
            .1
            .update(&self.0, "commit")
            .with_arg(args)
            .call_and_wait()
            .await?;
        Ok(Decode!(&bytes)?)
    }
    pub async fn list(&self) -> Result<Vec<Metadata>> {
        let args = Encode!()?;
        let bytes = self.1.query(&self.0, "list").with_arg(args).call().await?;
        Ok(Decode!(&bytes, Vec<Metadata>)?)
    }
    pub async fn upload(&self, arg0: u32, arg1: UploadData) -> Result<()> {
        let args = Encode!(&arg0, &arg1)?;
        let bytes = self
            .1
            .update(&self.0, "upload")
            .with_arg(args)
            .call_and_wait()
            .await?;
        Ok(Decode!(&bytes)?)
    }
}
