rustyfim enables you to run frequent itemset mining algorithm in Python

## Supported Algorithms

- [`NEclatClosed`](https://github.com/aryabarzan/NEclatClosed) - a vertical algorithm for mining frequent closed itemsets

## Setup

```bash
pip install maturin
maturin develop
```

## Running FIM in python

```py
>>> import rustyfim

>>> rustyfim.neclat(min_support=0.3, transactions=[[1,2,3],[2,3,4],[3,4,10],[3,4,20]])
[([2, 3], 2), ([4, 3], 3), ([3], 4)]
```

## Build wheel

```bash
maturin build
```

## Lint the project
```
cargo clippy
```
