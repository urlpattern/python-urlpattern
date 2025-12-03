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
    if entry["pattern"] == [{"pathname": "*{}**?"}]:
        pytest.skip("unsupported in the implementation")

    if entry.get("expected_obj") == "error":
        with pytest.raises(Exception):
            URLPattern(*entry["pattern"])
        return

    try:
        pattern = URLPattern(*entry["pattern"])

    except UnicodeEncodeError as e:
        if e.reason == "surrogates not allowed":
            pytest.skip("unsupported in the implementation")
        raise

    if "expected_obj" in entry:
        for key in entry["expected_obj"]:
            assert getattr(pattern, key) == entry["expected_obj"][key]

    if entry.get("expected_match") == "error":
        with pytest.raises(Exception):
            pattern.exec(*entry["inputs"])
        return

    if isinstance(entry.get("expected_match"), dict):
        result = pattern.exec(*entry["inputs"])
        assert result

        for key in entry["expected_match"]:
            assert result[key] == entry["expected_match"][key]

    if "exactly_empty_components" in entry:
        result = pattern.exec(*entry["inputs"])

        for component in entry["exactly_empty_components"]:
            if result:
                assert result[component]["groups"] == {}