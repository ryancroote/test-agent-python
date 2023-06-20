use pyo3::prelude::*;

use crate::types::principal::Principal;

#[pyclass]
pub struct Func {
    #[pyo3(get)]
    pub canister_id: Principal,
    #[pyo3(get)]
    pub method_name: String,
}

#[pymethods]
impl Func {
    #[new]
    pub fn new(canister_id: Principal, method_name: String) -> Self {
        Self {
            canister_id,
            method_name,
        }
    }
}
