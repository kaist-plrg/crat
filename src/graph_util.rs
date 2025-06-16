//! utils for graphs

use std::collections::{HashMap, HashSet};

use rustc_data_structures::graph::{scc, vec_graph::VecGraph};
use rustc_hash::{FxHashMap, FxHashSet};
use rustc_index::IndexVec;

rustc_index::newtype_index! {
    #[orderable]
    #[debug_format = "scc{}"]
    /// A unique identifier for a strongly connected component (SCC).
    pub struct SccId {}
}

/// Strongly connected components (SCCs) of a directed graph.
#[derive(Debug, Clone)]
pub struct Sccs<T> {
    /// SCC to its successors
    pub graph: FxHashMap<SccId, FxHashSet<SccId>>,
    /// SCC to its element nodes
    pub sccs: IndexVec<SccId, FxHashSet<T>>,
    /// Node to SCC it belongs to
    pub indices: FxHashMap<T, SccId>,
}

impl<T: Eq + std::hash::Hash> Sccs<T> {
    #[inline]
    pub fn scc(&self, t: &T) -> &FxHashSet<T> {
        &self.sccs[self.indices[t]]
    }

    #[inline]
    pub fn post_order(
        &self,
    ) -> impl DoubleEndedIterator<Item = SccId> + ExactSizeIterator + Clone + 'static {
        self.sccs.indices()
    }
}

/// Computes the strongly connected components (SCCs) of a directed graph whose nodes are copyable.
pub fn sccs_copied<T: Copy + Eq + std::hash::Hash, S1, S2>(
    graph: &HashMap<T, HashSet<T, S1>, S2>,
) -> Sccs<T> {
    let mut id_to_node = vec![];
    let mut node_to_id = FxHashMap::default();
    for (i, node) in graph.keys().copied().enumerate() {
        id_to_node.push(node);
        node_to_id.insert(node, i);
    }
    let edges = graph
        .iter()
        .flat_map(|(node, succs)| {
            let node = node_to_id[node];
            let node_to_id = &node_to_id;
            succs.iter().map(move |succ| (node, node_to_id[succ]))
        })
        .collect();
    sccs_common(id_to_node, edges)
}

/// Computes the strongly connected components (SCCs) of a directed graph.
pub fn sccs<T: Eq + std::hash::Hash, S1, S2>(graph: &HashMap<T, HashSet<T, S1>, S2>) -> Sccs<&T> {
    let mut id_to_node = vec![];
    let mut node_to_id = FxHashMap::default();
    for (i, node) in graph.keys().enumerate() {
        id_to_node.push(node);
        node_to_id.insert(node, i);
    }
    let edges = graph
        .iter()
        .flat_map(|(node, succs)| {
            let node = node_to_id[node];
            let node_to_id = &node_to_id;
            succs.iter().map(move |succ| (node, node_to_id[succ]))
        })
        .collect();
    sccs_common(id_to_node, edges)
}

fn sccs_common<T: Copy + Eq + std::hash::Hash>(
    id_to_node: Vec<T>,
    edges: Vec<(usize, usize)>,
) -> Sccs<T> {
    let vec_graph: VecGraph<usize> = VecGraph::new(id_to_node.len(), edges);

    let sccs: scc::Sccs<usize, SccId> = scc::Sccs::new(&vec_graph);

    let mut scc_graph = FxHashMap::default();
    let mut scc_elems: IndexVec<SccId, _> = IndexVec::new();
    let mut scc_indices = FxHashMap::default();
    for scc in sccs.all_sccs() {
        let succs: FxHashSet<_> = sccs.successors(scc).iter().cloned().collect();
        scc_graph.insert(scc, succs);
        scc_elems.push(FxHashSet::default());
    }
    for (i, node) in id_to_node.into_iter().enumerate() {
        let scc = sccs.scc(i);
        scc_elems[scc].insert(node);
        scc_indices.insert(node, scc);
    }

    Sccs {
        graph: scc_graph,
        sccs: scc_elems,
        indices: scc_indices,
    }
}
