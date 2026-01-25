# URL Pattern

[![PyPI - Version](https://img.shields.io/pypi/v/urlpattern)](https://pypi.org/project/urlpattern/)
[![PyPI - Python Version](https://img.shields.io/pypi/pyversions/urlpattern)](https://pypi.org/project/urlpattern/)
[![Ruff](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/astral-sh/ruff/main/assets/badge/v2.json)](https://github.com/astral-sh/ruff)
[![CI](https://github.com/urlpattern/python-urlpattern/actions/workflows/CI.yml/badge.svg)](https://github.com/urlpattern/python-urlpattern/actions)

An implementation of [the URL Pattern Standard](https://urlpattern.spec.whatwg.org/) for Python written in Rust.

## Introduction

The URL Pattern Standard is a web standard for URL pattern matching. It is useful on the server side when serving different pages based on the URL (a.k.a. routing). It provides pattern matching syntax like `/users/:id`, similar to [route parameters in Express](https://expressjs.com/en/guide/routing.html#route-parameters) or [Path-to-RegExp](https://github.com/pillarjs/path-to-regexp). You can use it as a foundation to build your own web server or framework.

It's a thin wrapper of [denoland/rust-urlpattern](https://github.com/denoland/rust-urlpattern) with [PyO3](https://github.com/PyO3/pyo3) + [Maturin](https://github.com/PyO3/maturin).

The naming conventions follow [the standard](https://urlpattern.spec.whatwg.org/) as closely as possible, similar to [xml.dom](https://docs.python.org/3/library/xml.dom.html).

## Installation

On Linux/UNIX or macOS:

```sh
pip install urlpattern
```

On Windows:

```sh
py -m pip install urlpattern
```

## Usage

Check [urlpattern.pyi](https://github.com/urlpattern/python-urlpattern/blob/main/urlpattern.pyi) or the examples below.

For various usage examples, you can also check [Chrome for Developers](https://developer.chrome.com/docs/web-platform/urlpattern) or [MDN](https://developer.mozilla.org/en-US/docs/Web/API/URL_Pattern_API) (you need to convert JavaScript into Python).

## Examples

### `test`

```py
from urlpattern import URLPattern

pattern = URLPattern("https://example.com/admin/*")
print(pattern.test("https://example.com/admin/main/"))  # output: True
print(pattern.test("https://example.com/main/"))  # output: False
```

### `exec`

```py
from urlpattern import URLPattern

pattern = URLPattern({"pathname": "/users/:id/"})
result = pattern.exec({"pathname": "/users/4163/"})
print(result["pathname"]["groups"]["id"])  # output: 4163
```

### `baseURL`

```py
from urlpattern import URLPattern

pattern = URLPattern({"pathname": "/admin/*"}, "https://example.com")
print(pattern.test({"pathname": "/admin/main/"}, "https://example.com"))  # output: True
print(pattern.test("/admin/main/", "https://example.com"))  # output: True
```

### `ignoreCase`

```py
from urlpattern import URLPattern

pattern = URLPattern("https://example.com/test")
print(pattern.test("https://example.com/test"))  # output: True
print(pattern.test("https://example.com/TeST"))  # output: False

pattern = URLPattern("https://example.com/test", {"ignoreCase": True})
print(pattern.test("https://example.com/test"))  # output: True
print(pattern.test("https://example.com/TeST"))  # output: True
```

### A simple WSGI app

```py
from wsgiref.simple_server import make_server

from urlpattern import URLPattern

user_id_pattern = URLPattern({"pathname": "/users/:id"})


def get_user_id(environ, start_response):
    user_id = environ["result"]["pathname"]["groups"]["id"]
    status = "200 OK"
    response_headers = [("Content-type", "text/plain; charset=utf-8")]
    start_response(status, response_headers)
    return [f"{user_id=}".encode()]


def app(environ, start_response):
    path = environ["PATH_INFO"]
    method = environ["REQUEST_METHOD"]

    if result := user_id_pattern.exec({"pathname": path}):
        if method == "GET":
            return get_user_id(environ | {"result": result}, start_response)

    status = "404 Not Found"
    response_headers = [("Content-type", "text/plain; charset=utf-8")]
    start_response(status, response_headers)
    return [b"Not Found"]


with make_server("", 8000, app) as httpd:
    httpd.serve_forever()
```

## Limitations

Due to limitations in the dependency [denoland/rust-urlpattern](https://github.com/denoland/rust-urlpattern), it may not support all features specified in [the standard](https://urlpattern.spec.whatwg.org/).

Check `pytest.skip` in [`tests/test_lib.py`](https://github.com/urlpattern/python-urlpattern/blob/main/tests/test_lib.py).
