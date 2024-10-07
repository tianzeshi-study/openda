use pyo3::prelude::*;
// use pyo3::types::IntoPyDict;


#[pyfunction]
pub fn message(input_message: &str) -> PyResult<()> {
    // Initialize the Python interpreter
    Python::with_gil(|py| {
        // Import the Python module (example.py should be in the working directory)
        let ui = PyModule::import_bound(py, "ui")?;
        let _message: PyObject = ui.getattr("message")?.call1((input_message, ))?.into();
        // let ui = PyModule::from_code_bound(py, code, "rs_kernel.py", "kernel_ui")?;
        // let _message: PyObject = ui.getattr("message")?.call1(("hello world from rust", ))?.into();

        // Get the class `MyClass` from the module
        // let my_class = example.getattr("MyClass")?;


        // Create an instance of `MyClass` with parameters 5 and 10
        // let instance = my_class.call1((5, 10))?;

        // Call the `add` method of the instance
        // let result: i32 = instance.call_method0("add")?.extract()?;

        // println!("The result of add is: {}", result); // Output: The result of add is: 15

        Ok(())
    })
}


#[pyfunction]
pub fn report_text_copied_to_clipboard(text: Option<&str>) -> PyResult<()> {
    Python::with_gil(|py| {
        let ui = PyModule::import_bound(py, "ui")?;
        let _message: PyObject = ui.getattr("reportTextCopiedToClipboard")?.call1((text, ))?.into();
        Ok(())
    })
}



// 定义 Python 模块 `ui`，并将 `message` 函数添加到该模块中
#[pymodule]
pub fn ui(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(message, m)?)?;
    Ok(())
}

// pub fn  reviewMessage(text: &str, speechPriority: Option<speech.Spri] = None):