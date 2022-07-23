rustyfim enables you to run frequent itemset mining algorithm in Python

## Supported Algorithms

- `FPGrowth` - mines frequent itemset
- `DCI` - mines closed item sets
- [`NEclatClosed`](https://github.com/aryabarzan/NEclatClosed) - a vertical algorithm for mining frequent closed itemsets

## Setup

```bash
pip install maturin
maturin develop
```

## Running FIM in python

```py
>>> import rustyfim

>>> rustyfim.fpgrowth(min_support=0.3, transactions=[[1,2,3],[2,3,4],[3,4,10],[3,4,20]])
First pass took 0 ms, num_transactions=4.
Total runtime: 1 ms
[([3], 4), ([3, 4], 3), ([4], 3)]

# in dci algorithm, n_features is required for creating the bitmatrix in the first place
>>> rustyfim.dci(min_support=0.3, transactions=[[1,2,3],[2,3,4],[3,4,10],[3,4,20]], n_features=21)
Total runtime: 4 ms
[([3], 4), ([1, 2, 3], 1), ([2, 3], 2), ([3, 4], 3), ([2, 3, 4], 1), ([3, 4, 10], 1), ([3, 4, 20], 1)]

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
