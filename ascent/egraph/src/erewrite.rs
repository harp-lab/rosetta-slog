/**
 * Rewrite and saturate a e graph
 *  - Datalog code will be suspended when it needs a new fact id by throw facts in `new_expr` rule
 *  - id is generated in rust code, and added back by editing `assign_new_expr_id` rule, then call `run` again
 *  - id is generated outside so it's possible there is a loop in graph (can be used to represent infinite expression)
 */
use std::ops::Deref;
use std::rc::Rc;

use ascent::aggregators::*;
use ascent::lattice::set::Set;
use ascent::*;

use crate::graph::PatternExpr::*;
use crate::graph::*;

ascent! {
    pub struct EGraphRewrite;

    lattice root(ENodeId, Set<ENodeId>);
    // e node
    lattice calc_expr_3_left(ENodeId, Sym, Set<ENodeId>);
    lattice calc_expr_3_right(ENodeId, Sym, Set<ENodeId>);
    relation var(ENodeId, Sym);
    relation num(ENodeId, i32);
    relation e_node(ENodeId);
    e_node(e_id) <-- (calc_expr_3_left(e_id,_,_) || var(e_id,_) || num(e_id,_)|| root(e_id, _));

    relation do_match(Rc<PatternExpr>, ENodeId);
    relation e_node_match(Rc<PatternExpr>, ENodeId);

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
    e_node_match(pat, e_id) <-- do_match(pat, e_id), if let ENode(m_id) = pat.deref(), if e_id == m_id;

        // need travese the graph to pull all eq_set
    e_node_match(pat, e_id) <--
        e_node_match(pat, matched_id)
        , (root(_, eq_set) || calc_expr_3_left(_, _, eq_set) || calc_expr_3_right(_, _, eq_set))
        , if eq_set.contains(matched_id)
        , for e_id in eq_set.deref()
        ;
    // <<<<<<<<<<<<<<<<<<<<<<< above this same as EGraphMatch

    // union a enode to everything match given pattern
    relation do_union_pattern(Rc<PatternExpr>, ENodeId);
    // temporary rule
    relation do_add_new_expr(Rc<PatternExpr>);          // output
    relation assign_new_expr_id(Rc<PatternExpr>, ENodeId);     // input
    relation do_uinon_id(ENodeId, ENodeId);
    do_add_new_expr(pat) <-- do_union_pattern(pat, _);
    do_add_new_expr(l), do_add_new_expr(r) <-- do_add_new_expr(pat), if let Calc(op, l, r) = pat.deref();
    do_match(p, e) <-- do_add_new_expr(p), e_node(e);
    e_node_match(pat, id) <-- assign_new_expr_id(pat, id);

    // add generated new expression into database
    do_uinon_id(e_id, new_e_id) <-- do_union_pattern(pat, e_id), e_node_match(pat, new_e_id);
    num(new_e_id, n) <--
        do_add_new_expr(pat), assign_new_expr_id(pat, new_e_id)
        , if let Num(n) = pat.deref()
        ;
    var(new_e_id, n) <--
        do_add_new_expr(pat), assign_new_expr_id(pat, new_e_id)
        , if let Var(n) = pat.deref()
        ;
    calc_expr_3_left(new_e_id, op, Set::singleton(*l_id))
    , calc_expr_3_right(new_e_id, op, Set::singleton(*r_id)) <--
        do_add_new_expr(pat), assign_new_expr_id(pat, new_e_id)
        , if let Calc(op, lp, rp) = pat.deref()
        , e_node_match(lp, l_id), e_node_match(rp, r_id)
        ;

    // Union equivalent node
    calc_expr_3_left(e_id, op, Set::singleton(*node_to_union)) <--
        (do_uinon_id(old_id, node_to_union) || do_uinon_id(node_to_union, old_id))
        , calc_expr_3_left(e_id, op, eq_set)
        , if eq_set.contains(old_id)
        ;
    calc_expr_3_right(e_id, op, Set::singleton(*node_to_union)) <--
        (do_uinon_id(old_id, node_to_union) || do_uinon_id(node_to_union, old_id))
        , calc_expr_3_right(e_id, op, eq_set)
        , if eq_set.contains(old_id)
        ;
    root(e_id, Set::singleton(*node_to_union)) <--
        (do_uinon_id(old_id, node_to_union) || do_uinon_id(node_to_union, old_id))
        , root(e_id, eq_set)
        , if eq_set.contains(old_id)
        ;

    // rebuild the graph, rebuild is expensive, only call by need
    relation do_rebuild(bool);
    calc_expr_3_left(e_id, op, eq2_set) <--
        do_rebuild(true)
        , calc_expr_3_left(e_id, op, eq1_set)
        , (root(_,eq2_set) || calc_expr_3_left(_,_,eq2_set) || calc_expr_3_right(_,_,eq2_set))
        , if !eq1_set.is_disjoint(eq2_set)
        ;

    // WARNING: intermediate output relation
    relation new_expr(Rc<PatternExpr>);
    // new_expr(pat) <-- do_union(pat, e_id), agg () = not() in e_node_match(pat, _);
    new_expr(pat) <-- do_add_new_expr(pat), agg () = not() in e_node_match(pat, _);

    // generate potential rewrite, union the generated expression into graph
    // during this time new expression's id hasn't been assigned yet
    // NOTE: set lattice column can't be unified?
    do_union_pattern(new_pat_expr, e_id) <--
        calc_expr_3_left(e_id, "/", l_set)
        , calc_expr_3_right(e_id, "/", r_set)
        , if l_set.deref() == r_set.deref()
        , let new_pat_expr = Rc::new(Num(1))
        ;

    // TODO: need rewrite this rule use do_union
    do_uinon_id(a, e_id) <--
        calc_expr_3_left(e_id, "*", l_set)
        , calc_expr_3_right(e_id, "*", r_set)
        , num(n1_id, 1)
        , if r_set.contains(n1_id)
        , for a in l_set.deref()
        ;

    // do_union(new_pat_expr, e_id) <--
    //     e_node(e_id)
    //     , let new_pat_expr = Rc::new(Calc("*", Rc::new(ENode(*e_id)), Rc::new(Num(1))))
    //     ;

    do_union_pattern(new_pat_expr, div_e_id) <--
        calc_expr_3_left(div_e_id, "/", l_div_set)
        , calc_expr_3_right(div_e_id, "/", r_div_set)
        , for div_l_id in l_div_set.deref()
        , calc_expr_3_left(div_l_id, "*", l_mult_set)
        , calc_expr_3_right(div_l_id, "*", r_mult_set)
        , for a_id in l_mult_set.deref()
        , for b_id in r_mult_set.deref()
        , for c_id in r_div_set.deref()
        , let new_pat_expr = Rc::new(Calc("*", Rc::new(ENode(*a_id))
                                             , Rc::new(Calc("/" , Rc::new(ENode(*b_id))
                                                                , Rc::new(ENode(*c_id))))))
        ;
}

