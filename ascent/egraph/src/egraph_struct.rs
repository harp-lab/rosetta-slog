use std::fmt::Display;
use std::fmt::Debug;
use std::ops::Deref;
use std::rc::Rc;

use ascent::*;
use ascent::lattice::set::Set;

type Sym = &'static str;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
enum Op {
    Mul,
    Div,
    Plus,
    Minus,
    Lshift,
    Rshift,
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Mul => "*", Div => "/", Plus => "+", Minus => "-", Lshift => "<<", Rshift => ">>"
        };
        write!(f, "{}", s)
    }
}

#[derive(Clone, Eq, Debug, Hash)]
enum Expr {
    Var(Sym),
    Num(u32),
    Calc(Op, Rc<Expr>, Rc<Expr>),
    // WildCard,
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Var(x) => write!(f, "(Var {})", x),
            Num(n) => write!(f, "(Num {})", n),
            Calc(op, l, r) => write!(f, "({} {} {})", op, l, r),
            // WildCard => write!(f, "{}", "_")
        }
    }
}

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Var(x1) => match other {
                Var(x2) => x1 == x2,
                // WildCard => true,
                _ => false
            }
            Num(n1) => match other {
                Num(n2) => n1 == n2,
                // WildCard => true,
                _ => false
            }
            Calc(op1, l1, r1) => match other {
                Calc(op2, l2, r2) => op1 == op2 && l1 == l2 && r1 == r2,
                // WildCard => true,
                _ => false
            }
            // WildCard => true
        }
    }
    fn ne(&self, other: &Self) -> bool {
        ! (self == other)
    }
}

impl PartialOrd for Expr {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self {
            Var(x) => match other {
                Var(y) => x.partial_cmp(y),
                _ => None,
            },
            Num(x) => match other {
                Num(y) => x.partial_cmp(y),
                _ => None,
            },
            Calc(op1, l1, r1) => match other {
                Calc(op2, l2, r2) => {
                    if op1 == op2 {
                        if l1 == l2 {
                            r1.partial_cmp(r2)
                        } else {
                            l1.partial_cmp(l2)
                        }
                    } else {
                        op1.partial_cmp(op2)
                    }
                }
                _ => None,
            },
            // WildCard => match other {
            //     WildCard => Some(std::cmp::Ordering::Equal),
            //     _ => Some(std::cmp::Ordering::Equal),
            // },
        }
    }
}

impl Ord for Expr {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if let Some(res) = self.partial_cmp(other) {
            return res;
        } else {
            return match self {
                Var(_) => std::cmp::Ordering::Less,
                Num(_) => match other {
                    Var(_) => std::cmp::Ordering::Greater,
                    _ => std::cmp::Ordering::Less,
                },
                Calc(_, _, _) => std::cmp::Ordering::Greater,
                // WildCard => std::cmp::Ordering::Equal,
            };
        }
    }
}



use Expr::*;
use Op::*;

fn expr_depth(e_ptr: &Rc<Expr>) -> i32 {
    if let Calc(_, l, r) = e_ptr.deref() {
        let l_depth = expr_depth(l);
        let r_depth = expr_depth(r);
        // if l_depth > 10 || r_depth > 10 {
        //     println!(">>>>>>>>>> Too deep!!! ")
        // }
        if l_depth < r_depth {
            r_depth + 1
        } else {
            l_depth + 1
        }
    } else {
        1
    }
}

// pub fn expand_equiv<'a, N: 'a>(inp: impl Iterator<Item = (&'a N,)>) -> impl Iterator<Item = N>
// where N: Ord + Clone  {

// }

