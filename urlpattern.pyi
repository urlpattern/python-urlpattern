from typing_extensions import TypeAlias, TypedDict, overload


class URLPatternOptions(TypedDict, total=False):
    ignoreCase: bool


class URLPatternInit(TypedDict, total=False):
    protocol: str
    username: str
    password: str
    hostname: str
    port: str
    pathname: str
    search: str
    hash: str
    baseURL: str

class URLPattern:
    @overload
    def __init__(
        self,
        input: URLPatternInit,
        baseURL: str,
        options: URLPatternOptions | None = None,
    ): ...
    @overload
    def __init__(
        self,
        input: str,
        baseURL: str,
        options: URLPatternOptions | None = None,
    ): ...
    @overload
    def __init__(
        self, input: URLPatternInit, options: URLPatternOptions | None = None
    ): ...
    @overload
    def __init__(
        self, input: str, options: URLPatternOptions | None = None
    ): ...
    @overload
    def test(self, input: URLPatternInit, baseURL: str | None = None) -> bool: ...
    @overload
    def test(self, input: str, baseURL: str | None = None) -> bool: ...
    @overload
    def exec(
        self, input: URLPatternInit, baseURL: str | None = None
    ) -> URLPatternResult | None: ...
    @overload
    def exec(
        self, input: str, baseURL: str | None = None
    ) -> URLPatternResult | None: ...
    @property
    def protocol(self) -> str: ...
    @property
    def username(self) -> str: ...
    @property
    def password(self) -> str: ...
    @property
    def hostname(self) -> str: ...
    @property
    def port(self) -> str: ...
    @property
    def pathname(self) -> str: ...
    @property
    def search(self) -> str: ...
    @property
    def hash(self) -> str: ...


class URLPatternResult(TypedDict):
    inputs: list[str | URLPatternInit]

    protocol: URLPatternComponentResult
    username: URLPatternComponentResult
    password: URLPatternComponentResult
    hostname: URLPatternComponentResult
    port: URLPatternComponentResult
    pathname: URLPatternComponentResult
    search: URLPatternComponentResult
    hash: URLPatternComponentResult

class URLPatternComponentResult(TypedDict):
    input: str
    groups: dict[str, str]

URLPatternCompatible: TypeAlias = str | URLPatternInit | URLPattern
