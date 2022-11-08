use std::collections::{HashSet};
/**
* Rewrite and saturate a e graph
*  - Datalog code will be suspended when it needs a new fact id by throw facts in `new_expr` rule
*  - id is generated in rust code, and added back by editing `assign_new_expr_id` rule, then call `run` again
*  - id is generated outside so it's possible there is a loop in graph (can be used to represent infinite expression)
*
*  1   def equality_saturation(expr, rewrites):
   2       egraph = initial_egraph(expr)
   3
   4   while not egraph.is_saturated_or_timeout():
   5
   6
   7   # reading and writing is mixed
   8   for rw in rewrites:
   9   for (subst, eclass) in egraph.ematch(rw.lhs):
   10
   11      # in traditional equality saturation,
   12      # matches can be applied right away
   13      # because invariants are always maintained
   14      eclass2 = egraph.add(rw.rhs.subst(subst))
   15      egraph.merge(eclass, eclass2)
   16
   17      # restore the invariants after each merge
   18      egraph.rebuild()
   19
   20 return egraph.extract_best()
*/
use std::ops::Deref;
use std::rc::Rc;

use ascent::aggregators::*;
use ascent::lattice::set::Set;
use ascent::*;

use crate::graph::PatternExpr::*;
use crate::graph::*;

ascent! {
    #![measure_rule_times]
    pub struct EGraphRewrite;

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

    do_match(pat) <-- do_match_input(pat);
    do_match(lp), do_match(rp) <-- do_match(pat), if let Calc(op, lp, rp) = pat.deref();

    e_class_match(pat, eq_set) <--
        do_match(pat)
        , (if let Num(n) = pat.deref(), num(e_id, n) ||
           if let Var(x) = pat.deref(), var(e_id, x))
        , (root(_,eq_set) || calc_expr_3_left(_,_,eq_set) || calc_expr_3_right(_,_,eq_set))
        , if eq_set.contains(e_id)
        ;

    e_class_match(pat, eq_set) <--
        do_match(pat), if let EClass(representive_node) = pat.deref()
        , (root(_,eq_set) || calc_expr_3_left(_,_,eq_set) || calc_expr_3_right(_,_,eq_set))
        , if eq_set.contains(representive_node)
        ;

    e_class_match(pat, eq_set) <--
        do_match(pat), if let Calc(op, lp, rp) = pat.deref()
        , e_class_match(lp, l_matched)
        , e_class_match(rp, r_matched)
        , calc_expr_3_left(e_id, op, l_eq), if !l_eq.is_disjoint(l_matched)
        , calc_expr_3_right(e_id, op, r_eq), if r_eq.is_disjoint(r_matched)
        , (root(_,eq_set) || calc_expr_3_left(_,_,eq_set) || calc_expr_3_right(_,_,eq_set))
        , if eq_set.contains(e_id)
        ;
    // <<<<<<<<<<<<<<<<<<<<<<< above this same as EGraphMatch

    // union a enode to everything match given pattern
    relation do_union_pattern(Rc<PatternExpr>, ENodeId);
    // temporary rule
    relation do_add_new_expr(Rc<PatternExpr>);                  // output
    relation assign_new_expr_id(Rc<PatternExpr>, ENodeId);      // input
    relation new_expr_need_new_eclass(Rc<PatternExpr>);         // input
    relation rewrite_rule(Sym);
    // relation do_uinon_id(ENodeId, ENodeId);
    relation do_uinon_id(ENodeId, Set<ENodeId>);
    do_add_new_expr(pat) <-- do_union_pattern(pat, _);
    do_add_new_expr(l), do_add_new_expr(r) <-- do_union_pattern(pat, _), if let Calc(op, l, r) = pat.deref();
    do_match(p) <-- do_add_new_expr(p);
    // e_node_match(pat, id) <-- assign_new_expr_id(pat, id);
    // if sub expression in generated pattern is not in existing graph, need create new e-class for it
    e_class_match(pat, Set::singleton(*id)) <--
        assign_new_expr_id(pat, id)
        // , new_expr_need_new_eclass(pat)
        ;

    // add generated new expression into database
    do_uinon_id(e_id, matched) <--
        do_union_pattern(pat, e_id)
        // , e_node_match(pat, new_e_id)
        , e_class_match(pat, matched)
        // , if let Some(rep) = matched.first()
        ;
    num(new_e_id, n) <--
        do_add_new_expr(pat), assign_new_expr_id(pat, new_e_id)
        , if let Num(n) = pat.deref()
        ;
    var(new_e_id, n) <--
        do_add_new_expr(pat), assign_new_expr_id(pat, new_e_id)
        , if let Var(n) = pat.deref()
        ;

    calc_expr_3_left(new_e_id, op, Set::singleton(*le)) <--
        do_add_new_expr(pat), assign_new_expr_id(pat, new_e_id)
        , if let Calc(op, lp, rp) = pat.deref()
        , e_class_match(lp, l_match), for le in l_match.iter()
        ;
    calc_expr_3_right(new_e_id, op, Set::singleton(*re)) <--
        do_add_new_expr(pat), assign_new_expr_id(pat, new_e_id)
        , if let Calc(op, lp, rp) = pat.deref()
        , e_class_match(rp, r_match), for re in r_match.iter()
        ;

    // Union equivalent node
    calc_expr_3_left(e_id, op, class_to_union) <--
        calc_expr_3_left(e_id, op, eq_set)
        // , for old_id in eq_set.iter()
        , do_uinon_id(old_id, class_to_union)
        , if eq_set.contains(old_id)
        ;
    calc_expr_3_right(e_id, op, class_to_union) <--
        calc_expr_3_right(e_id, op, eq_set)
        // , for old_id in eq_set.iter()
        , do_uinon_id(old_id, class_to_union)
        , if eq_set.contains(old_id)
        ;
    root(e_id, class_to_union) <--
        root(e_id, eq_set)
        // , for old_id in eq_set.iter()
        , do_uinon_id(old_id, class_to_union)
        , if eq_set.contains(old_id)
        ;

    // populate node need to be merged, its manually subsumption
    // this should be a eq rel maybe
    // a enode ... in e-class ... need to be merged
    lattice node_need_merge(ENodeId, Set<ENodeId>);             //output
    node_need_merge(n1, Set::singleton(*n2))
        <--
        (root(_,eq_set) || calc_expr_3_left(_,_,eq_set) || calc_expr_3_right(_,_,eq_set))
        , for n1 in eq_set.iter()
        , for n2 in eq_set.iter()
        , if n1 != n2
        , calc_expr_3_left(n1, op, n1_l_eq)
        , calc_expr_3_left(n2, op, n2_l_eq)
        , if !n1_l_eq.is_disjoint(n2_l_eq)
        , calc_expr_3_right(n1, op, n1_r_eq)
        , calc_expr_3_right(n2, op, n2_r_eq)
        , if !n1_r_eq.is_disjoint(n2_r_eq)
        // , let _ = println!("{:?} {:?} -------- {:?} <><><> {:?}", n1, n2, n1_r_eq.deref(), n2_r_eq.0)
        ;

    // WARNING: intermediate output relation
    relation new_expr(Rc<PatternExpr>);
    // if new expression's sub express is not on the graph, it needs to be a new e-class
    relation new_sub_expr(Rc<PatternExpr>);
    // new_expr(pat) <-- do_union(pat, e_id), agg () = not() in e_node_match(pat, _);
    new_expr(pat) <-- do_add_new_expr(pat), agg () = not() in e_class_match(pat, _);
    new_sub_expr(pat) <-- do_add_new_expr(pat)
        , agg () = not() in e_class_match(pat, _)
        , agg () = not() in do_union_pattern(pat,_)
        ;

    // generate potential rewrite, union the generated expression into graph
    // during this time new expression's id hasn't been assigned yet
    // NOTE: set lattice column can't be unified?
    do_union_pattern(new_pat_expr, e_id) <-- rewrite_rule("elim-div-1")
        , calc_expr_3_left(e_id, "/", l_set)
        , calc_expr_3_right(e_id, "/", r_set)
        , if !l_set.is_disjoint(r_set)
        , let new_pat_expr = Rc::new(Num(1))
        ;

    do_union_pattern(new_pat_expr, e_id) <-- rewrite_rule("expand-mul-1")
        , e_node(e_id)
        , (root(_,eq_set) || calc_expr_3_left(_,_,eq_set) || calc_expr_3_right(_,_,eq_set))
        , if eq_set.contains(e_id)
        // , let new_pat_expr = Rc::new(Calc("*", Rc::new(EClass(eq_set.deref().clone())), Rc::new(Num(1))))
        , if let Some(eq_representive_node) = eq_set.first()
        , let new_pat_expr = Rc::new(Calc("*", Rc::new(EClass(*eq_representive_node)), Rc::new(Num(1))))
        ;

    do_union_pattern(new_pat_expr, div_e_id) <-- rewrite_rule("mul-comm-1")
        , calc_expr_3_left(div_e_id, "/", l_div_set)
        , calc_expr_3_right(div_e_id, "/", r_div_set)
        , for div_l_id in l_div_set.iter()
        , calc_expr_3_left(div_l_id, "*", l_mult_set)
        , calc_expr_3_right(div_l_id, "*", r_mult_set)
        // , let new_pat_expr = Rc::new(Calc("*", Rc::new(EClass(l_mult_set.deref().clone()))
        //                                      , Rc::new(Calc("/" , Rc::new(EClass(r_mult_set.deref().clone()))
        //                                                         , Rc::new(EClass(r_div_set.deref().clone()))))))
        , if let Some(l_mult_set_rep) = l_mult_set.first()
        , if let Some(r_mult_set_rep) = r_mult_set.first()
        , if let Some(r_div_set_rep) = r_div_set.first()
        , let new_pat_expr = Rc::new(Calc("*", Rc::new(EClass(*l_mult_set_rep))
                                             , Rc::new(Calc("/" , Rc::new(EClass(*r_mult_set_rep))
                                                                , Rc::new(EClass(*r_div_set_rep))))))
        ;
}