ascent! {
    struct EGraph;

    relation max_expand_depth(i32);
    relation ast(Rc<Expr>);

    relation do_input(Rc<Expr>);

    relation calc_expr_3(Rc<Expr>);
    relation calc_expr_3_filtered(Rc<Expr>);
    relation var(Rc<Expr>);
    relation num(Rc<Expr>);

    // relation e_rewrite(Rc<Expr>, Rc<Expr>);
    // relation e_equiv(Rc<Expr>, Rc<Expr>);
    lattice e_equiv_set(Rc<Expr>, Set<Rc<Expr>>);
    relation e_match(Rc<Expr>, Set<Rc<Expr>>);

    relation do_e_match(Rc<Expr>);

    max_expand_depth(5);

    calc_expr_3_filtered(e) <--
        calc_expr_3(e), max_expand_depth(m),
        // let _ = println!("filtering {:?}", e),
        if expr_depth(e) < *m;
    

    // input
    calc_expr_3(e), do_input(l.clone()), do_input(r.clone()) <--
        do_input(e), if let Calc(_, l, r) = e.deref();

    num(e) <-- do_input(e), if matches!(e.deref(), Num(_));
    var(e) <-- do_input(e), if matches!(e.deref(), Var(_));

    // a * 2 = a >> 1
    e_equiv_set(e, Set::singleton(new_e.clone())),
    calc_expr_3(new_e)
        <--
        calc_expr_3_filtered(e),
        if let Calc(Mul, l, r) = e.deref(),
        if let Num(2) = r.deref(),
        let new_e = Rc::new(Calc(Lshift, l.clone(), r.clone()));

    // a / b * c = a / (b * c)
    e_equiv_set(e, Set::singleton(rw_e.clone())),
    calc_expr_3(rw_e), calc_expr_3(rw_e_r) <--
        calc_expr_3_filtered(e),
        if let Calc(Div, ltxy, z) = e.deref(),
        if let Calc(Mul, x, y) = ltxy.deref(),
        let rw_e_r = Rc::new(Calc(Div, y.clone(), z.clone())),
        let rw_e = Rc::new(Calc(Mul, x.clone(), rw_e_r.clone()));

    // a / a = 1
    e_equiv_set(e, Set::singleton(new_n.clone())),
    num(new_n) <--
        calc_expr_3_filtered(e),
        if let Calc(Div, e1, e2) = e.deref(),
        if e1 == e2,
        let new_n = Rc::new(Num(1));

    // a * 1 = a
    // e_rewrite(e, new_e1.clone()),
    e_equiv_set(e, Set::singleton(new_e2.clone())),
    // calc_expr_3(new_e1), 
    calc_expr_3(new_e2),
    num(new_n.clone()) <--
        (num(e) || var(e)),
        let new_n = Rc::new(Num(1)),
        let new_e1 = Rc::new(Calc(Mul, new_n.clone(), e.clone())),
        let new_e2 = Rc::new(Calc(Mul, e.clone(), new_n.clone(),));

    e_equiv_set(e, Set::singleton(new_e.clone())),
    calc_expr_3(new_e) <--
        calc_expr_3_filtered(e),
        if let Calc(Mul, l, r) = e.deref(),
        let new_e = Rc::new(Calc(Mul, r.clone(), l.clone()));

    // compute equiv naively, bottom up
    // e_equiv(e1, e2) <-- e_rewrite(e1, e2);

    // e_equiv(e, e) <-- (num(e) || var(e) || calc_expr_3_filtered(e));
    // e_equiv(e2, e1) <-- e_equiv(e1, e2);
    // e_equiv(e1, e3) <-- e_equiv(e1, e2), e_equiv(e2, e3);

    // e_rewrite(upper_e, new_e.clone()),
    // calc_expr_3(new_e.clone()) <--
    //     calc_expr_3_filtered(upper_e),
    //     if let Calc(op, l, r) = upper_e.deref(),
    //     e_equiv(l, new_l),
    //     e_equiv(r, new_r),
    //     let new_e = Rc::new(Calc(*op, (*new_l).clone(), (*new_r).clone()));

    e_equiv_set(e, Set::singleton(e.clone())) <--
        (calc_expr_3_filtered(e) || num(e) || var(e));

    // e_equiv_set(e1.clone(), eq2_set),
    // e_equiv_set(e2.clone(), eq1_set) <--
    //     e_rewrite(e1, e2),
    //     e_equiv_set(e1, eq1_set),
    //     e_equiv_set(e2, eq2_set);
    
    // e_equiv_set(e2.clone(), eq_set) <--
    //     (calc_expr_3_filtered(e1) || num(e1) || var(e1)),
    //     e_equiv_set(e1, eq_set),
    //     let _ = println!("Union {:?}", e1),
    //     for e2 in eq_set.0.clone();

    e_equiv_set(e2.clone(), e1_eq_set),
    e_equiv_set(e1.clone(), e2_eq_set) <--
        e_equiv_set(e1, e1_eq_set),
        e_equiv_set(e2, e2_eq_set),
        let _ = println!("Union {:?} &&&&&&& {:?}", e1, e2),
        if !e1_eq_set.0.is_disjoint(&e2_eq_set.0);

    // recursive rewrite ?
    calc_expr_3(new_e.clone()),
    e_equiv_set(e, Set::singleton(new_e)) <--
        calc_expr_3_filtered(e),
        if let Calc(op, l, r) = e.deref(),
        e_equiv_set(l, l_eq_set),
        for l_eq in l_eq_set.0.clone(),
        e_equiv_set(r, r_eq_set),
        for r_eq in r_eq_set.0.clone(),
        let new_e = Rc::new(Calc(*op, l_eq.clone(), r_eq.clone()))
        , let _ = println!("inductively generate eq {:?} ", new_e)
        ;

    // e_rewrite(e1, e2) <-- e_equiv_set(e1, eq_set), for e2 in eq_set.0.clone();

    e_match(e1, e2_set) <-- 
        do_e_match(e1), e_equiv_set(e1, e2_set);

}

ascent! {
    struct Test;
    lattice foo(i32, Set<i32>);
    lattice bar(Dual<i32>, i32);

    foo(1, Set::singleton(1));
    foo(1, Set::singleton(2));
}

pub fn run_egraph_struct() {
    let mut eg = EGraph::default();
    let num_2: Rc<Expr> = Rc::new(Expr::Num(2));
    let var_a = Rc::new(Var("a"));
    let original_ast = Rc::new(Calc(
        Div,
        Rc::new(Calc(Mul, var_a.clone(), num_2.clone())),
        num_2.clone(),
    ));

    let mut test = Test::default();
    test.run();
    for (i, i_set) in test.foo {
        println!("Foo: {:?} >>>>>> {:?}", i, i_set.0);

    }

    // eg.ast = vec![(original_ast.clone(),)];
    // eg.do_input = vec![(original_ast.clone(),)];
    // eg.do_e_match = vec![(var_a.clone(),)];
    // eg.run();
    // println!("var: {:?}", eg.var);
    // println!("num: {:?}", eg.num);
    // for e in eg.calc_expr_3 {
    //     println!("expr: {:?}", e);
    // }
    // for (e1, e2) in eg.e_equiv_set {
    //     println!("e_equiv  ***  : {:?} \n         ***    {:?} ", e1, e2.0);
    // }
    // for (e1, e2) in eg.e_match {
    //     println!("e_match  >>>  : {:?} \n         >>>    {:?} ", e1, e2.0);
    // }
}
