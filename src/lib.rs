use pyo3::prelude::*;
use rillrate::rill::providers;
use rillrate::{EntryId, Path, RillRate};

#[pymodule]
fn rill(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    // TODO: Return error here
    let rillrate = RillRate::from_env("rillrate-py").unwrap();
    // TODO: Use OnceCell instead of keep it in the interpreter if possible.
    std::mem::forget(rillrate);
    m.add_class::<LogProvider>()?;
    m.add_class::<CounterProvider>()?;
    Ok(())
}

fn make_path(entries: Vec<String>) -> Path {
    let entries: Vec<_> = entries.into_iter().map(EntryId::from).collect();
    Path::from(entries)
}

#[pyclass]
pub struct LogProvider {
    provider: providers::LogProvider,
}

#[pymethods]
impl LogProvider {
    #[new]
    fn new(entries: Vec<String>) -> Self {
        let path = make_path(entries);
        let provider = providers::LogProvider::new(path);
        Self { provider }
    }

    fn is_active(&mut self, _py: Python) -> bool {
        self.provider.is_active()
    }

    fn log(&mut self, _py: Python, msg: String) {
        self.provider.log(msg, None);
    }
}

#[pyclass]
pub struct CounterProvider {
    provider: providers::CounterProvider,
}

#[pymethods]
impl CounterProvider {
    #[new]
    fn new(entries: Vec<String>) -> Self {
        let path = make_path(entries);
        let provider = providers::CounterProvider::new(path);
        Self { provider }
    }

    fn is_active(&mut self, _py: Python) -> bool {
        self.provider.is_active()
    }

    fn inc(&mut self, _py: Python, delta: f64) {
        self.provider.inc(delta, None);
    }
}

#[pyclass]
pub struct GaugeProvider {
    provider: providers::GaugeProvider,
}

#[pymethods]
impl GaugeProvider {
    #[new]
    fn new(entries: Vec<String>) -> Self {
        let path = make_path(entries);
        let provider = providers::GaugeProvider::new(path);
        Self { provider }
    }

    fn is_active(&mut self, _py: Python) -> bool {
        self.provider.is_active()
    }

    fn inc(&mut self, _py: Python, delta: f64) {
        self.provider.inc(delta, None);
    }

    fn dec(&mut self, _py: Python, delta: f64) {
        self.provider.dec(delta, None);
    }

    fn set(&mut self, _py: Python, delta: f64) {
        self.provider.set(delta, None);
    }
}
