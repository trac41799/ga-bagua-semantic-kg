use crate::prelude::*;
use crate::RelationType;
use pyo3::prelude::*;

#[pyclass(name = "Multivector")]
#[derive(Clone)]
pub struct PyMultivector {
    inner: Multivector,
}

#[pymethods]
impl PyMultivector {
    #[new]
    fn new(coeffs: [f64; 8]) -> Self {
        PyMultivector { inner: Multivector::new(coeffs) }
    }

    #[staticmethod]
    fn zero() -> Self { PyMultivector { inner: Multivector::zero() } }

    #[staticmethod]
    fn one() -> Self { PyMultivector { inner: Multivector::one() } }

    fn coefficients(&self) -> [f64; 8] { *self.inner.coefficients() }

    fn norm(&self) -> f64 { self.inner.norm() }

    fn geo_product(&self, other: &PyMultivector) -> PyMultivector {
        PyMultivector { inner: self.inner.geo_product(&other.inner) }
    }

    fn inner_product(&self, other: &PyMultivector) -> f64 {
        self.inner.inner_product(&other.inner)
    }

    fn inverse(&self) -> Option<PyMultivector> {
        self.inner.inverse().ok().map(|mv| PyMultivector { inner: mv })
    }

    fn dominant_role(&self) -> String {
        self.inner.dominant_role().role_name().to_string()
    }

    fn semantic_similarity(&self, other: &PyMultivector) -> f64 {
        semantic_similarity(&self.inner, &other.inner)
    }

    fn semantic_difference(&self, other: &PyMultivector) -> f64 {
        semantic_difference(&self.inner, &other.inner)
    }

    fn classify_relation(&self, other: &PyMultivector) -> (String, f64) {
        let (role, conf) = RelationType::from_pair(&self.inner, &other.inner);
        (role.role_name().to_string(), conf)
    }

    fn is_contradictory(&self, other: &PyMultivector, threshold: f64) -> bool {
        is_contradictory(&self.inner, &other.inner, threshold)
    }

    fn relation_strength(&self, other: &PyMultivector) -> f64 {
        relation_strength(&self.inner, &other.inner)
    }

    fn describe(&self) -> String { multivector_describe(&self.inner) }

    fn __repr__(&self) -> String {
        format!("Multivector({})", self.inner)
    }

    fn __str__(&self) -> String { format!("{}", self.inner) }
}

#[pyfunction]
fn text_to_multivector(text: &str) -> PyMultivector {
    PyMultivector { inner: text_to_multivector(text) }
}

#[pyfunction]
fn word_to_multivector(word: &str) -> PyMultivector {
    PyMultivector { inner: word_to_multivector(word) }
}

#[pyfunction]
fn analogy(a: &PyMultivector, b: &PyMultivector, c: &PyMultivector) -> Option<PyMultivector> {
    analogy(&a.inner, &b.inner, &c.inner).map(|mv| PyMultivector { inner: mv })
}

#[pymodule]
pub fn ga_semantics(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_class::<PyMultivector>()?;
    m.add_function(wrap_pyfunction!(text_to_multivector, m)?)?;
    m.add_function(wrap_pyfunction!(word_to_multivector, m)?)?;
    m.add_function(wrap_pyfunction!(analogy, m)?)?;
    Ok(())
}
