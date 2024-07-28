# URL Pattern

[![PyPI - Version](https://img.shields.io/pypi/v/urlpattern)](https://pypi.org/project/urlpattern/)
[![PyPI - Python Version](https://img.shields.io/pypi/pyversions/urlpattern)](https://pypi.org/project/urlpattern/)
[![Ruff](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/astral-sh/ruff/main/assets/badge/v2.json)](https://github.com/astral-sh/ruff)
[![CI](https://github.com/urlpattern/python-urlpattern/actions/workflows/CI.yml/badge.svg)](https://github.com/urlpattern/python-urlpattern/actions)

This library is an implementation of [the URL Pattern Standard](https://urlpattern.spec.whatwg.org/) for Python written in Rust.

You can match URLs to patterns using the syntax like `/users/:id/`, similar to [Express](https://expressjs.com/) or [Path-to-RegExp](https://github.com/pillarjs/path-to-regexp) in Node.js.

It can help you create a routing mechanism for your web servers or frameworks.

It's a thin wrapper of [denoland/rust-urlpattern](https://github.com/denoland/rust-urlpattern) with [PyO3](https://github.com/PyO3/pyo3) + [Maturin](https://github.com/PyO3/maturin).

## Installation

On Linux/UNIX or macOS:

```sh
pip install urlpattern
```

On Windows:

```sh
py -m pip install urlpattern
```

## Example

```py
from urlpattern import URLPattern

pattern = URLPattern("https://example.com/*")
print(pattern.test("https://example.com/foo/bar"))  # output: True

pattern = URLPattern({"pathname": "/:foo/:bar"})
result = pattern.exec("/abc/def", "https://test.example")
print(result["pathname"]["groups"]["foo"])  # output: abc
print(result["pathname"]["groups"]["bar"])  # output: def
```
