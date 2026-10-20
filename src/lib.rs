mod engine;
mod syntax;
pub use engine::{simulate, Report};
pub use syntax::{parse, Distribution, Model, Station};

pub fn run(source: &str, seed: u32) -> Result<Report, String> {
    simulate(&parse(source)?, seed)
}

// Single-threaded browser ABI. Buffers remain owned by Rust; JS copies output.
#[cfg(target_arch = "wasm32")]
mod wasm {
    use std::cell::RefCell;
    thread_local! {
        static INPUT: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
        static OUTPUT: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
    }
    #[no_mangle]
    pub extern "C" fn input_buffer(len: usize) -> *mut u8 {
        if len > 65536 {
            return std::ptr::null_mut();
        }
        INPUT.with(|b| {
            let mut b = b.borrow_mut();
            b.resize(len, 0);
            b.as_mut_ptr()
        })
    }
    #[no_mangle]
    pub extern "C" fn execute(seed: u32) {
        let result = INPUT.with(|b| {
            let b = b.borrow();
            match std::str::from_utf8(&b)
                .map_err(|e| e.to_string())
                .and_then(|s| super::run(s, seed))
            {
                Ok(report) => serde_json::json!({"ok": true, "report": report}),
                Err(error) => serde_json::json!({"ok": false, "error": error}),
            }
        });
        OUTPUT.with(|b| *b.borrow_mut() = result.to_string().into_bytes());
    }
    #[no_mangle]
    pub extern "C" fn output_pointer() -> *const u8 {
        OUTPUT.with(|b| b.borrow().as_ptr())
    }
    #[no_mangle]
    pub extern "C" fn output_length() -> usize {
        OUTPUT.with(|b| b.borrow().len())
    }
}
