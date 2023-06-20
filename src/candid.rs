pub mod decode;
pub mod def;
pub mod encode;

use pyo3::prelude::*;

use self::{decode::DecodeOptions, def::Definition};

#[pymodule]
pub fn candid_module(py: Python<'_>, parent_module: &PyModule) -> PyResult<()> {
    let candid_module = PyModule::new(py, "candid")?;
    candid_module.add_function(wrap_pyfunction!(decode::decode, candid_module)?)?;
    candid_module.add_function(wrap_pyfunction!(encode::encode, candid_module)?)?;
    candid_module.add_class::<Definition>()?;
    candid_module.add_class::<DecodeOptions>()?;
    parent_module.add_submodule(candid_module)?;
    Ok(())
}
