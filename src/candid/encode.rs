use candid::{
    parser::value::{IDLField, IDLValue, VariantValue},
    types::Label,
    IDLArgs,
};
use pyo3::{
    exceptions::PyTypeError,
    prelude::*,
    types::{PyBool, PyBytes, PyDict, PyFloat, PyInt, PyList, PyString, PyTuple},
};

use crate::types::{
    int::{Int, Int16, Int32, Int64, Int8},
    nat::{Nat, Nat16, Nat32, Nat64, Nat8},
    variant::{Variant, VariantField},
};

#[pyfunction]
pub fn encode<'p>(py: Python<'p>, tuple: &'p PyTuple) -> PyResult<&'p PyBytes> {
    let values = tuple
        .into_iter()
        .map(|v| to_idl_value(py, v))
        .collect::<PyResult<Vec<IDLValue>>>()?;
    let args = IDLArgs::new(&values);

    let bytes = args
        .to_bytes()
        .map_err(|e| PyTypeError::new_err(e.to_string()))?;

    Ok(PyBytes::new(py, &bytes))
}

fn to_idl_value<'p>(py: Python<'p>, object: &PyAny) -> PyResult<IDLValue> {
    let value = if object.is_none() {
        IDLValue::Null
    } else if object.is_instance_of::<PyBool>()? {
        let b = object.extract::<bool>()?;
        IDLValue::Bool(b)
    } else if object.is_instance_of::<Nat>()? {
        let n = object.extract::<Nat>()?;
        IDLValue::Nat(candid::Nat(n.0))
    } else if object.is_instance_of::<Nat8>()? {
        let n = object.extract::<Nat8>()?;
        IDLValue::Nat8(n.0)
    } else if object.is_instance_of::<Nat16>()? {
        let n = object.extract::<Nat16>()?;
        IDLValue::Nat16(n.0)
    } else if object.is_instance_of::<Nat32>()? {
        let n = object.extract::<Nat32>()?;
        IDLValue::Nat32(n.0)
    } else if object.is_instance_of::<Nat64>()? {
        let n = object.extract::<Nat64>()?;
        IDLValue::Nat64(n.0)
    } else if object.is_instance_of::<PyInt>()? {
        let i = object.extract::<num_bigint::BigInt>()?;
        IDLValue::Int(candid::Int(i))
    } else if object.is_instance_of::<Int>()? {
        let i = object.extract::<Int>()?;
        IDLValue::Int(candid::Int(i.0))
    } else if object.is_instance_of::<Int8>()? {
        let n = object.extract::<Int8>()?;
        IDLValue::Int8(n.0)
    } else if object.is_instance_of::<Int16>()? {
        let n = object.extract::<Int16>()?;
        IDLValue::Int16(n.0)
    } else if object.is_instance_of::<Int32>()? {
        let n = object.extract::<Int32>()?;
        IDLValue::Int32(n.0)
    } else if object.is_instance_of::<Int64>()? {
        let n = object.extract::<Int64>()?;
        IDLValue::Int64(n.0)
    } else if object.is_instance_of::<PyString>()? {
        let s = object.extract::<String>()?;
        IDLValue::Text(s)
    } else if object.is_instance_of::<PyFloat>()? {
        let f = object.extract::<f64>()?;
        IDLValue::Float64(f)
    } else if object.is_instance_of::<PyList>()? {
        let list = object
            .downcast::<PyList>()?
            .iter()
            .map(|v| to_idl_value(py, v))
            .collect::<PyResult<Vec<_>>>()?;
        IDLValue::Vec(list)
    } else if object.is_instance_of::<PyDict>()? {
        let mut list = object
            .downcast::<PyDict>()?
            .iter()
            .map(|(key, value)| {
                let id = if key.is_instance_of::<PyString>()? {
                    let id = key.extract::<String>()?;
                    Label::Named(id)
                } else if key.is_instance_of::<PyInt>()? {
                    let id = key.extract::<u32>()?;
                    Label::Id(id)
                } else {
                    return Err(PyTypeError::new_err("Type is not hashable"));
                };

                Ok(IDLField {
                    id,
                    val: to_idl_value(py, value)?,
                })
            })
            .collect::<PyResult<Vec<IDLField>>>()?;
        list.sort_unstable_by(|a, b| a.id.get_id().cmp(&b.id.get_id()));
        IDLValue::Record(list)
    } else if object.is_instance_of::<Variant>()? {
        let variant = object.extract::<Variant>()?;
        variant_to_idl_field(py, variant)?
    } else {
        IDLValue::Reserved
    };
    Ok(value)
}

fn variant_to_idl_field<'p>(py: Python<'p>, variant: Variant) -> PyResult<IDLValue> {
    let idl_value = match variant.object {
        Some(object) => to_idl_value(py, object.as_ref(py))?,
        None => IDLValue::Null,
    };
    let id = match variant.value {
        VariantField::Int(i) => Label::Unnamed(i),
        VariantField::String(s) => Label::Named(s),
    };

    let variant_value = VariantValue(Box::new(IDLField { id, val: idl_value }), 0);
    Ok(IDLValue::Variant(variant_value))
}
