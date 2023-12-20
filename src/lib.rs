use std::collections::HashMap;
use pyo3::prelude::*;

#[pyclass]
#[derive(Debug)]
pub struct DictDiff {
    #[pyo3(get)]
    pub added: Vec<String>,
    #[pyo3(get)]
    pub removed: Vec<String>,
    #[pyo3(get)]
    pub modified: Vec<String>,
}

#[pymethods]
impl DictDiff {
    #[new]
    fn new() -> Self {
        DictDiff {
            added: Vec::new(),
            removed: Vec::new(),
            modified: Vec::new(),
        }
    }
}

#[pyfunction]
fn compare_dicts(dict1: HashMap<String, u32>, dict2: HashMap<String, u32>) -> DictDiff {
    let mut added: Vec<String> = Vec::new();
    let mut removed: Vec<String> = Vec::new();
    let mut modified: Vec<String> = Vec::new();

    for (key, value) in &dict1 {
        if !dict2.contains_key(key) {
            removed.push(key.clone());
        } else if dict2.get(key) != Some(value) {
            modified.push(key.clone());
        }
    }

    for key in dict2.keys() {
        if !dict1.contains_key(key) {
            added.push(key.clone());
        }
    }

    DictDiff {
        added,
        removed,
        modified,
    }
}

#[pymodule]
fn compars_dicts(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(compare_dicts, m)?)?;
    m.add_class::<DictDiff>()?;
    Ok(())
}
