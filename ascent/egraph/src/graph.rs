/**
 * EGraph representation in Ascent
 * - Use linked list style fact to represent a graph
 * - e-class here is implemented by a series graph edge relation (which is a Set lattice in Ascent)
 */
use std::fmt::Debug;
use std::rc::Rc;

use std::sync::atomic::{AtomicUsize, Ordering};

use ascent::lattice::set::Set;
use ascent::*;
pub type Sym = &'static str;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
pub enum ENodeId {
    Id(Sym, usize),
}

// Expression pattern used as e-match input and rewrite
#[derive(Clone, PartialEq, Eq, PartialOrd, Debug, Hash)]
pub enum PatternExpr {
    Var(Sym),
    Num(i32),
    Calc(Sym, Rc<PatternExpr>, Rc<PatternExpr>),
    WildCard(Sym),
    ENode(ENodeId),
}

use crate::graph::ENodeId::*;

// global counter used for id generation
pub static GLOBAL_CALC_COUNT: AtomicUsize = AtomicUsize::new(0);
pub static GLOBAL_NUM_COUNT: AtomicUsize = AtomicUsize::new(0);
pub static GLOBAL_VAR_COUNT: AtomicUsize = AtomicUsize::new(0);
pub static GLOBAL_ROOT_COUNT: AtomicUsize = AtomicUsize::new(0);
pub fn gen_id(t: &str) -> ENodeId {
    if t == "Var" {
        Id("Var", GLOBAL_VAR_COUNT.fetch_add(1, Ordering::SeqCst))
    } else if t == "Num" {
        Id("Num", GLOBAL_NUM_COUNT.fetch_add(1, Ordering::SeqCst))
    } else if t == "Calc" {
        Id("Calc", GLOBAL_CALC_COUNT.fetch_add(1, Ordering::SeqCst))
    } else {
        Id("None", 0)
    }
}

ascent! {
    pub struct EGraphData;
    // graph edge to e-class
    lattice root(ENodeId, Set<ENodeId>);
    lattice calc_expr_3_left(ENodeId, Sym, Set<ENodeId>);
    lattice calc_expr_3_right(ENodeId, Sym, Set<ENodeId>);
    relation var(ENodeId, Sym);
    relation num(ENodeId, i32);

    // in original e-graph enode don't have id, only e-class has
    relation e_node(ENodeId);
    e_node(e_id) <-- (calc_expr_3_left(e_id,_,_) || var(e_id,_) || num(e_id,_) || root(e_id, _));

    relation do_rebuild(bool);
    calc_expr_3_left(e_id, op, eq2_set) <--
        do_rebuild(true)
        , calc_expr_3_left(e_id, op, eq1_set)
        , (root(_,eq2_set) || calc_expr_3_left(_,_,eq2_set) || calc_expr_3_right(_,_,eq2_set))
        , if !eq1_set.is_disjoint(eq2_set)
        ;
}

pub fn rebuild(g: &EGraphData) -> EGraphData {
    let mut rebuilt_g = EGraphData::default();
    rebuilt_g.root = g.root.clone();
    rebuilt_g.calc_expr_3_left = g.calc_expr_3_left.clone();
    rebuilt_g.calc_expr_3_right = g.calc_expr_3_right.clone();
    rebuilt_g.var = g.var.clone();
    rebuilt_g.num = g.num.clone();
    rebuilt_g.do_rebuild = vec![(true,)];
    rebuilt_g.run();
    rebuilt_g
}
