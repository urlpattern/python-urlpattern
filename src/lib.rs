#![allow(non_snake_case)]

use std::collections::HashMap;

use pyo3::{
    exceptions::PyValueError,
    prelude::*,
    types::{PyDict, PyList},
};

#[pyclass]
pub struct URLPattern(pub urlpattern::UrlPattern);

#[pymethods]
impl URLPattern {
    #[new]
    #[pyo3(signature = (input=None, baseURL=None))]
    pub fn new(input: Option<URLPatternInput>, baseURL: Option<&str>) -> PyResult<Self> {
        let string_or_init_input = match input {
            Some(input) => urlpattern::quirks::StringOrInit::try_from(input)?,
            None => {
                urlpattern::quirks::StringOrInit::Init(urlpattern::quirks::UrlPatternInit::default())
            }
        };
        Ok(URLPattern(
            <urlpattern::UrlPattern>::parse(
                urlpattern::quirks::process_construct_pattern_input(string_or_init_input, baseURL)
                    .map_err(Error)?,
            )
            .map_err(Error)?,
        ))
    }

    pub fn __repr__(&self, py: Python) -> String {
        let dict = PyDict::new_bound(py);
        dict.set_item("protocol", self.0.protocol()).unwrap();
        dict.set_item("username", self.0.username()).unwrap();
        dict.set_item("password", self.0.password()).unwrap();
        dict.set_item("hostname", self.0.hostname()).unwrap();
        dict.set_item("port", self.0.port()).unwrap();
        dict.set_item("pathname", self.0.pathname()).unwrap();
        dict.set_item("search", self.0.search()).unwrap();
        dict.set_item("hash", self.0.hash()).unwrap();
        format!("URLPattern({})", dict)
    }

    #[pyo3(signature = (input=None, baseURL=None))]
    pub fn test(&self, input: Option<URLPatternInput>, baseURL: Option<&str>) -> PyResult<bool> {
        let string_or_init_input = match input {
            Some(input) => urlpattern::quirks::StringOrInit::try_from(input)?,
            None => {
                urlpattern::quirks::StringOrInit::Init(urlpattern::quirks::UrlPatternInit::default())
            }
        };
        let Some((match_input, _)) =
            urlpattern::quirks::process_match_input(string_or_init_input, baseURL)
                .map_err(Error)?
        else {
            return Ok(false);
        };
        Ok(self.0.test(match_input).map_err(Error)?)
    }

    #[pyo3(signature = (input=None, baseURL=None))]
    pub fn exec(
        &self,
        input: Option<URLPatternInput>,
        baseURL: Option<&str>,
    ) -> PyResult<Option<URLPatternResult>> {
        let string_or_init_input = match input {
            Some(input) => urlpattern::quirks::StringOrInit::try_from(input)?,
            None => {
                urlpattern::quirks::StringOrInit::Init(urlpattern::quirks::UrlPatternInit::default())
            }
        };
        let Some((match_input, inputs)) =
            urlpattern::quirks::process_match_input(string_or_init_input, baseURL)
                .map_err(Error)?
        else {
            return Ok(None);
        };
        let Some(result) = self.0.exec(match_input).map_err(Error)? else {
            return Ok(None);
        };

        Ok(Some(URLPatternResult {
            inputs,
            protocol: URLPatternComponentResult {
                input: result.protocol.input,
                groups: result.protocol.groups,
            },
            username: URLPatternComponentResult {
                input: result.username.input,
                groups: result.username.groups,
            },
            password: URLPatternComponentResult {
                input: result.password.input,
                groups: result.password.groups,
            },
            hostname: URLPatternComponentResult {
                input: result.hostname.input,
                groups: result.hostname.groups,
            },
            port: URLPatternComponentResult {
                input: result.port.input,
                groups: result.port.groups,
            },
            pathname: URLPatternComponentResult {
                input: result.pathname.input,
                groups: result.pathname.groups,
            },
            search: URLPatternComponentResult {
                input: result.search.input,
                groups: result.search.groups,
            },
            hash: URLPatternComponentResult {
                input: result.hash.input,
                groups: result.hash.groups,
            },
        }))
    }

    #[getter]
    pub fn get_protocol(&self) -> PyResult<&str> {
        Ok(self.0.protocol())
    }

    #[getter]
    pub fn get_username(&self) -> PyResult<&str> {
        Ok(self.0.username())
    }

    #[getter]
    pub fn get_password(&self) -> PyResult<&str> {
        Ok(self.0.password())
    }

    #[getter]
    pub fn get_hostname(&self) -> PyResult<&str> {
        Ok(self.0.hostname())
    }

    #[getter]
    pub fn get_port(&self) -> PyResult<&str> {
        Ok(self.0.port())
    }

    #[getter]
    pub fn get_pathname(&self) -> PyResult<&str> {
        Ok(self.0.pathname())
    }

    #[getter]
    pub fn get_search(&self) -> PyResult<&str> {
        Ok(self.0.search())
    }

    #[getter]
    pub fn get_hash(&self) -> PyResult<&str> {
        Ok(self.0.hash())
    }
}

