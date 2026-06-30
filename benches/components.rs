use criterion::{Criterion, criterion_group, criterion_main};
use rsomics_graph_components::{components::Components, io};
use std::path::Path;

fn bench_gnm_large(c: &mut Criterion) {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../rsomics-fixtures/graph-components/gnm_100k_1m.el");

    if !fixture.exists() {
        eprintln!("skipping bench: fixture not found at {}", fixture.display());
        return;
    }

    let g = io::read_edgelist(Some(&fixture)).expect("read fixture");
    c.bench_function("count_components_100k_1m", |b| {
        b.iter(|| {
            let mut comp = Components::compute(&g);
            std::hint::black_box(comp.count())
        })
    });
}

criterion_group!(benches, bench_gnm_large);
criterion_main!(benches);
