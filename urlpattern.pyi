from typing_extensions import TypeAlias, TypedDict, overload

URLPatternInput: TypeAlias = str | URLPatternInit

class URLPattern:
    @overload
    def __init__(input: URLPatternInput, baseURL: str): ...
    @overload
    def __init__(input: URLPatternInput = {}): ...
    def test(input: URLPatternInput = {}, baseURL: str | None = None) -> bool: ...
    def exec(
        input: URLPatternInput = {}, baseURL: str | None = None
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

class URLPatternInit(TypedDict):
    protocol: str
    username: str
    password: str
    hostname: str
    port: str
    pathname: str
    search: str
    hash: str
    baseURL: str

class URLPatternResult(TypedDict):
    inputs: list[URLPatternInput]

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
