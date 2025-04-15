# run pip install -e . to install locally defined dependencies
from setuptools import setup, find_packages

setup(
    name="pylmatdoc-test",
    version="0.1.0",
    packages=find_packages(),  # This finds all packages with __init__.py files
    install_requires=[
        "maturin", 
        "pytest"
    ],
)
