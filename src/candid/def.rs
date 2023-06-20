use candid::{check_prog, types::Type, IDLProg, TypeEnv};
use pyo3::prelude::*;

const BASE: &str = r#"
type Tester = record {
    a: text;
};

type Vari = variant {
    Yes;
    No;
};

type Basic = record {
    a : int;
    b : int8;
    c : int16;
    d : int32;
    e : int64;
    f : int;
    g : nat8;
    h : nat16;
    i : nat32;
    j : nat64;
    k : opt Tester; 
    v : Vari;
};
"#;

#[derive(Clone)]
#[pyclass]
pub struct Definition {
    env: TypeEnv,
    service: Option<Type>,
}

#[pymethods]
impl Definition {
    #[new]
    pub fn new() -> Self {
        // use check_file to get an env and actor

        let prog: IDLProg = BASE.parse().expect("failed to parse");
        let mut env = TypeEnv::new();
        let service = check_prog(&mut env, &prog).expect("failed");
        println!("{:#?}", env);
        let r = env.rec_find_type("Basic").unwrap();
        println!("{:#?}", r);

        Self { env, service }
    }
}

impl Definition {
    pub fn env(&self) -> &TypeEnv {
        &self.env
    }

    pub fn get_type(&self, type_name: &str) -> Option<Type> {
        self.env.find_type(type_name).cloned().ok()
    }
}
