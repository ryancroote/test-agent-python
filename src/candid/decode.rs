use candid::types::Label;
use candid::{parser::value::IDLValue, IDLArgs};

use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::types::func::Func;
use crate::types::principal::Principal;

use super::def::Definition;

#[derive(Clone)]
#[pyclass]
pub struct DecodeOptions {
    definition: Definition,
    type_names: Vec<String>,
}

#[pymethods]
impl DecodeOptions {
    #[new]
    pub fn new(definition: Definition, type_names: Vec<String>) -> Self {
        Self {
            definition,
            type_names,
        }
    }
}

#[pyfunction]
pub fn decode<'p>(py: Python<'p>, bytes: &[u8], options: Option<DecodeOptions>) -> Vec<&'p PyAny> {
    let args = if let Some(options) = options {
        let env = options.definition.env();
        let types = options
            .type_names
            .iter()
            .map(|type_name| {
                env.find_type(type_name)
                    .cloned()
                    .expect("failed to find type")
            })
            .collect::<Vec<_>>();
        IDLArgs::from_bytes_with_types(bytes, env, &types).expect("Failed to decode")
    } else {
        IDLArgs::from_bytes(bytes).unwrap()
    };

    args_to_python(py, &args)
}

fn args_to_python<'p>(py: Python<'p>, args: &IDLArgs) -> Vec<&'p PyAny> {
    args.args
        .iter()
        .map(|arg| to_py_object(py, arg))
        .collect::<Vec<_>>()
}

fn to_py_object<'p>(py: Python<'p>, value: &IDLValue) -> &'p PyAny {
    let r = match value {
        IDLValue::Bool(b) => b.to_object(py).into_ref(py),
        IDLValue::Null => py.None().into_ref(py),
        IDLValue::Text(s) => s.to_object(py).into_ref(py),
        IDLValue::Number(s) => s.to_object(py).into_ref(py),
        IDLValue::Float64(f) => f.to_object(py).into_ref(py),
        IDLValue::Opt(value) => to_py_object(py, value),
        IDLValue::Vec(v) => v
            .into_iter()
            .map(|value| to_py_object(py, value))
            .collect::<Vec<_>>()
            .to_object(py)
            .into_ref(py),
        IDLValue::Record(fields) => {
            let dict = PyDict::new(py);
            fields.into_iter().for_each(|field| {
                let id = match &field.id {
                    Label::Id(i) => i.to_object(py).into_ref(py),
                    Label::Named(label) => label.to_object(py).into_ref(py),
                    Label::Unnamed(i) => i.to_object(py).into_ref(py),
                };
                let value = to_py_object(py, &field.val);
                dict.set_item(id, value).ok();
            });
            dict
        }
        IDLValue::Variant(field) => {
            let dict = PyDict::new(py);
            let id = match &field.0.id {
                Label::Id(i) => i.to_object(py).into_ref(py),
                Label::Named(label) => label.to_object(py).into_ref(py),
                Label::Unnamed(i) => i.to_object(py).into_ref(py),
            };
            let value = to_py_object(py, &field.0.val);
            dict.set_item(id, value).ok();
            dict
        }
        IDLValue::Principal(p) => PyCell::new(py, Principal::from_principal(p.clone())).unwrap(),
        IDLValue::Service(p) => PyCell::new(py, Principal::from_principal(p.clone())).unwrap(),
        IDLValue::Func(canister_id, method_name) => {
            let canister_id = Principal::from_principal(canister_id.clone());
            let func = Func::new(canister_id, method_name.clone());
            PyCell::new(py, func).unwrap()
        }
        IDLValue::None => py.None().into_ref(py),
        IDLValue::Int(i) => i.0.to_object(py).into_ref(py),
        IDLValue::Nat(n) => n.0.to_object(py).into_ref(py),
        IDLValue::Nat8(n) => n.to_object(py).into_ref(py),
        IDLValue::Nat16(n) => n.to_object(py).into_ref(py),
        IDLValue::Nat32(n) => n.to_object(py).into_ref(py),
        IDLValue::Nat64(n) => n.to_object(py).into_ref(py),
        IDLValue::Int8(i) => i.to_object(py).into_ref(py),
        IDLValue::Int16(i) => i.to_object(py).into_ref(py),
        IDLValue::Int32(i) => i.to_object(py).into_ref(py),
        IDLValue::Int64(i) => i.to_object(py).into_ref(py),
        IDLValue::Float32(f) => f.to_object(py).into_ref(py),
        IDLValue::Reserved => todo!(),
    };
    r
}
