use wasm_bindgen::prelude::*;
use tokio::runtime::Runtime;
use tokio::task;

// Necessary to initialize the runtime for the browser environment
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    Ok(())
}

#[wasm_bindgen]
pub fn run_task() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let handle = task::spawn(async {
            println!("Hello from Tokio");
        });

        handle.await.unwrap();
    });
}
