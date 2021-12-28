# coding-katas
Various coding katas


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


#### 
