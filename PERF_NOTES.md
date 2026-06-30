# Performance Notes — rsomics-graph-components

## Fixture

- **gnm_500k_200k.el**: 275,492 nodes, 200,000 edges, 75,492 connected components
- Generator: `random.Random(1234)`, 500k node universe, 200k unique non-self-loop edges
- File: `/Volumes/KIOXIA/rsomics-fixtures/graph-components/gnm_500k_200k.el` (2.6 MB)

## Machine

- **Machine**: Apple M2 mini (mini_m2), macOS Darwin 25.5.0, single thread
- **Upstream**: networkx 3.6.1, Python 3.12.0
- **rsomics-graph-components**: 0.1.0, Rust stable, `--release`

## Upstream versions

```
NetworkX 3.6.1 (pip, scanpy env)
```

## Results

### networkx compute-only (pre-built graph, `number_connected_components`)

5 runs:

```
142.2ms, 145.2ms, 145.6ms, 142.2ms, 153.5ms
mean=145.7ms  min=142.2ms
```

### networkx end-to-end (`read_edgelist` + `number_connected_components`)

5 runs:

```
708ms, 693ms, 690ms, 731ms, 706ms
mean=706ms  min=690ms
```

### rsomics-graph-components end-to-end (`--metric count`)

5 runs (wall time from `/usr/bin/time -l`):

```
110ms, 70ms, 70ms, 80ms, 80ms
mean=82ms  min=70ms
```

CPU user time: 60-70ms. Peak RSS: 62 MB.

### networkx peak RSS

167 MB (vs 62 MB ours — 2.7× lower memory).

## Ratios

| Comparison | Ratio |
|---|---|
| rsomics end-to-end vs networkx compute-only | **2.0× faster** (70ms vs 142ms) |
| rsomics end-to-end vs networkx end-to-end | **9.9× faster** (70ms vs 690ms) |
| Peak RSS | **2.7× lower** (62 MB vs 167 MB) |

## Gate

**PASS** — rsomics end-to-end (70ms) is faster than even networkx compute-only (142ms),
meeting the `>1.0×` contract on both axes.

networkx is pure-Python BFS; rsomics uses union-find with path compression and union by
size (O(α(n)·(n+m))), plus zero GC overhead and cache-friendly flat arrays. The win is
structural, not incidental.
