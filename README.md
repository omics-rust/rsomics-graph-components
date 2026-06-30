# rsomics-graph-components

Connected-components queries for undirected graphs — value-exact equivalent of
[NetworkX](https://networkx.org/) 3.x `connected_components`, `number_connected_components`,
`is_connected`, and a canonical membership assignment. Part of the rsomics campaign
(single-binary bioinformatics CLIs outperforming the Python/R toolchain).

Used in biological network analysis: assembly graphs, protein-protein interaction
modules, co-expression clusters.

## Install

```sh
cargo install rsomics-graph-components
```

## Usage

```
rsomics-graph-components [OPTIONS] [EDGELIST]
```

Reads an undirected edge list: one `u v` pair per line (whitespace or TSV). Lines
starting with `#` are comments; blank lines are ignored. Self-loops are dropped
(no effect on components). Duplicate edges collapse. Only nodes appearing as
endpoints of non-self-loop edges exist in the graph (`nx.read_edgelist` semantics).

### `--metric`

| Value | Output |
|---|---|
| `count` (default) | Number of connected components (single integer) |
| `sizes` | Component sizes, sorted descending, one per line |
| `largest` | Size `\t` fraction of the largest component |
| `is-connected` | `true` or `false`; fails loud on empty graph (matches `nx.is_connected`) |
| `membership` | Per-node `node\tcomponent_id`, insertion order; see Canonical IDs |

### Canonical component IDs

networkx `connected_components` yields Python sets with no intrinsic ordering. The
`membership` metric assigns deterministic IDs: components sorted by size descending,
ties broken by smallest node-label ascending (string comparison), numbered 0, 1, 2, …

The **count** and **size multiset** are the networkx-exact invariants. The label
numbering is an rsomics-defined canonical ordering.

### JSON envelope

Pass `--json` for machine-readable output via rsomics-common's JSON envelope. Do not
combine with text-redirecting tools that consume stdout.

## Accuracy

All integer outputs (counts, sizes) are bit-exact vs networkx 3.6.1. The `largest`
fraction is one exact IEEE-754 division (`largest_size as f64 / total_nodes as f64`),
equal to the Python equivalent. Verified across: path, star, cliques, isolated pairs,
self-loops, duplicate edges, and `gnm(500, 400, seed=42)` — 37 compat tests, no
Python at runtime.

## Origin

This crate is an independent Rust reimplementation of NetworkX's connected-components
algorithms based on:
- The NetworkX 3.6.1 source (`networkx/algorithms/components/connected.py`) — BSD-3-Clause,
  reading and citing permitted
- The algorithm is BFS (networkx) / union-find with path compression and union by size (ours)

The BFS in networkx `_plain_bfs` and a union-find over the same adjacency structure are
equivalent for component queries. Our algorithm is O(α(n) · (n + m)) vs O(n + m) for BFS;
in practice union-find is faster due to better cache behaviour.

Golden fixtures were generated once from networkx 3.6.1 (2026-06-30) and are frozen.
Tests do not call Python at runtime.

License: MIT OR Apache-2.0.
Upstream credit: [NetworkX](https://github.com/networkx/networkx) — BSD-3-Clause.
