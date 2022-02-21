# Python Katas

## Katas

### ROSALIND problems
- [Counting point mutations with Hamming distance](https://rosalind.info/problems/hamm/): [Code](./katas/hamming_distance.py) and [tests](./tests/test_hamming_distance.py)
- [Computing GC Content](https://rosalind.info/problems/gc/)
- [Rabbits and Recurrence Relations](https://rosalind.info/problems/fib/)
- [Complementing a Strand of DNA](https://rosalind.info/problems/revc/)
- [Transcribing DNA into RNA](https://rosalind.info/problems/rna/)
- [Counting DNA Nucleotides](https://rosalind.info/problems/dna/)

## General Python configuration

- [Python Best Practices for a New Project in 2021](https://mitelman.engineering/blog/automating-python-best-practices-for-a-new-project/)

### Using `pyenv` to manage the Python interpreter

#### Installation via Homebrew

```shell
brew update && brew install pyenv
```

If you're using pyenv with pipenv and encountering the same issue, you can add the following lines to your **~/.zshrc** or **~/.zprofile** file:

```shell
export PYENV_ROOT="$HOME/.pyenv/shims"
export PATH="$PYENV_ROOT:$PATH"
export PIPENV_PYTHON="$PYENV_ROOT/python"
```

Restart your terminal after making these changes.

Referencing `pyenv`'s `/shims` folder helps to keep it more general and to allow you to easily switch between different Python versions, should you have more than one installed. The `pipenv` tool will then always reference the version of Python that is currently set as global by `pyenv`.

#### Installing a new Python interpreter

```shell
pyenv list
pyenv versions
pyenv install --list | grep " 3\."
pyenv install 3.10.1
pyenv local 3.10.1
python -V
```


### Using `poetry` to manage Python dependencies

#### Installation via Homebrew

```shell
brew update && brew install poetry
```

Add Poetry to the `PATH`, adding the following line to your **~/.zshrc** or **~/.zprofile** file:

```shell
export PATH="$HOME/.poetry/bin:$PATH"
```


#### Creating new Python projects with `poetry`

To create a new Python project named `my-project` using python `3.10.1` with Poetry:

```shell
poetry new my-project
cd my-project
pyenv local 3.10.1
python -V
poetry env use python
```


#### Activating the virtual environment via `poetry`

```shell
poetry shell
```


#### Adding dependencies with `poetry`

To install dependencies (installing `aiohttp` for this example):

```shell
poetry add aiohttp
```


#### Installing dependencies for local development

If you pull an existing project and want to install its dependencies for local development, simply run:

```shell
poetry install
```

#### Updating all dependencies to the latest versions:

```shell
poetry update
```

