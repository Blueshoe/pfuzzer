# Pfuzzer

Pfuzzer is a Python package designed for fuzzy matching, implemented in Rust. It leverages the performance and efficiency of Rust while providing a user-friendly interface for Python developers. 

## Features
- **Fuzzy String Matching**: Compare a list of target strings against a query string to find the best matches.
- **High Performance**: Built with Rust, Pfuzzer offers fast execution times and low overhead.
- **Easy to Use**: Simple API that integrates seamlessly with Python.

## Usage
```python
from pfuzzer import Pfuzzer

pf = Pfuzzer()

print(pf.compare_strings(["hello world", "hello blueshoe"], "helo world"))

>>> [257, None]
```
