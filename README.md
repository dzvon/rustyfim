rustyfim enables you to run frequent itemset mining algorithm (both `FPGrowth` that mines frequent itemset, and `DCI` algorithm that mines closed item sets) in Python

## Setup

```bash
pip install maturin
maturin develop
```

## Running FIM in python

```py
from rustyfim import fpgrowth, dci

res = fpgrowth(min_support=0.3, transactions=[[1,2,3],[2,3,4],[3,4,10],[3,4,20]])
# in dci algorithm, n_features is required for creating the bitmatrix in the first place
res = dci(min_support=0.3, transactions=[[1,2,3],[2,3,4],[3,4,10],[3,4,20]], n_features=21)
```

Result:

```
First pass took 0 ms, num_transactions=4.
Total runtime: 1 ms
[([3], 4), ([3, 4], 3), ([4], 3)]
```

## Build wheel

```bash
maturin build
```

## Lint the project
```
cargo clippy
```
