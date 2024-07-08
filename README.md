## ic-asset

An extremely simplified implementation of asset canister. Client batches files in 2M chunks, efficient when files are small. Also built as a case study for the `cargo canister bindgen` command.

### Not implemented

* No access control
* No certification
* No streaming http request
* Only identity encoding
* File equality is decided by size and modified timestamp, not SHA256

### TODO for canister bindgen

* Better template for stub. Ideally wrapping state inside a struct, emitting types in a separate module. It can be difficult for async calls.
* Add a start symbol for match path
