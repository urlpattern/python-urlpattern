#![allow(non_snake_case)]

use pyo3::{
    BoundObject,
    exceptions::{PyTypeError, PyValueError},
    prelude::*,
    types::{PyDict, PyList, PyString},
};
use std::collections::HashMap;

#[derive(FromPyObject)]
enum UrlPatternInput<'py> {
    String(String),
    Init(Bound<'py, PyDict>),
}

#[pyclass(name = "URLPattern")]
struct UrlPattern(::urlpattern::UrlPattern);

#[pymethods]
impl UrlPattern {
    #[new]
    #[pyo3(signature = (input=None, baseURL=None, options=None))]
    fn new(
        input: Option<UrlPatternInput>,
        baseURL: Option<&Bound<'_, PyAny>>,
        options: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<Self> {
        let base_url = baseURL;

        let (base_url, options) = match base_url {
            Some(value) => {
                if let Ok(options_dict) = value.cast::<PyDict>() {
                    (None, Some(options_dict))
                } else if value.is_none() {
                    (None, options)
                } else {
                    (base_url, options)
                }
            }
            None => (None, options),
        };

        let base_url = match base_url {
            Some(base_url) => Some(
                base_url
                    .extract::<String>()?
                    .parse::<url::Url>()
                    .map_err(::urlpattern::Error::Url)
                    .map_err(Error)?,
            ),
            None => None,
        };

        let options = match options {
            Some(options) => ::urlpattern::UrlPatternOptions {
                ignore_case: options
                    .get_item("ignoreCase")?
                    .map(|v| v.extract::<bool>())
                    .transpose()?
                    .unwrap_or(false),
                ..::urlpattern::UrlPatternOptions::default()
            },
            None => ::urlpattern::UrlPatternOptions::default(),
        };

        let init: ::urlpattern::UrlPatternInit = match input {
            Some(UrlPatternInput::String(input)) => {
                ::urlpattern::UrlPatternInit::parse_constructor_string::<regex::Regex>(
                    input.as_str(),
                    base_url,
                )
                .map_err(Error)?
            }
            Some(UrlPatternInput::Init(init)) => {
                if base_url.is_some() {
                    return Err(PyTypeError::new_err("cannot use dict input with baseURL"));
                }

                ::urlpattern::UrlPatternInit {
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
                        .transpose()?
                        .map(|v| v.parse::<url::Url>())
                        .transpose()
                        .map_err(::urlpattern::Error::Url)
                        .map_err(Error)?,
                }
            }
            None => ::urlpattern::UrlPatternInit::default(),
        };

        Ok(Self(
            ::urlpattern::UrlPattern::parse(init, options).map_err(Error)?,
        ))
    }

    fn __repr__(&self, py: Python) -> PyResult<String> {
        let dict = PyDict::new(py);
        dict.set_item("protocol", self.0.protocol())?;
        dict.set_item("username", self.0.username())?;
        dict.set_item("password", self.0.password())?;
        dict.set_item("hostname", self.0.hostname())?;
        dict.set_item("port", self.0.port())?;
        dict.set_item("pathname", self.0.pathname())?;
        dict.set_item("search", self.0.search())?;
        dict.set_item("hash", self.0.hash())?;
        dict.set_item("hasRegExpGroups", self.0.has_regexp_groups())?;
        Ok(format!("URLPattern({})", dict))
    }

    #[pyo3(signature = (input=None, baseURL=None))]
    fn test(&self, input: Option<UrlPatternInput>, baseURL: Option<&str>) -> PyResult<bool> {
        let base_url = baseURL;

        let input: ::urlpattern::UrlPatternMatchInput = match input {
            Some(UrlPatternInput::String(input)) => match base_url {
                Some(base_url) => {
                    let base_url = match url::Url::parse(base_url) {
                        Ok(url) => url,
                        Err(_) => return Ok(false),
                    };
                    ::urlpattern::UrlPatternMatchInput::Url(
                        match url::Url::options().base_url(Some(&base_url)).parse(&input) {
                            Ok(url) => url,
                            Err(_) => return Ok(false),
                        },
                    )
                }
                None => ::urlpattern::UrlPatternMatchInput::Url(match input.parse::<url::Url>() {
                    Ok(url) => url,
                    Err(_) => return Ok(false),
                }),
            },
            Some(UrlPatternInput::Init(init)) => {
                if base_url.is_some() {
                    return Err(PyTypeError::new_err("cannot use dict input with baseURL"));
                }

                ::urlpattern::UrlPatternMatchInput::Init(::urlpattern::UrlPatternInit {
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
                        .transpose()?
                        .map(|v| v.parse::<url::Url>())
                        .transpose()
                        .map_err(::urlpattern::Error::Url)
                        .map_err(Error)?,
                })
            }
            None => {
                ::urlpattern::UrlPatternMatchInput::Init(::urlpattern::UrlPatternInit::default())
            }
        };

        Ok(self.0.test(input).map_err(Error)?)
    }

    #[pyo3(signature = (input=None, baseURL=None))]
    fn exec<'py>(
        &self,
        py: Python<'py>,
        input: Option<&Bound<'py, PyAny>>,
        baseURL: Option<&Bound<'py, PyString>>,
    ) -> PyResult<Option<UrlPatternResult<'py>>> {
        let base_url = baseURL;

