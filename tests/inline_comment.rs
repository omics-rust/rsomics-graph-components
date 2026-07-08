//! An inline `#` comment must yield the same graph as the comment-free edge list,
//! matching `nx.parse_edgelist` (a `#` starts a comment anywhere in a line, so the
//! text from it is dropped before tokenising).

use std::io::Write;

use rsomics_graph_components::components::Components;
use rsomics_graph_components::io::read_edgelist;
use tempfile::NamedTempFile;

fn components_of(text: &str) -> (Vec<usize>, Vec<Vec<String>>) {
    let mut f = NamedTempFile::new_in("/Volumes/KIOXIA/tmp").unwrap();
    f.write_all(text.as_bytes()).unwrap();
    f.flush().unwrap();
    let g = read_edgelist(Some(f.path())).unwrap();
    let mut c = Components::compute(&g);
    let (sizes, groups) = c.sizes_and_groups();
    let labelled: Vec<Vec<String>> = groups
        .iter()
        .map(|grp| {
            grp.iter()
                .map(|&id| g.labels[id as usize].clone())
                .collect()
        })
        .collect();
    (sizes, labelled)
}

#[test]
fn inline_hash_matches_comment_free() {
    // "1 2#note": without stripping this parses the second token as the node "2#note",
    // splitting the graph; nx (and the fix) drops "#note" and yields edge (1,2).
    // The bare "#full line" is a comment-only line and must be skipped.
    let with_comments = "0 1\n1 2#note\n2 3\n#full line\n";
    let clean = "0 1\n1 2\n2 3\n";
    assert_eq!(components_of(with_comments), components_of(clean));
}
