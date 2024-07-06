use std::collections::BTreeMap;
use serde_bytes::ByteBuf;
use candid::{CandidType, Deserialize};
use std::cell::RefCell;

#[derive(CandidType, Deserialize)]
pub struct UploadData {
    // assume item is sorted by end_index
    pub item: Vec<Item>,
    pub blob: ByteBuf,
}
#[derive(CandidType, Deserialize)]
pub struct Item {
    pub key: String,
    pub len: u32,
    pub data_type: DataType,
}
#[derive(CandidType, Deserialize)]
pub enum DataType {
    #[serde(rename = "new")]
    New,
    #[serde(rename = "append")]
    Append,
}

#[derive(Default)]
struct State {
    map: BTreeMap<String, Vec<u8>>,
    buffer: BTreeMap<u32, UploadData>,
}

impl State {
    fn upload(&mut self, id: u32, data: UploadData) {
        self.buffer.insert(id, data);
    }
    fn commit(&mut self) {
        use std::collections::btree_map::Entry;
        while let Some((_, mut data)) = self.buffer.pop_first() {
            for item in data.item {
                let blob = data.blob.drain(..item.len as usize);
                match (item.data_type, self.map.entry(item.key)) {
                    (DataType::Append, Entry::Occupied(mut v)) => v.get_mut().extend(blob),
                    (DataType::New, Entry::Occupied(mut v)) => *v.get_mut() = blob.collect(),
                    (DataType::New, Entry::Vacant(v)) => drop(v.insert(blob.collect())),
                    (DataType::Append, Entry::Vacant(_)) => panic!("append to non-exist key"),
                }
            }
        }
    }
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}
#[ic_cdk::update]
fn upload(id: u32, data: UploadData) {
    STATE.with_borrow_mut(|state| {
        state.upload(id, data);
    });
}
#[ic_cdk::update]
fn commit() {
    STATE.with_borrow_mut(|state| {
        state.commit();
    });
}
#[link_section = "icp:public candid:service"]
pub static __SERVICE: [u8; 243] = *br#"type data_type = variant { new; append };
type item = record { key : text; len : nat32; data_type : data_type };
type upload_data = record { "blob" : blob; item : vec item };
service : { commit : () -> (); upload : (nat32, upload_data) -> () }"#;
