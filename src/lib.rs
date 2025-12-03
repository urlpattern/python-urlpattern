#![allow(non_snake_case)]

use std::collections::HashMap;

use pyo3::{
    BoundObject,
    exceptions::PyValueError,
    prelude::*,
    types::{PyDict, PyList},
};

#[pyclass(name = "URLPattern")]
struct UrlPattern(deno_urlpattern::UrlPattern);

#[pymethods]
impl UrlPattern {
    #[new]
    #[pyo3(signature = (input=None, baseURL=None, options=None))]
    pub fn new(
        input: Option<UrlPatternInput>,
        baseURL: Option<&Bound<'_, PyAny>>,
        options: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<Self> {
        let (base_url, options) = match baseURL {
            Some(value) => {
                if let Ok(options_dict) = value.cast::<PyDict>() {
                    (None, Some(options_dict))
                } else if value.is_none() {
                    (None, options)
                } else {
                    (Some(value.extract::<String>()?), options)
                }
            }
            None => (None, options),
        };

        let string_or_init_input = match input {
            Some(input) => deno_urlpattern::quirks::StringOrInit::try_from(input)?,
            None => deno_urlpattern::quirks::StringOrInit::Init(
                deno_urlpattern::quirks::UrlPatternInit::default(),
            ),
        };
        let options = if let Some(options) = options {
            deno_urlpattern::UrlPatternOptions {
                ignore_case: options
                    .get_item("ignoreCase")?
                    .map(|v| v.extract::<bool>())
                    .transpose()?
                    .unwrap_or(false),
                ..deno_urlpattern::UrlPatternOptions::default()
            }
        } else {
            deno_urlpattern::UrlPatternOptions::default()
        };
        Ok(UrlPattern(
            <deno_urlpattern::UrlPattern>::parse(
                deno_urlpattern::quirks::process_construct_pattern_input(
                    string_or_init_input,
                    base_url.as_deref(),
                )
                .map_err(Error)?,
                options,
            )
            .map_err(Error)?,
        ))
    }

    pub fn __repr__(&self, py: Python) -> String {
        let dict = PyDict::new(py);
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
    pub fn test(&self, input: Option<UrlPatternInput>, baseURL: Option<&str>) -> PyResult<bool> {
        let string_or_init_input = match input {
            Some(input) => deno_urlpattern::quirks::StringOrInit::try_from(input)?,
            None => deno_urlpattern::quirks::StringOrInit::Init(
                deno_urlpattern::quirks::UrlPatternInit::default(),
            ),
        };
        let Some((match_input, _)) =
            deno_urlpattern::quirks::process_match_input(string_or_init_input, baseURL)
                .map_err(Error)?
        else {
            return Ok(false);
        };
        Ok(self.0.test(match_input).map_err(Error)?)
    }

    #[pyo3(signature = (input=None, baseURL=None))]
    pub fn exec(
        &self,
        input: Option<UrlPatternInput>,
        baseURL: Option<&str>,
    ) -> PyResult<Option<UrlPatternResult>> {
        let string_or_init_input = match input {
            Some(input) => deno_urlpattern::quirks::StringOrInit::try_from(input)?,
            None => deno_urlpattern::quirks::StringOrInit::Init(
                deno_urlpattern::quirks::UrlPatternInit::default(),
            ),
        };
        let Some((match_input, inputs)) =
            deno_urlpattern::quirks::process_match_input(string_or_init_input, baseURL)
                .map_err(Error)?
        else {
            return Ok(None);
        };
        let Some(result) = self.0.exec(match_input).map_err(Error)? else {
            return Ok(None);
        };

        Ok(Some(UrlPatternResult {
            inputs,
            protocol: UrlPatternComponentResult {
                input: result.protocol.input,
                groups: result.protocol.groups,
            },
            username: UrlPatternComponentResult {
                input: result.username.input,
                groups: result.username.groups,
            },
            password: UrlPatternComponentResult {
                input: result.password.input,
                groups: result.password.groups,
            },
            hostname: UrlPatternComponentResult {
                input: result.hostname.input,
                groups: result.hostname.groups,
            },
            port: UrlPatternComponentResult {
                input: result.port.input,
                groups: result.port.groups,
            },
            pathname: UrlPatternComponentResult {
                input: result.pathname.input,
                groups: result.pathname.groups,
            },
            search: UrlPatternComponentResult {
                input: result.search.input,
                groups: result.search.groups,
            },
            hash: UrlPatternComponentResult {
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
pub enum UrlPatternInput<'py> {
    String(String),
    Init(Bound<'py, PyDict>),
}

impl<'py> TryFrom<UrlPatternInput<'py>> for deno_urlpattern::quirks::StringOrInit {
    type Error = pyo3::PyErr;

    fn try_from(input: UrlPatternInput<'py>) -> Result<Self, Self::Error> {
        Ok(match input {
            UrlPatternInput::String(pattern) => {
                deno_urlpattern::quirks::StringOrInit::String(pattern)
            }
            UrlPatternInput::Init(init) => deno_urlpattern::quirks::StringOrInit::Init(
                deno_urlpattern::quirks::UrlPatternInit {
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
                },
            ),
        })
    }
}

pub struct UrlPatternResult {
    pub inputs: (deno_urlpattern::quirks::StringOrInit, Option<String>),
    pub protocol: UrlPatternComponentResult,
    pub username: UrlPatternComponentResult,
    pub password: UrlPatternComponentResult,
    pub hostname: UrlPatternComponentResult,
    pub port: UrlPatternComponentResult,
    pub pathname: UrlPatternComponentResult,
    pub search: UrlPatternComponentResult,
    pub hash: UrlPatternComponentResult,
}

impl<'py> IntoPyObject<'py> for UrlPatternResult {
    type Target = PyDict;
    type Output = Bound<'py, Self::Target>;
    type Error = std::convert::Infallible;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let dict = PyDict::new(py);

        let (string_or_init, base_url) = self.inputs;
        let list = PyList::empty(py);

        match string_or_init {
            deno_urlpattern::quirks::StringOrInit::String(string) => {
                list.append(string).unwrap();
            }
            deno_urlpattern::quirks::StringOrInit::Init(init) => {
                let init_dict = PyDict::new(py);
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

        Ok(dict.into_bound())
    }
}

#[derive(IntoPyObject, IntoPyObjectRef)]
pub struct UrlPatternComponentResult {
    input: String,
    groups: HashMap<String, Option<String>>,
}

pub struct Error(deno_urlpattern::Error);

impl From<Error> for PyErr {
    fn from(error: Error) -> Self {
        PyValueError::new_err(error.0.to_string())
    }
}

impl From<deno_urlpattern::Error> for Error {
    fn from(other: deno_urlpattern::Error) -> Self {
        Self(other)
    }
}

/// A Python module implemented in Rust.
#[pymodule]
mod urlpattern {
    #[pymodule_export]
    use super::UrlPattern;
}
