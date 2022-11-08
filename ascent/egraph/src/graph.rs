use std::collections::{HashMap, BTreeSet};
/**
 * EGraph representation in Ascent
 * - Use linked list style fact to represent a graph
 * - e-class here is implemented by a series graph edge relation (which is a Set lattice in Ascent)
 */
use std::fmt::Debug;
use std::ops::Deref;
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
    // ENode(ENodeId),
    // eclass only store the represenive id in inside an e-calss
    EClass(BTreeSet<ENodeId>)
}

use crate::graph::ENodeId::*;

// global counter used for id generation
static GLOBAL_CALC_COUNT: AtomicUsize = AtomicUsize::new(0);
static GLOBAL_NUM_COUNT: AtomicUsize = AtomicUsize::new(0);
static GLOBAL_VAR_COUNT: AtomicUsize = AtomicUsize::new(0);
// static GLOBAL_ROOT_COUNT: AtomicUsize = AtomicUsize::new(0);
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

    relation e_classes(Set<ENodeId>);
    e_classes(eqs) <-- (root(_, eqs) || calc_expr_3_left(_, _, eqs) || calc_expr_3_right(_,_, eqs));

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

pub fn graph_to_dot(g: &EGraphData) -> String {
    // g.run();
    let mut dot_str = String::new();

    dot_str.push_str("digraph egraph {\n");
    dot_str.push_str("  compound=true\n");
    dot_str.push_str("  clusterrank=local\n");

    // config?
    // populate all eclasses
    let mut class_map: HashMap<Vec<ENodeId>, String> = HashMap::new();
    let mut eclass_cnt = 0;
    for (n,) in g.e_classes.iter() {
        let n_list: Vec<ENodeId> = n.deref().clone().into_iter().collect();
        if class_map.contains_key(&n_list) {
            continue;
        }
        class_map.insert(n_list, format!("{}", eclass_cnt));
        eclass_cnt = eclass_cnt + 1;
    }

    fn in_class(m: &HashMap<Vec<ENodeId>, String>, e: ENodeId) -> String {
        let mut found = "none";
        for (nodes, i) in m.iter() {
            if nodes.contains(&e) {
                found = i.as_str();
            }
        }
        found.to_string()
    }

    fn in_class_index(m: &HashMap<Vec<ENodeId>, String>, e: ENodeId) -> Option<usize> {
        for (nodes, _) in m.iter() {
            if nodes.contains(&e) {
                return nodes.iter().position(|&x| x == e)
            }
        }
        None
    }

    let mut node_sym_map: HashMap<ENodeId, String> = HashMap::new();

    for (e_id, v) in g.var.iter() {
        node_sym_map.insert(*e_id, v.to_string());
    }
    for (e_id, n) in g.num.iter() {
        node_sym_map.insert(*e_id, n.to_string());
    }
    for (e_id, op, _) in g.calc_expr_3_left.iter() {
        node_sym_map.insert(*e_id, op.to_string());
    }

    for (c, c_id) in class_map.iter() {
        dot_str.push_str(format!("  subgraph cluster_{} {{\n", c_id).as_str());
        dot_str.push_str("    style=dotted\n");
        for (i, e_id) in c.iter().enumerate() {
            if let Some(sym) = node_sym_map.get(e_id) {
                // let e_id_str = match e_id {
                //     Id(t, i) => format!("{}_{}", t, i),
                // };
                dot_str.push_str(format!("    {}.{}[label = \"{}\"]\n", c_id, i, sym).as_str());
            }
        }
        dot_str.push_str("  }\n");
    }
    for (e_id, _, to_eq_set) in g.calc_expr_3_left.iter() {
        let to_eq_list: Vec<ENodeId> = to_eq_set.deref().clone().into_iter().collect();
        if let Some(to_c_id) = class_map.get(&to_eq_list) {
            let from_c_id = in_class(&class_map, *e_id);
            if let Some(sub_id) = in_class_index(&class_map, *e_id) {
                dot_str.push_str(
                    format!(
                        "  {}.{}{} -> {}.0 [lhead = cluster_{}, ]\n",
                        from_c_id, sub_id, ":sw", to_c_id, to_c_id
                    )
                    .as_str(),
                );
            }
        }
    }
    for (e_id, _, to_eq_set) in g.calc_expr_3_right.iter() {
        let to_eq_list: Vec<ENodeId> = to_eq_set.deref().clone().into_iter().collect();
        if let Some(to_c_id) = class_map.get(&to_eq_list) {
            let from_c_id = in_class(&class_map, *e_id);
            if let Some(sub_id) = in_class_index(&class_map, *e_id) {
                dot_str.push_str(
                    format!(
                        "  {}.{}{} -> {}.0 [lhead = cluster_{}, ]\n",
                        from_c_id, sub_id, ":se", to_c_id, to_c_id
                    )
                    .as_str(),
                );
            }
        }
    }

    dot_str.push_str("}");
    dot_str
}
