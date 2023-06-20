use pyo3::{prelude::*, types::PyInt};

#[derive(Clone)]
#[pyclass]
pub struct Int(pub num_bigint::BigInt);

#[pymethods]
impl Int {
    #[new]
    pub fn new(object: &PyAny) -> PyResult<Self> {
        if !object.is_instance_of::<PyInt>()? {
            panic!("Parameter is not int");
        }
        let inner = object.extract::<num_bigint::BigInt>()?;

        Ok(Self(inner))
    }

    fn __repr__(&self) -> String {
        self.0.to_string()
    }

    fn __int__(&self) -> num_bigint::BigInt {
        self.0.clone()
    }
}

impl ToPyObject for Int {
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

generate_nat!(Int8, i8);
generate_nat!(Int16, i16);
generate_nat!(Int32, i32);
generate_nat!(Int64, i64);
