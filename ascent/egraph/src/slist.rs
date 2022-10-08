
use std::ops::Deref;
use std::rc::Rc;

use ascent::*;


#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum SList<T: Copy> {
    Cons(T, Rc<SList<T>>), Nil
}

fn list_hd<T: Copy>(lst_ptr: &Rc<SList<T>>) -> Option<T> {
    match lst_ptr.deref() {
        SList::Cons(hd, _) => Some(*hd),
        _ => None
    }
}

fn list_tail<T: Copy>(lst_ptr: &Rc<SList<T>>) -> Option<Rc<SList<T>>> {
    match lst_ptr.deref() {
        SList::Cons(_, rst) => Some(rst.clone()),
        _ => None
    }
}

ascent! {
    struct SlogList;
    // input
    relation construct_list(Option<Rc<SList<i32>>>);
    relation list(Rc<SList<i32>>);

    list(lst), construct_list(list_tail(lst)) <--
        construct_list(?Some(lst)),
        if matches!(lst.deref(), SList::Cons(_,_));

    list(lst) <--
        construct_list(?Some(lst)),
        if matches!(lst.deref(), SList::Nil);
}

pub fn test_slog_list () {
    let mut sl = SlogList::default();
    let a = SList::Cons(3, Rc::new(SList::Nil));
    let b = SList::Cons(3, Rc::new(SList::Nil));
    sl.construct_list = vec![
        (Some(Rc::new(SList::Cons(1, Rc::new(SList::Cons(2, Rc::new(SList::Cons(3, Rc::new(SList::Nil)))))))),),
        (Some(Rc::new(SList::Cons(3, Rc::new(SList::Nil)))),)
    ];

    assert_eq!(a, b);
    println!("a == n? {:?}", a == b);
    sl.run();
    println!("list {:?}\n", sl.list);
}
