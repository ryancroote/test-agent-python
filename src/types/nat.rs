use pyo3::{prelude::*, types::PyInt};

#[derive(Clone)]
#[pyclass]
pub struct Nat(pub num_bigint::BigUint);

#[pymethods]
impl Nat {
    #[new]
    pub fn new(object: &PyAny) -> PyResult<Self> {
        if !object.is_instance_of::<PyInt>()? {
            panic!("Parameter is not int");
        }
        let inner = object.extract::<num_bigint::BigUint>()?;

        Ok(Self(inner))
    }

    fn __repr__(&self) -> String {
        self.0.to_string()
    }

    fn __int__(&self) -> num_bigint::BigUint {
        self.0.clone()
    }
}

impl ToPyObject for Nat {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        self.0.to_object(py)
    }
}

macro_rules! generate_nat {
    ($name:ident, $value_ty:ty) => {
        #[derive(Clone)]
        #[pyclass]
        pub struct $name(pub $value_ty);

        #[pymethods]
        impl $name {
            #[new]
            pub fn new(value: $value_ty) -> Self {
                Self(value)
            }
        }

        impl $name {
            fn __repr__(&self) -> String {
                self.0.to_string()
            }

            fn __int__(&self) -> $value_ty {
                self.0.clone()
            }
        }

        impl ToPyObject for $name {
            fn to_object(&self, py: Python<'_>) -> PyObject {
                self.0.to_object(py)
            }
        }
    };
}

generate_nat!(Nat8, u8);
generate_nat!(Nat16, u16);
generate_nat!(Nat32, u32);
generate_nat!(Nat64, u64);
