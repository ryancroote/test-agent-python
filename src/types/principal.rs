use pyo3::prelude::*;

use ic_agent::export::Principal as IcPrincipal;

#[derive(Clone)]
#[pyclass]
pub struct Principal {
    inner: IcPrincipal,
}

#[pymethods]
impl Principal {
    #[staticmethod]
    pub fn from_text(text: &str) -> Self {
        let inner = IcPrincipal::from_text(text).expect("Failed to decode principal");
        Self { inner }
    }

    #[staticmethod]
    pub fn management_canister() -> Self {
        let inner = IcPrincipal::management_canister();
        Self { inner }
    }
}

impl Principal {
    pub fn from_principal(inner: IcPrincipal) -> Self {
        Self { inner }
    }

    pub fn inner_ref(&self) -> &IcPrincipal {
        &self.inner
    }
}
