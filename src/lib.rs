//! A documented subset of QNAP2 for queueing network simulation.
//!
//! Parse and run a model with an explicit random seed:
//!
//! ```
//! let source = r#"
//! /DECLARE/
//! QUEUE INPUT, DESK;
//! /STATION/
//! NAME=INPUT;
//! TYPE=SOURCE;
//! SERVICE=EXP(2);
//! TRANSIT=DESK;
//! /STATION/
//! NAME=DESK;
//! TYPE=SERVER,SINGLE;
//! SERVICE=EXP(1);
//! TRANSIT=OUT;
//! /CONTROL/
//! TMAX=10000;
//! /EXEC/
//! BEGIN
//! SIMUL;
//! END;
//! /END/
//! "#;
//! let report = qrunap2::run(source, 42)?;
//! assert_eq!(report.seed, 42);
//! assert_eq!(report.stations.len(), 2);
//! # Ok::<(), String>(())
//! ```
//!
//! This crate implements a limited language subset, not full QNAP2 compatibility.
//! See the [compatibility guide](https://github.com/Medialoco/QRUNAP2/blob/main/docs/compatibility.md)
//! for supported constructs and limitations.

mod engine;
mod syntax;
pub use engine::{simulate, Report, StationReport, Trace};
pub use syntax::{parse, Distribution, Model, Station};

/// Parse a QNAP2-subset model and simulate it using the supplied random seed.
///
/// Returns an error if the source is invalid, uses unsupported constructs,
/// or simulation validation or resource limits fail.
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
