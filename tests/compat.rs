//! Value-exact compat tests against frozen networkx 3.6.1 expectations.
//!
//! Frozen expectations were generated once with:
//!   networkx 3.6.1, python 3.12, mac M2 (2026-06-30)
//! Tests MUST NOT call python at runtime — expectations are hard-coded here.

use std::path::Path;
use std::process::Command;

fn bin() -> std::path::PathBuf {
    env!("CARGO_BIN_EXE_rsomics-graph-components").into()
}

fn golden(name: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/golden")
        .join(name)
}

fn run(metric: &str, file: &str) -> String {
    let out = Command::new(bin())
        .args(["--metric", metric, golden(file).to_str().unwrap()])
        .output()
        .expect("binary failed to launch");
    assert!(
        out.status.success(),
        "binary exited non-zero for {file} --metric {metric}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).unwrap().trim().to_owned()
}

fn run_expect_fail(metric: &str, file: &str) {
    let out = Command::new(bin())
        .args(["--metric", metric, golden(file).to_str().unwrap()])
        .output()
        .expect("binary failed to launch");
    assert!(
        !out.status.success(),
        "expected non-zero exit for {file} --metric {metric}"
    );
}

fn sizes_from_output(s: &str) -> Vec<usize> {
    s.lines()
        .map(|l| {
            l.trim()
                .parse::<usize>()
                .expect("expected integer size line")
        })
        .collect()
}

fn membership_from_output(s: &str) -> Vec<(String, usize)> {
    s.lines()
        .map(|l| {
            let mut parts = l.splitn(2, '\t');
            let node = parts.next().unwrap().to_owned();
            let cid: usize = parts.next().unwrap().parse().unwrap();
            (node, cid)
        })
        .collect()
}

// ── path6 ──────────────────────────────────────────────────────────────────
// networkx: nodes=6, edges=5, count=1, sizes=[6], largest=6, fraction=1.0

#[test]
fn path6_count() {
    assert_eq!(run("count", "path6.el"), "1");
}

#[test]
fn path6_sizes() {
    assert_eq!(sizes_from_output(&run("sizes", "path6.el")), vec![6]);
}

#[test]
fn path6_largest() {
    let out = run("largest", "path6.el");
    let mut parts = out.splitn(2, '\t');
    let sz: usize = parts.next().unwrap().parse().unwrap();
    let frac: f64 = parts.next().unwrap().parse().unwrap();
    assert_eq!(sz, 6);
    assert_eq!(frac, 1.0_f64);
}

#[test]
fn path6_is_connected() {
    assert_eq!(run("is-connected", "path6.el"), "true");
}

#[test]
fn path6_membership() {
    let m = membership_from_output(&run("membership", "path6.el"));
    // all 6 nodes same component (id=0)
    assert_eq!(m.len(), 6);
    let ids: Vec<usize> = m.iter().map(|(_, id)| *id).collect();
    assert!(ids.iter().all(|&id| id == 0));
}

// ── isolated_pairs ──────────────────────────────────────────────────────────
// networkx: nodes=10, edges=5, count=5, sizes=[2,2,2,2,2]

#[test]
fn isolated_pairs_count() {
    assert_eq!(run("count", "isolated_pairs.el"), "5");
}

#[test]
fn isolated_pairs_sizes() {
    assert_eq!(
        sizes_from_output(&run("sizes", "isolated_pairs.el")),
        vec![2, 2, 2, 2, 2]
    );
}

#[test]
fn isolated_pairs_largest() {
    let out = run("largest", "isolated_pairs.el");
    let mut parts = out.splitn(2, '\t');
    let sz: usize = parts.next().unwrap().parse().unwrap();
    let frac: f64 = parts.next().unwrap().parse().unwrap();
    assert_eq!(sz, 2);
    assert_eq!(frac, 0.2_f64);
}

#[test]
fn isolated_pairs_is_connected() {
    assert_eq!(run("is-connected", "isolated_pairs.el"), "false");
}

// ── star5 ──────────────────────────────────────────────────────────────────
// networkx: nodes=5, count=1, sizes=[5]

#[test]
fn star5_count() {
    assert_eq!(run("count", "star5.el"), "1");
}

#[test]
fn star5_sizes() {
    assert_eq!(sizes_from_output(&run("sizes", "star5.el")), vec![5]);
}

#[test]
fn star5_is_connected() {
    assert_eq!(run("is-connected", "star5.el"), "true");
}

// ── two_cliques_disjoint ────────────────────────────────────────────────────
// networkx: nodes=8, count=2, sizes=[4,4], largest=4, fraction=0.5

#[test]
fn two_cliques_disjoint_count() {
    assert_eq!(run("count", "two_cliques_disjoint.el"), "2");
}

#[test]
fn two_cliques_disjoint_sizes() {
    assert_eq!(
        sizes_from_output(&run("sizes", "two_cliques_disjoint.el")),
        vec![4, 4]
    );
}

#[test]
fn two_cliques_disjoint_largest() {
    let out = run("largest", "two_cliques_disjoint.el");
    let mut parts = out.splitn(2, '\t');
    let sz: usize = parts.next().unwrap().parse().unwrap();
    let frac: f64 = parts.next().unwrap().parse().unwrap();
    assert_eq!(sz, 4);
    assert_eq!(frac, 0.5_f64);
}

#[test]
fn two_cliques_disjoint_is_connected() {
    assert_eq!(run("is-connected", "two_cliques_disjoint.el"), "false");
}

#[test]
fn two_cliques_disjoint_membership() {
    let m = membership_from_output(&run("membership", "two_cliques_disjoint.el"));
    // nodes 0-3 are one component, 4-7 another; canonical: both size 4, smaller
    // min-node wins → comp 0 has min-node=0 (nodes 0-3), comp 1 has min-node=4 (nodes 4-7)
    let map: std::collections::HashMap<String, usize> = m.into_iter().collect();
    let cid_0 = map["0"];
    let cid_4 = map["4"];
    assert_ne!(cid_0, cid_4, "two cliques must be different components");
    // all nodes 0-3 same cid
    for n in &["0", "1", "2", "3"] {
        assert_eq!(map[*n], cid_0);
    }
    // all nodes 4-7 same cid
    for n in &["4", "5", "6", "7"] {
        assert_eq!(map[*n], cid_4);
    }
    // canonical: larger min-node group gets higher id; both size=4, min-nodes 0 vs 4 → cid_0 < cid_4
    assert!(cid_0 < cid_4);
}

// ── two_cliques_joined ──────────────────────────────────────────────────────
// networkx: nodes=8, count=1, sizes=[8]

#[test]
fn two_cliques_joined_count() {
    assert_eq!(run("count", "two_cliques_joined.el"), "1");
}

#[test]
fn two_cliques_joined_sizes() {
    assert_eq!(
        sizes_from_output(&run("sizes", "two_cliques_joined.el")),
        vec![8]
    );
}

#[test]
fn two_cliques_joined_is_connected() {
    assert_eq!(run("is-connected", "two_cliques_joined.el"), "true");
}

// ── selfloop_dup ────────────────────────────────────────────────────────────
// networkx (read_edgelist): nodes=4 (A,B,C,D), count=1, sizes=[4]
// self-loops don't affect connectivity; duplicate edges collapse

#[test]
fn selfloop_dup_count() {
    assert_eq!(run("count", "selfloop_dup.el"), "1");
}

#[test]
fn selfloop_dup_sizes() {
    assert_eq!(sizes_from_output(&run("sizes", "selfloop_dup.el")), vec![4]);
}

#[test]
fn selfloop_dup_is_connected() {
    assert_eq!(run("is-connected", "selfloop_dup.el"), "true");
}

// ── selfloop_singleton ──────────────────────────────────────────────────────
// networkx (read_edgelist): nodes=3 (1,2,3), edges 1-2 + self-loop 3-3.
// A self-loop-only node is its own singleton → count=2, sizes=[2,1].

#[test]
fn selfloop_singleton_count() {
    assert_eq!(run("count", "selfloop_singleton.el"), "2");
}

#[test]
fn selfloop_singleton_sizes() {
    assert_eq!(
        sizes_from_output(&run("sizes", "selfloop_singleton.el")),
        vec![2, 1]
    );
}

#[test]
fn selfloop_singleton_largest() {
    let out = run("largest", "selfloop_singleton.el");
    let mut parts = out.splitn(2, '\t');
    let sz: usize = parts.next().unwrap().parse().unwrap();
    let frac: f64 = parts.next().unwrap().parse().unwrap();
    assert_eq!(sz, 2);
    assert_eq!(frac, 2.0_f64 / 3.0_f64);
}

#[test]
fn selfloop_singleton_is_connected() {
    assert_eq!(run("is-connected", "selfloop_singleton.el"), "false");
}

#[test]
fn selfloop_singleton_membership() {
    let m = membership_from_output(&run("membership", "selfloop_singleton.el"));
    let map: std::collections::HashMap<String, usize> = m.into_iter().collect();
    assert_eq!(map.len(), 3);
    assert_eq!(map["1"], map["2"], "1 and 2 share a component");
    assert_ne!(map["1"], map["3"], "3 is its own singleton");
}

// ── selfloop_all_isolated ───────────────────────────────────────────────────
// networkx: nodes=2 (5,6), both self-loop-only → count=2, sizes=[1,1].
// Regression guard for the drop-to-empty bug (was: 0 components).

#[test]
fn selfloop_all_isolated_count() {
    assert_eq!(run("count", "selfloop_all_isolated.el"), "2");
}

#[test]
fn selfloop_all_isolated_sizes() {
    assert_eq!(
        sizes_from_output(&run("sizes", "selfloop_all_isolated.el")),
        vec![1, 1]
    );
}

#[test]
fn selfloop_all_isolated_largest() {
    let out = run("largest", "selfloop_all_isolated.el");
    let mut parts = out.splitn(2, '\t');
    let sz: usize = parts.next().unwrap().parse().unwrap();
    let frac: f64 = parts.next().unwrap().parse().unwrap();
    assert_eq!(sz, 1);
    assert_eq!(frac, 0.5_f64);
}

#[test]
fn selfloop_all_isolated_is_connected() {
    assert_eq!(run("is-connected", "selfloop_all_isolated.el"), "false");
}

// ── selfloop_mixed ──────────────────────────────────────────────────────────
// networkx: nodes=5 (a,b,c,d,e), a-b-d-a triangle + self-loop-only c and e.
// count=3, sizes=[3,1,1]. Verifies singleton nodes interleaved among real edges.

#[test]
fn selfloop_mixed_count() {
    assert_eq!(run("count", "selfloop_mixed.el"), "3");
}

#[test]
fn selfloop_mixed_sizes() {
    assert_eq!(
        sizes_from_output(&run("sizes", "selfloop_mixed.el")),
        vec![3, 1, 1]
    );
}

#[test]
fn selfloop_mixed_largest() {
    let out = run("largest", "selfloop_mixed.el");
    let mut parts = out.splitn(2, '\t');
    let sz: usize = parts.next().unwrap().parse().unwrap();
    let frac: f64 = parts.next().unwrap().parse().unwrap();
    assert_eq!(sz, 3);
    assert_eq!(frac, 3.0_f64 / 5.0_f64);
}

#[test]
fn selfloop_mixed_membership() {
    let m = membership_from_output(&run("membership", "selfloop_mixed.el"));
    let map: std::collections::HashMap<String, usize> = m.into_iter().collect();
    assert_eq!(map.len(), 5);
    // triangle a,b,d share one component (canonical id 0: largest, min-node first)
    assert_eq!(map["a"], 0);
    assert_eq!(map["b"], 0);
    assert_eq!(map["d"], 0);
    // singletons c (inserted before e) and e get distinct ids, ordered by node id
    assert_eq!(map["c"], 1);
    assert_eq!(map["e"], 2);
}

// ── comments ────────────────────────────────────────────────────────────────
// networkx: nodes=6 (X,Y,Z,A,B,C), count=2, sizes=[3,3]

#[test]
fn comments_count() {
    assert_eq!(run("count", "comments.el"), "2");
}

#[test]
fn comments_sizes() {
    assert_eq!(sizes_from_output(&run("sizes", "comments.el")), vec![3, 3]);
}

#[test]
fn comments_is_connected() {
    assert_eq!(run("is-connected", "comments.el"), "false");
}

// ── single_edge ─────────────────────────────────────────────────────────────
// networkx: nodes=2, count=1, sizes=[2], fraction=1.0

#[test]
fn single_edge_count() {
    assert_eq!(run("count", "single_edge.el"), "1");
}

#[test]
fn single_edge_sizes() {
    assert_eq!(sizes_from_output(&run("sizes", "single_edge.el")), vec![2]);
}

#[test]
fn single_edge_largest() {
    let out = run("largest", "single_edge.el");
    let mut parts = out.splitn(2, '\t');
    let sz: usize = parts.next().unwrap().parse().unwrap();
    let frac: f64 = parts.next().unwrap().parse().unwrap();
    assert_eq!(sz, 2);
    assert_eq!(frac, 1.0_f64);
}

#[test]
fn single_edge_is_connected() {
    assert_eq!(run("is-connected", "single_edge.el"), "true");
}

// ── gnm500_400 ───────────────────────────────────────────────────────────────
// networkx (edge-only nodes): nodes=396, count=30
// sizes=[301,14,7,6,6,5,4,3,3,3,3,3,3,3,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2]

#[test]
fn gnm500_400_count() {
    assert_eq!(run("count", "gnm500_400.el"), "30");
}

#[test]
fn gnm500_400_sizes() {
    let expected = vec![
        301, 14, 7, 6, 6, 5, 4, 3, 3, 3, 3, 3, 3, 3, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    ];
    assert_eq!(sizes_from_output(&run("sizes", "gnm500_400.el")), expected);
}

#[test]
fn gnm500_400_largest() {
    let out = run("largest", "gnm500_400.el");
    let mut parts = out.splitn(2, '\t');
    let sz: usize = parts.next().unwrap().parse().unwrap();
    let frac: f64 = parts.next().unwrap().parse().unwrap();
    assert_eq!(sz, 301);
    // 301/396 = 0.76010101010101...
    assert_eq!(frac, 301.0_f64 / 396.0_f64);
}

#[test]
fn gnm500_400_is_connected() {
    assert_eq!(run("is-connected", "gnm500_400.el"), "false");
}

// ── is-connected on empty graph must error ───────────────────────────────────
// networkx raises NetworkXPointlessConcept; we must exit non-zero

#[test]
fn empty_graph_is_connected_fails() {
    // create an empty file (all comments)
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("empty.el");
    std::fs::write(&p, "# no edges\n").unwrap();
    let out = Command::new(bin())
        .args(["--metric", "is-connected", p.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(!out.status.success(), "must fail on empty graph");
}

#[test]
fn empty_graph_largest_fails() {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("empty.el");
    std::fs::write(&p, "# no edges\n").unwrap();
    run_expect_fail("largest", p.to_str().unwrap());
}

// ── count on empty graph returns 0 (no components) ──────────────────────────
#[test]
fn empty_graph_count_zero() {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("empty.el");
    std::fs::write(&p, "# no edges\n").unwrap();
    let out = Command::new(bin())
        .args(["--metric", "count", p.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(out.status.success());
    assert_eq!(String::from_utf8(out.stdout).unwrap().trim(), "0");
}
