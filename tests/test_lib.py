import json
import pathlib

import pytest

from urlpattern import URLPattern

# This test is based on the web-platform-tests Project.
#
# To update the test data:
#
# 1. Go to https://github.com/web-platform-tests/wpt/blob/master/urlpattern/resources/urlpatterntestdata.json.
# 2. Copy the content.
# 3. Paste into `tests/urlpatterntestdata.json`.
urlpatterntestdata = json.loads(
    pathlib.Path("tests/urlpatterntestdata.json").read_text("utf-8")
)


@pytest.mark.parametrize("entry", urlpatterntestdata)
def test(entry):
    if len(entry["pattern"]) == 2 and isinstance(entry["pattern"][1], dict):
        pytest.xfail("unsupported parameter")

    if len(entry["pattern"]) == 3 and isinstance(entry["pattern"][2], dict):
        pytest.xfail("unsupported parameter")

    if entry.get("expected_obj") == "error":
        with pytest.raises(Exception):
            URLPattern(*entry["pattern"])
        return

    try:
        pattern = URLPattern(*entry["pattern"])

    except UnicodeEncodeError as e:
        if e.reason == "surrogates not allowed":
            pytest.xfail(e.reason)
        raise

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
                assert result
                assert result[key] == entry["expected_match"][key]
