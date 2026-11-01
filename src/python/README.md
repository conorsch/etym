# etym-py

Python bindings and CLI utility for [etym](https://github.com/conorsch/etym), an EtymOnline lookup engine written in Rust.

## Features

- **Blazing Fast**: Native Rust parsing performance under the hood.
- **Terminal Beautification**: Retains formatted text output and headers directly in your terminal.
- **PEP 561 Type Hints**: Built-in type stubs (`__init__.pyi`) and `py.typed` marker for IDE auto-completion, `mypy`, and `pyright`.
- **CLI Commands**: Exposes both `etym` and `etym-py` command-line entry points.

---

## Installation

### Build from Source

Ensure you have Rust (`cargo`) and Python (>= 3.8) installed.

```bash
# Clone the repository
git clone [https://github.com/conorsch/etym.git](https://github.com/conorsch/etym.git)
cd etym

##Python Usage

from etym import Etymology

# Fetch etymology entry
entry = Etymology("viking")

print(entry.word)       # Output: viking
print(entry.label)      # Output: Viking (n.)
print(entry.etymology)  # Formatted etymology text


# Install in editable mode via pip
pip install -e src/python

Project Structure
src/python/
├── Cargo.toml         # Dedicated PyO3 manifest (path = "../../")
├── pyproject.toml     # Hatchling + hatch-maturin-build configuration
├── src/
│   └── lib.rs         # PyO3 Rust bindings
└── etym/
    ├── __init__.py    # Python package entrypoint
    ├── __init__.pyi   # PEP 561 Type Stubs
    ├── __main__.py   # CLI entrypoint script
    └── py.typed       # Type marker for mypy/pyright

Building Distributions
​This package uses hatchling with hatch-maturin-build to compile binary wheel distributions:
pip install hatch
hatch build src/python



