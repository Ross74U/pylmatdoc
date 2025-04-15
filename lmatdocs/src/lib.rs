mod docprocessor;
mod makedoc;
mod parser;
mod tokenizer;

use docprocessor::*;
use makedoc::*;

use pyo3::prelude::*;
use pyo3::types::PyBytes;

#[pyclass]
pub struct LmatDocProcessor {
    input: String,
}

#[pymethods]
impl LmatDocProcessor {
    #[new]
    fn new(input: String) -> Self {
        Self { input }
    }

    fn make_docx(&self, py: Python) -> PyResult<Py<PyBytes>> {
        let docbytes = py.allow_threads(|| {
            let mut docprocessor = DocProcessor::new(&self.input);
            let tree = docprocessor.process().expect("error in processing");
            makedoc(&tree).expect("error during makedoc")
        });
        Ok(PyBytes::new(py, &docbytes).into())
    }
}

#[pymodule]
fn lmatdoc(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<LmatDocProcessor>()?;
    Ok(())
}
