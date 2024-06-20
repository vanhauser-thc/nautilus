// Nautilus
// Copyright (C) 2024  Daniel Teuchert, Cornelius Aschermann, Sergej Schumilo

use pyo3::prelude::*;
use pyo3::types::{IntoPyDict};

use crate::Context;

#[pyclass]
struct PyContext {
    ctx: Context,
}
impl PyContext {
    fn get_context(&self) -> Context {
        self.ctx.clone()
    }
}

#[pymethods]
impl PyContext {
    #[new]
    fn new() -> Self {
        PyContext {
            ctx: Context::new(),
        }
    }

    fn rule(&mut self, py: Python, nt: &str, format: &PyAny) -> PyResult<()> {
        if let Ok(s) = format.extract::<&str>() {
            self.ctx.add_rule(nt, s.as_bytes());
        } else if let Ok(s) = format.extract::<&[u8]>() {
            self.ctx.add_rule(nt, s);
        } else {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "format argument should be string or bytes",
            ));
        }
        return Ok(());
    }

    fn script(&mut self, nt: &str, nts: Vec<String>, script: PyObject) {
        self.ctx.add_script(nt, nts, script);
    }

    fn regex(&mut self, nt: &str, regex: &str) {
        self.ctx.add_regex(nt, regex);
    }
}

fn main_(py: Python, grammar_path: &str) -> PyResult<Context> {
    let py_ctx = PyCell::new(py, PyContext::new()).unwrap();
    let locals = [("ctx", py_ctx)].into_py_dict(py);
    py.run(
        &std::fs::read_to_string(grammar_path).expect("couldn't read grammar file"),
        None,
        Some(&locals),
    )?;
    return Ok(py_ctx.borrow().get_context());
}

pub fn load_python_grammar(grammar_path: &str) -> Context {
    return Python::with_gil(|py| {
        main_(py, grammar_path)
            .map_err(|e| e.print_and_set_sys_last_vars(py))
            .unwrap()
    });
}
