# URL Pattern

[![PyPI - Version](https://img.shields.io/pypi/v/urlpattern)](https://pypi.org/project/urlpattern/)
[![PyPI - Python Version](https://img.shields.io/pypi/pyversions/urlpattern)](https://pypi.org/project/urlpattern/)
[![Ruff](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/astral-sh/ruff/main/assets/badge/v2.json)](https://github.com/astral-sh/ruff)
[![CI](https://github.com/urlpattern/python-urlpattern/actions/workflows/CI.yml/badge.svg)](https://github.com/urlpattern/python-urlpattern/actions)

An implementation of [the URL Pattern Standard](https://urlpattern.spec.whatwg.org/) for Python written in Rust

## Introduction

It provides a pattern matching syntax like `/users/:id/`, similar to [Express](https://expressjs.com/) or [Path-to-RegExp](https://github.com/pillarjs/path-to-regexp) in Node.js. You can use it as a foundation to build your own web server or framework.

It's a thin wrapper of [denoland/rust-urlpattern](https://github.com/denoland/rust-urlpattern) with [PyO3](https://github.com/PyO3/pyo3) + [Maturin](https://github.com/PyO3/maturin).

## Examples

```py
from urlpattern import URLPattern

pattern = URLPattern("https://example.com/admin/*")
print(pattern.test("https://example.com/admin/main/"))  # output: True
print(pattern.test("https://example.com/main/"))  # output: False
```

```py
from urlpattern import URLPattern

pattern = URLPattern({"pathname": "/users/:id/"})
result = pattern.exec({"pathname": "/users/4163/"})
print(result["pathname"]["groups"]["id"])  # output: 4163
```

## Installation

On Linux/UNIX or macOS:

```sh
pip install urlpattern
```

On Windows:

```sh
py -m pip install urlpattern
```

## Limitations

Due to limitations in the dependency [denoland/rust-urlpattern](https://github.com/denoland/rust-urlpattern), it may not support all features specified in [the standard](https://urlpattern.spec.whatwg.org/).

Check the limitations in [`tests/test_lib.py`](tests/test_lib.py).