// fn merge_node

fn merge_node(g: &mut EGraphRewrite) -> EGraphRewrite {
    let mut merged: HashSet<ENodeId> = HashSet::new();
    for (selected, n_eqs) in g.node_need_merge.iter() {
        if merged.contains(selected) {
            continue;
        }
        println!("Node {:?} needs merge with {:?}", n_eqs.deref(), selected);

        for n_merged in n_eqs.iter() {
            merged.insert(*n_merged);
        }
        let filtered_new_calc_expr_3_left = g
            .calc_expr_3_left
            .clone()
            .into_iter()
            .filter(|(a, _, _)| !n_eqs.contains(a))
            .collect();
        g.calc_expr_3_left = filtered_new_calc_expr_3_left;
        let filtered_calc_expr_3_right = g
            .calc_expr_3_right
            .clone()
            .into_iter()
            .filter(|(a, _, _)| !n_eqs.contains(a))
            .collect();
        // clear all duplicated path
        g.calc_expr_3_right = filtered_calc_expr_3_right;
    }
    let mut new_rw = EGraphRewrite::default();
    new_rw.rewrite_rule = vec![("elim-div-1",), ("expand-mul-1",), ("mul-comm-1",)];
    new_rw.num = g.num.clone();
    new_rw.var = g.var.clone();
    new_rw.root = g.root.clone();
    new_rw.calc_expr_3_left = g.calc_expr_3_left.clone();
    new_rw.calc_expr_3_right = g.calc_expr_3_right.clone();
    new_rw
}

