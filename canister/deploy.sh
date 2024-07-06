#!ic-repl
function deploy_new(wasm) {
  let id = call ic.provisional_create_canister_with_cycles(record { settings = null; amount = null });
  let _ = deploy(id.canister_id, wasm);
  id.canister_id
};
function deploy(id, wasm, mode) {
  call ic.install_code(
    record {
      arg = encode wasm.__init_args();
      wasm_module = wasm;
      mode = mode;
      canister_id = id;
    },
  );
};
let id = principal "bkyz2-fmaaa-aaaaa-qaaaq-cai";
let wasm = file("../target/wasm32-unknown-unknown/release/chunked_map.wasm");
deploy(id, wasm, variant { reinstall });


