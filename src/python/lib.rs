use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use etym::Etymology as CoreEtymology;

#[pyclass(name = "Etymology")]
#[derive(Clone)]
pub struct PyEtymology {
    inner: CoreEtymology,
}

#[pymethods]
impl PyEtymology {
    #[new]
    pub fn new(word: &str) -> PyResult<Self> {
        let inner = CoreEtymology::new(word)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(PyEtymology { inner })
    }

    #[getter]
    pub fn word(&self) -> String {
        self.inner.word.clone()
    }

    #[getter]
    pub fn label(&self) -> String {
        self.inner.label.clone()
    }

    #[getter]
    pub fn etymology(&self) -> String {
        self.inner.etymology.clone()
    }

    fn __repr__(&self) -> String {
        format!("<Etymology label='{}'>", self.inner.label)
    }

    fn __str__(&self) -> String {
        format!("{}:\n{}", self.inner.label, self.inner.etymology)
    }
}

#[pymodule]
fn etym(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyEtymology>()?;
    Ok(())
}
