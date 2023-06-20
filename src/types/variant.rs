use candid::parser::value::IDLField;
use pyo3::prelude::*;

#[derive(Clone, FromPyObject)]
pub enum VariantField {
    Int(u32),
    String(String),
}

#[derive(Clone)]
#[pyclass]
pub struct Variant {
    #[pyo3(get)]
    pub value: VariantField,
    #[pyo3(get)]
    pub object: Option<PyObject>,
}

#[pymethods]
impl Variant {
    #[new]
    pub fn new(value: VariantField, object: Option<PyObject>) -> Self {
        Self { value, object }
    }
}
