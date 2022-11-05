use std::ops::Deref;
use std::rc::Rc;
use std::collections::BTreeSet;

use ascent::lattice::set::Set;
use ascent::*;

use crate::graph::*;
use crate::graph::ENodeId;
use crate::graph::PatternExpr::*;

ascent! {
    struct EGraphMatch;

    lattice root(ENodeId, Set<ENodeId>);
    lattice calc_expr_3_left(ENodeId, Sym, Set<ENodeId>);
    lattice calc_expr_3_right(ENodeId, Sym, Set<ENodeId>);
    relation var(ENodeId, Sym);
    relation num(ENodeId, i32);
    relation e_node(ENodeId);
    e_node(e_id) <-- (calc_expr_3_left(e_id,_,_) || var(e_id,_) || num(e_id,_) || root(e_id, _));
    // >>>>>>>>>>>> above same as EGraphData
    // match helper rule
    relation do_match(Rc<PatternExpr>, ENodeId);
    relation e_node_match(Rc<PatternExpr>, ENodeId);
    relation matched_pattern_var(Rc<PatternExpr>, ENodeId);

    relation do_match_input(Rc<PatternExpr>);           // e match input rule
    relation matched_eclass(BTreeSet<ENodeId>);         // output rule, instead of id, use a set object contain all enode

    // ? this might be very bad idea, maybe will cause graph keep rebuilding
    // lattice eq_sets(Set<E>)

    do_match(p, e) <-- do_match_input(p), e_node(e);
    matched_eclass(eq_set.deref().clone()) <--
        do_match_input(p)
        , (root(_,eq_set) || calc_expr_3_left(_,_,eq_set) || calc_expr_3_right(_,_,eq_set))
        , e_node_match(p, matched_e)
        , if eq_set.contains(matched_e)
        ;

    matched_pattern_var(p, e) <-- e_node_match(p, e), if let WildCard(v) = p.deref();

    do_match(left_p, l_eq), do_match(right_p, r_eq) <--
        do_match(pat, e_id)
        , if let Calc(op, left_p, right_p) = pat.deref()
        , calc_expr_3_left(e_id, op, l_set), calc_expr_3_right(e_id, op, r_set)
        , for l_eq in l_set.deref(), for r_eq in r_set.deref()
        ;

    e_node_match(pat, e_id) <-- do_match(pat, e_id), if let Num(n) = pat.deref(), num(e_id, n);
    e_node_match(pat, e_id) <-- do_match(pat, e_id), if let Var(x) = pat.deref(), var(e_id, x);

    e_node_match(pat, e_id) <--
        do_match(pat, e_id)
        , root(e_id, root_eq_set)
        , for root_eq_id in root_eq_set.deref()
        , e_node_match(pat, root_eq_id)
        ;

    e_node_match(pat, e_id) <--
        do_match(pat, e_id)
        , if let Calc(op, left_p, right_p) = pat.deref()
        , calc_expr_3_left(e_id, op, l_set), calc_expr_3_right(e_id, op, r_set)
        , e_node_match(left_p, l_matched), e_node_match(right_p, r_matched)
        , if l_set.contains(l_matched)
        , if r_set.contains(r_matched)
        ;

    e_node_match(pat, e_id) <-- do_match(pat, e_id), if let WildCard(mv) = pat.deref();
    e_node_match(pat, e_id) <-- 
        do_match(pat, e_id)
        , if let EClass(m_representive_node) = pat.deref()
        , (root(_, eq_set) || calc_expr_3_left(_, _, eq_set) || calc_expr_3_right(_, _, eq_set))
        , if eq_set.contains(e_id)
        // , if !eq_set.0.is_disjoint(m_set)
        , if eq_set.contains(m_representive_node)
        ;

    // need traverse the graph to pull all eq_set, this is slow
    e_node_match(pat, e_id) <--
        e_node_match(pat, matched_id)
        , (root(_, eq_set) || calc_expr_3_left(_, _, eq_set) || calc_expr_3_right(_, _, eq_set))
        , if eq_set.deref().contains(matched_id)
        , for e_id in eq_set.deref()
        ;
}

pub fn e_match(g: &EGraphData, pattern: &Rc<PatternExpr>) -> Vec<(BTreeSet<ENodeId>,)> {
    let mut matched = EGraphMatch::default();
    matched.root = g.root.clone();
    matched.calc_expr_3_left = g.calc_expr_3_left.clone();
    matched.calc_expr_3_right = g.calc_expr_3_right.clone();
    matched.num = g.num.clone();
    matched.var = g.var.clone();
    matched.do_match_input = vec![(pattern.clone(),)];
    matched.run();
    matched.matched_eclass
}
