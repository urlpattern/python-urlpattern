# URL Pattern

An implementation of [the URL Pattern Standard](https://urlpattern.spec.whatwg.org/) for Python written in Rust

This is a thin wrapper of [denoland/rust-urlpattern](https://github.com/denoland/rust-urlpattern) with [PyO3](https://github.com/PyO3/pyo3) + [Maturin](https://github.com/PyO3/maturin).

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

## License

[MIT License](/LICENSE)