        let urlpattern_input: Option<UrlPatternInput> = input.map(|i| i.extract()).transpose()?;
        let input: ::urlpattern::UrlPatternMatchInput = match &urlpattern_input {
            Some(UrlPatternInput::String(input)) => match base_url {
                Some(base_url) => {
                    let base_url = match url::Url::parse(base_url.to_str()?) {
                        Ok(url) => url,
                        Err(_) => return Ok(None),
                    };
                    ::urlpattern::UrlPatternMatchInput::Url(
                        match url::Url::options().base_url(Some(&base_url)).parse(input) {
                            Ok(url) => url,
                            Err(_) => return Ok(None),
                        },
                    )
                }
                None => ::urlpattern::UrlPatternMatchInput::Url(match input.parse::<url::Url>() {
                    Ok(url) => url,
                    Err(_) => return Ok(None),
                }),
            },
            Some(UrlPatternInput::Init(init)) => {
                if base_url.is_some() {
                    return Err(PyTypeError::new_err("cannot use dict input with baseURL"));
                }

                ::urlpattern::UrlPatternMatchInput::Init(::urlpattern::UrlPatternInit {
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
                        .transpose()?
                        .map(|v| v.parse::<url::Url>())
                        .transpose()
                        .map_err(::urlpattern::Error::Url)
                        .map_err(Error)?,
                })
            }
            None => {
                ::urlpattern::UrlPatternMatchInput::Init(::urlpattern::UrlPatternInit::default())
            }
        };

        let Some(result) = self.0.exec(input).map_err(Error)? else {
            return Ok(None);
        };

        Ok(Some(UrlPatternResult {
            inputs: {
                let mut vec = Vec::new();
                vec.push(
                    urlpattern_input.unwrap_or(UrlPatternInput::Init(PyDict::new(py).into_bound())),
                );
                if let Some(base_url) = base_url {
                    vec.push(UrlPatternInput::String(base_url.to_string()));
                }
                vec
            },
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
    fn protocol(&self) -> PyResult<&str> {
        Ok(self.0.protocol())
    }

    #[getter]
    fn username(&self) -> PyResult<&str> {
        Ok(self.0.username())
    }

    #[getter]
    fn password(&self) -> PyResult<&str> {
        Ok(self.0.password())
    }

    #[getter]
    fn hostname(&self) -> PyResult<&str> {
        Ok(self.0.hostname())
    }

    #[getter]
    fn port(&self) -> PyResult<&str> {
        Ok(self.0.port())
    }

    #[getter]
    fn pathname(&self) -> PyResult<&str> {
        Ok(self.0.pathname())
    }

    #[getter]
    fn search(&self) -> PyResult<&str> {
        Ok(self.0.search())
    }

    #[getter]
    fn hash(&self) -> PyResult<&str> {
        Ok(self.0.hash())
    }

    #[getter(hasRegExpGroups)]
    fn has_regexp_groups(&self) -> PyResult<bool> {
        Ok(self.0.has_regexp_groups())
    }
}

struct UrlPatternResult<'py> {
    inputs: Vec<UrlPatternInput<'py>>,
    protocol: UrlPatternComponentResult,
    username: UrlPatternComponentResult,
    password: UrlPatternComponentResult,
    hostname: UrlPatternComponentResult,
    port: UrlPatternComponentResult,
    pathname: UrlPatternComponentResult,
    search: UrlPatternComponentResult,
    hash: UrlPatternComponentResult,
}

impl<'py> IntoPyObject<'py> for UrlPatternResult<'py> {
    type Target = PyDict;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let dict = PyDict::new(py);

        let inputs = PyList::empty(py);
        for input in self.inputs {
            match input {
                UrlPatternInput::String(string) => {
                    inputs.append(string)?;
                }
                UrlPatternInput::Init(init) => {
                    inputs.append(init)?;
                }
            }
        }

        dict.set_item("inputs", inputs)?;
        dict.set_item("protocol", self.protocol)?;
        dict.set_item("username", self.username)?;
        dict.set_item("password", self.password)?;
        dict.set_item("hostname", self.hostname)?;
        dict.set_item("port", self.port)?;
        dict.set_item("pathname", self.pathname)?;
        dict.set_item("search", self.search)?;
        dict.set_item("hash", self.hash)?;
        Ok(dict.into_bound())
    }
}

#[derive(IntoPyObject, IntoPyObjectRef)]
struct UrlPatternComponentResult {
    input: String,
    groups: HashMap<String, Option<String>>,
}

struct Error(::urlpattern::Error);

impl From<Error> for PyErr {
    fn from(error: Error) -> Self {
        PyValueError::new_err(error.0.to_string())
    }
}

impl From<::urlpattern::Error> for Error {
    fn from(other: ::urlpattern::Error) -> Self {
        Self(other)
    }
}

/// A Python module implemented in Rust.
#[pymodule]
mod urlpattern {
    #[pymodule_export]
    use super::UrlPattern;
}