#[derive(FromPyObject)]
pub enum URLPatternInput<'py> {
    String(String),
    Init(Bound<'py, PyDict>),
}

impl<'py> TryFrom<URLPatternInput<'py>> for urlpattern::quirks::StringOrInit {
    type Error = pyo3::PyErr;

    fn try_from(input: URLPatternInput<'py>) -> Result<Self, Self::Error> {
        Ok(match input {
            URLPatternInput::String(pattern) => urlpattern::quirks::StringOrInit::String(pattern),
            URLPatternInput::Init(init) => {
                urlpattern::quirks::StringOrInit::Init(urlpattern::quirks::UrlPatternInit {
                    protocol: init
                        .get_item("protocol")?
                        .map(|v| v.extract::<String>())
                        .transpose()?,
                    username: init
                        .get_item("username")?
                        .map(|v| v.extract::<String>())
                        .transpose()?,
                    password: init
                        .get_item("password")?
                        .map(|v| v.extract::<String>())
                        .transpose()?,
                    hostname: init
                        .get_item("hostname")?
                        .map(|v| v.extract::<String>())
                        .transpose()?,
                    port: init
                        .get_item("port")?
                        .map(|v| v.extract::<String>())
                        .transpose()?,
                    pathname: init
                        .get_item("pathname")?
                        .map(|v| v.extract::<String>())
                        .transpose()?,
                    search: init
                        .get_item("search")?
                        .map(|v| v.extract::<String>())
                        .transpose()?,
                    hash: init
                        .get_item("hash")?
                        .map(|v| v.extract::<String>())
                        .transpose()?,
                    base_url: init
                        .get_item("baseURL")?
                        .map(|v| v.extract::<String>())
                        .transpose()?,
                })
            }
        })
    }
}

pub struct URLPatternResult {
    pub inputs: (urlpattern::quirks::StringOrInit, Option<String>),
    pub protocol: URLPatternComponentResult,
    pub username: URLPatternComponentResult,
    pub password: URLPatternComponentResult,
    pub hostname: URLPatternComponentResult,
    pub port: URLPatternComponentResult,
    pub pathname: URLPatternComponentResult,
    pub search: URLPatternComponentResult,
    pub hash: URLPatternComponentResult,
}

impl IntoPy<PyObject> for URLPatternResult {
    fn into_py(self, py: Python<'_>) -> PyObject {
        let dict = PyDict::new_bound(py);

        let (string_or_init, base_url) = self.inputs;
        let list = PyList::empty_bound(py);

        match string_or_init {
            urlpattern::quirks::StringOrInit::String(string) => {
                list.append(string).unwrap();
            }
            urlpattern::quirks::StringOrInit::Init(init) => {
                let init_dict = PyDict::new_bound(py);
                if let Some(protocol) = init.protocol {
                    init_dict.set_item("protocol", protocol).unwrap();
                }
                if let Some(username) = init.username {
                    init_dict.set_item("username", username).unwrap();
                }
                if let Some(password) = init.password {
                    init_dict.set_item("password", password).unwrap();
                }
                if let Some(hostname) = init.hostname {
                    init_dict.set_item("hostname", hostname).unwrap();
                }
                if let Some(port) = init.port {
                    init_dict.set_item("port", port).unwrap();
                }
                if let Some(pathname) = init.pathname {
                    init_dict.set_item("pathname", pathname).unwrap();
                }
                if let Some(search) = init.search {
                    init_dict.set_item("search", search).unwrap();
                }
                if let Some(hash) = init.hash {
                    init_dict.set_item("hash", hash).unwrap();
                }
                if let Some(base_url) = init.base_url {
                    init_dict.set_item("baseURL", base_url).unwrap();
                }
                list.append(init_dict).unwrap();
            }
        }

        if let Some(base_url) = base_url {
            list.append(base_url).unwrap();
        }

        dict.set_item("inputs", list).unwrap();

        dict.set_item("protocol", self.protocol).unwrap();
        dict.set_item("username", self.username).unwrap();
        dict.set_item("password", self.password).unwrap();
        dict.set_item("hostname", self.hostname).unwrap();
        dict.set_item("port", self.port).unwrap();
        dict.set_item("pathname", self.pathname).unwrap();
        dict.set_item("search", self.search).unwrap();
        dict.set_item("hash", self.hash).unwrap();

        dict.into()
    }
}

pub struct URLPatternComponentResult {
    input: String,
    groups: HashMap<String, String>,
}

impl ToPyObject for URLPatternComponentResult {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        let dict = PyDict::new_bound(py);
        dict.set_item("input", self.input.clone()).unwrap();
        dict.set_item("groups", self.groups.clone()).unwrap();
        dict.into()
    }
}

pub struct Error(urlpattern::Error);

impl From<Error> for PyErr {
    fn from(error: Error) -> Self {
        PyValueError::new_err(error.0.to_string())
    }
}

impl From<urlpattern::Error> for Error {
    fn from(other: urlpattern::Error) -> Self {
        Self(other)
    }
}

/// A Python module implemented in Rust.
#[pymodule]
#[pyo3(name = "urlpattern")]
pub fn python_urlpattern(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<URLPattern>()?;
    Ok(())
}
