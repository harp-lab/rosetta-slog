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
    relation do_match(Rc<PatternExpr>);
    // match the pattern with representive element in existing a e-class
    // relation e_class_representive_match(Rc<PatternExpr>, ENodeId);
    relation e_class_match(Rc<PatternExpr>, Set<ENodeId>);
    // relation matched_pattern_var(Rc<PatternExpr>, ENodeId);

    relation do_match_input(Rc<PatternExpr>);           // e match input rule
    relation matched_eclass(BTreeSet<ENodeId>);         // output rule, instead of id, use a set object contain all enode

    do_match(pat) <-- do_match_input(pat);
    do_match(lp), do_match(rp) <-- do_match(pat), if let Calc(op, lp, rp) = pat.deref();

    // e_class_representive_match(pat, *representive_node) <--
    //     do_match(pat)
    //     , (if let Num(n) = pat.deref(), num(e_id, n) ||
    //        if let Var(x) = pat.deref(), var(e_id, x))
    //     , (root(_,eq_set) || calc_expr_3_left(_,_,eq_set) || calc_expr_3_right(_,_,eq_set))
    //     , if eq_set.contains(e_id), if let Some(representive_node) = eq_set.first()
    //     ;
    e_class_match(pat, eq_set.deref()) <--
        do_match(pat)
        , (if let Num(n) = pat.deref(), num(e_id, n) ||
           if let Var(x) = pat.deref(), var(e_id, x))
        , (root(_,eq_set) || calc_expr_3_left(_,_,eq_set) || calc_expr_3_right(_,_,eq_set))
        , if eq_set.contains(e_id)
        ;
    
    // e_class_representive_match(pat, representive_node) <--
    //     do_match(pat), if let EClass(representive_node) = pat.deref()
    //     ;
    e_class_match(pat, ec) <-- do_match(pat), if let EClass(ec) = pat.deref();
    
    // e_class_representive_match(pat, representive_node) <--
    //     do_match(pat), if let Calc(op, lp, rp) = pat.deref()
    //     , e_class_representive_match(lp, lp_class_r)
    //     , e_class_representive_match(rp, rp_class_r)
    //     , calc_expr_3_left(e_id, op, l_eq), if l_eq.contains(lp_class_r)
    //     , calc_expr_3_right(e_id, op, r_eq), if r_eq.contains(rp_class_r)
    //     , (root(_,eq_set) || calc_expr_3_left(_,_,eq_set) || calc_expr_3_right(_,_,eq_set))
    //     , if eq_set.contains(e_id), if let Some(representive_node) = eq_set.first()
    //     ;
    e_class_match(pat, representive_node) <--
        do_match(pat), if let Calc(op, lp, rp) = pat.deref()
        , e_class_match(lp, lp_class)
        , e_class_match(rp, rp_class)
        , calc_expr_3_left(e_id, op, l_eq), if !l_eq.is_disjoint(lp_class)
        , calc_expr_3_right(e_id, op, r_eq), if !r_eq.is_disjoint(rp_class)
        , (root(_,eq_set) || calc_expr_3_left(_,_,eq_set) || calc_expr_3_right(_,_,eq_set))
        , if eq_set.contains(e_id), if let Some(representive_node) = eq_set.first()
        ;

    // matched_eclass(eq_set.deref().clone()) <-- 
    //     do_match_input(pat)
    //     , e_class_representive_match(pat, ec_rep)
    //     , (root(_,eq_set) || calc_expr_3_left(_,_,eq_set) || calc_expr_3_right(_,_,eq_set))
    //     , if eq_set.contains(ec_rep)
    //     ;
    matched_eclass(ec.deref()) <-- do_match_input(pat), e_class_match(pat, ec);
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
