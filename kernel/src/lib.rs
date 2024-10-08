mod core;
mod  speech;
mod ui;
use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn kernel(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(core::main, m)?)?;
    m.add_function(wrap_pyfunction!(ui::message, m)?)?;
    
    let ui_module = PyModule::new_bound(py, "ui")?;
    ui_module.add_function(wrap_pyfunction!(ui::message, &ui_module)?)?;
    ui_module.add_function(wrap_pyfunction!(ui::report_text_copied_to_clipboard, &ui_module)?)?;
    m.add_submodule(&ui_module)?;
    
    let speech_module = PyModule::new_bound(py, "speech")?;
    ui_module.add_function(wrap_pyfunction!(speech::speak, &speech_module)?)?;
    m.add_submodule(&speech_module)?;
    
    Ok(())
    // });


    // Ok(())
}
 
 