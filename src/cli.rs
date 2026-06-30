use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, ValueEnum};
use serde::Serialize;

use rsomics_common::{CommonFlags, Result, RsomicsError, ToolMeta, run};

use rsomics_graph_components::{components::Components, io};

pub const META: ToolMeta = ToolMeta {
    name: env!("CARGO_PKG_NAME"),
    version: env!("CARGO_PKG_VERSION"),
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Metric {
    /// Number of connected components (default).
    Count,
    /// Component sizes, sorted descending (ties: smallest node label first).
    Sizes,
    /// Size and fraction of the largest component.
    Largest,
    /// True/false: is the graph fully connected? (Errors on empty graph.)
    #[value(name = "is-connected")]
    IsConnected,
    /// Per-node component membership (node TAB component_id).
    Membership,
}

/// Connected-components queries for undirected graphs.
///
/// Reads an edge list (one `u v` per line, whitespace-separated) from a file
/// argument or stdin (`-`). Comment lines starting with `#` and blank lines
/// are ignored. Self-loops are dropped; duplicate edges collapse to a simple
/// graph. Only nodes appearing as endpoints exist in the graph.
///
/// Canonical component IDs (for `--metric membership`) are assigned by sorting
/// components: largest first, then by smallest node-label ascending within ties.
/// This ordering is rsomics-defined — networkx `connected_components` yields sets
/// with no intrinsic ordering; the component COUNT and SIZE multiset are the
/// networkx-exact invariants.
#[derive(Parser, Debug)]
#[command(name = "rsomics-graph-components", version, about, long_about = None)]
pub struct Cli {
    /// Metric to compute.
    #[arg(long = "metric", value_enum, default_value = "count")]
    pub metric: Metric,

    /// Edge list file (`-` or omitted reads stdin).
    #[arg(value_name = "EDGELIST")]
    pub edgelist: Option<PathBuf>,

    #[command(flatten)]
    pub common: CommonFlags,
}

#[derive(Serialize)]
#[serde(untagged)]
enum Out {
    Count {
        count: usize,
    },
    Sizes {
        sizes: Vec<usize>,
    },
    Largest {
        largest: usize,
        fraction: f64,
    },
    IsConnected {
        is_connected: bool,
    },
    Membership {
        nodes: Vec<String>,
        component_ids: Vec<usize>,
    },
}

impl Cli {
    pub fn run(self) -> ExitCode {
        let common = self.common.clone();
        run(&common, META, || self.execute(&common))
    }

    fn execute(self, common: &CommonFlags) -> Result<Out> {
        let path = self.edgelist.as_deref();
        let g = io::read_edgelist(path)?;
        let mut comp = Components::compute(&g);

        match self.metric {
            Metric::Count => {
                let n = comp.count();
                if !common.json {
                    println!("{n}");
                }
                Ok(Out::Count { count: n })
            }

            Metric::Sizes => {
                let (sizes, _) = comp.sizes_and_groups();
                if !common.json {
                    let stdout = std::io::stdout().lock();
                    let mut w = BufWriter::new(stdout);
                    for s in &sizes {
                        writeln!(w, "{s}").map_err(RsomicsError::Io)?;
                    }
                    w.flush().map_err(RsomicsError::Io)?;
                }
                Ok(Out::Sizes { sizes })
            }

            Metric::Largest => {
                if g.n() == 0 {
                    return Err(RsomicsError::InvalidInput(
                        "empty graph: no nodes in edge list".into(),
                    ));
                }
                let (sizes, _) = comp.sizes_and_groups();
                let largest = sizes[0];
                let fraction = largest as f64 / g.n() as f64;
                if !common.json {
                    println!("{largest}\t{fraction}");
                }
                Ok(Out::Largest { largest, fraction })
            }

            Metric::IsConnected => {
                if g.n() == 0 {
                    return Err(RsomicsError::InvalidInput(
                        "Connectivity is undefined for the null graph.".into(),
                    ));
                }
                let connected = comp.count() == 1;
                if !common.json {
                    println!("{}", if connected { "true" } else { "false" });
                }
                Ok(Out::IsConnected {
                    is_connected: connected,
                })
            }

            Metric::Membership => {
                let (_, groups) = comp.sizes_and_groups();
                let mut node_cid = vec![0usize; g.n()];
                for (cid, group) in groups.iter().enumerate() {
                    for &nid in group {
                        node_cid[nid as usize] = cid;
                    }
                }
                if !common.json {
                    let stdout = std::io::stdout().lock();
                    let mut w = BufWriter::new(stdout);
                    for (label, cid) in g.labels.iter().zip(node_cid.iter()) {
                        writeln!(w, "{label}\t{cid}").map_err(RsomicsError::Io)?;
                    }
                    w.flush().map_err(RsomicsError::Io)?;
                }
                Ok(Out::Membership {
                    nodes: g.labels.clone(),
                    component_ids: node_cid,
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    #[test]
    fn cli_definition_is_valid() {
        super::Cli::command().debug_assert();
    }
}
