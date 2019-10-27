mod bitset;
mod fast;
mod slow;
mod wl;

use crate::Graph;

pub fn are_isomorphic(g1: &Graph, g2: &Graph) -> bool {
    // Check some graph features first.
    // If these dont match, the graphs cannot be isomorphic.
    if !fast::are_isomorphic(g1, g2) {
        return false;
    }

    // if they are equal, they are also isomorphic.
    if g1.atoms() == g2.atoms() && g1.bonds() == g2.bonds() {
        return true;
    }

    // create two copies of the graphs so that we can alter the node names.
    let mut g1 = g1.clone();
    let mut g2 = g2.clone();

    // how many iterations of the relabeling-algorithm to do.
    const N_ITERS: usize = 1;

    for _ in 0..N_ITERS {
        // do a relabelling iteration
        // rename each node by appending their direct neighbors to themselves.
        wl::relabel(&mut g1);
        wl::relabel(&mut g2);

        let l1 = g1.label_counts();
        let l2 = g2.label_counts();

        // check if the label counts are the same.
        // If they are not, the graphs cannot be isomorphic.
        if l1 != l2 {
            return false;
        }
    }

    slow::are_isomorphic(&g1, &g2)
}
