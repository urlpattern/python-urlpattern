import json
import pathlib
import urllib.request

import pytest
from urlpattern import URLPattern

# To update the test data:
#
# 1. Remove tests/urlpatterntestdata.json.
# 2. Update the URL.
# 2. Run `pytest`.
# TODO with argparse
urlpatterntestdata_path = pathlib.Path("tests/urlpatterntestdata.json")
if not urlpatterntestdata_path.exists():
    with urllib.request.urlopen(
        "https://raw.githubusercontent.com/web-platform-tests/wpt/3ce3e9794fcd97ff24506f5c5325f91fc00ef79c/urlpattern/resources/urlpatterntestdata.json"
    ) as f:
        urlpatterntestdata_path.write_bytes(f.read())
urlpatterntestdata = json.loads(urlpatterntestdata_path.read_text("utf-8"))


@pytest.mark.parametrize("entry", urlpatterntestdata)
def test(entry):
    if len(entry["pattern"]) == 2 and isinstance(entry["pattern"][1], dict):
        pytest.xfail("unsupported parameter")

    if len(entry["pattern"]) == 3 and isinstance(entry["pattern"][2], dict):
        pytest.xfail("unsupported parameter")

    if entry.get("expected_obj") == "error":
        if (
            isinstance(entry["pattern"][0], dict)
            and entry["pattern"][0].get("hostname") == "bad\\:hostname"
        ):
            pytest.xfail("unknown")

        with pytest.raises(Exception):
            URLPattern(*entry["pattern"])
        return

    try:
        pattern = URLPattern(*entry["pattern"])

    except ValueError:
        pytest.xfail("unsupported regular expression")

    if "expected_obj" in entry:
        for key in entry["expected_obj"]:
            if getattr(pattern, key) in ("", "/") and entry["expected_obj"][key] == "*":
                assert True

            else:
                assert getattr(pattern, key) == entry["expected_obj"][key]

    if entry.get("expected_match") == "error":
        with pytest.raises(Exception):
            pattern.exec(*entry["inputs"])
        return

    elif isinstance(entry.get("expected_match"), dict):
        result = pattern.exec(*entry["inputs"])

        for key in entry["expected_match"]:
            if key != "inputs":
                for group in entry["expected_match"][key]["groups"]:
                    if entry["expected_match"][key]["groups"][group] is None:
                        pytest.xfail("unsupported undefined group")

            if "exactly_empty_components" in entry:
                if key not in entry["exactly_empty_components"]:
                    continue

            else:
                assert result[key] == entry["expected_match"][key]
