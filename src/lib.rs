mod python_classes;

use pyo3::prelude::*;

/// A Python fuzzy searcher module implemented in Rust.
#[pymodule]
fn pfuzzer(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<python_classes::pfuzzer::Pfuzzer>()?;
    Ok(())
}
