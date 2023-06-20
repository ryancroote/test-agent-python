use std::sync::Arc;

use candid::Encode;
use pyo3::{exceptions::PyTypeError, prelude::*, types::PyBytes};

use ic_agent::{agent::http_transport::ReqwestHttpReplicaV2Transport, Agent as IcAgent};
use tokio::sync::Mutex;

use crate::types::principal::Principal;

#[pyclass]
pub struct Agent {
    inner: Arc<Mutex<IcAgent>>,
}

#[pymethods]
impl Agent {
    #[new]
    pub fn new() -> Self {
        let transport = ReqwestHttpReplicaV2Transport::create("https://ic0.app").expect("");
        let ic_agent = IcAgent::builder()
            .with_transport(transport)
            .build()
            .expect("");

        Self {
            inner: Arc::new(Mutex::new(ic_agent)),
        }
    }

    pub fn query<'p>(
        &self,
        py: Python<'p>,
        canister_id: Principal,
        method_name: String,
        arg: Vec<u8>,
    ) -> PyResult<&'p PyAny> {
        let ic_agent = self.inner.clone();
        let runtime = pyo3_asyncio::tokio::get_runtime();
        let a = Encode!().unwrap();
        runtime
            .block_on(async {
                let guard = ic_agent.lock().await;
                guard
                    .query(canister_id.inner_ref(), method_name)
                    .with_arg(a)
                    .call()
                    .await
                    .map_err(|e| PyTypeError::new_err(e.to_string()))
            })
            .map(|b| PyBytes::new(py, &b).to_object(py).into_ref(py))
    }

    pub fn query_async<'p>(
        &self,
        py: Python<'p>,
        canister_id: Principal,
        method_name: String,
        arg: Vec<u8>,
    ) -> PyResult<&'p PyAny> {
        let ic_agent = self.inner.clone();
        let a = Encode!().unwrap();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let guard = ic_agent.lock().await;
            guard
                .query(canister_id.inner_ref(), method_name)
                .with_arg(a)
                .call()
                .await
                .map_err(|e| PyTypeError::new_err(e.to_string()))
        })
    }
}
