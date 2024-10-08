use pyo3::prelude::*;
// use pyo3::types::IntoPyDict;


#[pyfunction]
pub fn speak(input_message: &str) -> PyResult<()> {
    // Initialize the Python interpreter
    Python::with_gil(|py| {
        // Import the Python module (example.py should be in the working directory)
        let ui = PyModule::import_bound(py, "speech")?;
        let _message: PyObject = ui.getattr("speak")?.call1((input_message, ))?.into();

        Ok(())
    })
}

#[pymodule]
pub fn speech(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(speak, m)?)?;
    Ok(())
}
