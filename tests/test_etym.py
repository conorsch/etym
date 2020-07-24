from etym import __version__
import os

from etym.utils import query_etym_online

import random


def test_version() -> None:
    assert __version__ == "0.0.4"


def test_lookup_viking() -> None:
    """
    Test for a known etymology and assert string contents
    appear accurate.
    """
    s = "viking"
    result = query_etym_online(s)
    assert result
    assert result.startswith("Scandinavian pirate, 1801")


def test_lookup_random() -> None:
    word = get_random_word()
    result = query_etym_online(word)
    assert result
    print(result)


def get_random_word() -> str:
    """Fetch random word from system dictionary."""
    dictFile = "/usr/share/dict/words"
    assert os.path.exists(dictFile), "Could not find dictionary file at %s." % dictFile
    candidate = random.choice(open(dictFile).readlines()).rstrip("\n")
    return candidate