pub fn e_saturate(g: &EGraphData, max_iteration: usize) -> EGraphData {
    let mut rw_g = EGraphRewrite::default();
    rw_g.rewrite_rule = vec![("elim-div-1",), ("expand-mul-1",), ("mul-comm-1",)];
    rw_g.root = g.root.clone();
    rw_g.calc_expr_3_left = g.calc_expr_3_left.clone();
    rw_g.calc_expr_3_right = g.calc_expr_3_right.clone();
    rw_g.var = g.var.clone();
    rw_g.num = g.num.clone();
    rw_g.e_node = g.e_node.clone();
    let mut cnt = 0;
    // loop until no new e-node generated
    // NOTE: if clear a relation, all its indices need to be cleared too
    loop {
        println!("Iteration {:?}", cnt);
        rw_g.run();
        // println!(
        //     "{} \n {}",
        //     rw_g.scc_times_summary(),
        //     rw_g.relation_sizes_summary()
        // );
        let new_expr = rw_g.new_expr.clone();
        let new_eclass_expr = rw_g.new_sub_expr.clone();
        if new_expr.is_empty() {
            // println!("Do union: {:?}", rw_g.do_union);
            break;
        }
        // merger will create a new rewrite graph
        rw_g = merge_node(&mut rw_g);

        for (ne,) in new_expr.iter() {
            let new_id = match ne.deref() {
                Var(_) => gen_id("Var"),
                Num(_) => gen_id("Num"),
                Calc(_, _, _) => gen_id("Calc"),
                _ => gen_id("None"),
            };
            println!("Assign Id {:?} to new expression >> {:?}", new_id, ne);
            rw_g.assign_new_expr_id.push((ne.clone(), new_id));
        }
        for (pat,) in new_eclass_expr.iter() {
            rw_g.new_expr_need_new_eclass.push((pat.clone(),));
        }
        if cnt >= max_iteration {
            break;
        }
        cnt = cnt + 1;
        // break;
    }
    // rw_g = merge_node(&mut rw_g);
    let mut saturated_graph = EGraphData::default();
    saturated_graph.root = rw_g.root;
    saturated_graph.calc_expr_3_left = rw_g.calc_expr_3_left;
    saturated_graph.calc_expr_3_right = rw_g.calc_expr_3_right;
    saturated_graph.var = rw_g.var;
    saturated_graph.num = rw_g.num;
    saturated_graph.e_node = rw_g.e_node;
    saturated_graph
}
