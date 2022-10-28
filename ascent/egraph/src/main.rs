/*
 * E-graph, Ascent version
 *  - EMatch and Rewrite is separated into 2 different programs, mainly because write rule generate a lot of intermediate data,
 *    however these EDB/IDB won't be used in normal matching task at all. During rewriting, seems matching only e-node is more
 *    useful than match e-class (this is like inlining in souffle, I want a kind of manually "GC").
 * TODO:
 *  - Add cost function, need dig more into egg paper
*/

use std::collections::BTreeSet;
use std::ops::Deref;
use std::rc::Rc;

mod ematch;
mod erewrite;
pub mod graph;

use ascent::lattice::set::Set;
use graph::*;

use graph::ENodeId::*;
use graph::PatternExpr::*;

use ematch::*;
use erewrite::*;

fn init_test_egraph() -> EGraphData {
    // (a * 2) / 2 => a
    let mut g = EGraphData::default();
    let calc_id_1 = gen_id("Calc");
    let calc_id_2 = gen_id("Calc");
    let var_id_1 = gen_id("Var");
    let num_id_1 = gen_id("Num");
    g.calc_expr_3_left = vec![
        (calc_id_1, "*", Set::singleton(var_id_1)),
        (calc_id_2, "/", Set::singleton(calc_id_1)),
    ];
    g.calc_expr_3_right = vec![
        (calc_id_1, "*", Set::singleton(num_id_1)),
        (calc_id_2, "/", Set::singleton(num_id_1)),
    ];
    g.root = vec![(Id("Root", 1), Set::singleton(calc_id_2))];

    g.var = vec![(var_id_1, "a")];
    g.num = vec![(num_id_1, 2)];
    g.run();
    g
}

// test entrance
fn run_egraph_test() {
    let mut test_g = init_test_egraph();
    // let w_a = Rc::new(WildCard("a"));
    // let w_b = Rc::new(WildCard("b"));
    // let test_match_pat_1 = Rc::new(Calc("*", w_a.clone(), w_b.clone()));
    // e_match(&mut test_g, &test_match_pat_1);
    // println!("Match res {:?}", test_g.e_node_match);

    test_g = e_saturate(&test_g);
    let test_match_pat_2 = Rc::new(Var("a"));
    let matched_test_2 = e_match(&test_g, &test_match_pat_2);

    // print the egraph
    println!("Vars: {:?}", test_g.var);
    println!("Nums: {:?}", test_g.num);

    let mut node_eq_sets: BTreeSet<BTreeSet<ENodeId>> = BTreeSet::new();
    for (e_id, op, eq_set) in &test_g.calc_expr_3_left {
        node_eq_sets.insert(eq_set.deref().clone());
        println!("left set {:?} {:?} >>> {:?}", e_id, op, eq_set.deref());
    }
    for (e_id, op, eq_set) in &test_g.calc_expr_3_right {
        node_eq_sets.insert(eq_set.deref().clone());
        println!("right set {:?} {:?} >>> {:?}", e_id, op, eq_set.deref());
    }
    for (e_id, eq_set) in &test_g.root {
        node_eq_sets.insert(eq_set.deref().clone());
        println!("root set {:?} >>> {:?}", e_id, eq_set.deref());
    }

    println!("Saturated!");
    println!("Match {:?} got : {:?}", test_match_pat_2, matched_test_2);
}

fn main() {
    run_egraph_test();
}
