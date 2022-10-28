/**
 * Another version of e-graph
 *  - Use linked list style fact to represent a graph
 *  - Datalog code will be suspended when it needs a new fact id by throw facts in `new_expr` rule
 *  - id is generated in rust code, and added back by editing `e_node_match` rule, then call `run` again
 *  - id is generated outside so it's possible there is a loop in graph (can be used to represent infinite expression)
 *  - e-class here is implemented by a series graph edge relation (which is a Set lattice in Ascent)
 *  - EMatch and Rewrite is separated into 2 different programs, mainly because write rule generate a lot of intermediate data,
 *    however these EDB/IDB won't be used in normal matching task at all. During rewriting, seems matching only e-node is more
 *    useful than match e-class (this is like inlining in souffle, I want a kind of manually "GC").
 *  - e-match here is not using relational e-matching technique
 * TODO:
 *  - Add cost function, need dig more into egg paper
 */

use std::collections::BTreeSet;
use std::fmt::Debug;
use std::ops::Deref;
use std::rc::Rc;

use std::sync::atomic::{AtomicUsize, Ordering};

use ascent::aggregators::*;
use ascent::lattice::set::Set;
use ascent::*;

use crate::egraph_id::ENodeId::*;
use crate::egraph_id::PatternExpr::*;

type Sym = &'static str;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
enum ENodeId {
    Id(Sym, usize),
}

// Expression pattern used as e-match input and rewrite
#[derive(Clone, PartialEq, Eq, PartialOrd, Debug, Hash)]
enum PatternExpr {
    Var(Sym),
    Num(i32),
    Calc(Sym, Rc<PatternExpr>, Rc<PatternExpr>),
    WildCard(Sym),
    ENode(ENodeId),
}

fn is_wildcard(p: &Rc<PatternExpr>) -> bool {
    if let WildCard(_) = p.deref() {
        true
    } else {
        false
    }
}

// global counter used for id generation
static GLOBAL_CALC_COUNT: AtomicUsize = AtomicUsize::new(0);
static GLOBAL_NUM_COUNT: AtomicUsize = AtomicUsize::new(0);
static GLOBAL_VAR_COUNT: AtomicUsize = AtomicUsize::new(0);
static GLOBAL_ROOT_COUNT: AtomicUsize = AtomicUsize::new(0);
fn gen_id(t: &str) -> ENodeId {
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
    struct EGraphMatch;

    // graph edge to e-class
    lattice root(ENodeId, Set<ENodeId>);
    lattice calc_expr_3_left(ENodeId, Sym, Set<ENodeId>);
    lattice calc_expr_3_right(ENodeId, Sym, Set<ENodeId>);
    relation var(ENodeId, Sym);
    relation num(ENodeId, i32);

    // in original e-graph enode don't have id, only e-class has
    relation e_node(ENodeId);
    e_node(e_id) <-- (calc_expr_3_left(e_id,_,_) || var(e_id,_) || num(e_id,_) || root(e_id, _));

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
        , if l_set.deref().contains(l_matched)
        , if r_set.deref().contains(r_matched)
        ;

    e_node_match(pat, e_id) <-- do_match(pat, e_id), if let WildCard(mv) = pat.deref();
    e_node_match(pat, e_id) <-- do_match(pat, e_id), if let ENode(m_id) = pat.deref(), if e_id == m_id;

    // need traverse the graph to pull all eq_set, this is slow
    e_node_match(pat, e_id) <-- 
        e_node_match(pat, matched_id)
        , (root(_, eq_set) || calc_expr_3_left(_, _, eq_set) || calc_expr_3_right(_, _, eq_set))
        , if eq_set.deref().contains(matched_id)
        , for e_id in eq_set.deref()
        ;

}

