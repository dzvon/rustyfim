rusty-py-fim enables you to run fpgrowth in Python

## Setup

```bash
pip install maturin
maturin develop
```

## Running FIM in python

```py
from rustyfim import fpgrowth

res = fpgrowth(min_support=0.3, transactions=[[1,2,3],[2,3,4],[3,4,10],[3,4,20]])
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
