use pyo3::prelude::*;
use pyo3::types::PyBytes;

/// Runs tectonics latex to pdf function.
#[pyfunction]
fn tex2pdf(py: Python, input: String) -> PyObject {
    let pdfoutput: Vec<u8> = tectonic::latex_to_pdf(input).expect("tex2pdf processing failed");
    PyBytes::new(py, &pdfoutput).into()
}

/// A Python module implemented in Rust.
#[pymodule]
fn pytex2pdf(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(tex2pdf, m)?)?;
    Ok(())
}