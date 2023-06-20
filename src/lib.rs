pub mod agent;
pub mod candid;
pub mod types;

pub use crate::candid::candid_module;
pub use agent::Agent;
use types::types_module;

use crate::types::principal::Principal;
use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
fn agent_py(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Agent>()?;
    m.add_class::<Principal>()?;

    candid_module(py, m)?;
    types_module(py, m)?;

    Ok(())
}
