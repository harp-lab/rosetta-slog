/*
   E-graph, Ascent version
*/

// use crate::slist::test_slog_list;

use egraph_id::run_egraph_id;
use egraph_struct::run_egraph_struct;

mod egraph_id;
mod egraph_struct;
mod set_lattice;
mod slist;

fn main() {
    // run_egraph_struct();
    //  test_slog_list();
    run_egraph_id();
}
