import pytest
from etym import Etymology

def test_etymology_lookup_success():
    """Verify looking up a valid word returns expected properties."""
    entry = Etymology("viking")
    
    assert isinstance(entry.word, str)
    assert entry.word == "viking"
    
    assert isinstance(entry.label, str)
    assert "Viking" in entry.label
    
    assert isinstance(entry.etymology, str)
    assert len(entry.etymology) > 0

def test_etymology_string_representation():
    """Verify __repr__ and __str__ output formatting."""
    entry = Etymology("robot")
    
    repr_str = repr(entry)
    assert repr_str.startswith("<Etymology label=")
    
    str_output = str(entry)
    assert entry.label in str_output
    assert entry.etymology in str_output

def test_etymology_invalid_argument_type():
    """Verify passing a non-string argument raises a TypeError."""
    with pytest.raises(TypeError):
        Etymology(12345)  # type: ignore

def test_etymology_not_found_error():
    """Verify looking up a non-existent word raises a ValueError from Rust."""
    with pytest.raises(ValueError, match="Failed to find an etymology"):
        Etymology("xyznonexistentword123456")