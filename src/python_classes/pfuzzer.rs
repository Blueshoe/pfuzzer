use nucleo::{Config, Matcher, Utf32Str};
use pyo3::prelude::*;

#[pyclass]
pub struct Pfuzzer {
    pub matcher: Matcher,
}

#[pymethods]
impl Pfuzzer {
    #[new]
    pub fn new() -> PyResult<Self> {
        Ok(Pfuzzer {
            matcher: Matcher::new(Config::DEFAULT),
        })
    }

    /// Compares a list of target strings against a query string using fuzzy matching.
    /// Returns a vector of optional match scores, where each score corresponds to the
    /// match quality of the respective target string. A higher score indicates a better match.
    /// A None value indicates that the query did not match the target at all.
    pub fn compare_strings(&self, targets: Vec<String>, query: String) -> Vec<Option<u16>> {
        let mut res = Vec::<Option<u16>>::new();
        for target in targets {
            res.push(self.matcher.to_owned().fuzzy_match(
                Utf32Str::Ascii(target.as_bytes()),
                Utf32Str::Ascii(query.as_bytes()),
            ))
        }
        return res.to_owned();
    }
}
