# Compare Datasets

Documentation for the `pandas.DataFrame.compare` method:
https://pandas.pydata.org/pandas-docs/stable/reference/api/pandas.DataFrame.compare.html

## Setup

Tested with Python 3.12

```bash
python3 -m venv venv
source venv/bin/activate
pip install --upgrade pip

pip install --upgrade pipenv
pipenv sync
# or without pipenv
pip install -r requirements.txt
```

## Usage

`pipenv run python compare_datasets.py [path/file1.csv] [path/file2.csv] [id_key_name]`

or without pipenv

`python compare_datasets.py [path/file1.csv] [path/file2.csv] [id_key_name]`


Example:

```bash
pipenv run python compare_datasets.py data/example1.csv data/example2.csv PassengerId
# or
python compare_datasets.py data/example1.csv data/example2.csv PassengerId
```