pub fn e_saturate(g: &EGraphData) -> EGraphData {
    let mut rw_g = EGraphRewrite::default();
    rw_g.root = g.root.clone();
    rw_g.calc_expr_3_left = g.calc_expr_3_left.clone();
    rw_g.calc_expr_3_right = g.calc_expr_3_right.clone();
    rw_g.var = g.var.clone();
    rw_g.num = g.num.clone();
    rw_g.e_node = g.e_node.clone();

    // loop until no new e-node generated
    // NOTE: if clear a relation, all its indices need to be cleared too
    loop {
        // clean temporary out before run
        rw_g.new_expr.clear();
        rw_g.new_expr_indices_0.clear();
        rw_g.run();
        // clean temporary in after run
        rw_g.assign_new_expr_id.clear();
        rw_g.assign_new_expr_id_indices_.clear();
        rw_g.assign_new_expr_id_indices_0.clear();
        rw_g.assign_new_expr_id_indices_0_1.clear();
        if rw_g.new_expr.is_empty() {
            // println!("Do union: {:?}", rw_g.do_union);
            break;
        }
        // println!("New expr: {:?}", rw_g.new_expr);
        // println!("Do union: {:?}", rw_g.do_union);
        for ne in &rw_g.new_expr {
            let new_id = match ne.0.deref() {
                Var(_) => gen_id("Var"),
                Num(_) => gen_id("Num"),
                Calc(_, _, _) => gen_id("Calc"),
                _ => gen_id("None"),
            };
            println!("Assign Id {:?} to new expression >> {:?}", new_id, ne.0);
            rw_g.assign_new_expr_id.push((ne.0.clone(), new_id));
        }

    }
    // println!("uinon_ids {:?}", rw_g.uinon_id);

    let mut saturated_graph = EGraphData::default();
    saturated_graph.root = rw_g.root;
    saturated_graph.calc_expr_3_left = rw_g.calc_expr_3_left;
    saturated_graph.calc_expr_3_right = rw_g.calc_expr_3_right;
    saturated_graph.var = rw_g.var;
    saturated_graph.num = rw_g.num;
    saturated_graph.e_node = rw_g.e_node;
    saturated_graph
}