ascent! {
    struct EGraphRewrite;

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
        , if l_set.deref().contains(l_matched)
        , if r_set.deref().contains(r_matched)
        ;

    e_node_match(pat, e_id) <-- do_match(pat, e_id), if let WildCard(mv) = pat.deref();
    e_node_match(pat, e_id) <-- do_match(pat, e_id), if let ENode(m_id) = pat.deref(), if e_id == m_id;

        // need travese the graph to pull all eq_set
    e_node_match(pat, e_id) <-- 
        e_node_match(pat, matched_id)
        , (root(_, eq_set) || calc_expr_3_left(_, _, eq_set) || calc_expr_3_right(_, _, eq_set))
        , if eq_set.deref().contains(matched_id)
        , for e_id in eq_set.deref()
        ;
    // <<<<<<<<<<<<<<<<<<<<<<< above this same as EGraphMatch

    // union a enode to everything match given pattern
    relation do_union(Rc<PatternExpr>, ENodeId);
    // temporary rule
    relation do_add_new_expr(Rc<PatternExpr>);
    relation equiv_id(ENodeId, ENodeId);
    do_add_new_expr(pat) <-- do_union(pat, _);
    do_add_new_expr(l), do_add_new_expr(r) <-- do_add_new_expr(pat), if let Calc(op, l, r) = pat.deref();
    do_match(p, e) <-- do_add_new_expr(p), e_node(e);

    // add generated new expression into database
    equiv_id(e_id, new_e_id) <-- do_union(pat, e_id), e_node_match(pat, new_e_id);
    num(new_e_id, n) <--
        do_add_new_expr(pat), e_node_match(pat, new_e_id)
        , if let Num(n) = pat.deref()
        ;
    var(new_e_id, n) <--
        do_add_new_expr(pat), e_node_match(pat, new_e_id)
        , if let Var(n) = pat.deref()
        ;
    calc_expr_3_left(new_e_id, op, Set::singleton(*l_id))
    , calc_expr_3_right(new_e_id, op, Set::singleton(*r_id)) <--
        do_add_new_expr(pat), e_node_match(pat, new_e_id)
        , if let Calc(op, lp, rp) = pat.deref()
        , e_node_match(lp, l_id), e_node_match(rp, r_id)
        ;

    // Union equivalent node
    calc_expr_3_left(e_id, op, Set::singleton(*node_to_union)) <--
        (equiv_id(old_id, node_to_union) || equiv_id(node_to_union, old_id))
        , calc_expr_3_left(e_id, op, eq_set)
        , if eq_set.deref().contains(old_id)
        ;
    calc_expr_3_right(e_id, op, Set::singleton(*node_to_union)) <--
        (equiv_id(old_id, node_to_union) || equiv_id(node_to_union, old_id))
        , calc_expr_3_right(e_id, op, eq_set)
        , if eq_set.deref().contains(old_id)
        ;
    root(e_id, Set::singleton(*node_to_union)) <--
        (equiv_id(old_id, node_to_union) || equiv_id(node_to_union, old_id))
        , root(e_id, eq_set)
        , if eq_set.deref().contains(old_id)
        ;

    // WARNING: intermediate output relation
    relation new_expr(Rc<PatternExpr>);
    // new_expr(pat) <-- do_union(pat, e_id), agg () = not() in e_node_match(pat, _);
    new_expr(pat) <-- do_add_new_expr(pat), agg () = not() in e_node_match(pat, _);

    // generate potential rewrite, union the generated expression into graph
    // during this time new expression's id hasn't been assigned yet
    // NOTE: set lattice column can't be unified?
    do_union(new_pat_expr, e_id) <--
        calc_expr_3_left(e_id, "/", l_set)
        , calc_expr_3_right(e_id, "/", r_set)
        , if l_set.deref() == r_set.deref()
        , let new_pat_expr = Rc::new(Num(1))
        ;

    // TODO: need rewrite this rule use do_union
    equiv_id(a, e_id) <--
        calc_expr_3_left(e_id, "*", l_set)
        , calc_expr_3_right(e_id, "*", r_set)
        , num(n1_id, 1)
        , if r_set.deref().contains(n1_id)
        , for a in l_set.deref()
        ;

    // do_union(new_pat_expr, e_id) <--
    //     e_node(e_id)
    //     , let new_pat_expr = Rc::new(Calc("*", Rc::new(ENode(*e_id)), Rc::new(Num(1))))
    //     ;

    do_union(new_pat_expr, div_e_id) <--
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

fn e_match(g: &mut EGraphMatch, pattern: &Rc<PatternExpr>) {
    g.do_match_input = vec![(pattern.clone(),)];
    g.run();
    g.do_match_input.clear();
}

fn e_saturate(g: &mut EGraphMatch) {
    let mut rw_g = EGraphRewrite::default();
    rw_g.root = g.root.clone();
    rw_g.calc_expr_3_left = g.calc_expr_3_left.clone();
    rw_g.calc_expr_3_right = g.calc_expr_3_right.clone();
    rw_g.var = g.var.clone();
    rw_g.num = g.num.clone();
    rw_g.e_node = g.e_node.clone();

    // loop until no new e-node generated
    loop {
        rw_g.run();
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
            rw_g.e_node_match.push((ne.0.clone(), new_id));
        }
        rw_g.new_expr.clear();
    }
    // println!("equiv_ids {:?}", rw_g.equiv_id);
    // swap back
    g.root = rw_g.root;
    g.calc_expr_3_left = rw_g.calc_expr_3_left;
    g.calc_expr_3_right = rw_g.calc_expr_3_right;
    g.var = rw_g.var;
    g.num = rw_g.num;
    g.e_node = rw_g.e_node;
}

fn init_test_egraph() -> EGraphMatch {
    // (a * 2) / 2 => a
    let mut g = EGraphMatch::default();
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
pub fn run_egraph_id() {
    let mut test_g = init_test_egraph();
    // let w_a = Rc::new(WildCard("a"));
    // let w_b = Rc::new(WildCard("b"));
    // let test_match_pat_1 = Rc::new(Calc("*", w_a.clone(), w_b.clone()));
    // e_match(&mut test_g, &test_match_pat_1);
    // println!("Match res {:?}", test_g.e_node_match);

    e_saturate(&mut test_g);
    let test_match_pat_2 = Rc::new(Var("a"));
    e_match(&mut test_g, &test_match_pat_2);

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
    println!("Match {:?} got : {:?}", test_match_pat_2, test_g.matched_eclass);

}
