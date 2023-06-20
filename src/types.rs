pub mod func;
pub mod int;
pub mod nat;
pub mod principal;
pub mod variant;

use pyo3::prelude::*;

use self::int::*;
use self::nat::*;
use self::variant::Variant;

#[pymodule]
pub fn types_module(py: Python<'_>, parent_module: &PyModule) -> PyResult<()> {
    let types_module = PyModule::new(py, "types")?;

    types_module.add_class::<Variant>()?;

    types_module.add_class::<Nat>()?;
    types_module.add_class::<Nat8>()?;
    types_module.add_class::<Nat16>()?;
    types_module.add_class::<Nat32>()?;
    types_module.add_class::<Nat64>()?;

    types_module.add_class::<Int>()?;
    types_module.add_class::<Int8>()?;
    types_module.add_class::<Int16>()?;
    types_module.add_class::<Int32>()?;
    types_module.add_class::<Int64>()?;

    parent_module.add_submodule(types_module)?;
    Ok(())
}
