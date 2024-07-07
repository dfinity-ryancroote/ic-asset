use percent_encoding::percent_decode_str;
use serde_bytes::Bytes;
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::BTreeMap;

mod types;
use types::*;

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
    fn list(&self) -> Vec<Metadata> {
        self.map
            .iter()
            .map(|(name, data)| Metadata {
                name: name.clone(),
                size: data.len().into(),
            })
            .collect()
    }
    fn http_request(&self, req: HttpRequest) -> HttpResponse<'_> {
        let path = match req.url.find('?') {
            Some(i) => &req.url[..i],
            None => &req.url,
        };
        match percent_decode_str(path)
            .decode_utf8()
            .map(|s| s.into_owned())
        {
            Ok(path) => match self.map.get(&path).or_else(|| self.map.get("index.html")) {
                Some(blob) => HttpResponse {
                    body: Cow::Borrowed(Bytes::new(blob)),
                    headers: vec![],
                    status_code: 200,
                },
                None => HttpResponse {
                    body: Cow::Owned(format!("{path} not found").into_bytes().into()),
                    headers: vec![],
                    status_code: 404,
                },
            },
            Err(_) => HttpResponse {
                body: Cow::Owned(format!("invalid path {path}").into_bytes().into()),
                headers: vec![],
                status_code: 400,
            },
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
    })
}
#[ic_cdk::update]
fn commit() {
    STATE.with_borrow_mut(|state| {
        state.commit();
    })
}
#[ic_cdk::query(manual_reply = true)]
fn http_request(req: HttpRequest) {
    STATE.with_borrow(|state| {
        let res = state.http_request(req);
        ic_cdk::api::call::reply((res,))
    })
}
#[ic_cdk::query]
fn list() -> Vec<Metadata> {
    STATE.with_borrow(|state| state.list())
}
#[link_section = "icp:public candid:service"]
pub static __SERVICE: [u8; 630] = *br#"type data_type = variant { new; append };
type header_field = record { text; text };
type http_request = record {
  url : text;
  body : blob;
  headers : vec header_field;
};
type http_response = record {
  body : blob;
  headers : vec header_field;
  status_code : nat16;
};
type item = record { key : text; len : nat32; data_type : data_type };
type upload_data = record { "blob" : blob; item : vec item };
type metadata = record { name : text; size : nat };
service : {
  commit : () -> ();
  http_request : (http_request) -> (http_response) query;
  upload : (nat32, upload_data) -> ();
  list : () -> (vec metadata) query;
}"#;
