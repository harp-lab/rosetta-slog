#![feature(prelude_import)]
#![feature(map_first_last)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::collections::BTreeSet;
use std::ops::Deref;
use std::rc::Rc;
mod ematch {
    use std::ops::Deref;
    use std::rc::Rc;
    use std::collections::BTreeSet;
    use ascent::lattice::set::Set;
    use ascent::*;
    use crate::graph::*;
    use crate::graph::ENodeId;
    use crate::graph::PatternExpr::*;
    struct EGraphMatch {
        pub do_match_input: Vec<(Rc<PatternExpr>,)>,
        #[allow(non_snake_case)]
        pub do_match_input_indices_: ascent::internal::RelIndexType<()>,
        #[allow(non_snake_case)]
        pub do_match_input_indices_0: ascent::internal::RelFullIndexType<
            (Rc<PatternExpr>,),
        >,
        pub matched_eclass: Vec<(BTreeSet<ENodeId>,)>,
        #[allow(non_snake_case)]
        pub matched_eclass_indices_0: ascent::internal::RelFullIndexType<
            (BTreeSet<ENodeId>,),
        >,
        pub calc_expr_3_left: Vec<(ENodeId, Sym, Set<ENodeId>)>,
        #[allow(non_snake_case)]
        pub calc_expr_3_left_indices_0_1: ascent::internal::LatticeIndexType<
            (ENodeId, Sym),
        >,
        #[allow(non_snake_case)]
        pub calc_expr_3_left_indices_0_1_2: ascent::internal::RelFullIndexType<
            (ENodeId, Sym, Set<ENodeId>),
        >,
        #[allow(non_snake_case)]
        pub calc_expr_3_left_indices_: ascent::internal::LatticeIndexType<()>,
        pub e_node: Vec<(ENodeId,)>,
        #[allow(non_snake_case)]
        pub e_node_indices_0: ascent::internal::RelFullIndexType<(ENodeId,)>,
        #[allow(non_snake_case)]
        pub e_node_indices_: ascent::internal::RelIndexType<()>,
        pub matched_pattern_var: Vec<(Rc<PatternExpr>, ENodeId)>,
        #[allow(non_snake_case)]
        pub matched_pattern_var_indices_0_1: ascent::internal::RelFullIndexType<
            (Rc<PatternExpr>, ENodeId),
        >,
        pub var: Vec<(ENodeId, Sym)>,
        #[allow(non_snake_case)]
        pub var_indices_: ascent::internal::RelIndexType<()>,
        #[allow(non_snake_case)]
        pub var_indices_0_1: ascent::internal::RelFullIndexType<(ENodeId, Sym)>,
        pub num: Vec<(ENodeId, i32)>,
        #[allow(non_snake_case)]
        pub num_indices_0_1: ascent::internal::RelFullIndexType<(ENodeId, i32)>,
        #[allow(non_snake_case)]
        pub num_indices_: ascent::internal::RelIndexType<()>,
        pub calc_expr_3_right: Vec<(ENodeId, Sym, Set<ENodeId>)>,
        #[allow(non_snake_case)]
        pub calc_expr_3_right_indices_: ascent::internal::LatticeIndexType<()>,
        #[allow(non_snake_case)]
        pub calc_expr_3_right_indices_0_1: ascent::internal::LatticeIndexType<
            (ENodeId, Sym),
        >,
        #[allow(non_snake_case)]
        pub calc_expr_3_right_indices_0_1_2: ascent::internal::RelFullIndexType<
            (ENodeId, Sym, Set<ENodeId>),
        >,
        pub root: Vec<(ENodeId, Set<ENodeId>)>,
        #[allow(non_snake_case)]
        pub root_indices_: ascent::internal::LatticeIndexType<()>,
        #[allow(non_snake_case)]
        pub root_indices_0: ascent::internal::LatticeIndexType<(ENodeId,)>,
        #[allow(non_snake_case)]
        pub root_indices_0_1: ascent::internal::RelFullIndexType<
            (ENodeId, Set<ENodeId>),
        >,
        pub e_node_match: Vec<(Rc<PatternExpr>, ENodeId)>,
        #[allow(non_snake_case)]
        pub e_node_match_indices_: ascent::internal::RelIndexType<()>,
        #[allow(non_snake_case)]
        pub e_node_match_indices_0_1: ascent::internal::RelFullIndexType<
            (Rc<PatternExpr>, ENodeId),
        >,
        #[allow(non_snake_case)]
        pub e_node_match_indices_0: ascent::internal::RelIndexType<(Rc<PatternExpr>,)>,
        pub do_match: Vec<(Rc<PatternExpr>, ENodeId)>,
        #[allow(non_snake_case)]
        pub do_match_indices_0_1: ascent::internal::RelFullIndexType<
            (Rc<PatternExpr>, ENodeId),
        >,
        #[allow(non_snake_case)]
        pub do_match_indices_: ascent::internal::RelIndexType<()>,
        pub scc0_duration: std::time::Duration,
        pub scc1_duration: std::time::Duration,
        pub scc2_duration: std::time::Duration,
        pub scc3_duration: std::time::Duration,
        pub scc4_duration: std::time::Duration,
        pub scc5_duration: std::time::Duration,
        pub scc6_duration: std::time::Duration,
        pub scc7_duration: std::time::Duration,
        pub scc8_duration: std::time::Duration,
        pub scc9_duration: std::time::Duration,
        pub scc10_duration: std::time::Duration,
        pub scc11_duration: std::time::Duration,
        pub scc12_duration: std::time::Duration,
        pub scc13_duration: std::time::Duration,
        pub scc14_duration: std::time::Duration,
        pub scc15_duration: std::time::Duration,
        pub scc16_duration: std::time::Duration,
    }
    impl EGraphMatch {
        #[allow(unused_imports)]
        ///Runs the Ascent program to a fixed point.
        pub fn run(&mut self) {
            use core::cmp::PartialEq;
            use ascent::internal::RelIndexRead;
            use ascent::internal::RelIndexReadAll;
            self.update_indices_priv();
            let _self = self;
            ascent::internal::comment("scc 0");
            {
                let _scc_start_time = ::ascent::internal::Instant::now();
                #[allow(non_snake_case)]
                let e_node_indices_0_delta: &mut ascent::internal::RelFullIndexType<
                    (ENodeId,),
                > = &mut _self.e_node_indices_0;
                #[allow(non_snake_case)]
                let mut e_node_indices_0_total: ascent::internal::RelFullIndexType<
                    (ENodeId,),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut e_node_indices_0_new: ascent::internal::RelFullIndexType<
                    (ENodeId,),
                > = Default::default();
                #[allow(non_snake_case)]
                let e_node_indices__delta: &mut ascent::internal::RelIndexType<()> = &mut _self
                    .e_node_indices_;
                #[allow(non_snake_case)]
                let mut e_node_indices__total: ascent::internal::RelIndexType<()> = Default::default();
                #[allow(non_snake_case)]
                let mut e_node_indices__new: ascent::internal::RelIndexType<()> = Default::default();
                #[allow(non_snake_case)]
                let num_indices__total: &mut ascent::internal::RelIndexType<()> = &mut _self
                    .num_indices_;
                #[allow(unused_assignments, unused_variables)]
                {
                    let mut __changed = false;
                    ascent::internal::comment("e_node <-- num_indices__total");
                    if let Some(__matching) = num_indices__total.index_get(&()) {
                        for __ind in __matching {
                            let __row = &_self.num[__ind].clone();
                            let e_id = &__row.0;
                            let __1 = &__row.1;
                            let __new_row: (ENodeId,) = (
                                ascent::internal::Convert::convert(e_id),
                            );
                            let __new_row_ind = _self.e_node.len();
                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                &e_node_indices_0_total,
                                &__new_row,
                            )
                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                    e_node_indices_0_delta,
                                    &__new_row,
                                )
                                && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                    &mut e_node_indices_0_new,
                                    &__new_row,
                                    __new_row_ind,
                                )
                            {
                                ::ascent::internal::RelIndexWrite::index_insert(
                                    &mut e_node_indices__new,
                                    (),
                                    __new_row_ind,
                                );
                                _self.e_node.push(__new_row);
                                __changed = true;
                            }
                        }
                    }
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_indices_0_delta,
                        &mut e_node_indices_0_total,
                    );
                    std::mem::swap(&mut e_node_indices_0_new, e_node_indices_0_delta);
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_indices__delta,
                        &mut e_node_indices__total,
                    );
                    std::mem::swap(&mut e_node_indices__new, e_node_indices__delta);
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_indices_0_delta,
                        &mut e_node_indices_0_total,
                    );
                    std::mem::swap(&mut e_node_indices_0_new, e_node_indices_0_delta);
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_indices__delta,
                        &mut e_node_indices__total,
                    );
                    std::mem::swap(&mut e_node_indices__new, e_node_indices__delta);
                }
                _self.e_node_indices_0 = e_node_indices_0_total;
                _self.e_node_indices_ = e_node_indices__total;
                _self.scc0_duration += _scc_start_time.elapsed();
            }
            ascent::internal::comment("scc 1");
            {
                let _scc_start_time = ::ascent::internal::Instant::now();
                #[allow(non_snake_case)]
                let e_node_indices__delta: &mut ascent::internal::RelIndexType<()> = &mut _self
                    .e_node_indices_;
                #[allow(non_snake_case)]
                let mut e_node_indices__total: ascent::internal::RelIndexType<()> = Default::default();
                #[allow(non_snake_case)]
                let mut e_node_indices__new: ascent::internal::RelIndexType<()> = Default::default();
                #[allow(non_snake_case)]
                let e_node_indices_0_delta: &mut ascent::internal::RelFullIndexType<
                    (ENodeId,),
                > = &mut _self.e_node_indices_0;
                #[allow(non_snake_case)]
                let mut e_node_indices_0_total: ascent::internal::RelFullIndexType<
                    (ENodeId,),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut e_node_indices_0_new: ascent::internal::RelFullIndexType<
                    (ENodeId,),
                > = Default::default();
                #[allow(non_snake_case)]
                let calc_expr_3_left_indices__total: &mut ascent::internal::LatticeIndexType<
                    (),
                > = &mut _self.calc_expr_3_left_indices_;
                #[allow(unused_assignments, unused_variables)]
                {
                    let mut __changed = false;
                    ascent::internal::comment(
                        "e_node <-- calc_expr_3_left_indices__total",
                    );
                    if let Some(__matching)
                        = calc_expr_3_left_indices__total.index_get(&())
                    {
                        for __ind in __matching {
                            let __row = &_self.calc_expr_3_left[__ind].clone();
                            let e_id = &__row.0;
                            let __1 = &__row.1;
                            let __2 = &__row.2;
                            let __new_row: (ENodeId,) = (
                                ascent::internal::Convert::convert(e_id),
                            );
                            let __new_row_ind = _self.e_node.len();
                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                &e_node_indices_0_total,
                                &__new_row,
                            )
                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                    e_node_indices_0_delta,
                                    &__new_row,
                                )
                                && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                    &mut e_node_indices_0_new,
                                    &__new_row,
                                    __new_row_ind,
                                )
                            {
                                ::ascent::internal::RelIndexWrite::index_insert(
                                    &mut e_node_indices__new,
                                    (),
                                    __new_row_ind,
                                );
                                _self.e_node.push(__new_row);
                                __changed = true;
                            }
                        }
                    }
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_indices__delta,
                        &mut e_node_indices__total,
                    );
                    std::mem::swap(&mut e_node_indices__new, e_node_indices__delta);
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_indices_0_delta,
                        &mut e_node_indices_0_total,
                    );
                    std::mem::swap(&mut e_node_indices_0_new, e_node_indices_0_delta);
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_indices__delta,
                        &mut e_node_indices__total,
                    );
                    std::mem::swap(&mut e_node_indices__new, e_node_indices__delta);
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_indices_0_delta,
                        &mut e_node_indices_0_total,
                    );
                    std::mem::swap(&mut e_node_indices_0_new, e_node_indices_0_delta);
                }
                _self.e_node_indices_ = e_node_indices__total;
                _self.e_node_indices_0 = e_node_indices_0_total;
                _self.scc1_duration += _scc_start_time.elapsed();
            }
            ascent::internal::comment("scc 2");
            {
                let _scc_start_time = ::ascent::internal::Instant::now();
                #[allow(non_snake_case)]
                let e_node_indices_0_delta: &mut ascent::internal::RelFullIndexType<
                    (ENodeId,),
                > = &mut _self.e_node_indices_0;
                #[allow(non_snake_case)]
                let mut e_node_indices_0_total: ascent::internal::RelFullIndexType<
                    (ENodeId,),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut e_node_indices_0_new: ascent::internal::RelFullIndexType<
                    (ENodeId,),
                > = Default::default();
                #[allow(non_snake_case)]
                let e_node_indices__delta: &mut ascent::internal::RelIndexType<()> = &mut _self
                    .e_node_indices_;
                #[allow(non_snake_case)]
                let mut e_node_indices__total: ascent::internal::RelIndexType<()> = Default::default();
                #[allow(non_snake_case)]
                let mut e_node_indices__new: ascent::internal::RelIndexType<()> = Default::default();
                #[allow(non_snake_case)]
                let var_indices__total: &mut ascent::internal::RelIndexType<()> = &mut _self
                    .var_indices_;
                #[allow(unused_assignments, unused_variables)]
                {
                    let mut __changed = false;
                    ascent::internal::comment("e_node <-- var_indices__total");
                    if let Some(__matching) = var_indices__total.index_get(&()) {
                        for __ind in __matching {
                            let __row = &_self.var[__ind].clone();
                            let e_id = &__row.0;
                            let __1 = &__row.1;
                            let __new_row: (ENodeId,) = (
                                ascent::internal::Convert::convert(e_id),
                            );
                            let __new_row_ind = _self.e_node.len();
                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                &e_node_indices_0_total,
                                &__new_row,
                            )
                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                    e_node_indices_0_delta,
                                    &__new_row,
                                )
                                && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                    &mut e_node_indices_0_new,
                                    &__new_row,
                                    __new_row_ind,
                                )
                            {
                                ::ascent::internal::RelIndexWrite::index_insert(
                                    &mut e_node_indices__new,
                                    (),
                                    __new_row_ind,
                                );
                                _self.e_node.push(__new_row);
                                __changed = true;
                            }
                        }
                    }
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_indices_0_delta,
                        &mut e_node_indices_0_total,
                    );
                    std::mem::swap(&mut e_node_indices_0_new, e_node_indices_0_delta);
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_indices__delta,
                        &mut e_node_indices__total,
                    );
                    std::mem::swap(&mut e_node_indices__new, e_node_indices__delta);
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_indices_0_delta,
                        &mut e_node_indices_0_total,
                    );
                    std::mem::swap(&mut e_node_indices_0_new, e_node_indices_0_delta);
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_indices__delta,
                        &mut e_node_indices__total,
                    );
                    std::mem::swap(&mut e_node_indices__new, e_node_indices__delta);
                }
                _self.e_node_indices_0 = e_node_indices_0_total;
                _self.e_node_indices_ = e_node_indices__total;
                _self.scc2_duration += _scc_start_time.elapsed();
            }
            ascent::internal::comment("scc 3");
            {
                let _scc_start_time = ::ascent::internal::Instant::now();
                #[allow(non_snake_case)]
                let e_node_indices_0_delta: &mut ascent::internal::RelFullIndexType<
                    (ENodeId,),
                > = &mut _self.e_node_indices_0;
                #[allow(non_snake_case)]
                let mut e_node_indices_0_total: ascent::internal::RelFullIndexType<
                    (ENodeId,),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut e_node_indices_0_new: ascent::internal::RelFullIndexType<
                    (ENodeId,),
                > = Default::default();
                #[allow(non_snake_case)]
                let e_node_indices__delta: &mut ascent::internal::RelIndexType<()> = &mut _self
                    .e_node_indices_;
                #[allow(non_snake_case)]
                let mut e_node_indices__total: ascent::internal::RelIndexType<()> = Default::default();
                #[allow(non_snake_case)]
                let mut e_node_indices__new: ascent::internal::RelIndexType<()> = Default::default();
                #[allow(non_snake_case)]
                let root_indices__total: &mut ascent::internal::LatticeIndexType<()> = &mut _self
                    .root_indices_;
                #[allow(unused_assignments, unused_variables)]
                {
                    let mut __changed = false;
                    ascent::internal::comment("e_node <-- root_indices__total");
                    if let Some(__matching) = root_indices__total.index_get(&()) {
                        for __ind in __matching {
                            let __row = &_self.root[__ind].clone();
                            let e_id = &__row.0;
                            let __1 = &__row.1;
                            let __new_row: (ENodeId,) = (
                                ascent::internal::Convert::convert(e_id),
                            );
                            let __new_row_ind = _self.e_node.len();
                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                &e_node_indices_0_total,
                                &__new_row,
                            )
                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                    e_node_indices_0_delta,
                                    &__new_row,
                                )
                                && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                    &mut e_node_indices_0_new,
                                    &__new_row,
                                    __new_row_ind,
                                )
                            {
                                ::ascent::internal::RelIndexWrite::index_insert(
                                    &mut e_node_indices__new,
                                    (),
                                    __new_row_ind,
                                );
                                _self.e_node.push(__new_row);
                                __changed = true;
                            }
                        }
                    }
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_indices_0_delta,
                        &mut e_node_indices_0_total,
                    );
                    std::mem::swap(&mut e_node_indices_0_new, e_node_indices_0_delta);
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_indices__delta,
                        &mut e_node_indices__total,
                    );
                    std::mem::swap(&mut e_node_indices__new, e_node_indices__delta);
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_indices_0_delta,
                        &mut e_node_indices_0_total,
                    );
                    std::mem::swap(&mut e_node_indices_0_new, e_node_indices_0_delta);
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_indices__delta,
                        &mut e_node_indices__total,
                    );
                    std::mem::swap(&mut e_node_indices__new, e_node_indices__delta);
                }
                _self.e_node_indices_0 = e_node_indices_0_total;
                _self.e_node_indices_ = e_node_indices__total;
                _self.scc3_duration += _scc_start_time.elapsed();
            }
            ascent::internal::comment("scc 4");
            {
                let _scc_start_time = ::ascent::internal::Instant::now();
                #[allow(non_snake_case)]
                let do_match_indices__delta: &mut ascent::internal::RelIndexType<()> = &mut _self
                    .do_match_indices_;
                #[allow(non_snake_case)]
                let mut do_match_indices__total: ascent::internal::RelIndexType<()> = Default::default();
                #[allow(non_snake_case)]
                let mut do_match_indices__new: ascent::internal::RelIndexType<()> = Default::default();
                #[allow(non_snake_case)]
                let do_match_indices_0_1_delta: &mut ascent::internal::RelFullIndexType<
                    (Rc<PatternExpr>, ENodeId),
                > = &mut _self.do_match_indices_0_1;
                #[allow(non_snake_case)]
                let mut do_match_indices_0_1_total: ascent::internal::RelFullIndexType<
                    (Rc<PatternExpr>, ENodeId),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut do_match_indices_0_1_new: ascent::internal::RelFullIndexType<
                    (Rc<PatternExpr>, ENodeId),
                > = Default::default();
                #[allow(non_snake_case)]
                let do_match_input_indices__total: &mut ascent::internal::RelIndexType<
                    (),
                > = &mut _self.do_match_input_indices_;
                #[allow(non_snake_case)]
                let e_node_indices__total: &mut ascent::internal::RelIndexType<()> = &mut _self
                    .e_node_indices_;
                #[allow(unused_assignments, unused_variables)]
                {
                    let mut __changed = false;
                    ascent::internal::comment(
                        "do_match <-- do_match_input_indices__total, e_node_indices__total [SIMPLE JOIN]",
                    );
                    if do_match_input_indices__total.len() <= e_node_indices__total.len()
                    {
                        for (__cl1_joined_columns, __cl1_tuple_indices) in do_match_input_indices__total
                            .iter_all()
                        {
                            if let Some(__matching)
                                = e_node_indices__total.index_get(&())
                            {
                                for cl1_ind in __cl1_tuple_indices {
                                    let __row = &_self.do_match_input[cl1_ind].clone();
                                    let p = &__row.0;
                                    for __ind in __matching.clone() {
                                        let __row = &_self.e_node[__ind].clone();
                                        let e = &__row.0;
                                        let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                            ascent::internal::Convert::convert(p),
                                            ascent::internal::Convert::convert(e),
                                        );
                                        let __new_row_ind = _self.do_match.len();
                                        if !::ascent::internal::RelFullIndexRead::contains_key(
                                            &do_match_indices_0_1_total,
                                            &__new_row,
                                        )
                                            && !::ascent::internal::RelFullIndexRead::contains_key(
                                                do_match_indices_0_1_delta,
                                                &__new_row,
                                            )
                                            && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                &mut do_match_indices_0_1_new,
                                                &__new_row,
                                                __new_row_ind,
                                            )
                                        {
                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                &mut do_match_indices__new,
                                                (),
                                                __new_row_ind,
                                            );
                                            _self.do_match.push(__new_row);
                                            __changed = true;
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        for (__cl1_joined_columns, __cl1_tuple_indices) in e_node_indices__total
                            .iter_all()
                        {
                            if let Some(__matching)
                                = do_match_input_indices__total.index_get(&())
                            {
                                for cl1_ind in __cl1_tuple_indices {
                                    let __row = &_self.e_node[cl1_ind].clone();
                                    let e = &__row.0;
                                    for __ind in __matching.clone() {
                                        let __row = &_self.do_match_input[__ind].clone();
                                        let p = &__row.0;
                                        let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                            ascent::internal::Convert::convert(p),
                                            ascent::internal::Convert::convert(e),
                                        );
                                        let __new_row_ind = _self.do_match.len();
                                        if !::ascent::internal::RelFullIndexRead::contains_key(
                                            &do_match_indices_0_1_total,
                                            &__new_row,
                                        )
                                            && !::ascent::internal::RelFullIndexRead::contains_key(
                                                do_match_indices_0_1_delta,
                                                &__new_row,
                                            )
                                            && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                &mut do_match_indices_0_1_new,
                                                &__new_row,
                                                __new_row_ind,
                                            )
                                        {
                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                &mut do_match_indices__new,
                                                (),
                                                __new_row_ind,
                                            );
                                            _self.do_match.push(__new_row);
                                            __changed = true;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        do_match_indices__delta,
                        &mut do_match_indices__total,
                    );
                    std::mem::swap(&mut do_match_indices__new, do_match_indices__delta);
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        do_match_indices_0_1_delta,
                        &mut do_match_indices_0_1_total,
                    );
                    std::mem::swap(
                        &mut do_match_indices_0_1_new,
                        do_match_indices_0_1_delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        do_match_indices__delta,
                        &mut do_match_indices__total,
                    );
                    std::mem::swap(&mut do_match_indices__new, do_match_indices__delta);
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        do_match_indices_0_1_delta,
                        &mut do_match_indices_0_1_total,
                    );
                    std::mem::swap(
                        &mut do_match_indices_0_1_new,
                        do_match_indices_0_1_delta,
                    );
                }
                _self.do_match_indices_ = do_match_indices__total;
                _self.do_match_indices_0_1 = do_match_indices_0_1_total;
                _self.scc4_duration += _scc_start_time.elapsed();
            }
            ascent::internal::comment("scc 5");
            {
                let _scc_start_time = ::ascent::internal::Instant::now();
                #[allow(non_snake_case)]
                let do_match_indices__delta: &mut ascent::internal::RelIndexType<()> = &mut _self
                    .do_match_indices_;
                #[allow(non_snake_case)]
                let mut do_match_indices__total: ascent::internal::RelIndexType<()> = Default::default();
                #[allow(non_snake_case)]
                let mut do_match_indices__new: ascent::internal::RelIndexType<()> = Default::default();
                #[allow(non_snake_case)]
                let do_match_indices_0_1_delta: &mut ascent::internal::RelFullIndexType<
                    (Rc<PatternExpr>, ENodeId),
                > = &mut _self.do_match_indices_0_1;
                #[allow(non_snake_case)]
                let mut do_match_indices_0_1_total: ascent::internal::RelFullIndexType<
                    (Rc<PatternExpr>, ENodeId),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut do_match_indices_0_1_new: ascent::internal::RelFullIndexType<
                    (Rc<PatternExpr>, ENodeId),
                > = Default::default();
                #[allow(non_snake_case)]
                let calc_expr_3_right_indices_0_1_total: &mut ascent::internal::LatticeIndexType<
                    (ENodeId, Sym),
                > = &mut _self.calc_expr_3_right_indices_0_1;
                #[allow(non_snake_case)]
                let calc_expr_3_left_indices_0_1_total: &mut ascent::internal::LatticeIndexType<
                    (ENodeId, Sym),
                > = &mut _self.calc_expr_3_left_indices_0_1;
                #[allow(unused_assignments, unused_variables)]
                loop {
                    let mut __changed = false;
                    ascent::internal::comment(
                        "do_match, do_match <-- do_match_indices__delta, if let ⋯, calc_expr_3_left_indices_0_1_total, calc_expr_3_right_indices_0_1_total, for_l_eq, for_r_eq",
                    );
                    if let Some(__matching) = do_match_indices__delta.index_get(&()) {
                        for __ind in __matching {
                            let __row = &_self.do_match[__ind].clone();
                            let pat = &__row.0;
                            let e_id = &__row.1;
                            if let Calc(op, left_p, right_p) = pat.deref() {
                                if let Some(__matching)
                                    = calc_expr_3_left_indices_0_1_total
                                        .index_get(&(e_id.clone(), op.clone()))
                                {
                                    for __ind in __matching {
                                        let __row = &_self.calc_expr_3_left[__ind].clone();
                                        let op = &__row.1;
                                        let l_set = &__row.2;
                                        if let Some(__matching)
                                            = calc_expr_3_right_indices_0_1_total
                                                .index_get(&(e_id.clone(), op.clone()))
                                        {
                                            for __ind in __matching {
                                                let __row = &_self.calc_expr_3_right[__ind].clone();
                                                let r_set = &__row.2;
                                                for l_eq in l_set.deref() {
                                                    for r_eq in r_set.deref() {
                                                        let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                                            ascent::internal::Convert::convert(left_p),
                                                            ascent::internal::Convert::convert(l_eq),
                                                        );
                                                        let __new_row_ind = _self.do_match.len();
                                                        if !::ascent::internal::RelFullIndexRead::contains_key(
                                                            &do_match_indices_0_1_total,
                                                            &__new_row,
                                                        )
                                                            && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                do_match_indices_0_1_delta,
                                                                &__new_row,
                                                            )
                                                            && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                &mut do_match_indices_0_1_new,
                                                                &__new_row,
                                                                __new_row_ind,
                                                            )
                                                        {
                                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                                &mut do_match_indices__new,
                                                                (),
                                                                __new_row_ind,
                                                            );
                                                            _self.do_match.push(__new_row);
                                                            __changed = true;
                                                        }
                                                        let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                                            ascent::internal::Convert::convert(right_p),
                                                            ascent::internal::Convert::convert(r_eq),
                                                        );
                                                        let __new_row_ind = _self.do_match.len();
                                                        if !::ascent::internal::RelFullIndexRead::contains_key(
                                                            &do_match_indices_0_1_total,
                                                            &__new_row,
                                                        )
                                                            && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                do_match_indices_0_1_delta,
                                                                &__new_row,
                                                            )
                                                            && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                &mut do_match_indices_0_1_new,
                                                                &__new_row,
                                                                __new_row_ind,
                                                            )
                                                        {
                                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                                &mut do_match_indices__new,
                                                                (),
                                                                __new_row_ind,
                                                            );
                                                            _self.do_match.push(__new_row);
                                                            __changed = true;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        do_match_indices__delta,
                        &mut do_match_indices__total,
                    );
                    std::mem::swap(&mut do_match_indices__new, do_match_indices__delta);
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        do_match_indices_0_1_delta,
                        &mut do_match_indices_0_1_total,
                    );
                    std::mem::swap(
                        &mut do_match_indices_0_1_new,
                        do_match_indices_0_1_delta,
                    );
                    if !__changed {
                        break;
                    }
                }
                _self.do_match_indices_ = do_match_indices__total;
                _self.do_match_indices_0_1 = do_match_indices_0_1_total;
                _self.scc5_duration += _scc_start_time.elapsed();
            }
            ascent::internal::comment("scc 6");
            {
                let _scc_start_time = ::ascent::internal::Instant::now();
                #[allow(non_snake_case)]
                let e_node_match_indices_0_1_delta: &mut ascent::internal::RelFullIndexType<
                    (Rc<PatternExpr>, ENodeId),
                > = &mut _self.e_node_match_indices_0_1;
                #[allow(non_snake_case)]
                let mut e_node_match_indices_0_1_total: ascent::internal::RelFullIndexType<
                    (Rc<PatternExpr>, ENodeId),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut e_node_match_indices_0_1_new: ascent::internal::RelFullIndexType<
                    (Rc<PatternExpr>, ENodeId),
                > = Default::default();
                #[allow(non_snake_case)]
                let e_node_match_indices_0_delta: &mut ascent::internal::RelIndexType<
                    (Rc<PatternExpr>,),
                > = &mut _self.e_node_match_indices_0;
                #[allow(non_snake_case)]
                let mut e_node_match_indices_0_total: ascent::internal::RelIndexType<
                    (Rc<PatternExpr>,),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut e_node_match_indices_0_new: ascent::internal::RelIndexType<
                    (Rc<PatternExpr>,),
                > = Default::default();
                #[allow(non_snake_case)]
                let e_node_match_indices__delta: &mut ascent::internal::RelIndexType<
                    (),
                > = &mut _self.e_node_match_indices_;
                #[allow(non_snake_case)]
                let mut e_node_match_indices__total: ascent::internal::RelIndexType<
                    (),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut e_node_match_indices__new: ascent::internal::RelIndexType<()> = Default::default();
                #[allow(non_snake_case)]
                let do_match_indices__total: &mut ascent::internal::RelIndexType<()> = &mut _self
                    .do_match_indices_;
                #[allow(unused_assignments, unused_variables)]
                {
                    let mut __changed = false;
                    ascent::internal::comment(
                        "e_node_match <-- do_match_indices__total, if let ⋯",
                    );
                    if let Some(__matching) = do_match_indices__total.index_get(&()) {
                        for __ind in __matching {
                            let __row = &_self.do_match[__ind].clone();
                            let pat = &__row.0;
                            let e_id = &__row.1;
                            if let WildCard(mv) = pat.deref() {
                                let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                    ascent::internal::Convert::convert(pat),
                                    ascent::internal::Convert::convert(e_id),
                                );
                                let __new_row_ind = _self.e_node_match.len();
                                if !::ascent::internal::RelFullIndexRead::contains_key(
                                    &e_node_match_indices_0_1_total,
                                    &__new_row,
                                )
                                    && !::ascent::internal::RelFullIndexRead::contains_key(
                                        e_node_match_indices_0_1_delta,
                                        &__new_row,
                                    )
                                    && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                        &mut e_node_match_indices_0_1_new,
                                        &__new_row,
                                        __new_row_ind,
                                    )
                                {
                                    ::ascent::internal::RelIndexWrite::index_insert(
                                        &mut e_node_match_indices_0_new,
                                        (__new_row.0.clone(),),
                                        __new_row_ind,
                                    );
                                    ::ascent::internal::RelIndexWrite::index_insert(
                                        &mut e_node_match_indices__new,
                                        (),
                                        __new_row_ind,
                                    );
                                    _self.e_node_match.push(__new_row);
                                    __changed = true;
                                }
                            }
                        }
                    }
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_match_indices_0_1_delta,
                        &mut e_node_match_indices_0_1_total,
                    );
                    std::mem::swap(
                        &mut e_node_match_indices_0_1_new,
                        e_node_match_indices_0_1_delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_match_indices_0_delta,
                        &mut e_node_match_indices_0_total,
                    );
                    std::mem::swap(
                        &mut e_node_match_indices_0_new,
                        e_node_match_indices_0_delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_match_indices__delta,
                        &mut e_node_match_indices__total,
                    );
                    std::mem::swap(
                        &mut e_node_match_indices__new,
                        e_node_match_indices__delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_match_indices_0_1_delta,
                        &mut e_node_match_indices_0_1_total,
                    );
                    std::mem::swap(
                        &mut e_node_match_indices_0_1_new,
                        e_node_match_indices_0_1_delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_match_indices_0_delta,
                        &mut e_node_match_indices_0_total,
                    );
                    std::mem::swap(
                        &mut e_node_match_indices_0_new,
                        e_node_match_indices_0_delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_match_indices__delta,
                        &mut e_node_match_indices__total,
                    );
                    std::mem::swap(
                        &mut e_node_match_indices__new,
                        e_node_match_indices__delta,
                    );
                }
                _self.e_node_match_indices_0_1 = e_node_match_indices_0_1_total;
                _self.e_node_match_indices_0 = e_node_match_indices_0_total;
                _self.e_node_match_indices_ = e_node_match_indices__total;
                _self.scc6_duration += _scc_start_time.elapsed();
            }
            ascent::internal::comment("scc 7");
            {
                let _scc_start_time = ::ascent::internal::Instant::now();
                #[allow(non_snake_case)]
                let e_node_match_indices_0_delta: &mut ascent::internal::RelIndexType<
                    (Rc<PatternExpr>,),
                > = &mut _self.e_node_match_indices_0;
                #[allow(non_snake_case)]
                let mut e_node_match_indices_0_total: ascent::internal::RelIndexType<
                    (Rc<PatternExpr>,),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut e_node_match_indices_0_new: ascent::internal::RelIndexType<
                    (Rc<PatternExpr>,),
                > = Default::default();
                #[allow(non_snake_case)]
                let e_node_match_indices_0_1_delta: &mut ascent::internal::RelFullIndexType<
                    (Rc<PatternExpr>, ENodeId),
                > = &mut _self.e_node_match_indices_0_1;
                #[allow(non_snake_case)]
                let mut e_node_match_indices_0_1_total: ascent::internal::RelFullIndexType<
                    (Rc<PatternExpr>, ENodeId),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut e_node_match_indices_0_1_new: ascent::internal::RelFullIndexType<
                    (Rc<PatternExpr>, ENodeId),
                > = Default::default();
                #[allow(non_snake_case)]
                let e_node_match_indices__delta: &mut ascent::internal::RelIndexType<
                    (),
                > = &mut _self.e_node_match_indices_;
                #[allow(non_snake_case)]
                let mut e_node_match_indices__total: ascent::internal::RelIndexType<
                    (),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut e_node_match_indices__new: ascent::internal::RelIndexType<()> = Default::default();
                #[allow(non_snake_case)]
                let do_match_indices__total: &mut ascent::internal::RelIndexType<()> = &mut _self
                    .do_match_indices_;
                #[allow(non_snake_case)]
                let calc_expr_3_left_indices__total: &mut ascent::internal::LatticeIndexType<
                    (),
                > = &mut _self.calc_expr_3_left_indices_;
                #[allow(unused_assignments, unused_variables)]
                {
                    let mut __changed = false;
                    ascent::internal::comment(
                        "e_node_match <-- do_match_indices__total, if let ⋯, calc_expr_3_left_indices__total, if ⋯, if ⋯",
                    );
                    if let Some(__matching) = do_match_indices__total.index_get(&()) {
                        for __ind in __matching {
                            let __row = &_self.do_match[__ind].clone();
                            let pat = &__row.0;
                            let e_id = &__row.1;
                            if let EClass(m_representive_node) = pat.deref() {
                                if let Some(__matching)
                                    = calc_expr_3_left_indices__total.index_get(&())
                                {
                                    for __ind in __matching {
                                        let __row = &_self.calc_expr_3_left[__ind].clone();
                                        let __1 = &__row.0;
                                        let __2 = &__row.1;
                                        let eq_set = &__row.2;
                                        if eq_set.contains(e_id) {
                                            if eq_set.contains(m_representive_node) {
                                                let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                                    ascent::internal::Convert::convert(pat),
                                                    ascent::internal::Convert::convert(e_id),
                                                );
                                                let __new_row_ind = _self.e_node_match.len();
                                                if !::ascent::internal::RelFullIndexRead::contains_key(
                                                    &e_node_match_indices_0_1_total,
                                                    &__new_row,
                                                )
                                                    && !::ascent::internal::RelFullIndexRead::contains_key(
                                                        e_node_match_indices_0_1_delta,
                                                        &__new_row,
                                                    )
                                                    && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                        &mut e_node_match_indices_0_1_new,
                                                        &__new_row,
                                                        __new_row_ind,
                                                    )
                                                {
                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                        &mut e_node_match_indices_0_new,
                                                        (__new_row.0.clone(),),
                                                        __new_row_ind,
                                                    );
                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                        &mut e_node_match_indices__new,
                                                        (),
                                                        __new_row_ind,
                                                    );
                                                    _self.e_node_match.push(__new_row);
                                                    __changed = true;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_match_indices_0_delta,
                        &mut e_node_match_indices_0_total,
                    );
                    std::mem::swap(
                        &mut e_node_match_indices_0_new,
                        e_node_match_indices_0_delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_match_indices_0_1_delta,
                        &mut e_node_match_indices_0_1_total,
                    );
                    std::mem::swap(
                        &mut e_node_match_indices_0_1_new,
                        e_node_match_indices_0_1_delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_match_indices__delta,
                        &mut e_node_match_indices__total,
                    );
                    std::mem::swap(
                        &mut e_node_match_indices__new,
                        e_node_match_indices__delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_match_indices_0_delta,
                        &mut e_node_match_indices_0_total,
                    );
                    std::mem::swap(
                        &mut e_node_match_indices_0_new,
                        e_node_match_indices_0_delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_match_indices_0_1_delta,
                        &mut e_node_match_indices_0_1_total,
                    );
                    std::mem::swap(
                        &mut e_node_match_indices_0_1_new,
                        e_node_match_indices_0_1_delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_match_indices__delta,
                        &mut e_node_match_indices__total,
                    );
                    std::mem::swap(
                        &mut e_node_match_indices__new,
                        e_node_match_indices__delta,
                    );
                }
                _self.e_node_match_indices_0 = e_node_match_indices_0_total;
                _self.e_node_match_indices_0_1 = e_node_match_indices_0_1_total;
                _self.e_node_match_indices_ = e_node_match_indices__total;
                _self.scc7_duration += _scc_start_time.elapsed();
            }
            ascent::internal::comment("scc 8");
            {
                let _scc_start_time = ::ascent::internal::Instant::now();
                #[allow(non_snake_case)]
                let e_node_match_indices__delta: &mut ascent::internal::RelIndexType<
                    (),
                > = &mut _self.e_node_match_indices_;
                #[allow(non_snake_case)]
                let mut e_node_match_indices__total: ascent::internal::RelIndexType<
                    (),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut e_node_match_indices__new: ascent::internal::RelIndexType<()> = Default::default();
                #[allow(non_snake_case)]
                let e_node_match_indices_0_1_delta: &mut ascent::internal::RelFullIndexType<
                    (Rc<PatternExpr>, ENodeId),
                > = &mut _self.e_node_match_indices_0_1;
                #[allow(non_snake_case)]
                let mut e_node_match_indices_0_1_total: ascent::internal::RelFullIndexType<
                    (Rc<PatternExpr>, ENodeId),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut e_node_match_indices_0_1_new: ascent::internal::RelFullIndexType<
                    (Rc<PatternExpr>, ENodeId),
                > = Default::default();
                #[allow(non_snake_case)]
                let e_node_match_indices_0_delta: &mut ascent::internal::RelIndexType<
                    (Rc<PatternExpr>,),
                > = &mut _self.e_node_match_indices_0;
                #[allow(non_snake_case)]
                let mut e_node_match_indices_0_total: ascent::internal::RelIndexType<
                    (Rc<PatternExpr>,),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut e_node_match_indices_0_new: ascent::internal::RelIndexType<
                    (Rc<PatternExpr>,),
                > = Default::default();
                #[allow(non_snake_case)]
                let num_indices_0_1_total: &mut ascent::internal::RelFullIndexType<
                    (ENodeId, i32),
                > = &mut _self.num_indices_0_1;
                #[allow(non_snake_case)]
                let do_match_indices__total: &mut ascent::internal::RelIndexType<()> = &mut _self
                    .do_match_indices_;
                #[allow(unused_assignments, unused_variables)]
                {
                    let mut __changed = false;
                    ascent::internal::comment(
                        "e_node_match <-- do_match_indices__total, if let ⋯, num_indices_0_1_total",
                    );
                    if let Some(__matching) = do_match_indices__total.index_get(&()) {
                        for __ind in __matching {
                            let __row = &_self.do_match[__ind].clone();
                            let pat = &__row.0;
                            let e_id = &__row.1;
                            if let Num(n) = pat.deref() {
                                if let Some(__matching)
                                    = num_indices_0_1_total
                                        .index_get(&(e_id.clone(), n.clone()))
                                {
                                    for __ind in __matching {
                                        let __row = &_self.num[__ind].clone();
                                        let n = &__row.1;
                                        let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                            ascent::internal::Convert::convert(pat),
                                            ascent::internal::Convert::convert(e_id),
                                        );
                                        let __new_row_ind = _self.e_node_match.len();
                                        if !::ascent::internal::RelFullIndexRead::contains_key(
                                            &e_node_match_indices_0_1_total,
                                            &__new_row,
                                        )
                                            && !::ascent::internal::RelFullIndexRead::contains_key(
                                                e_node_match_indices_0_1_delta,
                                                &__new_row,
                                            )
                                            && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                &mut e_node_match_indices_0_1_new,
                                                &__new_row,
                                                __new_row_ind,
                                            )
                                        {
                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                &mut e_node_match_indices__new,
                                                (),
                                                __new_row_ind,
                                            );
                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                &mut e_node_match_indices_0_new,
                                                (__new_row.0.clone(),),
                                                __new_row_ind,
                                            );
                                            _self.e_node_match.push(__new_row);
                                            __changed = true;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_match_indices__delta,
                        &mut e_node_match_indices__total,
                    );
                    std::mem::swap(
                        &mut e_node_match_indices__new,
                        e_node_match_indices__delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_match_indices_0_1_delta,
                        &mut e_node_match_indices_0_1_total,
                    );
                    std::mem::swap(
                        &mut e_node_match_indices_0_1_new,
                        e_node_match_indices_0_1_delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_match_indices_0_delta,
                        &mut e_node_match_indices_0_total,
                    );
                    std::mem::swap(
                        &mut e_node_match_indices_0_new,
                        e_node_match_indices_0_delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_match_indices__delta,
                        &mut e_node_match_indices__total,
                    );
                    std::mem::swap(
                        &mut e_node_match_indices__new,
                        e_node_match_indices__delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_match_indices_0_1_delta,
                        &mut e_node_match_indices_0_1_total,
                    );
                    std::mem::swap(
                        &mut e_node_match_indices_0_1_new,
                        e_node_match_indices_0_1_delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_match_indices_0_delta,
                        &mut e_node_match_indices_0_total,
                    );
                    std::mem::swap(
                        &mut e_node_match_indices_0_new,
                        e_node_match_indices_0_delta,
                    );
                }
                _self.e_node_match_indices_ = e_node_match_indices__total;
                _self.e_node_match_indices_0_1 = e_node_match_indices_0_1_total;
                _self.e_node_match_indices_0 = e_node_match_indices_0_total;
                _self.scc8_duration += _scc_start_time.elapsed();
            }
            ascent::internal::comment("scc 9");
            {
                let _scc_start_time = ::ascent::internal::Instant::now();
                #[allow(non_snake_case)]
                let e_node_match_indices__delta: &mut ascent::internal::RelIndexType<
                    (),
                > = &mut _self.e_node_match_indices_;
                #[allow(non_snake_case)]
                let mut e_node_match_indices__total: ascent::internal::RelIndexType<
                    (),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut e_node_match_indices__new: ascent::internal::RelIndexType<()> = Default::default();
                #[allow(non_snake_case)]
                let e_node_match_indices_0_1_delta: &mut ascent::internal::RelFullIndexType<
                    (Rc<PatternExpr>, ENodeId),
                > = &mut _self.e_node_match_indices_0_1;
                #[allow(non_snake_case)]
                let mut e_node_match_indices_0_1_total: ascent::internal::RelFullIndexType<
                    (Rc<PatternExpr>, ENodeId),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut e_node_match_indices_0_1_new: ascent::internal::RelFullIndexType<
                    (Rc<PatternExpr>, ENodeId),
                > = Default::default();
                #[allow(non_snake_case)]
                let e_node_match_indices_0_delta: &mut ascent::internal::RelIndexType<
                    (Rc<PatternExpr>,),
                > = &mut _self.e_node_match_indices_0;
                #[allow(non_snake_case)]
                let mut e_node_match_indices_0_total: ascent::internal::RelIndexType<
                    (Rc<PatternExpr>,),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut e_node_match_indices_0_new: ascent::internal::RelIndexType<
                    (Rc<PatternExpr>,),
                > = Default::default();
                #[allow(non_snake_case)]
                let root_indices__total: &mut ascent::internal::LatticeIndexType<()> = &mut _self
                    .root_indices_;
                #[allow(non_snake_case)]
                let do_match_indices__total: &mut ascent::internal::RelIndexType<()> = &mut _self
                    .do_match_indices_;
                #[allow(unused_assignments, unused_variables)]
                {
                    let mut __changed = false;
                    ascent::internal::comment(
                        "e_node_match <-- do_match_indices__total, if let ⋯, root_indices__total, if ⋯, if ⋯",
                    );
                    if let Some(__matching) = do_match_indices__total.index_get(&()) {
                        for __ind in __matching {
                            let __row = &_self.do_match[__ind].clone();
                            let pat = &__row.0;
                            let e_id = &__row.1;
                            if let EClass(m_representive_node) = pat.deref() {
                                if let Some(__matching) = root_indices__total.index_get(&())
                                {
                                    for __ind in __matching {
                                        let __row = &_self.root[__ind].clone();
                                        let __1 = &__row.0;
                                        let eq_set = &__row.1;
                                        if eq_set.contains(e_id) {
                                            if eq_set.contains(m_representive_node) {
                                                let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                                    ascent::internal::Convert::convert(pat),
                                                    ascent::internal::Convert::convert(e_id),
                                                );
                                                let __new_row_ind = _self.e_node_match.len();
                                                if !::ascent::internal::RelFullIndexRead::contains_key(
                                                    &e_node_match_indices_0_1_total,
                                                    &__new_row,
                                                )
                                                    && !::ascent::internal::RelFullIndexRead::contains_key(
                                                        e_node_match_indices_0_1_delta,
                                                        &__new_row,
                                                    )
                                                    && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                        &mut e_node_match_indices_0_1_new,
                                                        &__new_row,
                                                        __new_row_ind,
                                                    )
                                                {
                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                        &mut e_node_match_indices__new,
                                                        (),
                                                        __new_row_ind,
                                                    );
                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                        &mut e_node_match_indices_0_new,
                                                        (__new_row.0.clone(),),
                                                        __new_row_ind,
                                                    );
                                                    _self.e_node_match.push(__new_row);
                                                    __changed = true;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_match_indices__delta,
                        &mut e_node_match_indices__total,
                    );
                    std::mem::swap(
                        &mut e_node_match_indices__new,
                        e_node_match_indices__delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_match_indices_0_1_delta,
                        &mut e_node_match_indices_0_1_total,
                    );
                    std::mem::swap(
                        &mut e_node_match_indices_0_1_new,
                        e_node_match_indices_0_1_delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_match_indices_0_delta,
                        &mut e_node_match_indices_0_total,
                    );
                    std::mem::swap(
                        &mut e_node_match_indices_0_new,
                        e_node_match_indices_0_delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_match_indices__delta,
                        &mut e_node_match_indices__total,
                    );
                    std::mem::swap(
                        &mut e_node_match_indices__new,
                        e_node_match_indices__delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_match_indices_0_1_delta,
                        &mut e_node_match_indices_0_1_total,
                    );
                    std::mem::swap(
                        &mut e_node_match_indices_0_1_new,
                        e_node_match_indices_0_1_delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_match_indices_0_delta,
                        &mut e_node_match_indices_0_total,
                    );
                    std::mem::swap(
                        &mut e_node_match_indices_0_new,
                        e_node_match_indices_0_delta,
                    );
                }
                _self.e_node_match_indices_ = e_node_match_indices__total;
                _self.e_node_match_indices_0_1 = e_node_match_indices_0_1_total;
                _self.e_node_match_indices_0 = e_node_match_indices_0_total;
                _self.scc9_duration += _scc_start_time.elapsed();
            }
            ascent::internal::comment("scc 10");
            {
                let _scc_start_time = ::ascent::internal::Instant::now();
                #[allow(non_snake_case)]
                let e_node_match_indices_0_delta: &mut ascent::internal::RelIndexType<
                    (Rc<PatternExpr>,),
                > = &mut _self.e_node_match_indices_0;
                #[allow(non_snake_case)]
                let mut e_node_match_indices_0_total: ascent::internal::RelIndexType<
                    (Rc<PatternExpr>,),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut e_node_match_indices_0_new: ascent::internal::RelIndexType<
                    (Rc<PatternExpr>,),
                > = Default::default();
                #[allow(non_snake_case)]
                let e_node_match_indices__delta: &mut ascent::internal::RelIndexType<
                    (),
                > = &mut _self.e_node_match_indices_;
                #[allow(non_snake_case)]
                let mut e_node_match_indices__total: ascent::internal::RelIndexType<
                    (),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut e_node_match_indices__new: ascent::internal::RelIndexType<()> = Default::default();
                #[allow(non_snake_case)]
                let e_node_match_indices_0_1_delta: &mut ascent::internal::RelFullIndexType<
                    (Rc<PatternExpr>, ENodeId),
                > = &mut _self.e_node_match_indices_0_1;
                #[allow(non_snake_case)]
                let mut e_node_match_indices_0_1_total: ascent::internal::RelFullIndexType<
                    (Rc<PatternExpr>, ENodeId),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut e_node_match_indices_0_1_new: ascent::internal::RelFullIndexType<
                    (Rc<PatternExpr>, ENodeId),
                > = Default::default();
                #[allow(non_snake_case)]
                let var_indices_0_1_total: &mut ascent::internal::RelFullIndexType<
                    (ENodeId, Sym),
                > = &mut _self.var_indices_0_1;
                #[allow(non_snake_case)]
                let do_match_indices__total: &mut ascent::internal::RelIndexType<()> = &mut _self
                    .do_match_indices_;
                #[allow(unused_assignments, unused_variables)]
                {
                    let mut __changed = false;
                    ascent::internal::comment(
                        "e_node_match <-- do_match_indices__total, if let ⋯, var_indices_0_1_total",
                    );
                    if let Some(__matching) = do_match_indices__total.index_get(&()) {
                        for __ind in __matching {
                            let __row = &_self.do_match[__ind].clone();
                            let pat = &__row.0;
                            let e_id = &__row.1;
                            if let Var(x) = pat.deref() {
                                if let Some(__matching)
                                    = var_indices_0_1_total
                                        .index_get(&(e_id.clone(), x.clone()))
                                {
                                    for __ind in __matching {
                                        let __row = &_self.var[__ind].clone();
                                        let x = &__row.1;
                                        let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                            ascent::internal::Convert::convert(pat),
                                            ascent::internal::Convert::convert(e_id),
                                        );
                                        let __new_row_ind = _self.e_node_match.len();
                                        if !::ascent::internal::RelFullIndexRead::contains_key(
                                            &e_node_match_indices_0_1_total,
                                            &__new_row,
                                        )
                                            && !::ascent::internal::RelFullIndexRead::contains_key(
                                                e_node_match_indices_0_1_delta,
                                                &__new_row,
                                            )
                                            && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                &mut e_node_match_indices_0_1_new,
                                                &__new_row,
                                                __new_row_ind,
                                            )
                                        {
                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                &mut e_node_match_indices_0_new,
                                                (__new_row.0.clone(),),
                                                __new_row_ind,
                                            );
                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                &mut e_node_match_indices__new,
                                                (),
                                                __new_row_ind,
                                            );
                                            _self.e_node_match.push(__new_row);
                                            __changed = true;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_match_indices_0_delta,
                        &mut e_node_match_indices_0_total,
                    );
                    std::mem::swap(
                        &mut e_node_match_indices_0_new,
                        e_node_match_indices_0_delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_match_indices__delta,
                        &mut e_node_match_indices__total,
                    );
                    std::mem::swap(
                        &mut e_node_match_indices__new,
                        e_node_match_indices__delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_match_indices_0_1_delta,
                        &mut e_node_match_indices_0_1_total,
                    );
                    std::mem::swap(
                        &mut e_node_match_indices_0_1_new,
                        e_node_match_indices_0_1_delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_match_indices_0_delta,
                        &mut e_node_match_indices_0_total,
                    );
                    std::mem::swap(
                        &mut e_node_match_indices_0_new,
                        e_node_match_indices_0_delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_match_indices__delta,
                        &mut e_node_match_indices__total,
                    );
                    std::mem::swap(
                        &mut e_node_match_indices__new,
                        e_node_match_indices__delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_match_indices_0_1_delta,
                        &mut e_node_match_indices_0_1_total,
                    );
                    std::mem::swap(
                        &mut e_node_match_indices_0_1_new,
                        e_node_match_indices_0_1_delta,
                    );
                }
                _self.e_node_match_indices_0 = e_node_match_indices_0_total;
                _self.e_node_match_indices_ = e_node_match_indices__total;
                _self.e_node_match_indices_0_1 = e_node_match_indices_0_1_total;
                _self.scc10_duration += _scc_start_time.elapsed();
            }
            ascent::internal::comment("scc 11");
            {
                let _scc_start_time = ::ascent::internal::Instant::now();
                #[allow(non_snake_case)]
                let e_node_match_indices__delta: &mut ascent::internal::RelIndexType<
                    (),
                > = &mut _self.e_node_match_indices_;
                #[allow(non_snake_case)]
                let mut e_node_match_indices__total: ascent::internal::RelIndexType<
                    (),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut e_node_match_indices__new: ascent::internal::RelIndexType<()> = Default::default();
                #[allow(non_snake_case)]
                let e_node_match_indices_0_1_delta: &mut ascent::internal::RelFullIndexType<
                    (Rc<PatternExpr>, ENodeId),
                > = &mut _self.e_node_match_indices_0_1;
                #[allow(non_snake_case)]
                let mut e_node_match_indices_0_1_total: ascent::internal::RelFullIndexType<
                    (Rc<PatternExpr>, ENodeId),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut e_node_match_indices_0_1_new: ascent::internal::RelFullIndexType<
                    (Rc<PatternExpr>, ENodeId),
                > = Default::default();
                #[allow(non_snake_case)]
                let e_node_match_indices_0_delta: &mut ascent::internal::RelIndexType<
                    (Rc<PatternExpr>,),
                > = &mut _self.e_node_match_indices_0;
                #[allow(non_snake_case)]
                let mut e_node_match_indices_0_total: ascent::internal::RelIndexType<
                    (Rc<PatternExpr>,),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut e_node_match_indices_0_new: ascent::internal::RelIndexType<
                    (Rc<PatternExpr>,),
                > = Default::default();
                #[allow(non_snake_case)]
                let do_match_indices__total: &mut ascent::internal::RelIndexType<()> = &mut _self
                    .do_match_indices_;
                #[allow(non_snake_case)]
                let calc_expr_3_right_indices__total: &mut ascent::internal::LatticeIndexType<
                    (),
                > = &mut _self.calc_expr_3_right_indices_;
                #[allow(unused_assignments, unused_variables)]
                {
                    let mut __changed = false;
                    ascent::internal::comment(
                        "e_node_match <-- do_match_indices__total, if let ⋯, calc_expr_3_right_indices__total, if ⋯, if ⋯",
                    );
                    if let Some(__matching) = do_match_indices__total.index_get(&()) {
                        for __ind in __matching {
                            let __row = &_self.do_match[__ind].clone();
                            let pat = &__row.0;
                            let e_id = &__row.1;
                            if let EClass(m_representive_node) = pat.deref() {
                                if let Some(__matching)
                                    = calc_expr_3_right_indices__total.index_get(&())
                                {
                                    for __ind in __matching {
                                        let __row = &_self.calc_expr_3_right[__ind].clone();
                                        let __1 = &__row.0;
                                        let __2 = &__row.1;
                                        let eq_set = &__row.2;
                                        if eq_set.contains(e_id) {
                                            if eq_set.contains(m_representive_node) {
                                                let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                                    ascent::internal::Convert::convert(pat),
                                                    ascent::internal::Convert::convert(e_id),
                                                );
                                                let __new_row_ind = _self.e_node_match.len();
                                                if !::ascent::internal::RelFullIndexRead::contains_key(
                                                    &e_node_match_indices_0_1_total,
                                                    &__new_row,
                                                )
                                                    && !::ascent::internal::RelFullIndexRead::contains_key(
                                                        e_node_match_indices_0_1_delta,
                                                        &__new_row,
                                                    )
                                                    && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                        &mut e_node_match_indices_0_1_new,
                                                        &__new_row,
                                                        __new_row_ind,
                                                    )
                                                {
                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                        &mut e_node_match_indices__new,
                                                        (),
                                                        __new_row_ind,
                                                    );
                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                        &mut e_node_match_indices_0_new,
                                                        (__new_row.0.clone(),),
                                                        __new_row_ind,
                                                    );
                                                    _self.e_node_match.push(__new_row);
                                                    __changed = true;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_match_indices__delta,
                        &mut e_node_match_indices__total,
                    );
                    std::mem::swap(
                        &mut e_node_match_indices__new,
                        e_node_match_indices__delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_match_indices_0_1_delta,
                        &mut e_node_match_indices_0_1_total,
                    );
                    std::mem::swap(
                        &mut e_node_match_indices_0_1_new,
                        e_node_match_indices_0_1_delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_match_indices_0_delta,
                        &mut e_node_match_indices_0_total,
                    );
                    std::mem::swap(
                        &mut e_node_match_indices_0_new,
                        e_node_match_indices_0_delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_match_indices__delta,
                        &mut e_node_match_indices__total,
                    );
                    std::mem::swap(
                        &mut e_node_match_indices__new,
                        e_node_match_indices__delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_match_indices_0_1_delta,
                        &mut e_node_match_indices_0_1_total,
                    );
                    std::mem::swap(
                        &mut e_node_match_indices_0_1_new,
                        e_node_match_indices_0_1_delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_match_indices_0_delta,
                        &mut e_node_match_indices_0_total,
                    );
                    std::mem::swap(
                        &mut e_node_match_indices_0_new,
                        e_node_match_indices_0_delta,
                    );
                }
                _self.e_node_match_indices_ = e_node_match_indices__total;
                _self.e_node_match_indices_0_1 = e_node_match_indices_0_1_total;
                _self.e_node_match_indices_0 = e_node_match_indices_0_total;
                _self.scc11_duration += _scc_start_time.elapsed();
            }
            ascent::internal::comment("scc 12");
            {
                let _scc_start_time = ::ascent::internal::Instant::now();
                #[allow(non_snake_case)]
                let e_node_match_indices_0_delta: &mut ascent::internal::RelIndexType<
                    (Rc<PatternExpr>,),
                > = &mut _self.e_node_match_indices_0;
                #[allow(non_snake_case)]
                let mut e_node_match_indices_0_total: ascent::internal::RelIndexType<
                    (Rc<PatternExpr>,),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut e_node_match_indices_0_new: ascent::internal::RelIndexType<
                    (Rc<PatternExpr>,),
                > = Default::default();
                #[allow(non_snake_case)]
                let e_node_match_indices__delta: &mut ascent::internal::RelIndexType<
                    (),
                > = &mut _self.e_node_match_indices_;
                #[allow(non_snake_case)]
                let mut e_node_match_indices__total: ascent::internal::RelIndexType<
                    (),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut e_node_match_indices__new: ascent::internal::RelIndexType<()> = Default::default();
                #[allow(non_snake_case)]
                let e_node_match_indices_0_1_delta: &mut ascent::internal::RelFullIndexType<
                    (Rc<PatternExpr>, ENodeId),
                > = &mut _self.e_node_match_indices_0_1;
                #[allow(non_snake_case)]
                let mut e_node_match_indices_0_1_total: ascent::internal::RelFullIndexType<
                    (Rc<PatternExpr>, ENodeId),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut e_node_match_indices_0_1_new: ascent::internal::RelFullIndexType<
                    (Rc<PatternExpr>, ENodeId),
                > = Default::default();
                #[allow(non_snake_case)]
                let calc_expr_3_right_indices_0_1_total: &mut ascent::internal::LatticeIndexType<
                    (ENodeId, Sym),
                > = &mut _self.calc_expr_3_right_indices_0_1;
                #[allow(non_snake_case)]
                let calc_expr_3_right_indices__total: &mut ascent::internal::LatticeIndexType<
                    (),
                > = &mut _self.calc_expr_3_right_indices_;
                #[allow(non_snake_case)]
                let do_match_indices__total: &mut ascent::internal::RelIndexType<()> = &mut _self
                    .do_match_indices_;
                #[allow(non_snake_case)]
                let root_indices_0_total: &mut ascent::internal::LatticeIndexType<
                    (ENodeId,),
                > = &mut _self.root_indices_0;
                #[allow(non_snake_case)]
                let root_indices__total: &mut ascent::internal::LatticeIndexType<()> = &mut _self
                    .root_indices_;
                #[allow(non_snake_case)]
                let calc_expr_3_left_indices__total: &mut ascent::internal::LatticeIndexType<
                    (),
                > = &mut _self.calc_expr_3_left_indices_;
                #[allow(non_snake_case)]
                let calc_expr_3_left_indices_0_1_total: &mut ascent::internal::LatticeIndexType<
                    (ENodeId, Sym),
                > = &mut _self.calc_expr_3_left_indices_0_1;
                #[allow(unused_assignments, unused_variables)]
                loop {
                    let mut __changed = false;
                    ascent::internal::comment(
                        "e_node_match <-- e_node_match_indices__delta, calc_expr_3_left_indices__total, if ⋯, for_e_id",
                    );
                    if let Some(__matching) = e_node_match_indices__delta.index_get(&())
                    {
                        for __ind in __matching {
                            let __row = &_self.e_node_match[__ind].clone();
                            let pat = &__row.0;
                            let matched_id = &__row.1;
                            if let Some(__matching)
                                = calc_expr_3_left_indices__total.index_get(&())
                            {
                                for __ind in __matching {
                                    let __row = &_self.calc_expr_3_left[__ind].clone();
                                    let __1 = &__row.0;
                                    let __2 = &__row.1;
                                    let eq_set = &__row.2;
                                    if eq_set.deref().contains(matched_id) {
                                        for e_id in eq_set.deref() {
                                            let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                                ascent::internal::Convert::convert(pat),
                                                ascent::internal::Convert::convert(e_id),
                                            );
                                            let __new_row_ind = _self.e_node_match.len();
                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                &e_node_match_indices_0_1_total,
                                                &__new_row,
                                            )
                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                    e_node_match_indices_0_1_delta,
                                                    &__new_row,
                                                )
                                                && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                    &mut e_node_match_indices_0_1_new,
                                                    &__new_row,
                                                    __new_row_ind,
                                                )
                                            {
                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                    &mut e_node_match_indices_0_new,
                                                    (__new_row.0.clone(),),
                                                    __new_row_ind,
                                                );
                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                    &mut e_node_match_indices__new,
                                                    (),
                                                    __new_row_ind,
                                                );
                                                _self.e_node_match.push(__new_row);
                                                __changed = true;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment(
                        "e_node_match <-- e_node_match_indices__delta, calc_expr_3_right_indices__total, if ⋯, for_e_id",
                    );
                    if let Some(__matching) = e_node_match_indices__delta.index_get(&())
                    {
                        for __ind in __matching {
                            let __row = &_self.e_node_match[__ind].clone();
                            let pat = &__row.0;
                            let matched_id = &__row.1;
                            if let Some(__matching)
                                = calc_expr_3_right_indices__total.index_get(&())
                            {
                                for __ind in __matching {
                                    let __row = &_self.calc_expr_3_right[__ind].clone();
                                    let __1 = &__row.0;
                                    let __2 = &__row.1;
                                    let eq_set = &__row.2;
                                    if eq_set.deref().contains(matched_id) {
                                        for e_id in eq_set.deref() {
                                            let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                                ascent::internal::Convert::convert(pat),
                                                ascent::internal::Convert::convert(e_id),
                                            );
                                            let __new_row_ind = _self.e_node_match.len();
                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                &e_node_match_indices_0_1_total,
                                                &__new_row,
                                            )
                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                    e_node_match_indices_0_1_delta,
                                                    &__new_row,
                                                )
                                                && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                    &mut e_node_match_indices_0_1_new,
                                                    &__new_row,
                                                    __new_row_ind,
                                                )
                                            {
                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                    &mut e_node_match_indices_0_new,
                                                    (__new_row.0.clone(),),
                                                    __new_row_ind,
                                                );
                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                    &mut e_node_match_indices__new,
                                                    (),
                                                    __new_row_ind,
                                                );
                                                _self.e_node_match.push(__new_row);
                                                __changed = true;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment(
                        "e_node_match <-- do_match_indices__total, root_indices_0_total, for_root_eq_id, e_node_match_indices_0_1_delta",
                    );
                    if let Some(__matching) = do_match_indices__total.index_get(&()) {
                        for __ind in __matching {
                            let __row = &_self.do_match[__ind].clone();
                            let pat = &__row.0;
                            let e_id = &__row.1;
                            if let Some(__matching)
                                = root_indices_0_total.index_get(&(e_id.clone(),))
                            {
                                for __ind in __matching {
                                    let __row = &_self.root[__ind].clone();
                                    let root_eq_set = &__row.1;
                                    for root_eq_id in root_eq_set.deref() {
                                        if let Some(__matching)
                                            = e_node_match_indices_0_1_delta
                                                .index_get(&(pat.clone(), root_eq_id.clone()))
                                        {
                                            for __ind in __matching {
                                                let __row = &_self.e_node_match[__ind].clone();
                                                let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                                    ascent::internal::Convert::convert(pat),
                                                    ascent::internal::Convert::convert(e_id),
                                                );
                                                let __new_row_ind = _self.e_node_match.len();
                                                if !::ascent::internal::RelFullIndexRead::contains_key(
                                                    &e_node_match_indices_0_1_total,
                                                    &__new_row,
                                                )
                                                    && !::ascent::internal::RelFullIndexRead::contains_key(
                                                        e_node_match_indices_0_1_delta,
                                                        &__new_row,
                                                    )
                                                    && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                        &mut e_node_match_indices_0_1_new,
                                                        &__new_row,
                                                        __new_row_ind,
                                                    )
                                                {
                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                        &mut e_node_match_indices_0_new,
                                                        (__new_row.0.clone(),),
                                                        __new_row_ind,
                                                    );
                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                        &mut e_node_match_indices__new,
                                                        (),
                                                        __new_row_ind,
                                                    );
                                                    _self.e_node_match.push(__new_row);
                                                    __changed = true;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment(
                        "e_node_match <-- e_node_match_indices__delta, root_indices__total, if ⋯, for_e_id",
                    );
                    if let Some(__matching) = e_node_match_indices__delta.index_get(&())
                    {
                        for __ind in __matching {
                            let __row = &_self.e_node_match[__ind].clone();
                            let pat = &__row.0;
                            let matched_id = &__row.1;
                            if let Some(__matching) = root_indices__total.index_get(&())
                            {
                                for __ind in __matching {
                                    let __row = &_self.root[__ind].clone();
                                    let __1 = &__row.0;
                                    let eq_set = &__row.1;
                                    if eq_set.deref().contains(matched_id) {
                                        for e_id in eq_set.deref() {
                                            let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                                ascent::internal::Convert::convert(pat),
                                                ascent::internal::Convert::convert(e_id),
                                            );
                                            let __new_row_ind = _self.e_node_match.len();
                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                &e_node_match_indices_0_1_total,
                                                &__new_row,
                                            )
                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                    e_node_match_indices_0_1_delta,
                                                    &__new_row,
                                                )
                                                && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                    &mut e_node_match_indices_0_1_new,
                                                    &__new_row,
                                                    __new_row_ind,
                                                )
                                            {
                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                    &mut e_node_match_indices_0_new,
                                                    (__new_row.0.clone(),),
                                                    __new_row_ind,
                                                );
                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                    &mut e_node_match_indices__new,
                                                    (),
                                                    __new_row_ind,
                                                );
                                                _self.e_node_match.push(__new_row);
                                                __changed = true;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment(
                        "e_node_match <-- do_match_indices__total, if let ⋯, calc_expr_3_left_indices_0_1_total, calc_expr_3_right_indices_0_1_total, e_node_match_indices_0_delta, e_node_match_indices_0_total+delta, if ⋯, if ⋯",
                    );
                    if let Some(__matching) = do_match_indices__total.index_get(&()) {
                        for __ind in __matching {
                            let __row = &_self.do_match[__ind].clone();
                            let pat = &__row.0;
                            let e_id = &__row.1;
                            if let Calc(op, left_p, right_p) = pat.deref() {
                                if let Some(__matching)
                                    = calc_expr_3_left_indices_0_1_total
                                        .index_get(&(e_id.clone(), op.clone()))
                                {
                                    for __ind in __matching {
                                        let __row = &_self.calc_expr_3_left[__ind].clone();
                                        let op = &__row.1;
                                        let l_set = &__row.2;
                                        if let Some(__matching)
                                            = calc_expr_3_right_indices_0_1_total
                                                .index_get(&(e_id.clone(), op.clone()))
                                        {
                                            for __ind in __matching {
                                                let __row = &_self.calc_expr_3_right[__ind].clone();
                                                let r_set = &__row.2;
                                                if let Some(__matching)
                                                    = e_node_match_indices_0_delta.index_get(&(left_p.clone(),))
                                                {
                                                    for __ind in __matching {
                                                        let __row = &_self.e_node_match[__ind].clone();
                                                        let left_p = &__row.0;
                                                        let l_matched = &__row.1;
                                                        if let Some(__matching)
                                                            = ascent::internal::RelIndexCombined::new(
                                                                    &e_node_match_indices_0_total,
                                                                    e_node_match_indices_0_delta,
                                                                )
                                                                .index_get(&(right_p.clone(),))
                                                        {
                                                            for __ind in __matching {
                                                                let __row = &_self.e_node_match[__ind].clone();
                                                                let right_p = &__row.0;
                                                                let r_matched = &__row.1;
                                                                if l_set.contains(l_matched) {
                                                                    if r_set.contains(r_matched) {
                                                                        let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                                                            ascent::internal::Convert::convert(pat),
                                                                            ascent::internal::Convert::convert(e_id),
                                                                        );
                                                                        let __new_row_ind = _self.e_node_match.len();
                                                                        if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                            &e_node_match_indices_0_1_total,
                                                                            &__new_row,
                                                                        )
                                                                            && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                e_node_match_indices_0_1_delta,
                                                                                &__new_row,
                                                                            )
                                                                            && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                                &mut e_node_match_indices_0_1_new,
                                                                                &__new_row,
                                                                                __new_row_ind,
                                                                            )
                                                                        {
                                                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                                                &mut e_node_match_indices_0_new,
                                                                                (__new_row.0.clone(),),
                                                                                __new_row_ind,
                                                                            );
                                                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                                                &mut e_node_match_indices__new,
                                                                                (),
                                                                                __new_row_ind,
                                                                            );
                                                                            _self.e_node_match.push(__new_row);
                                                                            __changed = true;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment(
                        "e_node_match <-- do_match_indices__total, if let ⋯, calc_expr_3_left_indices_0_1_total, calc_expr_3_right_indices_0_1_total, e_node_match_indices_0_total, e_node_match_indices_0_delta, if ⋯, if ⋯",
                    );
                    if let Some(__matching) = do_match_indices__total.index_get(&()) {
                        for __ind in __matching {
                            let __row = &_self.do_match[__ind].clone();
                            let pat = &__row.0;
                            let e_id = &__row.1;
                            if let Calc(op, left_p, right_p) = pat.deref() {
                                if let Some(__matching)
                                    = calc_expr_3_left_indices_0_1_total
                                        .index_get(&(e_id.clone(), op.clone()))
                                {
                                    for __ind in __matching {
                                        let __row = &_self.calc_expr_3_left[__ind].clone();
                                        let op = &__row.1;
                                        let l_set = &__row.2;
                                        if let Some(__matching)
                                            = calc_expr_3_right_indices_0_1_total
                                                .index_get(&(e_id.clone(), op.clone()))
                                        {
                                            for __ind in __matching {
                                                let __row = &_self.calc_expr_3_right[__ind].clone();
                                                let r_set = &__row.2;
                                                if let Some(__matching)
                                                    = e_node_match_indices_0_total.index_get(&(left_p.clone(),))
                                                {
                                                    for __ind in __matching {
                                                        let __row = &_self.e_node_match[__ind].clone();
                                                        let left_p = &__row.0;
                                                        let l_matched = &__row.1;
                                                        if let Some(__matching)
                                                            = e_node_match_indices_0_delta
                                                                .index_get(&(right_p.clone(),))
                                                        {
                                                            for __ind in __matching {
                                                                let __row = &_self.e_node_match[__ind].clone();
                                                                let right_p = &__row.0;
                                                                let r_matched = &__row.1;
                                                                if l_set.contains(l_matched) {
                                                                    if r_set.contains(r_matched) {
                                                                        let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                                                            ascent::internal::Convert::convert(pat),
                                                                            ascent::internal::Convert::convert(e_id),
                                                                        );
                                                                        let __new_row_ind = _self.e_node_match.len();
                                                                        if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                            &e_node_match_indices_0_1_total,
                                                                            &__new_row,
                                                                        )
                                                                            && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                e_node_match_indices_0_1_delta,
                                                                                &__new_row,
                                                                            )
                                                                            && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                                &mut e_node_match_indices_0_1_new,
                                                                                &__new_row,
                                                                                __new_row_ind,
                                                                            )
                                                                        {
                                                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                                                &mut e_node_match_indices_0_new,
                                                                                (__new_row.0.clone(),),
                                                                                __new_row_ind,
                                                                            );
                                                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                                                &mut e_node_match_indices__new,
                                                                                (),
                                                                                __new_row_ind,
                                                                            );
                                                                            _self.e_node_match.push(__new_row);
                                                                            __changed = true;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_match_indices_0_delta,
                        &mut e_node_match_indices_0_total,
                    );
                    std::mem::swap(
                        &mut e_node_match_indices_0_new,
                        e_node_match_indices_0_delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_match_indices__delta,
                        &mut e_node_match_indices__total,
                    );
                    std::mem::swap(
                        &mut e_node_match_indices__new,
                        e_node_match_indices__delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_match_indices_0_1_delta,
                        &mut e_node_match_indices_0_1_total,
                    );
                    std::mem::swap(
                        &mut e_node_match_indices_0_1_new,
                        e_node_match_indices_0_1_delta,
                    );
                    if !__changed {
                        break;
                    }
                }
                _self.e_node_match_indices_0 = e_node_match_indices_0_total;
                _self.e_node_match_indices_ = e_node_match_indices__total;
                _self.e_node_match_indices_0_1 = e_node_match_indices_0_1_total;
                _self.scc12_duration += _scc_start_time.elapsed();
            }
            ascent::internal::comment("scc 13");
            {
                let _scc_start_time = ::ascent::internal::Instant::now();
                #[allow(non_snake_case)]
                let matched_eclass_indices_0_delta: &mut ascent::internal::RelFullIndexType<
                    (BTreeSet<ENodeId>,),
                > = &mut _self.matched_eclass_indices_0;
                #[allow(non_snake_case)]
                let mut matched_eclass_indices_0_total: ascent::internal::RelFullIndexType<
                    (BTreeSet<ENodeId>,),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut matched_eclass_indices_0_new: ascent::internal::RelFullIndexType<
                    (BTreeSet<ENodeId>,),
                > = Default::default();
                #[allow(non_snake_case)]
                let e_node_match_indices_0_total: &mut ascent::internal::RelIndexType<
                    (Rc<PatternExpr>,),
                > = &mut _self.e_node_match_indices_0;
                #[allow(non_snake_case)]
                let do_match_input_indices__total: &mut ascent::internal::RelIndexType<
                    (),
                > = &mut _self.do_match_input_indices_;
                #[allow(non_snake_case)]
                let root_indices__total: &mut ascent::internal::LatticeIndexType<()> = &mut _self
                    .root_indices_;
                #[allow(unused_assignments, unused_variables)]
                {
                    let mut __changed = false;
                    ascent::internal::comment(
                        "matched_eclass <-- do_match_input_indices__total, root_indices__total, e_node_match_indices_0_total, if ⋯",
                    );
                    if let Some(__matching)
                        = do_match_input_indices__total.index_get(&())
                    {
                        for __ind in __matching {
                            let __row = &_self.do_match_input[__ind].clone();
                            let p = &__row.0;
                            if let Some(__matching) = root_indices__total.index_get(&())
                            {
                                for __ind in __matching {
                                    let __row = &_self.root[__ind].clone();
                                    let __1 = &__row.0;
                                    let eq_set = &__row.1;
                                    if let Some(__matching)
                                        = e_node_match_indices_0_total.index_get(&(p.clone(),))
                                    {
                                        for __ind in __matching {
                                            let __row = &_self.e_node_match[__ind].clone();
                                            let matched_e = &__row.1;
                                            if eq_set.contains(matched_e) {
                                                let __new_row: (BTreeSet<ENodeId>,) = (
                                                    eq_set.deref().clone(),
                                                );
                                                let __new_row_ind = _self.matched_eclass.len();
                                                if !::ascent::internal::RelFullIndexRead::contains_key(
                                                    &matched_eclass_indices_0_total,
                                                    &__new_row,
                                                )
                                                    && !::ascent::internal::RelFullIndexRead::contains_key(
                                                        matched_eclass_indices_0_delta,
                                                        &__new_row,
                                                    )
                                                    && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                        &mut matched_eclass_indices_0_new,
                                                        &__new_row,
                                                        __new_row_ind,
                                                    )
                                                {
                                                    _self.matched_eclass.push(__new_row);
                                                    __changed = true;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        matched_eclass_indices_0_delta,
                        &mut matched_eclass_indices_0_total,
                    );
                    std::mem::swap(
                        &mut matched_eclass_indices_0_new,
                        matched_eclass_indices_0_delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        matched_eclass_indices_0_delta,
                        &mut matched_eclass_indices_0_total,
                    );
                    std::mem::swap(
                        &mut matched_eclass_indices_0_new,
                        matched_eclass_indices_0_delta,
                    );
                }
                _self.matched_eclass_indices_0 = matched_eclass_indices_0_total;
                _self.scc13_duration += _scc_start_time.elapsed();
            }
            ascent::internal::comment("scc 14");
            {
                let _scc_start_time = ::ascent::internal::Instant::now();
                #[allow(non_snake_case)]
                let matched_eclass_indices_0_delta: &mut ascent::internal::RelFullIndexType<
                    (BTreeSet<ENodeId>,),
                > = &mut _self.matched_eclass_indices_0;
                #[allow(non_snake_case)]
                let mut matched_eclass_indices_0_total: ascent::internal::RelFullIndexType<
                    (BTreeSet<ENodeId>,),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut matched_eclass_indices_0_new: ascent::internal::RelFullIndexType<
                    (BTreeSet<ENodeId>,),
                > = Default::default();
                #[allow(non_snake_case)]
                let e_node_match_indices_0_total: &mut ascent::internal::RelIndexType<
                    (Rc<PatternExpr>,),
                > = &mut _self.e_node_match_indices_0;
                #[allow(non_snake_case)]
                let calc_expr_3_left_indices__total: &mut ascent::internal::LatticeIndexType<
                    (),
                > = &mut _self.calc_expr_3_left_indices_;
                #[allow(non_snake_case)]
                let do_match_input_indices__total: &mut ascent::internal::RelIndexType<
                    (),
                > = &mut _self.do_match_input_indices_;
                #[allow(unused_assignments, unused_variables)]
                {
                    let mut __changed = false;
                    ascent::internal::comment(
                        "matched_eclass <-- do_match_input_indices__total, calc_expr_3_left_indices__total, e_node_match_indices_0_total, if ⋯",
                    );
                    if let Some(__matching)
                        = do_match_input_indices__total.index_get(&())
                    {
                        for __ind in __matching {
                            let __row = &_self.do_match_input[__ind].clone();
                            let p = &__row.0;
                            if let Some(__matching)
                                = calc_expr_3_left_indices__total.index_get(&())
                            {
                                for __ind in __matching {
                                    let __row = &_self.calc_expr_3_left[__ind].clone();
                                    let __1 = &__row.0;
                                    let __2 = &__row.1;
                                    let eq_set = &__row.2;
                                    if let Some(__matching)
                                        = e_node_match_indices_0_total.index_get(&(p.clone(),))
                                    {
                                        for __ind in __matching {
                                            let __row = &_self.e_node_match[__ind].clone();
                                            let matched_e = &__row.1;
                                            if eq_set.contains(matched_e) {
                                                let __new_row: (BTreeSet<ENodeId>,) = (
                                                    eq_set.deref().clone(),
                                                );
                                                let __new_row_ind = _self.matched_eclass.len();
                                                if !::ascent::internal::RelFullIndexRead::contains_key(
                                                    &matched_eclass_indices_0_total,
                                                    &__new_row,
                                                )
                                                    && !::ascent::internal::RelFullIndexRead::contains_key(
                                                        matched_eclass_indices_0_delta,
                                                        &__new_row,
                                                    )
                                                    && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                        &mut matched_eclass_indices_0_new,
                                                        &__new_row,
                                                        __new_row_ind,
                                                    )
                                                {
                                                    _self.matched_eclass.push(__new_row);
                                                    __changed = true;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        matched_eclass_indices_0_delta,
                        &mut matched_eclass_indices_0_total,
                    );
                    std::mem::swap(
                        &mut matched_eclass_indices_0_new,
                        matched_eclass_indices_0_delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        matched_eclass_indices_0_delta,
                        &mut matched_eclass_indices_0_total,
                    );
                    std::mem::swap(
                        &mut matched_eclass_indices_0_new,
                        matched_eclass_indices_0_delta,
                    );
                }
                _self.matched_eclass_indices_0 = matched_eclass_indices_0_total;
                _self.scc14_duration += _scc_start_time.elapsed();
            }
            ascent::internal::comment("scc 15");
            {
                let _scc_start_time = ::ascent::internal::Instant::now();
                #[allow(non_snake_case)]
                let matched_eclass_indices_0_delta: &mut ascent::internal::RelFullIndexType<
                    (BTreeSet<ENodeId>,),
                > = &mut _self.matched_eclass_indices_0;
                #[allow(non_snake_case)]
                let mut matched_eclass_indices_0_total: ascent::internal::RelFullIndexType<
                    (BTreeSet<ENodeId>,),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut matched_eclass_indices_0_new: ascent::internal::RelFullIndexType<
                    (BTreeSet<ENodeId>,),
                > = Default::default();
                #[allow(non_snake_case)]
                let e_node_match_indices_0_total: &mut ascent::internal::RelIndexType<
                    (Rc<PatternExpr>,),
                > = &mut _self.e_node_match_indices_0;
                #[allow(non_snake_case)]
                let calc_expr_3_right_indices__total: &mut ascent::internal::LatticeIndexType<
                    (),
                > = &mut _self.calc_expr_3_right_indices_;
                #[allow(non_snake_case)]
                let do_match_input_indices__total: &mut ascent::internal::RelIndexType<
                    (),
                > = &mut _self.do_match_input_indices_;
                #[allow(unused_assignments, unused_variables)]
                {
                    let mut __changed = false;
                    ascent::internal::comment(
                        "matched_eclass <-- do_match_input_indices__total, calc_expr_3_right_indices__total, e_node_match_indices_0_total, if ⋯",
                    );
                    if let Some(__matching)
                        = do_match_input_indices__total.index_get(&())
                    {
                        for __ind in __matching {
                            let __row = &_self.do_match_input[__ind].clone();
                            let p = &__row.0;
                            if let Some(__matching)
                                = calc_expr_3_right_indices__total.index_get(&())
                            {
                                for __ind in __matching {
                                    let __row = &_self.calc_expr_3_right[__ind].clone();
                                    let __1 = &__row.0;
                                    let __2 = &__row.1;
                                    let eq_set = &__row.2;
                                    if let Some(__matching)
                                        = e_node_match_indices_0_total.index_get(&(p.clone(),))
                                    {
                                        for __ind in __matching {
                                            let __row = &_self.e_node_match[__ind].clone();
                                            let matched_e = &__row.1;
                                            if eq_set.contains(matched_e) {
                                                let __new_row: (BTreeSet<ENodeId>,) = (
                                                    eq_set.deref().clone(),
                                                );
                                                let __new_row_ind = _self.matched_eclass.len();
                                                if !::ascent::internal::RelFullIndexRead::contains_key(
                                                    &matched_eclass_indices_0_total,
                                                    &__new_row,
                                                )
                                                    && !::ascent::internal::RelFullIndexRead::contains_key(
                                                        matched_eclass_indices_0_delta,
                                                        &__new_row,
                                                    )
                                                    && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                        &mut matched_eclass_indices_0_new,
                                                        &__new_row,
                                                        __new_row_ind,
                                                    )
                                                {
                                                    _self.matched_eclass.push(__new_row);
                                                    __changed = true;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        matched_eclass_indices_0_delta,
                        &mut matched_eclass_indices_0_total,
                    );
                    std::mem::swap(
                        &mut matched_eclass_indices_0_new,
                        matched_eclass_indices_0_delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        matched_eclass_indices_0_delta,
                        &mut matched_eclass_indices_0_total,
                    );
                    std::mem::swap(
                        &mut matched_eclass_indices_0_new,
                        matched_eclass_indices_0_delta,
                    );
                }
                _self.matched_eclass_indices_0 = matched_eclass_indices_0_total;
                _self.scc15_duration += _scc_start_time.elapsed();
            }
            ascent::internal::comment("scc 16");
            {
                let _scc_start_time = ::ascent::internal::Instant::now();
                #[allow(non_snake_case)]
                let matched_pattern_var_indices_0_1_delta: &mut ascent::internal::RelFullIndexType<
                    (Rc<PatternExpr>, ENodeId),
                > = &mut _self.matched_pattern_var_indices_0_1;
                #[allow(non_snake_case)]
                let mut matched_pattern_var_indices_0_1_total: ascent::internal::RelFullIndexType<
                    (Rc<PatternExpr>, ENodeId),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut matched_pattern_var_indices_0_1_new: ascent::internal::RelFullIndexType<
                    (Rc<PatternExpr>, ENodeId),
                > = Default::default();
                #[allow(non_snake_case)]
                let e_node_match_indices__total: &mut ascent::internal::RelIndexType<
                    (),
                > = &mut _self.e_node_match_indices_;
                #[allow(unused_assignments, unused_variables)]
                {
                    let mut __changed = false;
                    ascent::internal::comment(
                        "matched_pattern_var <-- e_node_match_indices__total, if let ⋯",
                    );
                    if let Some(__matching) = e_node_match_indices__total.index_get(&())
                    {
                        for __ind in __matching {
                            let __row = &_self.e_node_match[__ind].clone();
                            let p = &__row.0;
                            let e = &__row.1;
                            if let WildCard(v) = p.deref() {
                                let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                    ascent::internal::Convert::convert(p),
                                    ascent::internal::Convert::convert(e),
                                );
                                let __new_row_ind = _self.matched_pattern_var.len();
                                if !::ascent::internal::RelFullIndexRead::contains_key(
                                    &matched_pattern_var_indices_0_1_total,
                                    &__new_row,
                                )
                                    && !::ascent::internal::RelFullIndexRead::contains_key(
                                        matched_pattern_var_indices_0_1_delta,
                                        &__new_row,
                                    )
                                    && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                        &mut matched_pattern_var_indices_0_1_new,
                                        &__new_row,
                                        __new_row_ind,
                                    )
                                {
                                    _self.matched_pattern_var.push(__new_row);
                                    __changed = true;
                                }
                            }
                        }
                    }
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        matched_pattern_var_indices_0_1_delta,
                        &mut matched_pattern_var_indices_0_1_total,
                    );
                    std::mem::swap(
                        &mut matched_pattern_var_indices_0_1_new,
                        matched_pattern_var_indices_0_1_delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        matched_pattern_var_indices_0_1_delta,
                        &mut matched_pattern_var_indices_0_1_total,
                    );
                    std::mem::swap(
                        &mut matched_pattern_var_indices_0_1_new,
                        matched_pattern_var_indices_0_1_delta,
                    );
                }
                _self
                    .matched_pattern_var_indices_0_1 = matched_pattern_var_indices_0_1_total;
                _self.scc16_duration += _scc_start_time.elapsed();
            }
        }
        fn update_indices_priv(&mut self) {
            for (i, tuple) in self.do_match_input.iter().enumerate() {
                let selection_tuple = ();
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.do_match_input_indices_,
                    selection_tuple,
                    i,
                );
                let selection_tuple = (tuple.0.clone(),);
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.do_match_input_indices_0,
                    selection_tuple,
                    i,
                );
            }
            for (i, tuple) in self.matched_eclass.iter().enumerate() {
                let selection_tuple = (tuple.0.clone(),);
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.matched_eclass_indices_0,
                    selection_tuple,
                    i,
                );
            }
            for (i, tuple) in self.calc_expr_3_left.iter().enumerate() {
                let selection_tuple = (tuple.0.clone(), tuple.1.clone());
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.calc_expr_3_left_indices_0_1,
                    selection_tuple,
                    i,
                );
                let selection_tuple = (
                    tuple.0.clone(),
                    tuple.1.clone(),
                    tuple.2.clone(),
                );
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.calc_expr_3_left_indices_0_1_2,
                    selection_tuple,
                    i,
                );
                let selection_tuple = ();
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.calc_expr_3_left_indices_,
                    selection_tuple,
                    i,
                );
            }
            for (i, tuple) in self.e_node.iter().enumerate() {
                let selection_tuple = (tuple.0.clone(),);
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.e_node_indices_0,
                    selection_tuple,
                    i,
                );
                let selection_tuple = ();
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.e_node_indices_,
                    selection_tuple,
                    i,
                );
            }
            for (i, tuple) in self.matched_pattern_var.iter().enumerate() {
                let selection_tuple = (tuple.0.clone(), tuple.1.clone());
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.matched_pattern_var_indices_0_1,
                    selection_tuple,
                    i,
                );
            }
            for (i, tuple) in self.var.iter().enumerate() {
                let selection_tuple = ();
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.var_indices_,
                    selection_tuple,
                    i,
                );
                let selection_tuple = (tuple.0.clone(), tuple.1.clone());
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.var_indices_0_1,
                    selection_tuple,
                    i,
                );
            }
            for (i, tuple) in self.num.iter().enumerate() {
                let selection_tuple = (tuple.0.clone(), tuple.1.clone());
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.num_indices_0_1,
                    selection_tuple,
                    i,
                );
                let selection_tuple = ();
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.num_indices_,
                    selection_tuple,
                    i,
                );
            }
            for (i, tuple) in self.calc_expr_3_right.iter().enumerate() {
                let selection_tuple = ();
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.calc_expr_3_right_indices_,
                    selection_tuple,
                    i,
                );
                let selection_tuple = (tuple.0.clone(), tuple.1.clone());
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.calc_expr_3_right_indices_0_1,
                    selection_tuple,
                    i,
                );
                let selection_tuple = (
                    tuple.0.clone(),
                    tuple.1.clone(),
                    tuple.2.clone(),
                );
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.calc_expr_3_right_indices_0_1_2,
                    selection_tuple,
                    i,
                );
            }
            for (i, tuple) in self.root.iter().enumerate() {
                let selection_tuple = ();
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.root_indices_,
                    selection_tuple,
                    i,
                );
                let selection_tuple = (tuple.0.clone(),);
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.root_indices_0,
                    selection_tuple,
                    i,
                );
                let selection_tuple = (tuple.0.clone(), tuple.1.clone());
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.root_indices_0_1,
                    selection_tuple,
                    i,
                );
            }
            for (i, tuple) in self.e_node_match.iter().enumerate() {
                let selection_tuple = ();
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.e_node_match_indices_,
                    selection_tuple,
                    i,
                );
                let selection_tuple = (tuple.0.clone(), tuple.1.clone());
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.e_node_match_indices_0_1,
                    selection_tuple,
                    i,
                );
                let selection_tuple = (tuple.0.clone(),);
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.e_node_match_indices_0,
                    selection_tuple,
                    i,
                );
            }
            for (i, tuple) in self.do_match.iter().enumerate() {
                let selection_tuple = (tuple.0.clone(), tuple.1.clone());
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.do_match_indices_0_1,
                    selection_tuple,
                    i,
                );
                let selection_tuple = ();
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.do_match_indices_,
                    selection_tuple,
                    i,
                );
            }
        }
        #[deprecated = "Explicit call to update_indices not required anymore."]
        pub fn update_indices(&mut self) {
            self.update_indices_priv();
        }
        #[allow(unused_imports)]
        fn type_constaints() {
            let _type_constraints: ascent::internal::TypeConstraints<Rc<PatternExpr>>;
            let _type_constraints: ascent::internal::TypeConstraints<BTreeSet<ENodeId>>;
            let _type_constraints: ascent::internal::TypeConstraints<ENodeId>;
            let _type_constraints: ascent::internal::TypeConstraints<Sym>;
            let _type_constraints: ascent::internal::LatTypeConstraints<Set<ENodeId>>;
            let _type_constraints: ascent::internal::TypeConstraints<i32>;
        }
        pub fn summary() -> &'static str {
            "scc 0, is_looping: false:\n  e_node <-- num_indices__total\n  dynamic relations: e_node\nscc 1, is_looping: false:\n  e_node <-- calc_expr_3_left_indices__total\n  dynamic relations: e_node\nscc 2, is_looping: false:\n  e_node <-- var_indices__total\n  dynamic relations: e_node\nscc 3, is_looping: false:\n  e_node <-- root_indices__total\n  dynamic relations: e_node\nscc 4, is_looping: false:\n  do_match <-- do_match_input_indices__total, e_node_indices__total [SIMPLE JOIN]\n  dynamic relations: do_match\nscc 5, is_looping: true:\n  do_match, do_match <-- do_match_indices__delta, if let ⋯, calc_expr_3_left_indices_0_1_total, calc_expr_3_right_indices_0_1_total, for_l_eq, for_r_eq\n  dynamic relations: do_match\nscc 6, is_looping: false:\n  e_node_match <-- do_match_indices__total, if let ⋯\n  dynamic relations: e_node_match\nscc 7, is_looping: false:\n  e_node_match <-- do_match_indices__total, if let ⋯, calc_expr_3_left_indices__total, if ⋯, if ⋯\n  dynamic relations: e_node_match\nscc 8, is_looping: false:\n  e_node_match <-- do_match_indices__total, if let ⋯, num_indices_0_1_total\n  dynamic relations: e_node_match\nscc 9, is_looping: false:\n  e_node_match <-- do_match_indices__total, if let ⋯, root_indices__total, if ⋯, if ⋯\n  dynamic relations: e_node_match\nscc 10, is_looping: false:\n  e_node_match <-- do_match_indices__total, if let ⋯, var_indices_0_1_total\n  dynamic relations: e_node_match\nscc 11, is_looping: false:\n  e_node_match <-- do_match_indices__total, if let ⋯, calc_expr_3_right_indices__total, if ⋯, if ⋯\n  dynamic relations: e_node_match\nscc 12, is_looping: true:\n  e_node_match <-- e_node_match_indices__delta, calc_expr_3_left_indices__total, if ⋯, for_e_id\n  e_node_match <-- e_node_match_indices__delta, calc_expr_3_right_indices__total, if ⋯, for_e_id\n  e_node_match <-- do_match_indices__total, root_indices_0_total, for_root_eq_id, e_node_match_indices_0_1_delta\n  e_node_match <-- e_node_match_indices__delta, root_indices__total, if ⋯, for_e_id\n  e_node_match <-- do_match_indices__total, if let ⋯, calc_expr_3_left_indices_0_1_total, calc_expr_3_right_indices_0_1_total, e_node_match_indices_0_delta, e_node_match_indices_0_total+delta, if ⋯, if ⋯\n  e_node_match <-- do_match_indices__total, if let ⋯, calc_expr_3_left_indices_0_1_total, calc_expr_3_right_indices_0_1_total, e_node_match_indices_0_total, e_node_match_indices_0_delta, if ⋯, if ⋯\n  dynamic relations: e_node_match\nscc 13, is_looping: false:\n  matched_eclass <-- do_match_input_indices__total, root_indices__total, e_node_match_indices_0_total, if ⋯\n  dynamic relations: matched_eclass\nscc 14, is_looping: false:\n  matched_eclass <-- do_match_input_indices__total, calc_expr_3_left_indices__total, e_node_match_indices_0_total, if ⋯\n  dynamic relations: matched_eclass\nscc 15, is_looping: false:\n  matched_eclass <-- do_match_input_indices__total, calc_expr_3_right_indices__total, e_node_match_indices_0_total, if ⋯\n  dynamic relations: matched_eclass\nscc 16, is_looping: false:\n  matched_pattern_var <-- e_node_match_indices__total, if let ⋯\n  dynamic relations: matched_pattern_var\n"
        }
        pub fn relation_sizes_summary(&self) -> String {
            use std::fmt::Write;
            let mut res = String::new();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["", " size: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"do_match_input"),
                            ::core::fmt::ArgumentV1::new_display(
                                &self.do_match_input.len(),
                            ),
                        ],
                    ),
                )
                .unwrap();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["", " size: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"matched_eclass"),
                            ::core::fmt::ArgumentV1::new_display(
                                &self.matched_eclass.len(),
                            ),
                        ],
                    ),
                )
                .unwrap();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["", " size: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"calc_expr_3_left"),
                            ::core::fmt::ArgumentV1::new_display(
                                &self.calc_expr_3_left.len(),
                            ),
                        ],
                    ),
                )
                .unwrap();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["", " size: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"e_node"),
                            ::core::fmt::ArgumentV1::new_display(&self.e_node.len()),
                        ],
                    ),
                )
                .unwrap();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["", " size: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"matched_pattern_var"),
                            ::core::fmt::ArgumentV1::new_display(
                                &self.matched_pattern_var.len(),
                            ),
                        ],
                    ),
                )
                .unwrap();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["", " size: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"var"),
                            ::core::fmt::ArgumentV1::new_display(&self.var.len()),
                        ],
                    ),
                )
                .unwrap();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["", " size: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"num"),
                            ::core::fmt::ArgumentV1::new_display(&self.num.len()),
                        ],
                    ),
                )
                .unwrap();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["", " size: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"calc_expr_3_right"),
                            ::core::fmt::ArgumentV1::new_display(
                                &self.calc_expr_3_right.len(),
                            ),
                        ],
                    ),
                )
                .unwrap();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["", " size: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"root"),
                            ::core::fmt::ArgumentV1::new_display(&self.root.len()),
                        ],
                    ),
                )
                .unwrap();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["", " size: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"e_node_match"),
                            ::core::fmt::ArgumentV1::new_display(
                                &self.e_node_match.len(),
                            ),
                        ],
                    ),
                )
                .unwrap();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["", " size: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"do_match"),
                            ::core::fmt::ArgumentV1::new_display(&self.do_match.len()),
                        ],
                    ),
                )
                .unwrap();
            res
        }
        pub fn scc_times_summary(&self) -> String {
            use std::fmt::Write;
            let mut res = String::new();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["scc ", " time: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"0"),
                            ::core::fmt::ArgumentV1::new_debug(&self.scc0_duration),
                        ],
                    ),
                )
                .unwrap();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["scc ", " time: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"1"),
                            ::core::fmt::ArgumentV1::new_debug(&self.scc1_duration),
                        ],
                    ),
                )
                .unwrap();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["scc ", " time: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"2"),
                            ::core::fmt::ArgumentV1::new_debug(&self.scc2_duration),
                        ],
                    ),
                )
                .unwrap();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["scc ", " time: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"3"),
                            ::core::fmt::ArgumentV1::new_debug(&self.scc3_duration),
                        ],
                    ),
                )
                .unwrap();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["scc ", " time: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"4"),
                            ::core::fmt::ArgumentV1::new_debug(&self.scc4_duration),
                        ],
                    ),
                )
                .unwrap();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["scc ", " time: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"5"),
                            ::core::fmt::ArgumentV1::new_debug(&self.scc5_duration),
                        ],
                    ),
                )
                .unwrap();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["scc ", " time: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"6"),
                            ::core::fmt::ArgumentV1::new_debug(&self.scc6_duration),
                        ],
                    ),
                )
                .unwrap();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["scc ", " time: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"7"),
                            ::core::fmt::ArgumentV1::new_debug(&self.scc7_duration),
                        ],
                    ),
                )
                .unwrap();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["scc ", " time: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"8"),
                            ::core::fmt::ArgumentV1::new_debug(&self.scc8_duration),
                        ],
                    ),
                )
                .unwrap();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["scc ", " time: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"9"),
                            ::core::fmt::ArgumentV1::new_debug(&self.scc9_duration),
                        ],
                    ),
                )
                .unwrap();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["scc ", " time: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"10"),
                            ::core::fmt::ArgumentV1::new_debug(&self.scc10_duration),
                        ],
                    ),
                )
                .unwrap();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["scc ", " time: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"11"),
                            ::core::fmt::ArgumentV1::new_debug(&self.scc11_duration),
                        ],
                    ),
                )
                .unwrap();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["scc ", " time: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"12"),
                            ::core::fmt::ArgumentV1::new_debug(&self.scc12_duration),
                        ],
                    ),
                )
                .unwrap();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["scc ", " time: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"13"),
                            ::core::fmt::ArgumentV1::new_debug(&self.scc13_duration),
                        ],
                    ),
                )
                .unwrap();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["scc ", " time: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"14"),
                            ::core::fmt::ArgumentV1::new_debug(&self.scc14_duration),
                        ],
                    ),
                )
                .unwrap();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["scc ", " time: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"15"),
                            ::core::fmt::ArgumentV1::new_debug(&self.scc15_duration),
                        ],
                    ),
                )
                .unwrap();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["scc ", " time: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"16"),
                            ::core::fmt::ArgumentV1::new_debug(&self.scc16_duration),
                        ],
                    ),
                )
                .unwrap();
            res
        }
    }
    impl Default for EGraphMatch {
        fn default() -> Self {
            let mut _self = EGraphMatch {
                do_match_input: Default::default(),
                do_match_input_indices_: Default::default(),
                do_match_input_indices_0: Default::default(),
                matched_eclass: Default::default(),
                matched_eclass_indices_0: Default::default(),
                calc_expr_3_left: Default::default(),
                calc_expr_3_left_indices_0_1: Default::default(),
                calc_expr_3_left_indices_0_1_2: Default::default(),
                calc_expr_3_left_indices_: Default::default(),
                e_node: Default::default(),
                e_node_indices_0: Default::default(),
                e_node_indices_: Default::default(),
                matched_pattern_var: Default::default(),
                matched_pattern_var_indices_0_1: Default::default(),
                var: Default::default(),
                var_indices_: Default::default(),
                var_indices_0_1: Default::default(),
                num: Default::default(),
                num_indices_0_1: Default::default(),
                num_indices_: Default::default(),
                calc_expr_3_right: Default::default(),
                calc_expr_3_right_indices_: Default::default(),
                calc_expr_3_right_indices_0_1: Default::default(),
                calc_expr_3_right_indices_0_1_2: Default::default(),
                root: Default::default(),
                root_indices_: Default::default(),
                root_indices_0: Default::default(),
                root_indices_0_1: Default::default(),
                e_node_match: Default::default(),
                e_node_match_indices_: Default::default(),
                e_node_match_indices_0_1: Default::default(),
                e_node_match_indices_0: Default::default(),
                do_match: Default::default(),
                do_match_indices_0_1: Default::default(),
                do_match_indices_: Default::default(),
                scc0_duration: std::time::Duration::ZERO,
                scc1_duration: std::time::Duration::ZERO,
                scc2_duration: std::time::Duration::ZERO,
                scc3_duration: std::time::Duration::ZERO,
                scc4_duration: std::time::Duration::ZERO,
                scc5_duration: std::time::Duration::ZERO,
                scc6_duration: std::time::Duration::ZERO,
                scc7_duration: std::time::Duration::ZERO,
                scc8_duration: std::time::Duration::ZERO,
                scc9_duration: std::time::Duration::ZERO,
                scc10_duration: std::time::Duration::ZERO,
                scc11_duration: std::time::Duration::ZERO,
                scc12_duration: std::time::Duration::ZERO,
                scc13_duration: std::time::Duration::ZERO,
                scc14_duration: std::time::Duration::ZERO,
                scc15_duration: std::time::Duration::ZERO,
                scc16_duration: std::time::Duration::ZERO,
            };
            _self
        }
    }
    pub fn e_match(
        g: &EGraphData,
        pattern: &Rc<PatternExpr>,
    ) -> Vec<(BTreeSet<ENodeId>,)> {
        let mut matched = EGraphMatch::default();
        matched.root = g.root.clone();
        matched.calc_expr_3_left = g.calc_expr_3_left.clone();
        matched.calc_expr_3_right = g.calc_expr_3_right.clone();
        matched.num = g.num.clone();
        matched.var = g.var.clone();
        matched
            .do_match_input = <[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([(pattern.clone(),)]),
        );
        matched.run();
        matched.matched_eclass
    }
}
mod erewrite {
    use std::collections::BTreeSet;
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
    pub struct EGraphRewrite {
        pub var: Vec<(ENodeId, Sym)>,
        #[allow(non_snake_case)]
        pub var_indices_: ascent::internal::RelIndexType<()>,
        #[allow(non_snake_case)]
        pub var_indices_0_1: ascent::internal::RelFullIndexType<(ENodeId, Sym)>,
        pub do_add_new_expr: Vec<(Rc<PatternExpr>,)>,
        #[allow(non_snake_case)]
        pub do_add_new_expr_indices_0: ascent::internal::RelFullIndexType<
            (Rc<PatternExpr>,),
        >,
        #[allow(non_snake_case)]
        pub do_add_new_expr_indices_: ascent::internal::RelIndexType<()>,
        pub assign_new_expr_id: Vec<(Rc<PatternExpr>, ENodeId)>,
        #[allow(non_snake_case)]
        pub assign_new_expr_id_indices_: ascent::internal::RelIndexType<()>,
        #[allow(non_snake_case)]
        pub assign_new_expr_id_indices_0: ascent::internal::RelIndexType<
            (Rc<PatternExpr>,),
        >,
        #[allow(non_snake_case)]
        pub assign_new_expr_id_indices_0_1: ascent::internal::RelFullIndexType<
            (Rc<PatternExpr>, ENodeId),
        >,
        pub rewrite_rule: Vec<(Sym,)>,
        #[allow(non_snake_case)]
        pub rewrite_rule_indices_0: ascent::internal::RelFullIndexType<(Sym,)>,
        pub do_union_pattern: Vec<(Rc<PatternExpr>, ENodeId)>,
        #[allow(non_snake_case)]
        pub do_union_pattern_indices_0: ascent::internal::RelIndexType<
            (Rc<PatternExpr>,),
        >,
        #[allow(non_snake_case)]
        pub do_union_pattern_indices_0_1: ascent::internal::RelFullIndexType<
            (Rc<PatternExpr>, ENodeId),
        >,
        #[allow(non_snake_case)]
        pub do_union_pattern_indices_: ascent::internal::RelIndexType<()>,
        pub new_expr: Vec<(Rc<PatternExpr>,)>,
        #[allow(non_snake_case)]
        pub new_expr_indices_0: ascent::internal::RelFullIndexType<(Rc<PatternExpr>,)>,
        pub root: Vec<(ENodeId, Set<ENodeId>)>,
        #[allow(non_snake_case)]
        pub root_indices_0: ascent::internal::LatticeIndexType<(ENodeId,)>,
        #[allow(non_snake_case)]
        pub root_indices_: ascent::internal::LatticeIndexType<()>,
        #[allow(non_snake_case)]
        pub root_indices_0_1: ascent::internal::RelFullIndexType<
            (ENodeId, Set<ENodeId>),
        >,
        pub e_node_match: Vec<(Rc<PatternExpr>, ENodeId)>,
        #[allow(non_snake_case)]
        pub e_node_match_indices_0_1: ascent::internal::RelFullIndexType<
            (Rc<PatternExpr>, ENodeId),
        >,
        #[allow(non_snake_case)]
        pub e_node_match_indices_0: ascent::internal::RelIndexType<(Rc<PatternExpr>,)>,
        #[allow(non_snake_case)]
        pub e_node_match_indices_: ascent::internal::RelIndexType<()>,
        pub calc_expr_3_left: Vec<(ENodeId, Sym, Set<ENodeId>)>,
        #[allow(non_snake_case)]
        pub calc_expr_3_left_indices_: ascent::internal::LatticeIndexType<()>,
        #[allow(non_snake_case)]
        pub calc_expr_3_left_indices_0_1: ascent::internal::LatticeIndexType<
            (ENodeId, Sym),
        >,
        #[allow(non_snake_case)]
        pub calc_expr_3_left_indices_0_1_2: ascent::internal::RelFullIndexType<
            (ENodeId, Sym, Set<ENodeId>),
        >,
        #[allow(non_snake_case)]
        pub calc_expr_3_left_indices_0: ascent::internal::LatticeIndexType<(ENodeId,)>,
        #[allow(non_snake_case)]
        pub calc_expr_3_left_indices_1: ascent::internal::LatticeIndexType<(Sym,)>,
        pub do_uinon_id: Vec<(ENodeId, ENodeId)>,
        #[allow(non_snake_case)]
        pub do_uinon_id_indices_0_1: ascent::internal::RelFullIndexType<
            (ENodeId, ENodeId),
        >,
        #[allow(non_snake_case)]
        pub do_uinon_id_indices_: ascent::internal::RelIndexType<()>,
        pub num: Vec<(ENodeId, i32)>,
        #[allow(non_snake_case)]
        pub num_indices_: ascent::internal::RelIndexType<()>,
        #[allow(non_snake_case)]
        pub num_indices_0_1: ascent::internal::RelFullIndexType<(ENodeId, i32)>,
        pub do_match: Vec<(Rc<PatternExpr>, ENodeId)>,
        #[allow(non_snake_case)]
        pub do_match_indices_: ascent::internal::RelIndexType<()>,
        #[allow(non_snake_case)]
        pub do_match_indices_0_1: ascent::internal::RelFullIndexType<
            (Rc<PatternExpr>, ENodeId),
        >,
        pub e_node: Vec<(ENodeId,)>,
        #[allow(non_snake_case)]
        pub e_node_indices_: ascent::internal::RelIndexType<()>,
        #[allow(non_snake_case)]
        pub e_node_indices_0: ascent::internal::RelFullIndexType<(ENodeId,)>,
        pub node_need_merge: Vec<(BTreeSet<ENodeId>, Set<ENodeId>)>,
        #[allow(non_snake_case)]
        pub node_need_merge_indices_0: ascent::internal::LatticeIndexType<
            (BTreeSet<ENodeId>,),
        >,
        #[allow(non_snake_case)]
        pub node_need_merge_indices_0_1: ascent::internal::RelFullIndexType<
            (BTreeSet<ENodeId>, Set<ENodeId>),
        >,
        pub calc_expr_3_right: Vec<(ENodeId, Sym, Set<ENodeId>)>,
        #[allow(non_snake_case)]
        pub calc_expr_3_right_indices_0: ascent::internal::LatticeIndexType<(ENodeId,)>,
        #[allow(non_snake_case)]
        pub calc_expr_3_right_indices_0_1: ascent::internal::LatticeIndexType<
            (ENodeId, Sym),
        >,
        #[allow(non_snake_case)]
        pub calc_expr_3_right_indices_0_1_2: ascent::internal::RelFullIndexType<
            (ENodeId, Sym, Set<ENodeId>),
        >,
        #[allow(non_snake_case)]
        pub calc_expr_3_right_indices_: ascent::internal::LatticeIndexType<()>,
        pub scc0_duration: std::time::Duration,
        pub scc1_duration: std::time::Duration,
        pub scc2_duration: std::time::Duration,
        pub scc3_duration: std::time::Duration,
        pub scc4_duration: std::time::Duration,
        pub scc5_duration: std::time::Duration,
    }
    impl EGraphRewrite {
        #[allow(unused_imports)]
        ///Runs the Ascent program to a fixed point.
        pub fn run(&mut self) {
            use core::cmp::PartialEq;
            use ascent::internal::RelIndexRead;
            use ascent::internal::RelIndexReadAll;
            self.update_indices_priv();
            let _self = self;
            ascent::internal::comment("scc 0");
            {
                let _scc_start_time = ::ascent::internal::Instant::now();
                #[allow(non_snake_case)]
                let e_node_match_indices__delta: &mut ascent::internal::RelIndexType<
                    (),
                > = &mut _self.e_node_match_indices_;
                #[allow(non_snake_case)]
                let mut e_node_match_indices__total: ascent::internal::RelIndexType<
                    (),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut e_node_match_indices__new: ascent::internal::RelIndexType<()> = Default::default();
                #[allow(non_snake_case)]
                let e_node_match_indices_0_delta: &mut ascent::internal::RelIndexType<
                    (Rc<PatternExpr>,),
                > = &mut _self.e_node_match_indices_0;
                #[allow(non_snake_case)]
                let mut e_node_match_indices_0_total: ascent::internal::RelIndexType<
                    (Rc<PatternExpr>,),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut e_node_match_indices_0_new: ascent::internal::RelIndexType<
                    (Rc<PatternExpr>,),
                > = Default::default();
                #[allow(non_snake_case)]
                let e_node_match_indices_0_1_delta: &mut ascent::internal::RelFullIndexType<
                    (Rc<PatternExpr>, ENodeId),
                > = &mut _self.e_node_match_indices_0_1;
                #[allow(non_snake_case)]
                let mut e_node_match_indices_0_1_total: ascent::internal::RelFullIndexType<
                    (Rc<PatternExpr>, ENodeId),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut e_node_match_indices_0_1_new: ascent::internal::RelFullIndexType<
                    (Rc<PatternExpr>, ENodeId),
                > = Default::default();
                #[allow(non_snake_case)]
                let assign_new_expr_id_indices__total: &mut ascent::internal::RelIndexType<
                    (),
                > = &mut _self.assign_new_expr_id_indices_;
                #[allow(unused_assignments, unused_variables)]
                {
                    let mut __changed = false;
                    ascent::internal::comment(
                        "e_node_match <-- assign_new_expr_id_indices__total",
                    );
                    if let Some(__matching)
                        = assign_new_expr_id_indices__total.index_get(&())
                    {
                        for __ind in __matching {
                            let __row = &_self.assign_new_expr_id[__ind].clone();
                            let pat = &__row.0;
                            let id = &__row.1;
                            let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                ascent::internal::Convert::convert(pat),
                                ascent::internal::Convert::convert(id),
                            );
                            let __new_row_ind = _self.e_node_match.len();
                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                &e_node_match_indices_0_1_total,
                                &__new_row,
                            )
                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                    e_node_match_indices_0_1_delta,
                                    &__new_row,
                                )
                                && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                    &mut e_node_match_indices_0_1_new,
                                    &__new_row,
                                    __new_row_ind,
                                )
                            {
                                ::ascent::internal::RelIndexWrite::index_insert(
                                    &mut e_node_match_indices__new,
                                    (),
                                    __new_row_ind,
                                );
                                ::ascent::internal::RelIndexWrite::index_insert(
                                    &mut e_node_match_indices_0_new,
                                    (__new_row.0.clone(),),
                                    __new_row_ind,
                                );
                                _self.e_node_match.push(__new_row);
                                __changed = true;
                            }
                        }
                    }
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_match_indices__delta,
                        &mut e_node_match_indices__total,
                    );
                    std::mem::swap(
                        &mut e_node_match_indices__new,
                        e_node_match_indices__delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_match_indices_0_delta,
                        &mut e_node_match_indices_0_total,
                    );
                    std::mem::swap(
                        &mut e_node_match_indices_0_new,
                        e_node_match_indices_0_delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_match_indices_0_1_delta,
                        &mut e_node_match_indices_0_1_total,
                    );
                    std::mem::swap(
                        &mut e_node_match_indices_0_1_new,
                        e_node_match_indices_0_1_delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_match_indices__delta,
                        &mut e_node_match_indices__total,
                    );
                    std::mem::swap(
                        &mut e_node_match_indices__new,
                        e_node_match_indices__delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_match_indices_0_delta,
                        &mut e_node_match_indices_0_total,
                    );
                    std::mem::swap(
                        &mut e_node_match_indices_0_new,
                        e_node_match_indices_0_delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_match_indices_0_1_delta,
                        &mut e_node_match_indices_0_1_total,
                    );
                    std::mem::swap(
                        &mut e_node_match_indices_0_1_new,
                        e_node_match_indices_0_1_delta,
                    );
                }
                _self.e_node_match_indices_ = e_node_match_indices__total;
                _self.e_node_match_indices_0 = e_node_match_indices_0_total;
                _self.e_node_match_indices_0_1 = e_node_match_indices_0_1_total;
                _self.scc0_duration += _scc_start_time.elapsed();
            }
            ascent::internal::comment("scc 1");
            {
                let _scc_start_time = ::ascent::internal::Instant::now();
                #[allow(non_snake_case)]
                let do_match_indices__delta: &mut ascent::internal::RelIndexType<()> = &mut _self
                    .do_match_indices_;
                #[allow(non_snake_case)]
                let mut do_match_indices__total: ascent::internal::RelIndexType<()> = Default::default();
                #[allow(non_snake_case)]
                let mut do_match_indices__new: ascent::internal::RelIndexType<()> = Default::default();
                #[allow(non_snake_case)]
                let do_match_indices_0_1_delta: &mut ascent::internal::RelFullIndexType<
                    (Rc<PatternExpr>, ENodeId),
                > = &mut _self.do_match_indices_0_1;
                #[allow(non_snake_case)]
                let mut do_match_indices_0_1_total: ascent::internal::RelFullIndexType<
                    (Rc<PatternExpr>, ENodeId),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut do_match_indices_0_1_new: ascent::internal::RelFullIndexType<
                    (Rc<PatternExpr>, ENodeId),
                > = Default::default();
                #[allow(non_snake_case)]
                let num_indices_0_1_delta: &mut ascent::internal::RelFullIndexType<
                    (ENodeId, i32),
                > = &mut _self.num_indices_0_1;
                #[allow(non_snake_case)]
                let mut num_indices_0_1_total: ascent::internal::RelFullIndexType<
                    (ENodeId, i32),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut num_indices_0_1_new: ascent::internal::RelFullIndexType<
                    (ENodeId, i32),
                > = Default::default();
                #[allow(non_snake_case)]
                let num_indices__delta: &mut ascent::internal::RelIndexType<()> = &mut _self
                    .num_indices_;
                #[allow(non_snake_case)]
                let mut num_indices__total: ascent::internal::RelIndexType<()> = Default::default();
                #[allow(non_snake_case)]
                let mut num_indices__new: ascent::internal::RelIndexType<()> = Default::default();
                #[allow(non_snake_case)]
                let root_indices_0_1_delta: &mut ascent::internal::RelFullIndexType<
                    (ENodeId, Set<ENodeId>),
                > = &mut _self.root_indices_0_1;
                #[allow(non_snake_case)]
                let mut root_indices_0_1_total: ascent::internal::RelFullIndexType<
                    (ENodeId, Set<ENodeId>),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut root_indices_0_1_new: ascent::internal::RelFullIndexType<
                    (ENodeId, Set<ENodeId>),
                > = Default::default();
                #[allow(non_snake_case)]
                let root_indices_0_delta: &mut ascent::internal::LatticeIndexType<
                    (ENodeId,),
                > = &mut _self.root_indices_0;
                #[allow(non_snake_case)]
                let mut root_indices_0_total: ascent::internal::LatticeIndexType<
                    (ENodeId,),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut root_indices_0_new: ascent::internal::LatticeIndexType<
                    (ENodeId,),
                > = Default::default();
                #[allow(non_snake_case)]
                let root_indices__delta: &mut ascent::internal::LatticeIndexType<()> = &mut _self
                    .root_indices_;
                #[allow(non_snake_case)]
                let mut root_indices__total: ascent::internal::LatticeIndexType<()> = Default::default();
                #[allow(non_snake_case)]
                let mut root_indices__new: ascent::internal::LatticeIndexType<()> = Default::default();
                #[allow(non_snake_case)]
                let e_node_match_indices_0_1_delta: &mut ascent::internal::RelFullIndexType<
                    (Rc<PatternExpr>, ENodeId),
                > = &mut _self.e_node_match_indices_0_1;
                #[allow(non_snake_case)]
                let mut e_node_match_indices_0_1_total: ascent::internal::RelFullIndexType<
                    (Rc<PatternExpr>, ENodeId),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut e_node_match_indices_0_1_new: ascent::internal::RelFullIndexType<
                    (Rc<PatternExpr>, ENodeId),
                > = Default::default();
                #[allow(non_snake_case)]
                let e_node_match_indices__delta: &mut ascent::internal::RelIndexType<
                    (),
                > = &mut _self.e_node_match_indices_;
                #[allow(non_snake_case)]
                let mut e_node_match_indices__total: ascent::internal::RelIndexType<
                    (),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut e_node_match_indices__new: ascent::internal::RelIndexType<()> = Default::default();
                #[allow(non_snake_case)]
                let e_node_match_indices_0_delta: &mut ascent::internal::RelIndexType<
                    (Rc<PatternExpr>,),
                > = &mut _self.e_node_match_indices_0;
                #[allow(non_snake_case)]
                let mut e_node_match_indices_0_total: ascent::internal::RelIndexType<
                    (Rc<PatternExpr>,),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut e_node_match_indices_0_new: ascent::internal::RelIndexType<
                    (Rc<PatternExpr>,),
                > = Default::default();
                #[allow(non_snake_case)]
                let calc_expr_3_right_indices__delta: &mut ascent::internal::LatticeIndexType<
                    (),
                > = &mut _self.calc_expr_3_right_indices_;
                #[allow(non_snake_case)]
                let mut calc_expr_3_right_indices__total: ascent::internal::LatticeIndexType<
                    (),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut calc_expr_3_right_indices__new: ascent::internal::LatticeIndexType<
                    (),
                > = Default::default();
                #[allow(non_snake_case)]
                let calc_expr_3_right_indices_0_1_delta: &mut ascent::internal::LatticeIndexType<
                    (ENodeId, Sym),
                > = &mut _self.calc_expr_3_right_indices_0_1;
                #[allow(non_snake_case)]
                let mut calc_expr_3_right_indices_0_1_total: ascent::internal::LatticeIndexType<
                    (ENodeId, Sym),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut calc_expr_3_right_indices_0_1_new: ascent::internal::LatticeIndexType<
                    (ENodeId, Sym),
                > = Default::default();
                #[allow(non_snake_case)]
                let calc_expr_3_right_indices_0_delta: &mut ascent::internal::LatticeIndexType<
                    (ENodeId,),
                > = &mut _self.calc_expr_3_right_indices_0;
                #[allow(non_snake_case)]
                let mut calc_expr_3_right_indices_0_total: ascent::internal::LatticeIndexType<
                    (ENodeId,),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut calc_expr_3_right_indices_0_new: ascent::internal::LatticeIndexType<
                    (ENodeId,),
                > = Default::default();
                #[allow(non_snake_case)]
                let calc_expr_3_right_indices_0_1_2_delta: &mut ascent::internal::RelFullIndexType<
                    (ENodeId, Sym, Set<ENodeId>),
                > = &mut _self.calc_expr_3_right_indices_0_1_2;
                #[allow(non_snake_case)]
                let mut calc_expr_3_right_indices_0_1_2_total: ascent::internal::RelFullIndexType<
                    (ENodeId, Sym, Set<ENodeId>),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut calc_expr_3_right_indices_0_1_2_new: ascent::internal::RelFullIndexType<
                    (ENodeId, Sym, Set<ENodeId>),
                > = Default::default();
                #[allow(non_snake_case)]
                let do_union_pattern_indices__delta: &mut ascent::internal::RelIndexType<
                    (),
                > = &mut _self.do_union_pattern_indices_;
                #[allow(non_snake_case)]
                let mut do_union_pattern_indices__total: ascent::internal::RelIndexType<
                    (),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut do_union_pattern_indices__new: ascent::internal::RelIndexType<
                    (),
                > = Default::default();
                #[allow(non_snake_case)]
                let do_union_pattern_indices_0_1_delta: &mut ascent::internal::RelFullIndexType<
                    (Rc<PatternExpr>, ENodeId),
                > = &mut _self.do_union_pattern_indices_0_1;
                #[allow(non_snake_case)]
                let mut do_union_pattern_indices_0_1_total: ascent::internal::RelFullIndexType<
                    (Rc<PatternExpr>, ENodeId),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut do_union_pattern_indices_0_1_new: ascent::internal::RelFullIndexType<
                    (Rc<PatternExpr>, ENodeId),
                > = Default::default();
                #[allow(non_snake_case)]
                let do_union_pattern_indices_0_delta: &mut ascent::internal::RelIndexType<
                    (Rc<PatternExpr>,),
                > = &mut _self.do_union_pattern_indices_0;
                #[allow(non_snake_case)]
                let mut do_union_pattern_indices_0_total: ascent::internal::RelIndexType<
                    (Rc<PatternExpr>,),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut do_union_pattern_indices_0_new: ascent::internal::RelIndexType<
                    (Rc<PatternExpr>,),
                > = Default::default();
                #[allow(non_snake_case)]
                let calc_expr_3_left_indices_0_1_delta: &mut ascent::internal::LatticeIndexType<
                    (ENodeId, Sym),
                > = &mut _self.calc_expr_3_left_indices_0_1;
                #[allow(non_snake_case)]
                let mut calc_expr_3_left_indices_0_1_total: ascent::internal::LatticeIndexType<
                    (ENodeId, Sym),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut calc_expr_3_left_indices_0_1_new: ascent::internal::LatticeIndexType<
                    (ENodeId, Sym),
                > = Default::default();
                #[allow(non_snake_case)]
                let calc_expr_3_left_indices_0_delta: &mut ascent::internal::LatticeIndexType<
                    (ENodeId,),
                > = &mut _self.calc_expr_3_left_indices_0;
                #[allow(non_snake_case)]
                let mut calc_expr_3_left_indices_0_total: ascent::internal::LatticeIndexType<
                    (ENodeId,),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut calc_expr_3_left_indices_0_new: ascent::internal::LatticeIndexType<
                    (ENodeId,),
                > = Default::default();
                #[allow(non_snake_case)]
                let calc_expr_3_left_indices_1_delta: &mut ascent::internal::LatticeIndexType<
                    (Sym,),
                > = &mut _self.calc_expr_3_left_indices_1;
                #[allow(non_snake_case)]
                let mut calc_expr_3_left_indices_1_total: ascent::internal::LatticeIndexType<
                    (Sym,),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut calc_expr_3_left_indices_1_new: ascent::internal::LatticeIndexType<
                    (Sym,),
                > = Default::default();
                #[allow(non_snake_case)]
                let calc_expr_3_left_indices_0_1_2_delta: &mut ascent::internal::RelFullIndexType<
                    (ENodeId, Sym, Set<ENodeId>),
                > = &mut _self.calc_expr_3_left_indices_0_1_2;
                #[allow(non_snake_case)]
                let mut calc_expr_3_left_indices_0_1_2_total: ascent::internal::RelFullIndexType<
                    (ENodeId, Sym, Set<ENodeId>),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut calc_expr_3_left_indices_0_1_2_new: ascent::internal::RelFullIndexType<
                    (ENodeId, Sym, Set<ENodeId>),
                > = Default::default();
                #[allow(non_snake_case)]
                let calc_expr_3_left_indices__delta: &mut ascent::internal::LatticeIndexType<
                    (),
                > = &mut _self.calc_expr_3_left_indices_;
                #[allow(non_snake_case)]
                let mut calc_expr_3_left_indices__total: ascent::internal::LatticeIndexType<
                    (),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut calc_expr_3_left_indices__new: ascent::internal::LatticeIndexType<
                    (),
                > = Default::default();
                #[allow(non_snake_case)]
                let var_indices__delta: &mut ascent::internal::RelIndexType<()> = &mut _self
                    .var_indices_;
                #[allow(non_snake_case)]
                let mut var_indices__total: ascent::internal::RelIndexType<()> = Default::default();
                #[allow(non_snake_case)]
                let mut var_indices__new: ascent::internal::RelIndexType<()> = Default::default();
                #[allow(non_snake_case)]
                let var_indices_0_1_delta: &mut ascent::internal::RelFullIndexType<
                    (ENodeId, Sym),
                > = &mut _self.var_indices_0_1;
                #[allow(non_snake_case)]
                let mut var_indices_0_1_total: ascent::internal::RelFullIndexType<
                    (ENodeId, Sym),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut var_indices_0_1_new: ascent::internal::RelFullIndexType<
                    (ENodeId, Sym),
                > = Default::default();
                #[allow(non_snake_case)]
                let e_node_indices_0_delta: &mut ascent::internal::RelFullIndexType<
                    (ENodeId,),
                > = &mut _self.e_node_indices_0;
                #[allow(non_snake_case)]
                let mut e_node_indices_0_total: ascent::internal::RelFullIndexType<
                    (ENodeId,),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut e_node_indices_0_new: ascent::internal::RelFullIndexType<
                    (ENodeId,),
                > = Default::default();
                #[allow(non_snake_case)]
                let e_node_indices__delta: &mut ascent::internal::RelIndexType<()> = &mut _self
                    .e_node_indices_;
                #[allow(non_snake_case)]
                let mut e_node_indices__total: ascent::internal::RelIndexType<()> = Default::default();
                #[allow(non_snake_case)]
                let mut e_node_indices__new: ascent::internal::RelIndexType<()> = Default::default();
                #[allow(non_snake_case)]
                let do_uinon_id_indices__delta: &mut ascent::internal::RelIndexType<
                    (),
                > = &mut _self.do_uinon_id_indices_;
                #[allow(non_snake_case)]
                let mut do_uinon_id_indices__total: ascent::internal::RelIndexType<()> = Default::default();
                #[allow(non_snake_case)]
                let mut do_uinon_id_indices__new: ascent::internal::RelIndexType<()> = Default::default();
                #[allow(non_snake_case)]
                let do_uinon_id_indices_0_1_delta: &mut ascent::internal::RelFullIndexType<
                    (ENodeId, ENodeId),
                > = &mut _self.do_uinon_id_indices_0_1;
                #[allow(non_snake_case)]
                let mut do_uinon_id_indices_0_1_total: ascent::internal::RelFullIndexType<
                    (ENodeId, ENodeId),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut do_uinon_id_indices_0_1_new: ascent::internal::RelFullIndexType<
                    (ENodeId, ENodeId),
                > = Default::default();
                #[allow(non_snake_case)]
                let do_add_new_expr_indices_0_delta: &mut ascent::internal::RelFullIndexType<
                    (Rc<PatternExpr>,),
                > = &mut _self.do_add_new_expr_indices_0;
                #[allow(non_snake_case)]
                let mut do_add_new_expr_indices_0_total: ascent::internal::RelFullIndexType<
                    (Rc<PatternExpr>,),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut do_add_new_expr_indices_0_new: ascent::internal::RelFullIndexType<
                    (Rc<PatternExpr>,),
                > = Default::default();
                #[allow(non_snake_case)]
                let do_add_new_expr_indices__delta: &mut ascent::internal::RelIndexType<
                    (),
                > = &mut _self.do_add_new_expr_indices_;
                #[allow(non_snake_case)]
                let mut do_add_new_expr_indices__total: ascent::internal::RelIndexType<
                    (),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut do_add_new_expr_indices__new: ascent::internal::RelIndexType<
                    (),
                > = Default::default();
                #[allow(non_snake_case)]
                let rewrite_rule_indices_0_total: &mut ascent::internal::RelFullIndexType<
                    (Sym,),
                > = &mut _self.rewrite_rule_indices_0;
                #[allow(non_snake_case)]
                let assign_new_expr_id_indices_0_total: &mut ascent::internal::RelIndexType<
                    (Rc<PatternExpr>,),
                > = &mut _self.assign_new_expr_id_indices_0;
                #[allow(unused_assignments, unused_variables)]
                loop {
                    let mut __changed = false;
                    ascent::internal::comment(
                        "calc_expr_3_left <-- calc_expr_3_left_indices__delta, do_uinon_id_indices__total+delta, if ⋯",
                    );
                    if let Some(__matching)
                        = calc_expr_3_left_indices__delta.index_get(&())
                    {
                        for __ind in __matching {
                            let __row = &_self.calc_expr_3_left[__ind].clone();
                            let e_id = &__row.0;
                            let op = &__row.1;
                            let eq_set = &__row.2;
                            if let Some(__matching)
                                = ascent::internal::RelIndexCombined::new(
                                        &do_uinon_id_indices__total,
                                        do_uinon_id_indices__delta,
                                    )
                                    .index_get(&())
                            {
                                for __ind in __matching {
                                    let __row = &_self.do_uinon_id[__ind].clone();
                                    let old_id = &__row.0;
                                    let node_to_union = &__row.1;
                                    if eq_set.contains(old_id) {
                                        let __new_row: (ENodeId, Sym, Set<ENodeId>) = (
                                            ascent::internal::Convert::convert(e_id),
                                            ascent::internal::Convert::convert(op),
                                            Set::singleton(*node_to_union),
                                        );
                                        let __lattice_key = (
                                            __new_row.0.clone(),
                                            __new_row.1.clone(),
                                        );
                                        if let Some(__existing_ind)
                                            = calc_expr_3_left_indices_0_1_new
                                                .get(&__lattice_key)
                                                .or_else(|| {
                                                    calc_expr_3_left_indices_0_1_delta.get(&__lattice_key)
                                                })
                                                .or_else(|| {
                                                    calc_expr_3_left_indices_0_1_total.get(&__lattice_key)
                                                })
                                        {
                                            let __existing_ind = *__existing_ind.iter().next().unwrap();
                                            let __lat_changed = ::ascent::Lattice::join_mut(
                                                &mut _self.calc_expr_3_left[__existing_ind].2,
                                                __new_row.2.clone(),
                                            );
                                            if __lat_changed {
                                                let __new_row_ind = __existing_ind;
                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                    &mut calc_expr_3_left_indices_0_1_new,
                                                    (__new_row.0.clone(), __new_row.1.clone()),
                                                    __new_row_ind,
                                                );
                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                    &mut calc_expr_3_left_indices_0_new,
                                                    (__new_row.0.clone(),),
                                                    __new_row_ind,
                                                );
                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                    &mut calc_expr_3_left_indices_1_new,
                                                    (__new_row.1.clone(),),
                                                    __new_row_ind,
                                                );
                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                    &mut calc_expr_3_left_indices__new,
                                                    (),
                                                    __new_row_ind,
                                                );
                                                __changed = true;
                                            }
                                        } else {
                                            let __new_row_ind = _self.calc_expr_3_left.len();
                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                &mut calc_expr_3_left_indices_0_1_new,
                                                (__new_row.0.clone(), __new_row.1.clone()),
                                                __new_row_ind,
                                            );
                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                &mut calc_expr_3_left_indices_0_new,
                                                (__new_row.0.clone(),),
                                                __new_row_ind,
                                            );
                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                &mut calc_expr_3_left_indices_1_new,
                                                (__new_row.1.clone(),),
                                                __new_row_ind,
                                            );
                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                &mut calc_expr_3_left_indices__new,
                                                (),
                                                __new_row_ind,
                                            );
                                            _self.calc_expr_3_left.push(__new_row);
                                            __changed = true;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment(
                        "calc_expr_3_left <-- calc_expr_3_left_indices__total, do_uinon_id_indices__delta, if ⋯",
                    );
                    if let Some(__matching)
                        = calc_expr_3_left_indices__total.index_get(&())
                    {
                        for __ind in __matching {
                            let __row = &_self.calc_expr_3_left[__ind].clone();
                            let e_id = &__row.0;
                            let op = &__row.1;
                            let eq_set = &__row.2;
                            if let Some(__matching)
                                = do_uinon_id_indices__delta.index_get(&())
                            {
                                for __ind in __matching {
                                    let __row = &_self.do_uinon_id[__ind].clone();
                                    let old_id = &__row.0;
                                    let node_to_union = &__row.1;
                                    if eq_set.contains(old_id) {
                                        let __new_row: (ENodeId, Sym, Set<ENodeId>) = (
                                            ascent::internal::Convert::convert(e_id),
                                            ascent::internal::Convert::convert(op),
                                            Set::singleton(*node_to_union),
                                        );
                                        let __lattice_key = (
                                            __new_row.0.clone(),
                                            __new_row.1.clone(),
                                        );
                                        if let Some(__existing_ind)
                                            = calc_expr_3_left_indices_0_1_new
                                                .get(&__lattice_key)
                                                .or_else(|| {
                                                    calc_expr_3_left_indices_0_1_delta.get(&__lattice_key)
                                                })
                                                .or_else(|| {
                                                    calc_expr_3_left_indices_0_1_total.get(&__lattice_key)
                                                })
                                        {
                                            let __existing_ind = *__existing_ind.iter().next().unwrap();
                                            let __lat_changed = ::ascent::Lattice::join_mut(
                                                &mut _self.calc_expr_3_left[__existing_ind].2,
                                                __new_row.2.clone(),
                                            );
                                            if __lat_changed {
                                                let __new_row_ind = __existing_ind;
                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                    &mut calc_expr_3_left_indices_0_1_new,
                                                    (__new_row.0.clone(), __new_row.1.clone()),
                                                    __new_row_ind,
                                                );
                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                    &mut calc_expr_3_left_indices_0_new,
                                                    (__new_row.0.clone(),),
                                                    __new_row_ind,
                                                );
                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                    &mut calc_expr_3_left_indices_1_new,
                                                    (__new_row.1.clone(),),
                                                    __new_row_ind,
                                                );
                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                    &mut calc_expr_3_left_indices__new,
                                                    (),
                                                    __new_row_ind,
                                                );
                                                __changed = true;
                                            }
                                        } else {
                                            let __new_row_ind = _self.calc_expr_3_left.len();
                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                &mut calc_expr_3_left_indices_0_1_new,
                                                (__new_row.0.clone(), __new_row.1.clone()),
                                                __new_row_ind,
                                            );
                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                &mut calc_expr_3_left_indices_0_new,
                                                (__new_row.0.clone(),),
                                                __new_row_ind,
                                            );
                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                &mut calc_expr_3_left_indices_1_new,
                                                (__new_row.1.clone(),),
                                                __new_row_ind,
                                            );
                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                &mut calc_expr_3_left_indices__new,
                                                (),
                                                __new_row_ind,
                                            );
                                            _self.calc_expr_3_left.push(__new_row);
                                            __changed = true;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment(
                        "e_node <-- calc_expr_3_left_indices__delta",
                    );
                    if let Some(__matching)
                        = calc_expr_3_left_indices__delta.index_get(&())
                    {
                        for __ind in __matching {
                            let __row = &_self.calc_expr_3_left[__ind].clone();
                            let e_id = &__row.0;
                            let __1 = &__row.1;
                            let __2 = &__row.2;
                            let __new_row: (ENodeId,) = (
                                ascent::internal::Convert::convert(e_id),
                            );
                            let __new_row_ind = _self.e_node.len();
                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                &e_node_indices_0_total,
                                &__new_row,
                            )
                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                    e_node_indices_0_delta,
                                    &__new_row,
                                )
                                && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                    &mut e_node_indices_0_new,
                                    &__new_row,
                                    __new_row_ind,
                                )
                            {
                                ::ascent::internal::RelIndexWrite::index_insert(
                                    &mut e_node_indices__new,
                                    (),
                                    __new_row_ind,
                                );
                                _self.e_node.push(__new_row);
                                __changed = true;
                            }
                        }
                    }
                    ascent::internal::comment(
                        "calc_expr_3_left, calc_expr_3_right <-- do_add_new_expr_indices_0_delta, assign_new_expr_id_indices_0_total, if let ⋯, e_node_match_indices_0_total+delta, e_node_match_indices_0_total+delta [SIMPLE JOIN]",
                    );
                    if do_add_new_expr_indices_0_delta.len()
                        <= assign_new_expr_id_indices_0_total.len()
                    {
                        for (__cl1_joined_columns, __cl1_tuple_indices) in do_add_new_expr_indices_0_delta
                            .iter_all()
                        {
                            let pat = &__cl1_joined_columns.0;
                            if let Some(__matching)
                                = assign_new_expr_id_indices_0_total
                                    .index_get(&(pat.clone(),))
                            {
                                for cl1_ind in __cl1_tuple_indices {
                                    let __row = &_self.do_add_new_expr[cl1_ind].clone();
                                    for __ind in __matching.clone() {
                                        let __row = &_self.assign_new_expr_id[__ind].clone();
                                        let new_e_id = &__row.1;
                                        if let Calc(op, lp, rp) = pat.deref() {
                                            if let Some(__matching)
                                                = ascent::internal::RelIndexCombined::new(
                                                        &e_node_match_indices_0_total,
                                                        e_node_match_indices_0_delta,
                                                    )
                                                    .index_get(&(lp.clone(),))
                                            {
                                                for __ind in __matching {
                                                    let __row = &_self.e_node_match[__ind].clone();
                                                    let lp = &__row.0;
                                                    let l_id = &__row.1;
                                                    if let Some(__matching)
                                                        = ascent::internal::RelIndexCombined::new(
                                                                &e_node_match_indices_0_total,
                                                                e_node_match_indices_0_delta,
                                                            )
                                                            .index_get(&(rp.clone(),))
                                                    {
                                                        for __ind in __matching {
                                                            let __row = &_self.e_node_match[__ind].clone();
                                                            let rp = &__row.0;
                                                            let r_id = &__row.1;
                                                            let __new_row: (ENodeId, Sym, Set<ENodeId>) = (
                                                                ascent::internal::Convert::convert(new_e_id),
                                                                ascent::internal::Convert::convert(op),
                                                                Set::singleton(*l_id),
                                                            );
                                                            let __lattice_key = (
                                                                __new_row.0.clone(),
                                                                __new_row.1.clone(),
                                                            );
                                                            if let Some(__existing_ind)
                                                                = calc_expr_3_left_indices_0_1_new
                                                                    .get(&__lattice_key)
                                                                    .or_else(|| {
                                                                        calc_expr_3_left_indices_0_1_delta.get(&__lattice_key)
                                                                    })
                                                                    .or_else(|| {
                                                                        calc_expr_3_left_indices_0_1_total.get(&__lattice_key)
                                                                    })
                                                            {
                                                                let __existing_ind = *__existing_ind.iter().next().unwrap();
                                                                let __lat_changed = ::ascent::Lattice::join_mut(
                                                                    &mut _self.calc_expr_3_left[__existing_ind].2,
                                                                    __new_row.2.clone(),
                                                                );
                                                                if __lat_changed {
                                                                    let __new_row_ind = __existing_ind;
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut calc_expr_3_left_indices_0_1_new,
                                                                        (__new_row.0.clone(), __new_row.1.clone()),
                                                                        __new_row_ind,
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut calc_expr_3_left_indices_0_new,
                                                                        (__new_row.0.clone(),),
                                                                        __new_row_ind,
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut calc_expr_3_left_indices_1_new,
                                                                        (__new_row.1.clone(),),
                                                                        __new_row_ind,
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut calc_expr_3_left_indices__new,
                                                                        (),
                                                                        __new_row_ind,
                                                                    );
                                                                    __changed = true;
                                                                }
                                                            } else {
                                                                let __new_row_ind = _self.calc_expr_3_left.len();
                                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                                    &mut calc_expr_3_left_indices_0_1_new,
                                                                    (__new_row.0.clone(), __new_row.1.clone()),
                                                                    __new_row_ind,
                                                                );
                                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                                    &mut calc_expr_3_left_indices_0_new,
                                                                    (__new_row.0.clone(),),
                                                                    __new_row_ind,
                                                                );
                                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                                    &mut calc_expr_3_left_indices_1_new,
                                                                    (__new_row.1.clone(),),
                                                                    __new_row_ind,
                                                                );
                                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                                    &mut calc_expr_3_left_indices__new,
                                                                    (),
                                                                    __new_row_ind,
                                                                );
                                                                _self.calc_expr_3_left.push(__new_row);
                                                                __changed = true;
                                                            }
                                                            let __new_row: (ENodeId, Sym, Set<ENodeId>) = (
                                                                ascent::internal::Convert::convert(new_e_id),
                                                                ascent::internal::Convert::convert(op),
                                                                Set::singleton(*r_id),
                                                            );
                                                            let __lattice_key = (
                                                                __new_row.0.clone(),
                                                                __new_row.1.clone(),
                                                            );
                                                            if let Some(__existing_ind)
                                                                = calc_expr_3_right_indices_0_1_new
                                                                    .get(&__lattice_key)
                                                                    .or_else(|| {
                                                                        calc_expr_3_right_indices_0_1_delta.get(&__lattice_key)
                                                                    })
                                                                    .or_else(|| {
                                                                        calc_expr_3_right_indices_0_1_total.get(&__lattice_key)
                                                                    })
                                                            {
                                                                let __existing_ind = *__existing_ind.iter().next().unwrap();
                                                                let __lat_changed = ::ascent::Lattice::join_mut(
                                                                    &mut _self.calc_expr_3_right[__existing_ind].2,
                                                                    __new_row.2.clone(),
                                                                );
                                                                if __lat_changed {
                                                                    let __new_row_ind = __existing_ind;
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut calc_expr_3_right_indices__new,
                                                                        (),
                                                                        __new_row_ind,
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut calc_expr_3_right_indices_0_1_new,
                                                                        (__new_row.0.clone(), __new_row.1.clone()),
                                                                        __new_row_ind,
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut calc_expr_3_right_indices_0_new,
                                                                        (__new_row.0.clone(),),
                                                                        __new_row_ind,
                                                                    );
                                                                    __changed = true;
                                                                }
                                                            } else {
                                                                let __new_row_ind = _self.calc_expr_3_right.len();
                                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                                    &mut calc_expr_3_right_indices__new,
                                                                    (),
                                                                    __new_row_ind,
                                                                );
                                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                                    &mut calc_expr_3_right_indices_0_1_new,
                                                                    (__new_row.0.clone(), __new_row.1.clone()),
                                                                    __new_row_ind,
                                                                );
                                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                                    &mut calc_expr_3_right_indices_0_new,
                                                                    (__new_row.0.clone(),),
                                                                    __new_row_ind,
                                                                );
                                                                _self.calc_expr_3_right.push(__new_row);
                                                                __changed = true;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        for (__cl1_joined_columns, __cl1_tuple_indices) in assign_new_expr_id_indices_0_total
                            .iter_all()
                        {
                            let pat = &__cl1_joined_columns.0;
                            if let Some(__matching)
                                = do_add_new_expr_indices_0_delta.index_get(&(pat.clone(),))
                            {
                                for cl1_ind in __cl1_tuple_indices {
                                    let __row = &_self.assign_new_expr_id[cl1_ind].clone();
                                    let new_e_id = &__row.1;
                                    for __ind in __matching.clone() {
                                        let __row = &_self.do_add_new_expr[__ind].clone();
                                        if let Calc(op, lp, rp) = pat.deref() {
                                            if let Some(__matching)
                                                = ascent::internal::RelIndexCombined::new(
                                                        &e_node_match_indices_0_total,
                                                        e_node_match_indices_0_delta,
                                                    )
                                                    .index_get(&(lp.clone(),))
                                            {
                                                for __ind in __matching {
                                                    let __row = &_self.e_node_match[__ind].clone();
                                                    let lp = &__row.0;
                                                    let l_id = &__row.1;
                                                    if let Some(__matching)
                                                        = ascent::internal::RelIndexCombined::new(
                                                                &e_node_match_indices_0_total,
                                                                e_node_match_indices_0_delta,
                                                            )
                                                            .index_get(&(rp.clone(),))
                                                    {
                                                        for __ind in __matching {
                                                            let __row = &_self.e_node_match[__ind].clone();
                                                            let rp = &__row.0;
                                                            let r_id = &__row.1;
                                                            let __new_row: (ENodeId, Sym, Set<ENodeId>) = (
                                                                ascent::internal::Convert::convert(new_e_id),
                                                                ascent::internal::Convert::convert(op),
                                                                Set::singleton(*l_id),
                                                            );
                                                            let __lattice_key = (
                                                                __new_row.0.clone(),
                                                                __new_row.1.clone(),
                                                            );
                                                            if let Some(__existing_ind)
                                                                = calc_expr_3_left_indices_0_1_new
                                                                    .get(&__lattice_key)
                                                                    .or_else(|| {
                                                                        calc_expr_3_left_indices_0_1_delta.get(&__lattice_key)
                                                                    })
                                                                    .or_else(|| {
                                                                        calc_expr_3_left_indices_0_1_total.get(&__lattice_key)
                                                                    })
                                                            {
                                                                let __existing_ind = *__existing_ind.iter().next().unwrap();
                                                                let __lat_changed = ::ascent::Lattice::join_mut(
                                                                    &mut _self.calc_expr_3_left[__existing_ind].2,
                                                                    __new_row.2.clone(),
                                                                );
                                                                if __lat_changed {
                                                                    let __new_row_ind = __existing_ind;
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut calc_expr_3_left_indices_0_1_new,
                                                                        (__new_row.0.clone(), __new_row.1.clone()),
                                                                        __new_row_ind,
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut calc_expr_3_left_indices_0_new,
                                                                        (__new_row.0.clone(),),
                                                                        __new_row_ind,
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut calc_expr_3_left_indices_1_new,
                                                                        (__new_row.1.clone(),),
                                                                        __new_row_ind,
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut calc_expr_3_left_indices__new,
                                                                        (),
                                                                        __new_row_ind,
                                                                    );
                                                                    __changed = true;
                                                                }
                                                            } else {
                                                                let __new_row_ind = _self.calc_expr_3_left.len();
                                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                                    &mut calc_expr_3_left_indices_0_1_new,
                                                                    (__new_row.0.clone(), __new_row.1.clone()),
                                                                    __new_row_ind,
                                                                );
                                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                                    &mut calc_expr_3_left_indices_0_new,
                                                                    (__new_row.0.clone(),),
                                                                    __new_row_ind,
                                                                );
                                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                                    &mut calc_expr_3_left_indices_1_new,
                                                                    (__new_row.1.clone(),),
                                                                    __new_row_ind,
                                                                );
                                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                                    &mut calc_expr_3_left_indices__new,
                                                                    (),
                                                                    __new_row_ind,
                                                                );
                                                                _self.calc_expr_3_left.push(__new_row);
                                                                __changed = true;
                                                            }
                                                            let __new_row: (ENodeId, Sym, Set<ENodeId>) = (
                                                                ascent::internal::Convert::convert(new_e_id),
                                                                ascent::internal::Convert::convert(op),
                                                                Set::singleton(*r_id),
                                                            );
                                                            let __lattice_key = (
                                                                __new_row.0.clone(),
                                                                __new_row.1.clone(),
                                                            );
                                                            if let Some(__existing_ind)
                                                                = calc_expr_3_right_indices_0_1_new
                                                                    .get(&__lattice_key)
                                                                    .or_else(|| {
                                                                        calc_expr_3_right_indices_0_1_delta.get(&__lattice_key)
                                                                    })
                                                                    .or_else(|| {
                                                                        calc_expr_3_right_indices_0_1_total.get(&__lattice_key)
                                                                    })
                                                            {
                                                                let __existing_ind = *__existing_ind.iter().next().unwrap();
                                                                let __lat_changed = ::ascent::Lattice::join_mut(
                                                                    &mut _self.calc_expr_3_right[__existing_ind].2,
                                                                    __new_row.2.clone(),
                                                                );
                                                                if __lat_changed {
                                                                    let __new_row_ind = __existing_ind;
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut calc_expr_3_right_indices__new,
                                                                        (),
                                                                        __new_row_ind,
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut calc_expr_3_right_indices_0_1_new,
                                                                        (__new_row.0.clone(), __new_row.1.clone()),
                                                                        __new_row_ind,
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut calc_expr_3_right_indices_0_new,
                                                                        (__new_row.0.clone(),),
                                                                        __new_row_ind,
                                                                    );
                                                                    __changed = true;
                                                                }
                                                            } else {
                                                                let __new_row_ind = _self.calc_expr_3_right.len();
                                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                                    &mut calc_expr_3_right_indices__new,
                                                                    (),
                                                                    __new_row_ind,
                                                                );
                                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                                    &mut calc_expr_3_right_indices_0_1_new,
                                                                    (__new_row.0.clone(), __new_row.1.clone()),
                                                                    __new_row_ind,
                                                                );
                                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                                    &mut calc_expr_3_right_indices_0_new,
                                                                    (__new_row.0.clone(),),
                                                                    __new_row_ind,
                                                                );
                                                                _self.calc_expr_3_right.push(__new_row);
                                                                __changed = true;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment(
                        "calc_expr_3_left, calc_expr_3_right <-- do_add_new_expr_indices_0_total, assign_new_expr_id_indices_0_total, if let ⋯, e_node_match_indices_0_delta, e_node_match_indices_0_total+delta [SIMPLE JOIN]",
                    );
                    if do_add_new_expr_indices_0_total.len()
                        <= assign_new_expr_id_indices_0_total.len()
                    {
                        for (__cl1_joined_columns, __cl1_tuple_indices) in do_add_new_expr_indices_0_total
                            .iter_all()
                        {
                            let pat = &__cl1_joined_columns.0;
                            if let Some(__matching)
                                = assign_new_expr_id_indices_0_total
                                    .index_get(&(pat.clone(),))
                            {
                                for cl1_ind in __cl1_tuple_indices {
                                    let __row = &_self.do_add_new_expr[cl1_ind].clone();
                                    for __ind in __matching.clone() {
                                        let __row = &_self.assign_new_expr_id[__ind].clone();
                                        let new_e_id = &__row.1;
                                        if let Calc(op, lp, rp) = pat.deref() {
                                            if let Some(__matching)
                                                = e_node_match_indices_0_delta.index_get(&(lp.clone(),))
                                            {
                                                for __ind in __matching {
                                                    let __row = &_self.e_node_match[__ind].clone();
                                                    let lp = &__row.0;
                                                    let l_id = &__row.1;
                                                    if let Some(__matching)
                                                        = ascent::internal::RelIndexCombined::new(
                                                                &e_node_match_indices_0_total,
                                                                e_node_match_indices_0_delta,
                                                            )
                                                            .index_get(&(rp.clone(),))
                                                    {
                                                        for __ind in __matching {
                                                            let __row = &_self.e_node_match[__ind].clone();
                                                            let rp = &__row.0;
                                                            let r_id = &__row.1;
                                                            let __new_row: (ENodeId, Sym, Set<ENodeId>) = (
                                                                ascent::internal::Convert::convert(new_e_id),
                                                                ascent::internal::Convert::convert(op),
                                                                Set::singleton(*l_id),
                                                            );
                                                            let __lattice_key = (
                                                                __new_row.0.clone(),
                                                                __new_row.1.clone(),
                                                            );
                                                            if let Some(__existing_ind)
                                                                = calc_expr_3_left_indices_0_1_new
                                                                    .get(&__lattice_key)
                                                                    .or_else(|| {
                                                                        calc_expr_3_left_indices_0_1_delta.get(&__lattice_key)
                                                                    })
                                                                    .or_else(|| {
                                                                        calc_expr_3_left_indices_0_1_total.get(&__lattice_key)
                                                                    })
                                                            {
                                                                let __existing_ind = *__existing_ind.iter().next().unwrap();
                                                                let __lat_changed = ::ascent::Lattice::join_mut(
                                                                    &mut _self.calc_expr_3_left[__existing_ind].2,
                                                                    __new_row.2.clone(),
                                                                );
                                                                if __lat_changed {
                                                                    let __new_row_ind = __existing_ind;
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut calc_expr_3_left_indices_0_1_new,
                                                                        (__new_row.0.clone(), __new_row.1.clone()),
                                                                        __new_row_ind,
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut calc_expr_3_left_indices_0_new,
                                                                        (__new_row.0.clone(),),
                                                                        __new_row_ind,
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut calc_expr_3_left_indices_1_new,
                                                                        (__new_row.1.clone(),),
                                                                        __new_row_ind,
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut calc_expr_3_left_indices__new,
                                                                        (),
                                                                        __new_row_ind,
                                                                    );
                                                                    __changed = true;
                                                                }
                                                            } else {
                                                                let __new_row_ind = _self.calc_expr_3_left.len();
                                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                                    &mut calc_expr_3_left_indices_0_1_new,
                                                                    (__new_row.0.clone(), __new_row.1.clone()),
                                                                    __new_row_ind,
                                                                );
                                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                                    &mut calc_expr_3_left_indices_0_new,
                                                                    (__new_row.0.clone(),),
                                                                    __new_row_ind,
                                                                );
                                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                                    &mut calc_expr_3_left_indices_1_new,
                                                                    (__new_row.1.clone(),),
                                                                    __new_row_ind,
                                                                );
                                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                                    &mut calc_expr_3_left_indices__new,
                                                                    (),
                                                                    __new_row_ind,
                                                                );
                                                                _self.calc_expr_3_left.push(__new_row);
                                                                __changed = true;
                                                            }
                                                            let __new_row: (ENodeId, Sym, Set<ENodeId>) = (
                                                                ascent::internal::Convert::convert(new_e_id),
                                                                ascent::internal::Convert::convert(op),
                                                                Set::singleton(*r_id),
                                                            );
                                                            let __lattice_key = (
                                                                __new_row.0.clone(),
                                                                __new_row.1.clone(),
                                                            );
                                                            if let Some(__existing_ind)
                                                                = calc_expr_3_right_indices_0_1_new
                                                                    .get(&__lattice_key)
                                                                    .or_else(|| {
                                                                        calc_expr_3_right_indices_0_1_delta.get(&__lattice_key)
                                                                    })
                                                                    .or_else(|| {
                                                                        calc_expr_3_right_indices_0_1_total.get(&__lattice_key)
                                                                    })
                                                            {
                                                                let __existing_ind = *__existing_ind.iter().next().unwrap();
                                                                let __lat_changed = ::ascent::Lattice::join_mut(
                                                                    &mut _self.calc_expr_3_right[__existing_ind].2,
                                                                    __new_row.2.clone(),
                                                                );
                                                                if __lat_changed {
                                                                    let __new_row_ind = __existing_ind;
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut calc_expr_3_right_indices__new,
                                                                        (),
                                                                        __new_row_ind,
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut calc_expr_3_right_indices_0_1_new,
                                                                        (__new_row.0.clone(), __new_row.1.clone()),
                                                                        __new_row_ind,
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut calc_expr_3_right_indices_0_new,
                                                                        (__new_row.0.clone(),),
                                                                        __new_row_ind,
                                                                    );
                                                                    __changed = true;
                                                                }
                                                            } else {
                                                                let __new_row_ind = _self.calc_expr_3_right.len();
                                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                                    &mut calc_expr_3_right_indices__new,
                                                                    (),
                                                                    __new_row_ind,
                                                                );
                                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                                    &mut calc_expr_3_right_indices_0_1_new,
                                                                    (__new_row.0.clone(), __new_row.1.clone()),
                                                                    __new_row_ind,
                                                                );
                                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                                    &mut calc_expr_3_right_indices_0_new,
                                                                    (__new_row.0.clone(),),
                                                                    __new_row_ind,
                                                                );
                                                                _self.calc_expr_3_right.push(__new_row);
                                                                __changed = true;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        for (__cl1_joined_columns, __cl1_tuple_indices) in assign_new_expr_id_indices_0_total
                            .iter_all()
                        {
                            let pat = &__cl1_joined_columns.0;
                            if let Some(__matching)
                                = do_add_new_expr_indices_0_total.index_get(&(pat.clone(),))
                            {
                                for cl1_ind in __cl1_tuple_indices {
                                    let __row = &_self.assign_new_expr_id[cl1_ind].clone();
                                    let new_e_id = &__row.1;
                                    for __ind in __matching.clone() {
                                        let __row = &_self.do_add_new_expr[__ind].clone();
                                        if let Calc(op, lp, rp) = pat.deref() {
                                            if let Some(__matching)
                                                = e_node_match_indices_0_delta.index_get(&(lp.clone(),))
                                            {
                                                for __ind in __matching {
                                                    let __row = &_self.e_node_match[__ind].clone();
                                                    let lp = &__row.0;
                                                    let l_id = &__row.1;
                                                    if let Some(__matching)
                                                        = ascent::internal::RelIndexCombined::new(
                                                                &e_node_match_indices_0_total,
                                                                e_node_match_indices_0_delta,
                                                            )
                                                            .index_get(&(rp.clone(),))
                                                    {
                                                        for __ind in __matching {
                                                            let __row = &_self.e_node_match[__ind].clone();
                                                            let rp = &__row.0;
                                                            let r_id = &__row.1;
                                                            let __new_row: (ENodeId, Sym, Set<ENodeId>) = (
                                                                ascent::internal::Convert::convert(new_e_id),
                                                                ascent::internal::Convert::convert(op),
                                                                Set::singleton(*l_id),
                                                            );
                                                            let __lattice_key = (
                                                                __new_row.0.clone(),
                                                                __new_row.1.clone(),
                                                            );
                                                            if let Some(__existing_ind)
                                                                = calc_expr_3_left_indices_0_1_new
                                                                    .get(&__lattice_key)
                                                                    .or_else(|| {
                                                                        calc_expr_3_left_indices_0_1_delta.get(&__lattice_key)
                                                                    })
                                                                    .or_else(|| {
                                                                        calc_expr_3_left_indices_0_1_total.get(&__lattice_key)
                                                                    })
                                                            {
                                                                let __existing_ind = *__existing_ind.iter().next().unwrap();
                                                                let __lat_changed = ::ascent::Lattice::join_mut(
                                                                    &mut _self.calc_expr_3_left[__existing_ind].2,
                                                                    __new_row.2.clone(),
                                                                );
                                                                if __lat_changed {
                                                                    let __new_row_ind = __existing_ind;
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut calc_expr_3_left_indices_0_1_new,
                                                                        (__new_row.0.clone(), __new_row.1.clone()),
                                                                        __new_row_ind,
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut calc_expr_3_left_indices_0_new,
                                                                        (__new_row.0.clone(),),
                                                                        __new_row_ind,
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut calc_expr_3_left_indices_1_new,
                                                                        (__new_row.1.clone(),),
                                                                        __new_row_ind,
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut calc_expr_3_left_indices__new,
                                                                        (),
                                                                        __new_row_ind,
                                                                    );
                                                                    __changed = true;
                                                                }
                                                            } else {
                                                                let __new_row_ind = _self.calc_expr_3_left.len();
                                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                                    &mut calc_expr_3_left_indices_0_1_new,
                                                                    (__new_row.0.clone(), __new_row.1.clone()),
                                                                    __new_row_ind,
                                                                );
                                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                                    &mut calc_expr_3_left_indices_0_new,
                                                                    (__new_row.0.clone(),),
                                                                    __new_row_ind,
                                                                );
                                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                                    &mut calc_expr_3_left_indices_1_new,
                                                                    (__new_row.1.clone(),),
                                                                    __new_row_ind,
                                                                );
                                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                                    &mut calc_expr_3_left_indices__new,
                                                                    (),
                                                                    __new_row_ind,
                                                                );
                                                                _self.calc_expr_3_left.push(__new_row);
                                                                __changed = true;
                                                            }
                                                            let __new_row: (ENodeId, Sym, Set<ENodeId>) = (
                                                                ascent::internal::Convert::convert(new_e_id),
                                                                ascent::internal::Convert::convert(op),
                                                                Set::singleton(*r_id),
                                                            );
                                                            let __lattice_key = (
                                                                __new_row.0.clone(),
                                                                __new_row.1.clone(),
                                                            );
                                                            if let Some(__existing_ind)
                                                                = calc_expr_3_right_indices_0_1_new
                                                                    .get(&__lattice_key)
                                                                    .or_else(|| {
                                                                        calc_expr_3_right_indices_0_1_delta.get(&__lattice_key)
                                                                    })
                                                                    .or_else(|| {
                                                                        calc_expr_3_right_indices_0_1_total.get(&__lattice_key)
                                                                    })
                                                            {
                                                                let __existing_ind = *__existing_ind.iter().next().unwrap();
                                                                let __lat_changed = ::ascent::Lattice::join_mut(
                                                                    &mut _self.calc_expr_3_right[__existing_ind].2,
                                                                    __new_row.2.clone(),
                                                                );
                                                                if __lat_changed {
                                                                    let __new_row_ind = __existing_ind;
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut calc_expr_3_right_indices__new,
                                                                        (),
                                                                        __new_row_ind,
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut calc_expr_3_right_indices_0_1_new,
                                                                        (__new_row.0.clone(), __new_row.1.clone()),
                                                                        __new_row_ind,
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut calc_expr_3_right_indices_0_new,
                                                                        (__new_row.0.clone(),),
                                                                        __new_row_ind,
                                                                    );
                                                                    __changed = true;
                                                                }
                                                            } else {
                                                                let __new_row_ind = _self.calc_expr_3_right.len();
                                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                                    &mut calc_expr_3_right_indices__new,
                                                                    (),
                                                                    __new_row_ind,
                                                                );
                                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                                    &mut calc_expr_3_right_indices_0_1_new,
                                                                    (__new_row.0.clone(), __new_row.1.clone()),
                                                                    __new_row_ind,
                                                                );
                                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                                    &mut calc_expr_3_right_indices_0_new,
                                                                    (__new_row.0.clone(),),
                                                                    __new_row_ind,
                                                                );
                                                                _self.calc_expr_3_right.push(__new_row);
                                                                __changed = true;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment(
                        "calc_expr_3_left, calc_expr_3_right <-- do_add_new_expr_indices_0_total, assign_new_expr_id_indices_0_total, if let ⋯, e_node_match_indices_0_total, e_node_match_indices_0_delta [SIMPLE JOIN]",
                    );
                    if do_add_new_expr_indices_0_total.len()
                        <= assign_new_expr_id_indices_0_total.len()
                    {
                        for (__cl1_joined_columns, __cl1_tuple_indices) in do_add_new_expr_indices_0_total
                            .iter_all()
                        {
                            let pat = &__cl1_joined_columns.0;
                            if let Some(__matching)
                                = assign_new_expr_id_indices_0_total
                                    .index_get(&(pat.clone(),))
                            {
                                for cl1_ind in __cl1_tuple_indices {
                                    let __row = &_self.do_add_new_expr[cl1_ind].clone();
                                    for __ind in __matching.clone() {
                                        let __row = &_self.assign_new_expr_id[__ind].clone();
                                        let new_e_id = &__row.1;
                                        if let Calc(op, lp, rp) = pat.deref() {
                                            if let Some(__matching)
                                                = e_node_match_indices_0_total.index_get(&(lp.clone(),))
                                            {
                                                for __ind in __matching {
                                                    let __row = &_self.e_node_match[__ind].clone();
                                                    let lp = &__row.0;
                                                    let l_id = &__row.1;
                                                    if let Some(__matching)
                                                        = e_node_match_indices_0_delta.index_get(&(rp.clone(),))
                                                    {
                                                        for __ind in __matching {
                                                            let __row = &_self.e_node_match[__ind].clone();
                                                            let rp = &__row.0;
                                                            let r_id = &__row.1;
                                                            let __new_row: (ENodeId, Sym, Set<ENodeId>) = (
                                                                ascent::internal::Convert::convert(new_e_id),
                                                                ascent::internal::Convert::convert(op),
                                                                Set::singleton(*l_id),
                                                            );
                                                            let __lattice_key = (
                                                                __new_row.0.clone(),
                                                                __new_row.1.clone(),
                                                            );
                                                            if let Some(__existing_ind)
                                                                = calc_expr_3_left_indices_0_1_new
                                                                    .get(&__lattice_key)
                                                                    .or_else(|| {
                                                                        calc_expr_3_left_indices_0_1_delta.get(&__lattice_key)
                                                                    })
                                                                    .or_else(|| {
                                                                        calc_expr_3_left_indices_0_1_total.get(&__lattice_key)
                                                                    })
                                                            {
                                                                let __existing_ind = *__existing_ind.iter().next().unwrap();
                                                                let __lat_changed = ::ascent::Lattice::join_mut(
                                                                    &mut _self.calc_expr_3_left[__existing_ind].2,
                                                                    __new_row.2.clone(),
                                                                );
                                                                if __lat_changed {
                                                                    let __new_row_ind = __existing_ind;
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut calc_expr_3_left_indices_0_1_new,
                                                                        (__new_row.0.clone(), __new_row.1.clone()),
                                                                        __new_row_ind,
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut calc_expr_3_left_indices_0_new,
                                                                        (__new_row.0.clone(),),
                                                                        __new_row_ind,
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut calc_expr_3_left_indices_1_new,
                                                                        (__new_row.1.clone(),),
                                                                        __new_row_ind,
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut calc_expr_3_left_indices__new,
                                                                        (),
                                                                        __new_row_ind,
                                                                    );
                                                                    __changed = true;
                                                                }
                                                            } else {
                                                                let __new_row_ind = _self.calc_expr_3_left.len();
                                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                                    &mut calc_expr_3_left_indices_0_1_new,
                                                                    (__new_row.0.clone(), __new_row.1.clone()),
                                                                    __new_row_ind,
                                                                );
                                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                                    &mut calc_expr_3_left_indices_0_new,
                                                                    (__new_row.0.clone(),),
                                                                    __new_row_ind,
                                                                );
                                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                                    &mut calc_expr_3_left_indices_1_new,
                                                                    (__new_row.1.clone(),),
                                                                    __new_row_ind,
                                                                );
                                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                                    &mut calc_expr_3_left_indices__new,
                                                                    (),
                                                                    __new_row_ind,
                                                                );
                                                                _self.calc_expr_3_left.push(__new_row);
                                                                __changed = true;
                                                            }
                                                            let __new_row: (ENodeId, Sym, Set<ENodeId>) = (
                                                                ascent::internal::Convert::convert(new_e_id),
                                                                ascent::internal::Convert::convert(op),
                                                                Set::singleton(*r_id),
                                                            );
                                                            let __lattice_key = (
                                                                __new_row.0.clone(),
                                                                __new_row.1.clone(),
                                                            );
                                                            if let Some(__existing_ind)
                                                                = calc_expr_3_right_indices_0_1_new
                                                                    .get(&__lattice_key)
                                                                    .or_else(|| {
                                                                        calc_expr_3_right_indices_0_1_delta.get(&__lattice_key)
                                                                    })
                                                                    .or_else(|| {
                                                                        calc_expr_3_right_indices_0_1_total.get(&__lattice_key)
                                                                    })
                                                            {
                                                                let __existing_ind = *__existing_ind.iter().next().unwrap();
                                                                let __lat_changed = ::ascent::Lattice::join_mut(
                                                                    &mut _self.calc_expr_3_right[__existing_ind].2,
                                                                    __new_row.2.clone(),
                                                                );
                                                                if __lat_changed {
                                                                    let __new_row_ind = __existing_ind;
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut calc_expr_3_right_indices__new,
                                                                        (),
                                                                        __new_row_ind,
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut calc_expr_3_right_indices_0_1_new,
                                                                        (__new_row.0.clone(), __new_row.1.clone()),
                                                                        __new_row_ind,
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut calc_expr_3_right_indices_0_new,
                                                                        (__new_row.0.clone(),),
                                                                        __new_row_ind,
                                                                    );
                                                                    __changed = true;
                                                                }
                                                            } else {
                                                                let __new_row_ind = _self.calc_expr_3_right.len();
                                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                                    &mut calc_expr_3_right_indices__new,
                                                                    (),
                                                                    __new_row_ind,
                                                                );
                                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                                    &mut calc_expr_3_right_indices_0_1_new,
                                                                    (__new_row.0.clone(), __new_row.1.clone()),
                                                                    __new_row_ind,
                                                                );
                                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                                    &mut calc_expr_3_right_indices_0_new,
                                                                    (__new_row.0.clone(),),
                                                                    __new_row_ind,
                                                                );
                                                                _self.calc_expr_3_right.push(__new_row);
                                                                __changed = true;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        for (__cl1_joined_columns, __cl1_tuple_indices) in assign_new_expr_id_indices_0_total
                            .iter_all()
                        {
                            let pat = &__cl1_joined_columns.0;
                            if let Some(__matching)
                                = do_add_new_expr_indices_0_total.index_get(&(pat.clone(),))
                            {
                                for cl1_ind in __cl1_tuple_indices {
                                    let __row = &_self.assign_new_expr_id[cl1_ind].clone();
                                    let new_e_id = &__row.1;
                                    for __ind in __matching.clone() {
                                        let __row = &_self.do_add_new_expr[__ind].clone();
                                        if let Calc(op, lp, rp) = pat.deref() {
                                            if let Some(__matching)
                                                = e_node_match_indices_0_total.index_get(&(lp.clone(),))
                                            {
                                                for __ind in __matching {
                                                    let __row = &_self.e_node_match[__ind].clone();
                                                    let lp = &__row.0;
                                                    let l_id = &__row.1;
                                                    if let Some(__matching)
                                                        = e_node_match_indices_0_delta.index_get(&(rp.clone(),))
                                                    {
                                                        for __ind in __matching {
                                                            let __row = &_self.e_node_match[__ind].clone();
                                                            let rp = &__row.0;
                                                            let r_id = &__row.1;
                                                            let __new_row: (ENodeId, Sym, Set<ENodeId>) = (
                                                                ascent::internal::Convert::convert(new_e_id),
                                                                ascent::internal::Convert::convert(op),
                                                                Set::singleton(*l_id),
                                                            );
                                                            let __lattice_key = (
                                                                __new_row.0.clone(),
                                                                __new_row.1.clone(),
                                                            );
                                                            if let Some(__existing_ind)
                                                                = calc_expr_3_left_indices_0_1_new
                                                                    .get(&__lattice_key)
                                                                    .or_else(|| {
                                                                        calc_expr_3_left_indices_0_1_delta.get(&__lattice_key)
                                                                    })
                                                                    .or_else(|| {
                                                                        calc_expr_3_left_indices_0_1_total.get(&__lattice_key)
                                                                    })
                                                            {
                                                                let __existing_ind = *__existing_ind.iter().next().unwrap();
                                                                let __lat_changed = ::ascent::Lattice::join_mut(
                                                                    &mut _self.calc_expr_3_left[__existing_ind].2,
                                                                    __new_row.2.clone(),
                                                                );
                                                                if __lat_changed {
                                                                    let __new_row_ind = __existing_ind;
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut calc_expr_3_left_indices_0_1_new,
                                                                        (__new_row.0.clone(), __new_row.1.clone()),
                                                                        __new_row_ind,
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut calc_expr_3_left_indices_0_new,
                                                                        (__new_row.0.clone(),),
                                                                        __new_row_ind,
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut calc_expr_3_left_indices_1_new,
                                                                        (__new_row.1.clone(),),
                                                                        __new_row_ind,
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut calc_expr_3_left_indices__new,
                                                                        (),
                                                                        __new_row_ind,
                                                                    );
                                                                    __changed = true;
                                                                }
                                                            } else {
                                                                let __new_row_ind = _self.calc_expr_3_left.len();
                                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                                    &mut calc_expr_3_left_indices_0_1_new,
                                                                    (__new_row.0.clone(), __new_row.1.clone()),
                                                                    __new_row_ind,
                                                                );
                                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                                    &mut calc_expr_3_left_indices_0_new,
                                                                    (__new_row.0.clone(),),
                                                                    __new_row_ind,
                                                                );
                                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                                    &mut calc_expr_3_left_indices_1_new,
                                                                    (__new_row.1.clone(),),
                                                                    __new_row_ind,
                                                                );
                                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                                    &mut calc_expr_3_left_indices__new,
                                                                    (),
                                                                    __new_row_ind,
                                                                );
                                                                _self.calc_expr_3_left.push(__new_row);
                                                                __changed = true;
                                                            }
                                                            let __new_row: (ENodeId, Sym, Set<ENodeId>) = (
                                                                ascent::internal::Convert::convert(new_e_id),
                                                                ascent::internal::Convert::convert(op),
                                                                Set::singleton(*r_id),
                                                            );
                                                            let __lattice_key = (
                                                                __new_row.0.clone(),
                                                                __new_row.1.clone(),
                                                            );
                                                            if let Some(__existing_ind)
                                                                = calc_expr_3_right_indices_0_1_new
                                                                    .get(&__lattice_key)
                                                                    .or_else(|| {
                                                                        calc_expr_3_right_indices_0_1_delta.get(&__lattice_key)
                                                                    })
                                                                    .or_else(|| {
                                                                        calc_expr_3_right_indices_0_1_total.get(&__lattice_key)
                                                                    })
                                                            {
                                                                let __existing_ind = *__existing_ind.iter().next().unwrap();
                                                                let __lat_changed = ::ascent::Lattice::join_mut(
                                                                    &mut _self.calc_expr_3_right[__existing_ind].2,
                                                                    __new_row.2.clone(),
                                                                );
                                                                if __lat_changed {
                                                                    let __new_row_ind = __existing_ind;
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut calc_expr_3_right_indices__new,
                                                                        (),
                                                                        __new_row_ind,
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut calc_expr_3_right_indices_0_1_new,
                                                                        (__new_row.0.clone(), __new_row.1.clone()),
                                                                        __new_row_ind,
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut calc_expr_3_right_indices_0_new,
                                                                        (__new_row.0.clone(),),
                                                                        __new_row_ind,
                                                                    );
                                                                    __changed = true;
                                                                }
                                                            } else {
                                                                let __new_row_ind = _self.calc_expr_3_right.len();
                                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                                    &mut calc_expr_3_right_indices__new,
                                                                    (),
                                                                    __new_row_ind,
                                                                );
                                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                                    &mut calc_expr_3_right_indices_0_1_new,
                                                                    (__new_row.0.clone(), __new_row.1.clone()),
                                                                    __new_row_ind,
                                                                );
                                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                                    &mut calc_expr_3_right_indices_0_new,
                                                                    (__new_row.0.clone(),),
                                                                    __new_row_ind,
                                                                );
                                                                _self.calc_expr_3_right.push(__new_row);
                                                                __changed = true;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment(
                        "var <-- do_add_new_expr_indices_0_delta, assign_new_expr_id_indices_0_total, if let ⋯ [SIMPLE JOIN]",
                    );
                    if do_add_new_expr_indices_0_delta.len()
                        <= assign_new_expr_id_indices_0_total.len()
                    {
                        for (__cl1_joined_columns, __cl1_tuple_indices) in do_add_new_expr_indices_0_delta
                            .iter_all()
                        {
                            let pat = &__cl1_joined_columns.0;
                            if let Some(__matching)
                                = assign_new_expr_id_indices_0_total
                                    .index_get(&(pat.clone(),))
                            {
                                for cl1_ind in __cl1_tuple_indices {
                                    let __row = &_self.do_add_new_expr[cl1_ind].clone();
                                    for __ind in __matching.clone() {
                                        let __row = &_self.assign_new_expr_id[__ind].clone();
                                        let new_e_id = &__row.1;
                                        if let Var(n) = pat.deref() {
                                            let __new_row: (ENodeId, Sym) = (
                                                ascent::internal::Convert::convert(new_e_id),
                                                ascent::internal::Convert::convert(n),
                                            );
                                            let __new_row_ind = _self.var.len();
                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                &var_indices_0_1_total,
                                                &__new_row,
                                            )
                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                    var_indices_0_1_delta,
                                                    &__new_row,
                                                )
                                                && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                    &mut var_indices_0_1_new,
                                                    &__new_row,
                                                    __new_row_ind,
                                                )
                                            {
                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                    &mut var_indices__new,
                                                    (),
                                                    __new_row_ind,
                                                );
                                                _self.var.push(__new_row);
                                                __changed = true;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        for (__cl1_joined_columns, __cl1_tuple_indices) in assign_new_expr_id_indices_0_total
                            .iter_all()
                        {
                            let pat = &__cl1_joined_columns.0;
                            if let Some(__matching)
                                = do_add_new_expr_indices_0_delta.index_get(&(pat.clone(),))
                            {
                                for cl1_ind in __cl1_tuple_indices {
                                    let __row = &_self.assign_new_expr_id[cl1_ind].clone();
                                    let new_e_id = &__row.1;
                                    for __ind in __matching.clone() {
                                        let __row = &_self.do_add_new_expr[__ind].clone();
                                        if let Var(n) = pat.deref() {
                                            let __new_row: (ENodeId, Sym) = (
                                                ascent::internal::Convert::convert(new_e_id),
                                                ascent::internal::Convert::convert(n),
                                            );
                                            let __new_row_ind = _self.var.len();
                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                &var_indices_0_1_total,
                                                &__new_row,
                                            )
                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                    var_indices_0_1_delta,
                                                    &__new_row,
                                                )
                                                && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                    &mut var_indices_0_1_new,
                                                    &__new_row,
                                                    __new_row_ind,
                                                )
                                            {
                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                    &mut var_indices__new,
                                                    (),
                                                    __new_row_ind,
                                                );
                                                _self.var.push(__new_row);
                                                __changed = true;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment("e_node <-- var_indices__delta");
                    if let Some(__matching) = var_indices__delta.index_get(&()) {
                        for __ind in __matching {
                            let __row = &_self.var[__ind].clone();
                            let e_id = &__row.0;
                            let __1 = &__row.1;
                            let __new_row: (ENodeId,) = (
                                ascent::internal::Convert::convert(e_id),
                            );
                            let __new_row_ind = _self.e_node.len();
                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                &e_node_indices_0_total,
                                &__new_row,
                            )
                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                    e_node_indices_0_delta,
                                    &__new_row,
                                )
                                && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                    &mut e_node_indices_0_new,
                                    &__new_row,
                                    __new_row_ind,
                                )
                            {
                                ::ascent::internal::RelIndexWrite::index_insert(
                                    &mut e_node_indices__new,
                                    (),
                                    __new_row_ind,
                                );
                                _self.e_node.push(__new_row);
                                __changed = true;
                            }
                        }
                    }
                    ascent::internal::comment(
                        "num <-- do_add_new_expr_indices_0_delta, assign_new_expr_id_indices_0_total, if let ⋯ [SIMPLE JOIN]",
                    );
                    if do_add_new_expr_indices_0_delta.len()
                        <= assign_new_expr_id_indices_0_total.len()
                    {
                        for (__cl1_joined_columns, __cl1_tuple_indices) in do_add_new_expr_indices_0_delta
                            .iter_all()
                        {
                            let pat = &__cl1_joined_columns.0;
                            if let Some(__matching)
                                = assign_new_expr_id_indices_0_total
                                    .index_get(&(pat.clone(),))
                            {
                                for cl1_ind in __cl1_tuple_indices {
                                    let __row = &_self.do_add_new_expr[cl1_ind].clone();
                                    for __ind in __matching.clone() {
                                        let __row = &_self.assign_new_expr_id[__ind].clone();
                                        let new_e_id = &__row.1;
                                        if let Num(n) = pat.deref() {
                                            let __new_row: (ENodeId, i32) = (
                                                ascent::internal::Convert::convert(new_e_id),
                                                ascent::internal::Convert::convert(n),
                                            );
                                            let __new_row_ind = _self.num.len();
                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                &num_indices_0_1_total,
                                                &__new_row,
                                            )
                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                    num_indices_0_1_delta,
                                                    &__new_row,
                                                )
                                                && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                    &mut num_indices_0_1_new,
                                                    &__new_row,
                                                    __new_row_ind,
                                                )
                                            {
                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                    &mut num_indices__new,
                                                    (),
                                                    __new_row_ind,
                                                );
                                                _self.num.push(__new_row);
                                                __changed = true;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        for (__cl1_joined_columns, __cl1_tuple_indices) in assign_new_expr_id_indices_0_total
                            .iter_all()
                        {
                            let pat = &__cl1_joined_columns.0;
                            if let Some(__matching)
                                = do_add_new_expr_indices_0_delta.index_get(&(pat.clone(),))
                            {
                                for cl1_ind in __cl1_tuple_indices {
                                    let __row = &_self.assign_new_expr_id[cl1_ind].clone();
                                    let new_e_id = &__row.1;
                                    for __ind in __matching.clone() {
                                        let __row = &_self.do_add_new_expr[__ind].clone();
                                        if let Num(n) = pat.deref() {
                                            let __new_row: (ENodeId, i32) = (
                                                ascent::internal::Convert::convert(new_e_id),
                                                ascent::internal::Convert::convert(n),
                                            );
                                            let __new_row_ind = _self.num.len();
                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                &num_indices_0_1_total,
                                                &__new_row,
                                            )
                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                    num_indices_0_1_delta,
                                                    &__new_row,
                                                )
                                                && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                    &mut num_indices_0_1_new,
                                                    &__new_row,
                                                    __new_row_ind,
                                                )
                                            {
                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                    &mut num_indices__new,
                                                    (),
                                                    __new_row_ind,
                                                );
                                                _self.num.push(__new_row);
                                                __changed = true;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment("e_node <-- num_indices__delta");
                    if let Some(__matching) = num_indices__delta.index_get(&()) {
                        for __ind in __matching {
                            let __row = &_self.num[__ind].clone();
                            let e_id = &__row.0;
                            let __1 = &__row.1;
                            let __new_row: (ENodeId,) = (
                                ascent::internal::Convert::convert(e_id),
                            );
                            let __new_row_ind = _self.e_node.len();
                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                &e_node_indices_0_total,
                                &__new_row,
                            )
                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                    e_node_indices_0_delta,
                                    &__new_row,
                                )
                                && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                    &mut e_node_indices_0_new,
                                    &__new_row,
                                    __new_row_ind,
                                )
                            {
                                ::ascent::internal::RelIndexWrite::index_insert(
                                    &mut e_node_indices__new,
                                    (),
                                    __new_row_ind,
                                );
                                _self.e_node.push(__new_row);
                                __changed = true;
                            }
                        }
                    }
                    ascent::internal::comment(
                        "root <-- root_indices__delta, do_uinon_id_indices__total+delta, if ⋯",
                    );
                    if let Some(__matching) = root_indices__delta.index_get(&()) {
                        for __ind in __matching {
                            let __row = &_self.root[__ind].clone();
                            let e_id = &__row.0;
                            let eq_set = &__row.1;
                            if let Some(__matching)
                                = ascent::internal::RelIndexCombined::new(
                                        &do_uinon_id_indices__total,
                                        do_uinon_id_indices__delta,
                                    )
                                    .index_get(&())
                            {
                                for __ind in __matching {
                                    let __row = &_self.do_uinon_id[__ind].clone();
                                    let old_id = &__row.0;
                                    let node_to_union = &__row.1;
                                    if eq_set.contains(old_id) {
                                        let __new_row: (ENodeId, Set<ENodeId>) = (
                                            ascent::internal::Convert::convert(e_id),
                                            Set::singleton(*node_to_union),
                                        );
                                        let __lattice_key = (__new_row.0.clone(),);
                                        if let Some(__existing_ind)
                                            = root_indices_0_new
                                                .get(&__lattice_key)
                                                .or_else(|| root_indices_0_delta.get(&__lattice_key))
                                                .or_else(|| root_indices_0_total.get(&__lattice_key))
                                        {
                                            let __existing_ind = *__existing_ind.iter().next().unwrap();
                                            let __lat_changed = ::ascent::Lattice::join_mut(
                                                &mut _self.root[__existing_ind].1,
                                                __new_row.1.clone(),
                                            );
                                            if __lat_changed {
                                                let __new_row_ind = __existing_ind;
                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                    &mut root_indices_0_new,
                                                    (__new_row.0.clone(),),
                                                    __new_row_ind,
                                                );
                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                    &mut root_indices__new,
                                                    (),
                                                    __new_row_ind,
                                                );
                                                __changed = true;
                                            }
                                        } else {
                                            let __new_row_ind = _self.root.len();
                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                &mut root_indices_0_new,
                                                (__new_row.0.clone(),),
                                                __new_row_ind,
                                            );
                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                &mut root_indices__new,
                                                (),
                                                __new_row_ind,
                                            );
                                            _self.root.push(__new_row);
                                            __changed = true;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment(
                        "root <-- root_indices__total, do_uinon_id_indices__delta, if ⋯",
                    );
                    if let Some(__matching) = root_indices__total.index_get(&()) {
                        for __ind in __matching {
                            let __row = &_self.root[__ind].clone();
                            let e_id = &__row.0;
                            let eq_set = &__row.1;
                            if let Some(__matching)
                                = do_uinon_id_indices__delta.index_get(&())
                            {
                                for __ind in __matching {
                                    let __row = &_self.do_uinon_id[__ind].clone();
                                    let old_id = &__row.0;
                                    let node_to_union = &__row.1;
                                    if eq_set.contains(old_id) {
                                        let __new_row: (ENodeId, Set<ENodeId>) = (
                                            ascent::internal::Convert::convert(e_id),
                                            Set::singleton(*node_to_union),
                                        );
                                        let __lattice_key = (__new_row.0.clone(),);
                                        if let Some(__existing_ind)
                                            = root_indices_0_new
                                                .get(&__lattice_key)
                                                .or_else(|| root_indices_0_delta.get(&__lattice_key))
                                                .or_else(|| root_indices_0_total.get(&__lattice_key))
                                        {
                                            let __existing_ind = *__existing_ind.iter().next().unwrap();
                                            let __lat_changed = ::ascent::Lattice::join_mut(
                                                &mut _self.root[__existing_ind].1,
                                                __new_row.1.clone(),
                                            );
                                            if __lat_changed {
                                                let __new_row_ind = __existing_ind;
                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                    &mut root_indices_0_new,
                                                    (__new_row.0.clone(),),
                                                    __new_row_ind,
                                                );
                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                    &mut root_indices__new,
                                                    (),
                                                    __new_row_ind,
                                                );
                                                __changed = true;
                                            }
                                        } else {
                                            let __new_row_ind = _self.root.len();
                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                &mut root_indices_0_new,
                                                (__new_row.0.clone(),),
                                                __new_row_ind,
                                            );
                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                &mut root_indices__new,
                                                (),
                                                __new_row_ind,
                                            );
                                            _self.root.push(__new_row);
                                            __changed = true;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment("e_node <-- root_indices__delta");
                    if let Some(__matching) = root_indices__delta.index_get(&()) {
                        for __ind in __matching {
                            let __row = &_self.root[__ind].clone();
                            let e_id = &__row.0;
                            let __1 = &__row.1;
                            let __new_row: (ENodeId,) = (
                                ascent::internal::Convert::convert(e_id),
                            );
                            let __new_row_ind = _self.e_node.len();
                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                &e_node_indices_0_total,
                                &__new_row,
                            )
                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                    e_node_indices_0_delta,
                                    &__new_row,
                                )
                                && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                    &mut e_node_indices_0_new,
                                    &__new_row,
                                    __new_row_ind,
                                )
                            {
                                ::ascent::internal::RelIndexWrite::index_insert(
                                    &mut e_node_indices__new,
                                    (),
                                    __new_row_ind,
                                );
                                _self.e_node.push(__new_row);
                                __changed = true;
                            }
                        }
                    }
                    ascent::internal::comment(
                        "do_match, do_match <-- do_match_indices__delta, if let ⋯, calc_expr_3_left_indices_0_1_total+delta, calc_expr_3_right_indices_0_1_total+delta, for_l_eq, for_r_eq",
                    );
                    if let Some(__matching) = do_match_indices__delta.index_get(&()) {
                        for __ind in __matching {
                            let __row = &_self.do_match[__ind].clone();
                            let pat = &__row.0;
                            let e_id = &__row.1;
                            if let Calc(op, left_p, right_p) = pat.deref() {
                                if let Some(__matching)
                                    = ascent::internal::RelIndexCombined::new(
                                            &calc_expr_3_left_indices_0_1_total,
                                            calc_expr_3_left_indices_0_1_delta,
                                        )
                                        .index_get(&(e_id.clone(), op.clone()))
                                {
                                    for __ind in __matching {
                                        let __row = &_self.calc_expr_3_left[__ind].clone();
                                        let op = &__row.1;
                                        let l_set = &__row.2;
                                        if let Some(__matching)
                                            = ascent::internal::RelIndexCombined::new(
                                                    &calc_expr_3_right_indices_0_1_total,
                                                    calc_expr_3_right_indices_0_1_delta,
                                                )
                                                .index_get(&(e_id.clone(), op.clone()))
                                        {
                                            for __ind in __matching {
                                                let __row = &_self.calc_expr_3_right[__ind].clone();
                                                let r_set = &__row.2;
                                                for l_eq in l_set.iter() {
                                                    for r_eq in r_set.iter() {
                                                        let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                                            ascent::internal::Convert::convert(left_p),
                                                            ascent::internal::Convert::convert(l_eq),
                                                        );
                                                        let __new_row_ind = _self.do_match.len();
                                                        if !::ascent::internal::RelFullIndexRead::contains_key(
                                                            &do_match_indices_0_1_total,
                                                            &__new_row,
                                                        )
                                                            && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                do_match_indices_0_1_delta,
                                                                &__new_row,
                                                            )
                                                            && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                &mut do_match_indices_0_1_new,
                                                                &__new_row,
                                                                __new_row_ind,
                                                            )
                                                        {
                                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                                &mut do_match_indices__new,
                                                                (),
                                                                __new_row_ind,
                                                            );
                                                            _self.do_match.push(__new_row);
                                                            __changed = true;
                                                        }
                                                        let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                                            ascent::internal::Convert::convert(right_p),
                                                            ascent::internal::Convert::convert(r_eq),
                                                        );
                                                        let __new_row_ind = _self.do_match.len();
                                                        if !::ascent::internal::RelFullIndexRead::contains_key(
                                                            &do_match_indices_0_1_total,
                                                            &__new_row,
                                                        )
                                                            && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                do_match_indices_0_1_delta,
                                                                &__new_row,
                                                            )
                                                            && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                &mut do_match_indices_0_1_new,
                                                                &__new_row,
                                                                __new_row_ind,
                                                            )
                                                        {
                                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                                &mut do_match_indices__new,
                                                                (),
                                                                __new_row_ind,
                                                            );
                                                            _self.do_match.push(__new_row);
                                                            __changed = true;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment(
                        "do_match, do_match <-- do_match_indices__total, if let ⋯, calc_expr_3_left_indices_0_1_delta, calc_expr_3_right_indices_0_1_total+delta, for_l_eq, for_r_eq",
                    );
                    if let Some(__matching) = do_match_indices__total.index_get(&()) {
                        for __ind in __matching {
                            let __row = &_self.do_match[__ind].clone();
                            let pat = &__row.0;
                            let e_id = &__row.1;
                            if let Calc(op, left_p, right_p) = pat.deref() {
                                if let Some(__matching)
                                    = calc_expr_3_left_indices_0_1_delta
                                        .index_get(&(e_id.clone(), op.clone()))
                                {
                                    for __ind in __matching {
                                        let __row = &_self.calc_expr_3_left[__ind].clone();
                                        let op = &__row.1;
                                        let l_set = &__row.2;
                                        if let Some(__matching)
                                            = ascent::internal::RelIndexCombined::new(
                                                    &calc_expr_3_right_indices_0_1_total,
                                                    calc_expr_3_right_indices_0_1_delta,
                                                )
                                                .index_get(&(e_id.clone(), op.clone()))
                                        {
                                            for __ind in __matching {
                                                let __row = &_self.calc_expr_3_right[__ind].clone();
                                                let r_set = &__row.2;
                                                for l_eq in l_set.iter() {
                                                    for r_eq in r_set.iter() {
                                                        let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                                            ascent::internal::Convert::convert(left_p),
                                                            ascent::internal::Convert::convert(l_eq),
                                                        );
                                                        let __new_row_ind = _self.do_match.len();
                                                        if !::ascent::internal::RelFullIndexRead::contains_key(
                                                            &do_match_indices_0_1_total,
                                                            &__new_row,
                                                        )
                                                            && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                do_match_indices_0_1_delta,
                                                                &__new_row,
                                                            )
                                                            && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                &mut do_match_indices_0_1_new,
                                                                &__new_row,
                                                                __new_row_ind,
                                                            )
                                                        {
                                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                                &mut do_match_indices__new,
                                                                (),
                                                                __new_row_ind,
                                                            );
                                                            _self.do_match.push(__new_row);
                                                            __changed = true;
                                                        }
                                                        let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                                            ascent::internal::Convert::convert(right_p),
                                                            ascent::internal::Convert::convert(r_eq),
                                                        );
                                                        let __new_row_ind = _self.do_match.len();
                                                        if !::ascent::internal::RelFullIndexRead::contains_key(
                                                            &do_match_indices_0_1_total,
                                                            &__new_row,
                                                        )
                                                            && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                do_match_indices_0_1_delta,
                                                                &__new_row,
                                                            )
                                                            && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                &mut do_match_indices_0_1_new,
                                                                &__new_row,
                                                                __new_row_ind,
                                                            )
                                                        {
                                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                                &mut do_match_indices__new,
                                                                (),
                                                                __new_row_ind,
                                                            );
                                                            _self.do_match.push(__new_row);
                                                            __changed = true;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment(
                        "do_match, do_match <-- do_match_indices__total, if let ⋯, calc_expr_3_left_indices_0_1_total, calc_expr_3_right_indices_0_1_delta, for_l_eq, for_r_eq",
                    );
                    if let Some(__matching) = do_match_indices__total.index_get(&()) {
                        for __ind in __matching {
                            let __row = &_self.do_match[__ind].clone();
                            let pat = &__row.0;
                            let e_id = &__row.1;
                            if let Calc(op, left_p, right_p) = pat.deref() {
                                if let Some(__matching)
                                    = calc_expr_3_left_indices_0_1_total
                                        .index_get(&(e_id.clone(), op.clone()))
                                {
                                    for __ind in __matching {
                                        let __row = &_self.calc_expr_3_left[__ind].clone();
                                        let op = &__row.1;
                                        let l_set = &__row.2;
                                        if let Some(__matching)
                                            = calc_expr_3_right_indices_0_1_delta
                                                .index_get(&(e_id.clone(), op.clone()))
                                        {
                                            for __ind in __matching {
                                                let __row = &_self.calc_expr_3_right[__ind].clone();
                                                let r_set = &__row.2;
                                                for l_eq in l_set.iter() {
                                                    for r_eq in r_set.iter() {
                                                        let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                                            ascent::internal::Convert::convert(left_p),
                                                            ascent::internal::Convert::convert(l_eq),
                                                        );
                                                        let __new_row_ind = _self.do_match.len();
                                                        if !::ascent::internal::RelFullIndexRead::contains_key(
                                                            &do_match_indices_0_1_total,
                                                            &__new_row,
                                                        )
                                                            && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                do_match_indices_0_1_delta,
                                                                &__new_row,
                                                            )
                                                            && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                &mut do_match_indices_0_1_new,
                                                                &__new_row,
                                                                __new_row_ind,
                                                            )
                                                        {
                                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                                &mut do_match_indices__new,
                                                                (),
                                                                __new_row_ind,
                                                            );
                                                            _self.do_match.push(__new_row);
                                                            __changed = true;
                                                        }
                                                        let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                                            ascent::internal::Convert::convert(right_p),
                                                            ascent::internal::Convert::convert(r_eq),
                                                        );
                                                        let __new_row_ind = _self.do_match.len();
                                                        if !::ascent::internal::RelFullIndexRead::contains_key(
                                                            &do_match_indices_0_1_total,
                                                            &__new_row,
                                                        )
                                                            && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                do_match_indices_0_1_delta,
                                                                &__new_row,
                                                            )
                                                            && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                &mut do_match_indices_0_1_new,
                                                                &__new_row,
                                                                __new_row_ind,
                                                            )
                                                        {
                                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                                &mut do_match_indices__new,
                                                                (),
                                                                __new_row_ind,
                                                            );
                                                            _self.do_match.push(__new_row);
                                                            __changed = true;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment(
                        "do_match <-- do_add_new_expr_indices__delta, e_node_indices__total+delta [SIMPLE JOIN]",
                    );
                    if do_add_new_expr_indices__delta.len()
                        <= ascent::internal::RelIndexCombined::new(
                                &e_node_indices__total,
                                e_node_indices__delta,
                            )
                            .len()
                    {
                        for (__cl1_joined_columns, __cl1_tuple_indices) in do_add_new_expr_indices__delta
                            .iter_all()
                        {
                            if let Some(__matching)
                                = ascent::internal::RelIndexCombined::new(
                                        &e_node_indices__total,
                                        e_node_indices__delta,
                                    )
                                    .index_get(&())
                            {
                                for cl1_ind in __cl1_tuple_indices {
                                    let __row = &_self.do_add_new_expr[cl1_ind].clone();
                                    let p = &__row.0;
                                    for __ind in __matching.clone() {
                                        let __row = &_self.e_node[__ind].clone();
                                        let e = &__row.0;
                                        let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                            ascent::internal::Convert::convert(p),
                                            ascent::internal::Convert::convert(e),
                                        );
                                        let __new_row_ind = _self.do_match.len();
                                        if !::ascent::internal::RelFullIndexRead::contains_key(
                                            &do_match_indices_0_1_total,
                                            &__new_row,
                                        )
                                            && !::ascent::internal::RelFullIndexRead::contains_key(
                                                do_match_indices_0_1_delta,
                                                &__new_row,
                                            )
                                            && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                &mut do_match_indices_0_1_new,
                                                &__new_row,
                                                __new_row_ind,
                                            )
                                        {
                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                &mut do_match_indices__new,
                                                (),
                                                __new_row_ind,
                                            );
                                            _self.do_match.push(__new_row);
                                            __changed = true;
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        for (__cl1_joined_columns, __cl1_tuple_indices) in ascent::internal::RelIndexCombined::new(
                                &e_node_indices__total,
                                e_node_indices__delta,
                            )
                            .iter_all()
                        {
                            if let Some(__matching)
                                = do_add_new_expr_indices__delta.index_get(&())
                            {
                                for cl1_ind in __cl1_tuple_indices {
                                    let __row = &_self.e_node[cl1_ind].clone();
                                    let e = &__row.0;
                                    for __ind in __matching.clone() {
                                        let __row = &_self.do_add_new_expr[__ind].clone();
                                        let p = &__row.0;
                                        let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                            ascent::internal::Convert::convert(p),
                                            ascent::internal::Convert::convert(e),
                                        );
                                        let __new_row_ind = _self.do_match.len();
                                        if !::ascent::internal::RelFullIndexRead::contains_key(
                                            &do_match_indices_0_1_total,
                                            &__new_row,
                                        )
                                            && !::ascent::internal::RelFullIndexRead::contains_key(
                                                do_match_indices_0_1_delta,
                                                &__new_row,
                                            )
                                            && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                &mut do_match_indices_0_1_new,
                                                &__new_row,
                                                __new_row_ind,
                                            )
                                        {
                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                &mut do_match_indices__new,
                                                (),
                                                __new_row_ind,
                                            );
                                            _self.do_match.push(__new_row);
                                            __changed = true;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment(
                        "do_match <-- do_add_new_expr_indices__total, e_node_indices__delta [SIMPLE JOIN]",
                    );
                    if do_add_new_expr_indices__total.len()
                        <= e_node_indices__delta.len()
                    {
                        for (__cl1_joined_columns, __cl1_tuple_indices) in do_add_new_expr_indices__total
                            .iter_all()
                        {
                            if let Some(__matching)
                                = e_node_indices__delta.index_get(&())
                            {
                                for cl1_ind in __cl1_tuple_indices {
                                    let __row = &_self.do_add_new_expr[cl1_ind].clone();
                                    let p = &__row.0;
                                    for __ind in __matching.clone() {
                                        let __row = &_self.e_node[__ind].clone();
                                        let e = &__row.0;
                                        let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                            ascent::internal::Convert::convert(p),
                                            ascent::internal::Convert::convert(e),
                                        );
                                        let __new_row_ind = _self.do_match.len();
                                        if !::ascent::internal::RelFullIndexRead::contains_key(
                                            &do_match_indices_0_1_total,
                                            &__new_row,
                                        )
                                            && !::ascent::internal::RelFullIndexRead::contains_key(
                                                do_match_indices_0_1_delta,
                                                &__new_row,
                                            )
                                            && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                &mut do_match_indices_0_1_new,
                                                &__new_row,
                                                __new_row_ind,
                                            )
                                        {
                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                &mut do_match_indices__new,
                                                (),
                                                __new_row_ind,
                                            );
                                            _self.do_match.push(__new_row);
                                            __changed = true;
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        for (__cl1_joined_columns, __cl1_tuple_indices) in e_node_indices__delta
                            .iter_all()
                        {
                            if let Some(__matching)
                                = do_add_new_expr_indices__total.index_get(&())
                            {
                                for cl1_ind in __cl1_tuple_indices {
                                    let __row = &_self.e_node[cl1_ind].clone();
                                    let e = &__row.0;
                                    for __ind in __matching.clone() {
                                        let __row = &_self.do_add_new_expr[__ind].clone();
                                        let p = &__row.0;
                                        let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                            ascent::internal::Convert::convert(p),
                                            ascent::internal::Convert::convert(e),
                                        );
                                        let __new_row_ind = _self.do_match.len();
                                        if !::ascent::internal::RelFullIndexRead::contains_key(
                                            &do_match_indices_0_1_total,
                                            &__new_row,
                                        )
                                            && !::ascent::internal::RelFullIndexRead::contains_key(
                                                do_match_indices_0_1_delta,
                                                &__new_row,
                                            )
                                            && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                &mut do_match_indices_0_1_new,
                                                &__new_row,
                                                __new_row_ind,
                                            )
                                        {
                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                &mut do_match_indices__new,
                                                (),
                                                __new_row_ind,
                                            );
                                            _self.do_match.push(__new_row);
                                            __changed = true;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment(
                        "calc_expr_3_right <-- calc_expr_3_right_indices__delta, do_uinon_id_indices__total+delta, if ⋯",
                    );
                    if let Some(__matching)
                        = calc_expr_3_right_indices__delta.index_get(&())
                    {
                        for __ind in __matching {
                            let __row = &_self.calc_expr_3_right[__ind].clone();
                            let e_id = &__row.0;
                            let op = &__row.1;
                            let eq_set = &__row.2;
                            if let Some(__matching)
                                = ascent::internal::RelIndexCombined::new(
                                        &do_uinon_id_indices__total,
                                        do_uinon_id_indices__delta,
                                    )
                                    .index_get(&())
                            {
                                for __ind in __matching {
                                    let __row = &_self.do_uinon_id[__ind].clone();
                                    let old_id = &__row.0;
                                    let node_to_union = &__row.1;
                                    if eq_set.contains(old_id) {
                                        let __new_row: (ENodeId, Sym, Set<ENodeId>) = (
                                            ascent::internal::Convert::convert(e_id),
                                            ascent::internal::Convert::convert(op),
                                            Set::singleton(*node_to_union),
                                        );
                                        let __lattice_key = (
                                            __new_row.0.clone(),
                                            __new_row.1.clone(),
                                        );
                                        if let Some(__existing_ind)
                                            = calc_expr_3_right_indices_0_1_new
                                                .get(&__lattice_key)
                                                .or_else(|| {
                                                    calc_expr_3_right_indices_0_1_delta.get(&__lattice_key)
                                                })
                                                .or_else(|| {
                                                    calc_expr_3_right_indices_0_1_total.get(&__lattice_key)
                                                })
                                        {
                                            let __existing_ind = *__existing_ind.iter().next().unwrap();
                                            let __lat_changed = ::ascent::Lattice::join_mut(
                                                &mut _self.calc_expr_3_right[__existing_ind].2,
                                                __new_row.2.clone(),
                                            );
                                            if __lat_changed {
                                                let __new_row_ind = __existing_ind;
                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                    &mut calc_expr_3_right_indices__new,
                                                    (),
                                                    __new_row_ind,
                                                );
                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                    &mut calc_expr_3_right_indices_0_1_new,
                                                    (__new_row.0.clone(), __new_row.1.clone()),
                                                    __new_row_ind,
                                                );
                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                    &mut calc_expr_3_right_indices_0_new,
                                                    (__new_row.0.clone(),),
                                                    __new_row_ind,
                                                );
                                                __changed = true;
                                            }
                                        } else {
                                            let __new_row_ind = _self.calc_expr_3_right.len();
                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                &mut calc_expr_3_right_indices__new,
                                                (),
                                                __new_row_ind,
                                            );
                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                &mut calc_expr_3_right_indices_0_1_new,
                                                (__new_row.0.clone(), __new_row.1.clone()),
                                                __new_row_ind,
                                            );
                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                &mut calc_expr_3_right_indices_0_new,
                                                (__new_row.0.clone(),),
                                                __new_row_ind,
                                            );
                                            _self.calc_expr_3_right.push(__new_row);
                                            __changed = true;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment(
                        "calc_expr_3_right <-- calc_expr_3_right_indices__total, do_uinon_id_indices__delta, if ⋯",
                    );
                    if let Some(__matching)
                        = calc_expr_3_right_indices__total.index_get(&())
                    {
                        for __ind in __matching {
                            let __row = &_self.calc_expr_3_right[__ind].clone();
                            let e_id = &__row.0;
                            let op = &__row.1;
                            let eq_set = &__row.2;
                            if let Some(__matching)
                                = do_uinon_id_indices__delta.index_get(&())
                            {
                                for __ind in __matching {
                                    let __row = &_self.do_uinon_id[__ind].clone();
                                    let old_id = &__row.0;
                                    let node_to_union = &__row.1;
                                    if eq_set.contains(old_id) {
                                        let __new_row: (ENodeId, Sym, Set<ENodeId>) = (
                                            ascent::internal::Convert::convert(e_id),
                                            ascent::internal::Convert::convert(op),
                                            Set::singleton(*node_to_union),
                                        );
                                        let __lattice_key = (
                                            __new_row.0.clone(),
                                            __new_row.1.clone(),
                                        );
                                        if let Some(__existing_ind)
                                            = calc_expr_3_right_indices_0_1_new
                                                .get(&__lattice_key)
                                                .or_else(|| {
                                                    calc_expr_3_right_indices_0_1_delta.get(&__lattice_key)
                                                })
                                                .or_else(|| {
                                                    calc_expr_3_right_indices_0_1_total.get(&__lattice_key)
                                                })
                                        {
                                            let __existing_ind = *__existing_ind.iter().next().unwrap();
                                            let __lat_changed = ::ascent::Lattice::join_mut(
                                                &mut _self.calc_expr_3_right[__existing_ind].2,
                                                __new_row.2.clone(),
                                            );
                                            if __lat_changed {
                                                let __new_row_ind = __existing_ind;
                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                    &mut calc_expr_3_right_indices__new,
                                                    (),
                                                    __new_row_ind,
                                                );
                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                    &mut calc_expr_3_right_indices_0_1_new,
                                                    (__new_row.0.clone(), __new_row.1.clone()),
                                                    __new_row_ind,
                                                );
                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                    &mut calc_expr_3_right_indices_0_new,
                                                    (__new_row.0.clone(),),
                                                    __new_row_ind,
                                                );
                                                __changed = true;
                                            }
                                        } else {
                                            let __new_row_ind = _self.calc_expr_3_right.len();
                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                &mut calc_expr_3_right_indices__new,
                                                (),
                                                __new_row_ind,
                                            );
                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                &mut calc_expr_3_right_indices_0_1_new,
                                                (__new_row.0.clone(), __new_row.1.clone()),
                                                __new_row_ind,
                                            );
                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                &mut calc_expr_3_right_indices_0_new,
                                                (__new_row.0.clone(),),
                                                __new_row_ind,
                                            );
                                            _self.calc_expr_3_right.push(__new_row);
                                            __changed = true;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment(
                        "e_node_match <-- do_match_indices__delta, if let ⋯, num_indices_0_1_total+delta",
                    );
                    if let Some(__matching) = do_match_indices__delta.index_get(&()) {
                        for __ind in __matching {
                            let __row = &_self.do_match[__ind].clone();
                            let pat = &__row.0;
                            let e_id = &__row.1;
                            if let Num(n) = pat.deref() {
                                if let Some(__matching)
                                    = ascent::internal::RelIndexCombined::new(
                                            &num_indices_0_1_total,
                                            num_indices_0_1_delta,
                                        )
                                        .index_get(&(e_id.clone(), n.clone()))
                                {
                                    for __ind in __matching {
                                        let __row = &_self.num[__ind].clone();
                                        let n = &__row.1;
                                        let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                            ascent::internal::Convert::convert(pat),
                                            ascent::internal::Convert::convert(e_id),
                                        );
                                        let __new_row_ind = _self.e_node_match.len();
                                        if !::ascent::internal::RelFullIndexRead::contains_key(
                                            &e_node_match_indices_0_1_total,
                                            &__new_row,
                                        )
                                            && !::ascent::internal::RelFullIndexRead::contains_key(
                                                e_node_match_indices_0_1_delta,
                                                &__new_row,
                                            )
                                            && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                &mut e_node_match_indices_0_1_new,
                                                &__new_row,
                                                __new_row_ind,
                                            )
                                        {
                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                &mut e_node_match_indices__new,
                                                (),
                                                __new_row_ind,
                                            );
                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                &mut e_node_match_indices_0_new,
                                                (__new_row.0.clone(),),
                                                __new_row_ind,
                                            );
                                            _self.e_node_match.push(__new_row);
                                            __changed = true;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment(
                        "e_node_match <-- do_match_indices__total, if let ⋯, num_indices_0_1_delta",
                    );
                    if let Some(__matching) = do_match_indices__total.index_get(&()) {
                        for __ind in __matching {
                            let __row = &_self.do_match[__ind].clone();
                            let pat = &__row.0;
                            let e_id = &__row.1;
                            if let Num(n) = pat.deref() {
                                if let Some(__matching)
                                    = num_indices_0_1_delta
                                        .index_get(&(e_id.clone(), n.clone()))
                                {
                                    for __ind in __matching {
                                        let __row = &_self.num[__ind].clone();
                                        let n = &__row.1;
                                        let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                            ascent::internal::Convert::convert(pat),
                                            ascent::internal::Convert::convert(e_id),
                                        );
                                        let __new_row_ind = _self.e_node_match.len();
                                        if !::ascent::internal::RelFullIndexRead::contains_key(
                                            &e_node_match_indices_0_1_total,
                                            &__new_row,
                                        )
                                            && !::ascent::internal::RelFullIndexRead::contains_key(
                                                e_node_match_indices_0_1_delta,
                                                &__new_row,
                                            )
                                            && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                &mut e_node_match_indices_0_1_new,
                                                &__new_row,
                                                __new_row_ind,
                                            )
                                        {
                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                &mut e_node_match_indices__new,
                                                (),
                                                __new_row_ind,
                                            );
                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                &mut e_node_match_indices_0_new,
                                                (__new_row.0.clone(),),
                                                __new_row_ind,
                                            );
                                            _self.e_node_match.push(__new_row);
                                            __changed = true;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment(
                        "e_node_match <-- do_match_indices__delta, if let ⋯, var_indices_0_1_total+delta",
                    );
                    if let Some(__matching) = do_match_indices__delta.index_get(&()) {
                        for __ind in __matching {
                            let __row = &_self.do_match[__ind].clone();
                            let pat = &__row.0;
                            let e_id = &__row.1;
                            if let Var(x) = pat.deref() {
                                if let Some(__matching)
                                    = ascent::internal::RelIndexCombined::new(
                                            &var_indices_0_1_total,
                                            var_indices_0_1_delta,
                                        )
                                        .index_get(&(e_id.clone(), x.clone()))
                                {
                                    for __ind in __matching {
                                        let __row = &_self.var[__ind].clone();
                                        let x = &__row.1;
                                        let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                            ascent::internal::Convert::convert(pat),
                                            ascent::internal::Convert::convert(e_id),
                                        );
                                        let __new_row_ind = _self.e_node_match.len();
                                        if !::ascent::internal::RelFullIndexRead::contains_key(
                                            &e_node_match_indices_0_1_total,
                                            &__new_row,
                                        )
                                            && !::ascent::internal::RelFullIndexRead::contains_key(
                                                e_node_match_indices_0_1_delta,
                                                &__new_row,
                                            )
                                            && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                &mut e_node_match_indices_0_1_new,
                                                &__new_row,
                                                __new_row_ind,
                                            )
                                        {
                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                &mut e_node_match_indices__new,
                                                (),
                                                __new_row_ind,
                                            );
                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                &mut e_node_match_indices_0_new,
                                                (__new_row.0.clone(),),
                                                __new_row_ind,
                                            );
                                            _self.e_node_match.push(__new_row);
                                            __changed = true;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment(
                        "e_node_match <-- do_match_indices__total, if let ⋯, var_indices_0_1_delta",
                    );
                    if let Some(__matching) = do_match_indices__total.index_get(&()) {
                        for __ind in __matching {
                            let __row = &_self.do_match[__ind].clone();
                            let pat = &__row.0;
                            let e_id = &__row.1;
                            if let Var(x) = pat.deref() {
                                if let Some(__matching)
                                    = var_indices_0_1_delta
                                        .index_get(&(e_id.clone(), x.clone()))
                                {
                                    for __ind in __matching {
                                        let __row = &_self.var[__ind].clone();
                                        let x = &__row.1;
                                        let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                            ascent::internal::Convert::convert(pat),
                                            ascent::internal::Convert::convert(e_id),
                                        );
                                        let __new_row_ind = _self.e_node_match.len();
                                        if !::ascent::internal::RelFullIndexRead::contains_key(
                                            &e_node_match_indices_0_1_total,
                                            &__new_row,
                                        )
                                            && !::ascent::internal::RelFullIndexRead::contains_key(
                                                e_node_match_indices_0_1_delta,
                                                &__new_row,
                                            )
                                            && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                &mut e_node_match_indices_0_1_new,
                                                &__new_row,
                                                __new_row_ind,
                                            )
                                        {
                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                &mut e_node_match_indices__new,
                                                (),
                                                __new_row_ind,
                                            );
                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                &mut e_node_match_indices_0_new,
                                                (__new_row.0.clone(),),
                                                __new_row_ind,
                                            );
                                            _self.e_node_match.push(__new_row);
                                            __changed = true;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment(
                        "e_node_match <-- do_match_indices__delta, root_indices_0_total+delta, for_root_eq_id, e_node_match_indices_0_1_total+delta",
                    );
                    if let Some(__matching) = do_match_indices__delta.index_get(&()) {
                        for __ind in __matching {
                            let __row = &_self.do_match[__ind].clone();
                            let pat = &__row.0;
                            let e_id = &__row.1;
                            if let Some(__matching)
                                = ascent::internal::RelIndexCombined::new(
                                        &root_indices_0_total,
                                        root_indices_0_delta,
                                    )
                                    .index_get(&(e_id.clone(),))
                            {
                                for __ind in __matching {
                                    let __row = &_self.root[__ind].clone();
                                    let root_eq_set = &__row.1;
                                    for root_eq_id in root_eq_set.deref() {
                                        if let Some(__matching)
                                            = ascent::internal::RelIndexCombined::new(
                                                    &e_node_match_indices_0_1_total,
                                                    e_node_match_indices_0_1_delta,
                                                )
                                                .index_get(&(pat.clone(), root_eq_id.clone()))
                                        {
                                            for __ind in __matching {
                                                let __row = &_self.e_node_match[__ind].clone();
                                                let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                                    ascent::internal::Convert::convert(pat),
                                                    ascent::internal::Convert::convert(e_id),
                                                );
                                                let __new_row_ind = _self.e_node_match.len();
                                                if !::ascent::internal::RelFullIndexRead::contains_key(
                                                    &e_node_match_indices_0_1_total,
                                                    &__new_row,
                                                )
                                                    && !::ascent::internal::RelFullIndexRead::contains_key(
                                                        e_node_match_indices_0_1_delta,
                                                        &__new_row,
                                                    )
                                                    && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                        &mut e_node_match_indices_0_1_new,
                                                        &__new_row,
                                                        __new_row_ind,
                                                    )
                                                {
                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                        &mut e_node_match_indices__new,
                                                        (),
                                                        __new_row_ind,
                                                    );
                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                        &mut e_node_match_indices_0_new,
                                                        (__new_row.0.clone(),),
                                                        __new_row_ind,
                                                    );
                                                    _self.e_node_match.push(__new_row);
                                                    __changed = true;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment(
                        "e_node_match <-- do_match_indices__total, root_indices_0_delta, for_root_eq_id, e_node_match_indices_0_1_total+delta",
                    );
                    if let Some(__matching) = do_match_indices__total.index_get(&()) {
                        for __ind in __matching {
                            let __row = &_self.do_match[__ind].clone();
                            let pat = &__row.0;
                            let e_id = &__row.1;
                            if let Some(__matching)
                                = root_indices_0_delta.index_get(&(e_id.clone(),))
                            {
                                for __ind in __matching {
                                    let __row = &_self.root[__ind].clone();
                                    let root_eq_set = &__row.1;
                                    for root_eq_id in root_eq_set.deref() {
                                        if let Some(__matching)
                                            = ascent::internal::RelIndexCombined::new(
                                                    &e_node_match_indices_0_1_total,
                                                    e_node_match_indices_0_1_delta,
                                                )
                                                .index_get(&(pat.clone(), root_eq_id.clone()))
                                        {
                                            for __ind in __matching {
                                                let __row = &_self.e_node_match[__ind].clone();
                                                let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                                    ascent::internal::Convert::convert(pat),
                                                    ascent::internal::Convert::convert(e_id),
                                                );
                                                let __new_row_ind = _self.e_node_match.len();
                                                if !::ascent::internal::RelFullIndexRead::contains_key(
                                                    &e_node_match_indices_0_1_total,
                                                    &__new_row,
                                                )
                                                    && !::ascent::internal::RelFullIndexRead::contains_key(
                                                        e_node_match_indices_0_1_delta,
                                                        &__new_row,
                                                    )
                                                    && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                        &mut e_node_match_indices_0_1_new,
                                                        &__new_row,
                                                        __new_row_ind,
                                                    )
                                                {
                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                        &mut e_node_match_indices__new,
                                                        (),
                                                        __new_row_ind,
                                                    );
                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                        &mut e_node_match_indices_0_new,
                                                        (__new_row.0.clone(),),
                                                        __new_row_ind,
                                                    );
                                                    _self.e_node_match.push(__new_row);
                                                    __changed = true;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment(
                        "e_node_match <-- do_match_indices__total, root_indices_0_total, for_root_eq_id, e_node_match_indices_0_1_delta",
                    );
                    if let Some(__matching) = do_match_indices__total.index_get(&()) {
                        for __ind in __matching {
                            let __row = &_self.do_match[__ind].clone();
                            let pat = &__row.0;
                            let e_id = &__row.1;
                            if let Some(__matching)
                                = root_indices_0_total.index_get(&(e_id.clone(),))
                            {
                                for __ind in __matching {
                                    let __row = &_self.root[__ind].clone();
                                    let root_eq_set = &__row.1;
                                    for root_eq_id in root_eq_set.deref() {
                                        if let Some(__matching)
                                            = e_node_match_indices_0_1_delta
                                                .index_get(&(pat.clone(), root_eq_id.clone()))
                                        {
                                            for __ind in __matching {
                                                let __row = &_self.e_node_match[__ind].clone();
                                                let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                                    ascent::internal::Convert::convert(pat),
                                                    ascent::internal::Convert::convert(e_id),
                                                );
                                                let __new_row_ind = _self.e_node_match.len();
                                                if !::ascent::internal::RelFullIndexRead::contains_key(
                                                    &e_node_match_indices_0_1_total,
                                                    &__new_row,
                                                )
                                                    && !::ascent::internal::RelFullIndexRead::contains_key(
                                                        e_node_match_indices_0_1_delta,
                                                        &__new_row,
                                                    )
                                                    && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                        &mut e_node_match_indices_0_1_new,
                                                        &__new_row,
                                                        __new_row_ind,
                                                    )
                                                {
                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                        &mut e_node_match_indices__new,
                                                        (),
                                                        __new_row_ind,
                                                    );
                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                        &mut e_node_match_indices_0_new,
                                                        (__new_row.0.clone(),),
                                                        __new_row_ind,
                                                    );
                                                    _self.e_node_match.push(__new_row);
                                                    __changed = true;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment(
                        "e_node_match <-- do_match_indices__delta, if let ⋯, calc_expr_3_right_indices__total+delta, if ⋯, if ⋯",
                    );
                    if let Some(__matching) = do_match_indices__delta.index_get(&()) {
                        for __ind in __matching {
                            let __row = &_self.do_match[__ind].clone();
                            let pat = &__row.0;
                            let e_id = &__row.1;
                            if let EClass(m_representive_node) = pat.deref() {
                                if let Some(__matching)
                                    = ascent::internal::RelIndexCombined::new(
                                            &calc_expr_3_right_indices__total,
                                            calc_expr_3_right_indices__delta,
                                        )
                                        .index_get(&())
                                {
                                    for __ind in __matching {
                                        let __row = &_self.calc_expr_3_right[__ind].clone();
                                        let __1 = &__row.0;
                                        let __2 = &__row.1;
                                        let eq_set = &__row.2;
                                        if eq_set.contains(e_id) {
                                            if eq_set.contains(m_representive_node) {
                                                let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                                    ascent::internal::Convert::convert(pat),
                                                    ascent::internal::Convert::convert(e_id),
                                                );
                                                let __new_row_ind = _self.e_node_match.len();
                                                if !::ascent::internal::RelFullIndexRead::contains_key(
                                                    &e_node_match_indices_0_1_total,
                                                    &__new_row,
                                                )
                                                    && !::ascent::internal::RelFullIndexRead::contains_key(
                                                        e_node_match_indices_0_1_delta,
                                                        &__new_row,
                                                    )
                                                    && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                        &mut e_node_match_indices_0_1_new,
                                                        &__new_row,
                                                        __new_row_ind,
                                                    )
                                                {
                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                        &mut e_node_match_indices__new,
                                                        (),
                                                        __new_row_ind,
                                                    );
                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                        &mut e_node_match_indices_0_new,
                                                        (__new_row.0.clone(),),
                                                        __new_row_ind,
                                                    );
                                                    _self.e_node_match.push(__new_row);
                                                    __changed = true;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment(
                        "e_node_match <-- do_match_indices__total, if let ⋯, calc_expr_3_right_indices__delta, if ⋯, if ⋯",
                    );
                    if let Some(__matching) = do_match_indices__total.index_get(&()) {
                        for __ind in __matching {
                            let __row = &_self.do_match[__ind].clone();
                            let pat = &__row.0;
                            let e_id = &__row.1;
                            if let EClass(m_representive_node) = pat.deref() {
                                if let Some(__matching)
                                    = calc_expr_3_right_indices__delta.index_get(&())
                                {
                                    for __ind in __matching {
                                        let __row = &_self.calc_expr_3_right[__ind].clone();
                                        let __1 = &__row.0;
                                        let __2 = &__row.1;
                                        let eq_set = &__row.2;
                                        if eq_set.contains(e_id) {
                                            if eq_set.contains(m_representive_node) {
                                                let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                                    ascent::internal::Convert::convert(pat),
                                                    ascent::internal::Convert::convert(e_id),
                                                );
                                                let __new_row_ind = _self.e_node_match.len();
                                                if !::ascent::internal::RelFullIndexRead::contains_key(
                                                    &e_node_match_indices_0_1_total,
                                                    &__new_row,
                                                )
                                                    && !::ascent::internal::RelFullIndexRead::contains_key(
                                                        e_node_match_indices_0_1_delta,
                                                        &__new_row,
                                                    )
                                                    && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                        &mut e_node_match_indices_0_1_new,
                                                        &__new_row,
                                                        __new_row_ind,
                                                    )
                                                {
                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                        &mut e_node_match_indices__new,
                                                        (),
                                                        __new_row_ind,
                                                    );
                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                        &mut e_node_match_indices_0_new,
                                                        (__new_row.0.clone(),),
                                                        __new_row_ind,
                                                    );
                                                    _self.e_node_match.push(__new_row);
                                                    __changed = true;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment(
                        "e_node_match <-- e_node_match_indices__delta, root_indices__total+delta, if ⋯, for_e_id",
                    );
                    if let Some(__matching) = e_node_match_indices__delta.index_get(&())
                    {
                        for __ind in __matching {
                            let __row = &_self.e_node_match[__ind].clone();
                            let pat = &__row.0;
                            let matched_id = &__row.1;
                            if let Some(__matching)
                                = ascent::internal::RelIndexCombined::new(
                                        &root_indices__total,
                                        root_indices__delta,
                                    )
                                    .index_get(&())
                            {
                                for __ind in __matching {
                                    let __row = &_self.root[__ind].clone();
                                    let __1 = &__row.0;
                                    let eq_set = &__row.1;
                                    if eq_set.contains(matched_id) {
                                        for e_id in eq_set.deref() {
                                            let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                                ascent::internal::Convert::convert(pat),
                                                ascent::internal::Convert::convert(e_id),
                                            );
                                            let __new_row_ind = _self.e_node_match.len();
                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                &e_node_match_indices_0_1_total,
                                                &__new_row,
                                            )
                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                    e_node_match_indices_0_1_delta,
                                                    &__new_row,
                                                )
                                                && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                    &mut e_node_match_indices_0_1_new,
                                                    &__new_row,
                                                    __new_row_ind,
                                                )
                                            {
                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                    &mut e_node_match_indices__new,
                                                    (),
                                                    __new_row_ind,
                                                );
                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                    &mut e_node_match_indices_0_new,
                                                    (__new_row.0.clone(),),
                                                    __new_row_ind,
                                                );
                                                _self.e_node_match.push(__new_row);
                                                __changed = true;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment(
                        "e_node_match <-- e_node_match_indices__total, root_indices__delta, if ⋯, for_e_id",
                    );
                    if let Some(__matching) = e_node_match_indices__total.index_get(&())
                    {
                        for __ind in __matching {
                            let __row = &_self.e_node_match[__ind].clone();
                            let pat = &__row.0;
                            let matched_id = &__row.1;
                            if let Some(__matching) = root_indices__delta.index_get(&())
                            {
                                for __ind in __matching {
                                    let __row = &_self.root[__ind].clone();
                                    let __1 = &__row.0;
                                    let eq_set = &__row.1;
                                    if eq_set.contains(matched_id) {
                                        for e_id in eq_set.deref() {
                                            let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                                ascent::internal::Convert::convert(pat),
                                                ascent::internal::Convert::convert(e_id),
                                            );
                                            let __new_row_ind = _self.e_node_match.len();
                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                &e_node_match_indices_0_1_total,
                                                &__new_row,
                                            )
                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                    e_node_match_indices_0_1_delta,
                                                    &__new_row,
                                                )
                                                && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                    &mut e_node_match_indices_0_1_new,
                                                    &__new_row,
                                                    __new_row_ind,
                                                )
                                            {
                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                    &mut e_node_match_indices__new,
                                                    (),
                                                    __new_row_ind,
                                                );
                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                    &mut e_node_match_indices_0_new,
                                                    (__new_row.0.clone(),),
                                                    __new_row_ind,
                                                );
                                                _self.e_node_match.push(__new_row);
                                                __changed = true;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment(
                        "e_node_match <-- do_match_indices__delta, if let ⋯, calc_expr_3_left_indices__total+delta, if ⋯, if ⋯",
                    );
                    if let Some(__matching) = do_match_indices__delta.index_get(&()) {
                        for __ind in __matching {
                            let __row = &_self.do_match[__ind].clone();
                            let pat = &__row.0;
                            let e_id = &__row.1;
                            if let EClass(m_representive_node) = pat.deref() {
                                if let Some(__matching)
                                    = ascent::internal::RelIndexCombined::new(
                                            &calc_expr_3_left_indices__total,
                                            calc_expr_3_left_indices__delta,
                                        )
                                        .index_get(&())
                                {
                                    for __ind in __matching {
                                        let __row = &_self.calc_expr_3_left[__ind].clone();
                                        let __1 = &__row.0;
                                        let __2 = &__row.1;
                                        let eq_set = &__row.2;
                                        if eq_set.contains(e_id) {
                                            if eq_set.contains(m_representive_node) {
                                                let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                                    ascent::internal::Convert::convert(pat),
                                                    ascent::internal::Convert::convert(e_id),
                                                );
                                                let __new_row_ind = _self.e_node_match.len();
                                                if !::ascent::internal::RelFullIndexRead::contains_key(
                                                    &e_node_match_indices_0_1_total,
                                                    &__new_row,
                                                )
                                                    && !::ascent::internal::RelFullIndexRead::contains_key(
                                                        e_node_match_indices_0_1_delta,
                                                        &__new_row,
                                                    )
                                                    && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                        &mut e_node_match_indices_0_1_new,
                                                        &__new_row,
                                                        __new_row_ind,
                                                    )
                                                {
                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                        &mut e_node_match_indices__new,
                                                        (),
                                                        __new_row_ind,
                                                    );
                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                        &mut e_node_match_indices_0_new,
                                                        (__new_row.0.clone(),),
                                                        __new_row_ind,
                                                    );
                                                    _self.e_node_match.push(__new_row);
                                                    __changed = true;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment(
                        "e_node_match <-- do_match_indices__total, if let ⋯, calc_expr_3_left_indices__delta, if ⋯, if ⋯",
                    );
                    if let Some(__matching) = do_match_indices__total.index_get(&()) {
                        for __ind in __matching {
                            let __row = &_self.do_match[__ind].clone();
                            let pat = &__row.0;
                            let e_id = &__row.1;
                            if let EClass(m_representive_node) = pat.deref() {
                                if let Some(__matching)
                                    = calc_expr_3_left_indices__delta.index_get(&())
                                {
                                    for __ind in __matching {
                                        let __row = &_self.calc_expr_3_left[__ind].clone();
                                        let __1 = &__row.0;
                                        let __2 = &__row.1;
                                        let eq_set = &__row.2;
                                        if eq_set.contains(e_id) {
                                            if eq_set.contains(m_representive_node) {
                                                let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                                    ascent::internal::Convert::convert(pat),
                                                    ascent::internal::Convert::convert(e_id),
                                                );
                                                let __new_row_ind = _self.e_node_match.len();
                                                if !::ascent::internal::RelFullIndexRead::contains_key(
                                                    &e_node_match_indices_0_1_total,
                                                    &__new_row,
                                                )
                                                    && !::ascent::internal::RelFullIndexRead::contains_key(
                                                        e_node_match_indices_0_1_delta,
                                                        &__new_row,
                                                    )
                                                    && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                        &mut e_node_match_indices_0_1_new,
                                                        &__new_row,
                                                        __new_row_ind,
                                                    )
                                                {
                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                        &mut e_node_match_indices__new,
                                                        (),
                                                        __new_row_ind,
                                                    );
                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                        &mut e_node_match_indices_0_new,
                                                        (__new_row.0.clone(),),
                                                        __new_row_ind,
                                                    );
                                                    _self.e_node_match.push(__new_row);
                                                    __changed = true;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment(
                        "e_node_match <-- e_node_match_indices__delta, calc_expr_3_right_indices__total+delta, if ⋯, for_e_id",
                    );
                    if let Some(__matching) = e_node_match_indices__delta.index_get(&())
                    {
                        for __ind in __matching {
                            let __row = &_self.e_node_match[__ind].clone();
                            let pat = &__row.0;
                            let matched_id = &__row.1;
                            if let Some(__matching)
                                = ascent::internal::RelIndexCombined::new(
                                        &calc_expr_3_right_indices__total,
                                        calc_expr_3_right_indices__delta,
                                    )
                                    .index_get(&())
                            {
                                for __ind in __matching {
                                    let __row = &_self.calc_expr_3_right[__ind].clone();
                                    let __1 = &__row.0;
                                    let __2 = &__row.1;
                                    let eq_set = &__row.2;
                                    if eq_set.contains(matched_id) {
                                        for e_id in eq_set.deref() {
                                            let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                                ascent::internal::Convert::convert(pat),
                                                ascent::internal::Convert::convert(e_id),
                                            );
                                            let __new_row_ind = _self.e_node_match.len();
                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                &e_node_match_indices_0_1_total,
                                                &__new_row,
                                            )
                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                    e_node_match_indices_0_1_delta,
                                                    &__new_row,
                                                )
                                                && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                    &mut e_node_match_indices_0_1_new,
                                                    &__new_row,
                                                    __new_row_ind,
                                                )
                                            {
                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                    &mut e_node_match_indices__new,
                                                    (),
                                                    __new_row_ind,
                                                );
                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                    &mut e_node_match_indices_0_new,
                                                    (__new_row.0.clone(),),
                                                    __new_row_ind,
                                                );
                                                _self.e_node_match.push(__new_row);
                                                __changed = true;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment(
                        "e_node_match <-- e_node_match_indices__total, calc_expr_3_right_indices__delta, if ⋯, for_e_id",
                    );
                    if let Some(__matching) = e_node_match_indices__total.index_get(&())
                    {
                        for __ind in __matching {
                            let __row = &_self.e_node_match[__ind].clone();
                            let pat = &__row.0;
                            let matched_id = &__row.1;
                            if let Some(__matching)
                                = calc_expr_3_right_indices__delta.index_get(&())
                            {
                                for __ind in __matching {
                                    let __row = &_self.calc_expr_3_right[__ind].clone();
                                    let __1 = &__row.0;
                                    let __2 = &__row.1;
                                    let eq_set = &__row.2;
                                    if eq_set.contains(matched_id) {
                                        for e_id in eq_set.deref() {
                                            let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                                ascent::internal::Convert::convert(pat),
                                                ascent::internal::Convert::convert(e_id),
                                            );
                                            let __new_row_ind = _self.e_node_match.len();
                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                &e_node_match_indices_0_1_total,
                                                &__new_row,
                                            )
                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                    e_node_match_indices_0_1_delta,
                                                    &__new_row,
                                                )
                                                && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                    &mut e_node_match_indices_0_1_new,
                                                    &__new_row,
                                                    __new_row_ind,
                                                )
                                            {
                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                    &mut e_node_match_indices__new,
                                                    (),
                                                    __new_row_ind,
                                                );
                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                    &mut e_node_match_indices_0_new,
                                                    (__new_row.0.clone(),),
                                                    __new_row_ind,
                                                );
                                                _self.e_node_match.push(__new_row);
                                                __changed = true;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment(
                        "e_node_match <-- e_node_match_indices__delta, calc_expr_3_left_indices__total+delta, if ⋯, for_e_id",
                    );
                    if let Some(__matching) = e_node_match_indices__delta.index_get(&())
                    {
                        for __ind in __matching {
                            let __row = &_self.e_node_match[__ind].clone();
                            let pat = &__row.0;
                            let matched_id = &__row.1;
                            if let Some(__matching)
                                = ascent::internal::RelIndexCombined::new(
                                        &calc_expr_3_left_indices__total,
                                        calc_expr_3_left_indices__delta,
                                    )
                                    .index_get(&())
                            {
                                for __ind in __matching {
                                    let __row = &_self.calc_expr_3_left[__ind].clone();
                                    let __1 = &__row.0;
                                    let __2 = &__row.1;
                                    let eq_set = &__row.2;
                                    if eq_set.contains(matched_id) {
                                        for e_id in eq_set.deref() {
                                            let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                                ascent::internal::Convert::convert(pat),
                                                ascent::internal::Convert::convert(e_id),
                                            );
                                            let __new_row_ind = _self.e_node_match.len();
                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                &e_node_match_indices_0_1_total,
                                                &__new_row,
                                            )
                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                    e_node_match_indices_0_1_delta,
                                                    &__new_row,
                                                )
                                                && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                    &mut e_node_match_indices_0_1_new,
                                                    &__new_row,
                                                    __new_row_ind,
                                                )
                                            {
                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                    &mut e_node_match_indices__new,
                                                    (),
                                                    __new_row_ind,
                                                );
                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                    &mut e_node_match_indices_0_new,
                                                    (__new_row.0.clone(),),
                                                    __new_row_ind,
                                                );
                                                _self.e_node_match.push(__new_row);
                                                __changed = true;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment(
                        "e_node_match <-- e_node_match_indices__total, calc_expr_3_left_indices__delta, if ⋯, for_e_id",
                    );
                    if let Some(__matching) = e_node_match_indices__total.index_get(&())
                    {
                        for __ind in __matching {
                            let __row = &_self.e_node_match[__ind].clone();
                            let pat = &__row.0;
                            let matched_id = &__row.1;
                            if let Some(__matching)
                                = calc_expr_3_left_indices__delta.index_get(&())
                            {
                                for __ind in __matching {
                                    let __row = &_self.calc_expr_3_left[__ind].clone();
                                    let __1 = &__row.0;
                                    let __2 = &__row.1;
                                    let eq_set = &__row.2;
                                    if eq_set.contains(matched_id) {
                                        for e_id in eq_set.deref() {
                                            let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                                ascent::internal::Convert::convert(pat),
                                                ascent::internal::Convert::convert(e_id),
                                            );
                                            let __new_row_ind = _self.e_node_match.len();
                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                &e_node_match_indices_0_1_total,
                                                &__new_row,
                                            )
                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                    e_node_match_indices_0_1_delta,
                                                    &__new_row,
                                                )
                                                && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                    &mut e_node_match_indices_0_1_new,
                                                    &__new_row,
                                                    __new_row_ind,
                                                )
                                            {
                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                    &mut e_node_match_indices__new,
                                                    (),
                                                    __new_row_ind,
                                                );
                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                    &mut e_node_match_indices_0_new,
                                                    (__new_row.0.clone(),),
                                                    __new_row_ind,
                                                );
                                                _self.e_node_match.push(__new_row);
                                                __changed = true;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment(
                        "e_node_match <-- do_match_indices__delta, if let ⋯, root_indices__total+delta, if ⋯, if ⋯",
                    );
                    if let Some(__matching) = do_match_indices__delta.index_get(&()) {
                        for __ind in __matching {
                            let __row = &_self.do_match[__ind].clone();
                            let pat = &__row.0;
                            let e_id = &__row.1;
                            if let EClass(m_representive_node) = pat.deref() {
                                if let Some(__matching)
                                    = ascent::internal::RelIndexCombined::new(
                                            &root_indices__total,
                                            root_indices__delta,
                                        )
                                        .index_get(&())
                                {
                                    for __ind in __matching {
                                        let __row = &_self.root[__ind].clone();
                                        let __1 = &__row.0;
                                        let eq_set = &__row.1;
                                        if eq_set.contains(e_id) {
                                            if eq_set.contains(m_representive_node) {
                                                let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                                    ascent::internal::Convert::convert(pat),
                                                    ascent::internal::Convert::convert(e_id),
                                                );
                                                let __new_row_ind = _self.e_node_match.len();
                                                if !::ascent::internal::RelFullIndexRead::contains_key(
                                                    &e_node_match_indices_0_1_total,
                                                    &__new_row,
                                                )
                                                    && !::ascent::internal::RelFullIndexRead::contains_key(
                                                        e_node_match_indices_0_1_delta,
                                                        &__new_row,
                                                    )
                                                    && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                        &mut e_node_match_indices_0_1_new,
                                                        &__new_row,
                                                        __new_row_ind,
                                                    )
                                                {
                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                        &mut e_node_match_indices__new,
                                                        (),
                                                        __new_row_ind,
                                                    );
                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                        &mut e_node_match_indices_0_new,
                                                        (__new_row.0.clone(),),
                                                        __new_row_ind,
                                                    );
                                                    _self.e_node_match.push(__new_row);
                                                    __changed = true;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment(
                        "e_node_match <-- do_match_indices__total, if let ⋯, root_indices__delta, if ⋯, if ⋯",
                    );
                    if let Some(__matching) = do_match_indices__total.index_get(&()) {
                        for __ind in __matching {
                            let __row = &_self.do_match[__ind].clone();
                            let pat = &__row.0;
                            let e_id = &__row.1;
                            if let EClass(m_representive_node) = pat.deref() {
                                if let Some(__matching) = root_indices__delta.index_get(&())
                                {
                                    for __ind in __matching {
                                        let __row = &_self.root[__ind].clone();
                                        let __1 = &__row.0;
                                        let eq_set = &__row.1;
                                        if eq_set.contains(e_id) {
                                            if eq_set.contains(m_representive_node) {
                                                let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                                    ascent::internal::Convert::convert(pat),
                                                    ascent::internal::Convert::convert(e_id),
                                                );
                                                let __new_row_ind = _self.e_node_match.len();
                                                if !::ascent::internal::RelFullIndexRead::contains_key(
                                                    &e_node_match_indices_0_1_total,
                                                    &__new_row,
                                                )
                                                    && !::ascent::internal::RelFullIndexRead::contains_key(
                                                        e_node_match_indices_0_1_delta,
                                                        &__new_row,
                                                    )
                                                    && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                        &mut e_node_match_indices_0_1_new,
                                                        &__new_row,
                                                        __new_row_ind,
                                                    )
                                                {
                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                        &mut e_node_match_indices__new,
                                                        (),
                                                        __new_row_ind,
                                                    );
                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                        &mut e_node_match_indices_0_new,
                                                        (__new_row.0.clone(),),
                                                        __new_row_ind,
                                                    );
                                                    _self.e_node_match.push(__new_row);
                                                    __changed = true;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment(
                        "e_node_match <-- do_match_indices__delta, if let ⋯, calc_expr_3_left_indices_0_1_total+delta, calc_expr_3_right_indices_0_1_total+delta, e_node_match_indices_0_total+delta, e_node_match_indices_0_total+delta, if ⋯, if ⋯",
                    );
                    if let Some(__matching) = do_match_indices__delta.index_get(&()) {
                        for __ind in __matching {
                            let __row = &_self.do_match[__ind].clone();
                            let pat = &__row.0;
                            let e_id = &__row.1;
                            if let Calc(op, left_p, right_p) = pat.deref() {
                                if let Some(__matching)
                                    = ascent::internal::RelIndexCombined::new(
                                            &calc_expr_3_left_indices_0_1_total,
                                            calc_expr_3_left_indices_0_1_delta,
                                        )
                                        .index_get(&(e_id.clone(), op.clone()))
                                {
                                    for __ind in __matching {
                                        let __row = &_self.calc_expr_3_left[__ind].clone();
                                        let op = &__row.1;
                                        let l_set = &__row.2;
                                        if let Some(__matching)
                                            = ascent::internal::RelIndexCombined::new(
                                                    &calc_expr_3_right_indices_0_1_total,
                                                    calc_expr_3_right_indices_0_1_delta,
                                                )
                                                .index_get(&(e_id.clone(), op.clone()))
                                        {
                                            for __ind in __matching {
                                                let __row = &_self.calc_expr_3_right[__ind].clone();
                                                let r_set = &__row.2;
                                                if let Some(__matching)
                                                    = ascent::internal::RelIndexCombined::new(
                                                            &e_node_match_indices_0_total,
                                                            e_node_match_indices_0_delta,
                                                        )
                                                        .index_get(&(left_p.clone(),))
                                                {
                                                    for __ind in __matching {
                                                        let __row = &_self.e_node_match[__ind].clone();
                                                        let left_p = &__row.0;
                                                        let l_matched = &__row.1;
                                                        if let Some(__matching)
                                                            = ascent::internal::RelIndexCombined::new(
                                                                    &e_node_match_indices_0_total,
                                                                    e_node_match_indices_0_delta,
                                                                )
                                                                .index_get(&(right_p.clone(),))
                                                        {
                                                            for __ind in __matching {
                                                                let __row = &_self.e_node_match[__ind].clone();
                                                                let right_p = &__row.0;
                                                                let r_matched = &__row.1;
                                                                if l_set.contains(l_matched) {
                                                                    if r_set.contains(r_matched) {
                                                                        let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                                                            ascent::internal::Convert::convert(pat),
                                                                            ascent::internal::Convert::convert(e_id),
                                                                        );
                                                                        let __new_row_ind = _self.e_node_match.len();
                                                                        if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                            &e_node_match_indices_0_1_total,
                                                                            &__new_row,
                                                                        )
                                                                            && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                e_node_match_indices_0_1_delta,
                                                                                &__new_row,
                                                                            )
                                                                            && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                                &mut e_node_match_indices_0_1_new,
                                                                                &__new_row,
                                                                                __new_row_ind,
                                                                            )
                                                                        {
                                                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                                                &mut e_node_match_indices__new,
                                                                                (),
                                                                                __new_row_ind,
                                                                            );
                                                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                                                &mut e_node_match_indices_0_new,
                                                                                (__new_row.0.clone(),),
                                                                                __new_row_ind,
                                                                            );
                                                                            _self.e_node_match.push(__new_row);
                                                                            __changed = true;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment(
                        "e_node_match <-- do_match_indices__total, if let ⋯, calc_expr_3_left_indices_0_1_delta, calc_expr_3_right_indices_0_1_total+delta, e_node_match_indices_0_total+delta, e_node_match_indices_0_total+delta, if ⋯, if ⋯",
                    );
                    if let Some(__matching) = do_match_indices__total.index_get(&()) {
                        for __ind in __matching {
                            let __row = &_self.do_match[__ind].clone();
                            let pat = &__row.0;
                            let e_id = &__row.1;
                            if let Calc(op, left_p, right_p) = pat.deref() {
                                if let Some(__matching)
                                    = calc_expr_3_left_indices_0_1_delta
                                        .index_get(&(e_id.clone(), op.clone()))
                                {
                                    for __ind in __matching {
                                        let __row = &_self.calc_expr_3_left[__ind].clone();
                                        let op = &__row.1;
                                        let l_set = &__row.2;
                                        if let Some(__matching)
                                            = ascent::internal::RelIndexCombined::new(
                                                    &calc_expr_3_right_indices_0_1_total,
                                                    calc_expr_3_right_indices_0_1_delta,
                                                )
                                                .index_get(&(e_id.clone(), op.clone()))
                                        {
                                            for __ind in __matching {
                                                let __row = &_self.calc_expr_3_right[__ind].clone();
                                                let r_set = &__row.2;
                                                if let Some(__matching)
                                                    = ascent::internal::RelIndexCombined::new(
                                                            &e_node_match_indices_0_total,
                                                            e_node_match_indices_0_delta,
                                                        )
                                                        .index_get(&(left_p.clone(),))
                                                {
                                                    for __ind in __matching {
                                                        let __row = &_self.e_node_match[__ind].clone();
                                                        let left_p = &__row.0;
                                                        let l_matched = &__row.1;
                                                        if let Some(__matching)
                                                            = ascent::internal::RelIndexCombined::new(
                                                                    &e_node_match_indices_0_total,
                                                                    e_node_match_indices_0_delta,
                                                                )
                                                                .index_get(&(right_p.clone(),))
                                                        {
                                                            for __ind in __matching {
                                                                let __row = &_self.e_node_match[__ind].clone();
                                                                let right_p = &__row.0;
                                                                let r_matched = &__row.1;
                                                                if l_set.contains(l_matched) {
                                                                    if r_set.contains(r_matched) {
                                                                        let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                                                            ascent::internal::Convert::convert(pat),
                                                                            ascent::internal::Convert::convert(e_id),
                                                                        );
                                                                        let __new_row_ind = _self.e_node_match.len();
                                                                        if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                            &e_node_match_indices_0_1_total,
                                                                            &__new_row,
                                                                        )
                                                                            && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                e_node_match_indices_0_1_delta,
                                                                                &__new_row,
                                                                            )
                                                                            && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                                &mut e_node_match_indices_0_1_new,
                                                                                &__new_row,
                                                                                __new_row_ind,
                                                                            )
                                                                        {
                                                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                                                &mut e_node_match_indices__new,
                                                                                (),
                                                                                __new_row_ind,
                                                                            );
                                                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                                                &mut e_node_match_indices_0_new,
                                                                                (__new_row.0.clone(),),
                                                                                __new_row_ind,
                                                                            );
                                                                            _self.e_node_match.push(__new_row);
                                                                            __changed = true;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment(
                        "e_node_match <-- do_match_indices__total, if let ⋯, calc_expr_3_left_indices_0_1_total, calc_expr_3_right_indices_0_1_delta, e_node_match_indices_0_total+delta, e_node_match_indices_0_total+delta, if ⋯, if ⋯",
                    );
                    if let Some(__matching) = do_match_indices__total.index_get(&()) {
                        for __ind in __matching {
                            let __row = &_self.do_match[__ind].clone();
                            let pat = &__row.0;
                            let e_id = &__row.1;
                            if let Calc(op, left_p, right_p) = pat.deref() {
                                if let Some(__matching)
                                    = calc_expr_3_left_indices_0_1_total
                                        .index_get(&(e_id.clone(), op.clone()))
                                {
                                    for __ind in __matching {
                                        let __row = &_self.calc_expr_3_left[__ind].clone();
                                        let op = &__row.1;
                                        let l_set = &__row.2;
                                        if let Some(__matching)
                                            = calc_expr_3_right_indices_0_1_delta
                                                .index_get(&(e_id.clone(), op.clone()))
                                        {
                                            for __ind in __matching {
                                                let __row = &_self.calc_expr_3_right[__ind].clone();
                                                let r_set = &__row.2;
                                                if let Some(__matching)
                                                    = ascent::internal::RelIndexCombined::new(
                                                            &e_node_match_indices_0_total,
                                                            e_node_match_indices_0_delta,
                                                        )
                                                        .index_get(&(left_p.clone(),))
                                                {
                                                    for __ind in __matching {
                                                        let __row = &_self.e_node_match[__ind].clone();
                                                        let left_p = &__row.0;
                                                        let l_matched = &__row.1;
                                                        if let Some(__matching)
                                                            = ascent::internal::RelIndexCombined::new(
                                                                    &e_node_match_indices_0_total,
                                                                    e_node_match_indices_0_delta,
                                                                )
                                                                .index_get(&(right_p.clone(),))
                                                        {
                                                            for __ind in __matching {
                                                                let __row = &_self.e_node_match[__ind].clone();
                                                                let right_p = &__row.0;
                                                                let r_matched = &__row.1;
                                                                if l_set.contains(l_matched) {
                                                                    if r_set.contains(r_matched) {
                                                                        let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                                                            ascent::internal::Convert::convert(pat),
                                                                            ascent::internal::Convert::convert(e_id),
                                                                        );
                                                                        let __new_row_ind = _self.e_node_match.len();
                                                                        if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                            &e_node_match_indices_0_1_total,
                                                                            &__new_row,
                                                                        )
                                                                            && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                e_node_match_indices_0_1_delta,
                                                                                &__new_row,
                                                                            )
                                                                            && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                                &mut e_node_match_indices_0_1_new,
                                                                                &__new_row,
                                                                                __new_row_ind,
                                                                            )
                                                                        {
                                                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                                                &mut e_node_match_indices__new,
                                                                                (),
                                                                                __new_row_ind,
                                                                            );
                                                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                                                &mut e_node_match_indices_0_new,
                                                                                (__new_row.0.clone(),),
                                                                                __new_row_ind,
                                                                            );
                                                                            _self.e_node_match.push(__new_row);
                                                                            __changed = true;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment(
                        "e_node_match <-- do_match_indices__total, if let ⋯, calc_expr_3_left_indices_0_1_total, calc_expr_3_right_indices_0_1_total, e_node_match_indices_0_delta, e_node_match_indices_0_total+delta, if ⋯, if ⋯",
                    );
                    if let Some(__matching) = do_match_indices__total.index_get(&()) {
                        for __ind in __matching {
                            let __row = &_self.do_match[__ind].clone();
                            let pat = &__row.0;
                            let e_id = &__row.1;
                            if let Calc(op, left_p, right_p) = pat.deref() {
                                if let Some(__matching)
                                    = calc_expr_3_left_indices_0_1_total
                                        .index_get(&(e_id.clone(), op.clone()))
                                {
                                    for __ind in __matching {
                                        let __row = &_self.calc_expr_3_left[__ind].clone();
                                        let op = &__row.1;
                                        let l_set = &__row.2;
                                        if let Some(__matching)
                                            = calc_expr_3_right_indices_0_1_total
                                                .index_get(&(e_id.clone(), op.clone()))
                                        {
                                            for __ind in __matching {
                                                let __row = &_self.calc_expr_3_right[__ind].clone();
                                                let r_set = &__row.2;
                                                if let Some(__matching)
                                                    = e_node_match_indices_0_delta.index_get(&(left_p.clone(),))
                                                {
                                                    for __ind in __matching {
                                                        let __row = &_self.e_node_match[__ind].clone();
                                                        let left_p = &__row.0;
                                                        let l_matched = &__row.1;
                                                        if let Some(__matching)
                                                            = ascent::internal::RelIndexCombined::new(
                                                                    &e_node_match_indices_0_total,
                                                                    e_node_match_indices_0_delta,
                                                                )
                                                                .index_get(&(right_p.clone(),))
                                                        {
                                                            for __ind in __matching {
                                                                let __row = &_self.e_node_match[__ind].clone();
                                                                let right_p = &__row.0;
                                                                let r_matched = &__row.1;
                                                                if l_set.contains(l_matched) {
                                                                    if r_set.contains(r_matched) {
                                                                        let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                                                            ascent::internal::Convert::convert(pat),
                                                                            ascent::internal::Convert::convert(e_id),
                                                                        );
                                                                        let __new_row_ind = _self.e_node_match.len();
                                                                        if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                            &e_node_match_indices_0_1_total,
                                                                            &__new_row,
                                                                        )
                                                                            && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                e_node_match_indices_0_1_delta,
                                                                                &__new_row,
                                                                            )
                                                                            && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                                &mut e_node_match_indices_0_1_new,
                                                                                &__new_row,
                                                                                __new_row_ind,
                                                                            )
                                                                        {
                                                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                                                &mut e_node_match_indices__new,
                                                                                (),
                                                                                __new_row_ind,
                                                                            );
                                                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                                                &mut e_node_match_indices_0_new,
                                                                                (__new_row.0.clone(),),
                                                                                __new_row_ind,
                                                                            );
                                                                            _self.e_node_match.push(__new_row);
                                                                            __changed = true;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment(
                        "e_node_match <-- do_match_indices__total, if let ⋯, calc_expr_3_left_indices_0_1_total, calc_expr_3_right_indices_0_1_total, e_node_match_indices_0_total, e_node_match_indices_0_delta, if ⋯, if ⋯",
                    );
                    if let Some(__matching) = do_match_indices__total.index_get(&()) {
                        for __ind in __matching {
                            let __row = &_self.do_match[__ind].clone();
                            let pat = &__row.0;
                            let e_id = &__row.1;
                            if let Calc(op, left_p, right_p) = pat.deref() {
                                if let Some(__matching)
                                    = calc_expr_3_left_indices_0_1_total
                                        .index_get(&(e_id.clone(), op.clone()))
                                {
                                    for __ind in __matching {
                                        let __row = &_self.calc_expr_3_left[__ind].clone();
                                        let op = &__row.1;
                                        let l_set = &__row.2;
                                        if let Some(__matching)
                                            = calc_expr_3_right_indices_0_1_total
                                                .index_get(&(e_id.clone(), op.clone()))
                                        {
                                            for __ind in __matching {
                                                let __row = &_self.calc_expr_3_right[__ind].clone();
                                                let r_set = &__row.2;
                                                if let Some(__matching)
                                                    = e_node_match_indices_0_total.index_get(&(left_p.clone(),))
                                                {
                                                    for __ind in __matching {
                                                        let __row = &_self.e_node_match[__ind].clone();
                                                        let left_p = &__row.0;
                                                        let l_matched = &__row.1;
                                                        if let Some(__matching)
                                                            = e_node_match_indices_0_delta
                                                                .index_get(&(right_p.clone(),))
                                                        {
                                                            for __ind in __matching {
                                                                let __row = &_self.e_node_match[__ind].clone();
                                                                let right_p = &__row.0;
                                                                let r_matched = &__row.1;
                                                                if l_set.contains(l_matched) {
                                                                    if r_set.contains(r_matched) {
                                                                        let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                                                            ascent::internal::Convert::convert(pat),
                                                                            ascent::internal::Convert::convert(e_id),
                                                                        );
                                                                        let __new_row_ind = _self.e_node_match.len();
                                                                        if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                            &e_node_match_indices_0_1_total,
                                                                            &__new_row,
                                                                        )
                                                                            && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                e_node_match_indices_0_1_delta,
                                                                                &__new_row,
                                                                            )
                                                                            && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                                &mut e_node_match_indices_0_1_new,
                                                                                &__new_row,
                                                                                __new_row_ind,
                                                                            )
                                                                        {
                                                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                                                &mut e_node_match_indices__new,
                                                                                (),
                                                                                __new_row_ind,
                                                                            );
                                                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                                                &mut e_node_match_indices_0_new,
                                                                                (__new_row.0.clone(),),
                                                                                __new_row_ind,
                                                                            );
                                                                            _self.e_node_match.push(__new_row);
                                                                            __changed = true;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment(
                        "e_node_match <-- do_match_indices__delta, if let ⋯",
                    );
                    if let Some(__matching) = do_match_indices__delta.index_get(&()) {
                        for __ind in __matching {
                            let __row = &_self.do_match[__ind].clone();
                            let pat = &__row.0;
                            let e_id = &__row.1;
                            if let WildCard(mv) = pat.deref() {
                                let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                    ascent::internal::Convert::convert(pat),
                                    ascent::internal::Convert::convert(e_id),
                                );
                                let __new_row_ind = _self.e_node_match.len();
                                if !::ascent::internal::RelFullIndexRead::contains_key(
                                    &e_node_match_indices_0_1_total,
                                    &__new_row,
                                )
                                    && !::ascent::internal::RelFullIndexRead::contains_key(
                                        e_node_match_indices_0_1_delta,
                                        &__new_row,
                                    )
                                    && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                        &mut e_node_match_indices_0_1_new,
                                        &__new_row,
                                        __new_row_ind,
                                    )
                                {
                                    ::ascent::internal::RelIndexWrite::index_insert(
                                        &mut e_node_match_indices__new,
                                        (),
                                        __new_row_ind,
                                    );
                                    ::ascent::internal::RelIndexWrite::index_insert(
                                        &mut e_node_match_indices_0_new,
                                        (__new_row.0.clone(),),
                                        __new_row_ind,
                                    );
                                    _self.e_node_match.push(__new_row);
                                    __changed = true;
                                }
                            }
                        }
                    }
                    ascent::internal::comment(
                        "do_union_pattern <-- rewrite_rule_indices_0_total, e_node_indices__delta, calc_expr_3_right_indices__total+delta, if ⋯, if let ⋯, let ⋯",
                    );
                    if let Some(__matching)
                        = rewrite_rule_indices_0_total
                            .index_get(&(("expand-mul-1").clone(),))
                    {
                        for __ind in __matching {
                            let __row = &_self.rewrite_rule[__ind].clone();
                            if let Some(__matching)
                                = e_node_indices__delta.index_get(&())
                            {
                                for __ind in __matching {
                                    let __row = &_self.e_node[__ind].clone();
                                    let e_id = &__row.0;
                                    if let Some(__matching)
                                        = ascent::internal::RelIndexCombined::new(
                                                &calc_expr_3_right_indices__total,
                                                calc_expr_3_right_indices__delta,
                                            )
                                            .index_get(&())
                                    {
                                        for __ind in __matching {
                                            let __row = &_self.calc_expr_3_right[__ind].clone();
                                            let __1 = &__row.0;
                                            let __2 = &__row.1;
                                            let eq_set = &__row.2;
                                            if eq_set.contains(e_id) {
                                                if let Some(eq_representive_node) = eq_set.first() {
                                                    let new_pat_expr = Rc::new(
                                                        Calc(
                                                            "*",
                                                            Rc::new(EClass(*eq_representive_node)),
                                                            Rc::new(Num(1)),
                                                        ),
                                                    );
                                                    let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                                        ascent::internal::Convert::convert(new_pat_expr),
                                                        ascent::internal::Convert::convert(e_id),
                                                    );
                                                    let __new_row_ind = _self.do_union_pattern.len();
                                                    if !::ascent::internal::RelFullIndexRead::contains_key(
                                                        &do_union_pattern_indices_0_1_total,
                                                        &__new_row,
                                                    )
                                                        && !::ascent::internal::RelFullIndexRead::contains_key(
                                                            do_union_pattern_indices_0_1_delta,
                                                            &__new_row,
                                                        )
                                                        && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                            &mut do_union_pattern_indices_0_1_new,
                                                            &__new_row,
                                                            __new_row_ind,
                                                        )
                                                    {
                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                            &mut do_union_pattern_indices__new,
                                                            (),
                                                            __new_row_ind,
                                                        );
                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                            &mut do_union_pattern_indices_0_new,
                                                            (__new_row.0.clone(),),
                                                            __new_row_ind,
                                                        );
                                                        _self.do_union_pattern.push(__new_row);
                                                        __changed = true;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment(
                        "do_union_pattern <-- rewrite_rule_indices_0_total, e_node_indices__total, calc_expr_3_right_indices__delta, if ⋯, if let ⋯, let ⋯",
                    );
                    if let Some(__matching)
                        = rewrite_rule_indices_0_total
                            .index_get(&(("expand-mul-1").clone(),))
                    {
                        for __ind in __matching {
                            let __row = &_self.rewrite_rule[__ind].clone();
                            if let Some(__matching)
                                = e_node_indices__total.index_get(&())
                            {
                                for __ind in __matching {
                                    let __row = &_self.e_node[__ind].clone();
                                    let e_id = &__row.0;
                                    if let Some(__matching)
                                        = calc_expr_3_right_indices__delta.index_get(&())
                                    {
                                        for __ind in __matching {
                                            let __row = &_self.calc_expr_3_right[__ind].clone();
                                            let __1 = &__row.0;
                                            let __2 = &__row.1;
                                            let eq_set = &__row.2;
                                            if eq_set.contains(e_id) {
                                                if let Some(eq_representive_node) = eq_set.first() {
                                                    let new_pat_expr = Rc::new(
                                                        Calc(
                                                            "*",
                                                            Rc::new(EClass(*eq_representive_node)),
                                                            Rc::new(Num(1)),
                                                        ),
                                                    );
                                                    let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                                        ascent::internal::Convert::convert(new_pat_expr),
                                                        ascent::internal::Convert::convert(e_id),
                                                    );
                                                    let __new_row_ind = _self.do_union_pattern.len();
                                                    if !::ascent::internal::RelFullIndexRead::contains_key(
                                                        &do_union_pattern_indices_0_1_total,
                                                        &__new_row,
                                                    )
                                                        && !::ascent::internal::RelFullIndexRead::contains_key(
                                                            do_union_pattern_indices_0_1_delta,
                                                            &__new_row,
                                                        )
                                                        && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                            &mut do_union_pattern_indices_0_1_new,
                                                            &__new_row,
                                                            __new_row_ind,
                                                        )
                                                    {
                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                            &mut do_union_pattern_indices__new,
                                                            (),
                                                            __new_row_ind,
                                                        );
                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                            &mut do_union_pattern_indices_0_new,
                                                            (__new_row.0.clone(),),
                                                            __new_row_ind,
                                                        );
                                                        _self.do_union_pattern.push(__new_row);
                                                        __changed = true;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment(
                        "do_add_new_expr <-- do_union_pattern_indices__delta",
                    );
                    if let Some(__matching)
                        = do_union_pattern_indices__delta.index_get(&())
                    {
                        for __ind in __matching {
                            let __row = &_self.do_union_pattern[__ind].clone();
                            let pat = &__row.0;
                            let __1 = &__row.1;
                            let __new_row: (Rc<PatternExpr>,) = (
                                ascent::internal::Convert::convert(pat),
                            );
                            let __new_row_ind = _self.do_add_new_expr.len();
                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                &do_add_new_expr_indices_0_total,
                                &__new_row,
                            )
                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                    do_add_new_expr_indices_0_delta,
                                    &__new_row,
                                )
                                && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                    &mut do_add_new_expr_indices_0_new,
                                    &__new_row,
                                    __new_row_ind,
                                )
                            {
                                ::ascent::internal::RelIndexWrite::index_insert(
                                    &mut do_add_new_expr_indices__new,
                                    (),
                                    __new_row_ind,
                                );
                                _self.do_add_new_expr.push(__new_row);
                                __changed = true;
                            }
                        }
                    }
                    ascent::internal::comment(
                        "do_union_pattern <-- rewrite_rule_indices_0_total, e_node_indices__delta, root_indices__total+delta, if ⋯, if let ⋯, let ⋯",
                    );
                    if let Some(__matching)
                        = rewrite_rule_indices_0_total
                            .index_get(&(("expand-mul-1").clone(),))
                    {
                        for __ind in __matching {
                            let __row = &_self.rewrite_rule[__ind].clone();
                            if let Some(__matching)
                                = e_node_indices__delta.index_get(&())
                            {
                                for __ind in __matching {
                                    let __row = &_self.e_node[__ind].clone();
                                    let e_id = &__row.0;
                                    if let Some(__matching)
                                        = ascent::internal::RelIndexCombined::new(
                                                &root_indices__total,
                                                root_indices__delta,
                                            )
                                            .index_get(&())
                                    {
                                        for __ind in __matching {
                                            let __row = &_self.root[__ind].clone();
                                            let __1 = &__row.0;
                                            let eq_set = &__row.1;
                                            if eq_set.contains(e_id) {
                                                if let Some(eq_representive_node) = eq_set.first() {
                                                    let new_pat_expr = Rc::new(
                                                        Calc(
                                                            "*",
                                                            Rc::new(EClass(*eq_representive_node)),
                                                            Rc::new(Num(1)),
                                                        ),
                                                    );
                                                    let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                                        ascent::internal::Convert::convert(new_pat_expr),
                                                        ascent::internal::Convert::convert(e_id),
                                                    );
                                                    let __new_row_ind = _self.do_union_pattern.len();
                                                    if !::ascent::internal::RelFullIndexRead::contains_key(
                                                        &do_union_pattern_indices_0_1_total,
                                                        &__new_row,
                                                    )
                                                        && !::ascent::internal::RelFullIndexRead::contains_key(
                                                            do_union_pattern_indices_0_1_delta,
                                                            &__new_row,
                                                        )
                                                        && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                            &mut do_union_pattern_indices_0_1_new,
                                                            &__new_row,
                                                            __new_row_ind,
                                                        )
                                                    {
                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                            &mut do_union_pattern_indices__new,
                                                            (),
                                                            __new_row_ind,
                                                        );
                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                            &mut do_union_pattern_indices_0_new,
                                                            (__new_row.0.clone(),),
                                                            __new_row_ind,
                                                        );
                                                        _self.do_union_pattern.push(__new_row);
                                                        __changed = true;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment(
                        "do_union_pattern <-- rewrite_rule_indices_0_total, e_node_indices__total, root_indices__delta, if ⋯, if let ⋯, let ⋯",
                    );
                    if let Some(__matching)
                        = rewrite_rule_indices_0_total
                            .index_get(&(("expand-mul-1").clone(),))
                    {
                        for __ind in __matching {
                            let __row = &_self.rewrite_rule[__ind].clone();
                            if let Some(__matching)
                                = e_node_indices__total.index_get(&())
                            {
                                for __ind in __matching {
                                    let __row = &_self.e_node[__ind].clone();
                                    let e_id = &__row.0;
                                    if let Some(__matching) = root_indices__delta.index_get(&())
                                    {
                                        for __ind in __matching {
                                            let __row = &_self.root[__ind].clone();
                                            let __1 = &__row.0;
                                            let eq_set = &__row.1;
                                            if eq_set.contains(e_id) {
                                                if let Some(eq_representive_node) = eq_set.first() {
                                                    let new_pat_expr = Rc::new(
                                                        Calc(
                                                            "*",
                                                            Rc::new(EClass(*eq_representive_node)),
                                                            Rc::new(Num(1)),
                                                        ),
                                                    );
                                                    let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                                        ascent::internal::Convert::convert(new_pat_expr),
                                                        ascent::internal::Convert::convert(e_id),
                                                    );
                                                    let __new_row_ind = _self.do_union_pattern.len();
                                                    if !::ascent::internal::RelFullIndexRead::contains_key(
                                                        &do_union_pattern_indices_0_1_total,
                                                        &__new_row,
                                                    )
                                                        && !::ascent::internal::RelFullIndexRead::contains_key(
                                                            do_union_pattern_indices_0_1_delta,
                                                            &__new_row,
                                                        )
                                                        && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                            &mut do_union_pattern_indices_0_1_new,
                                                            &__new_row,
                                                            __new_row_ind,
                                                        )
                                                    {
                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                            &mut do_union_pattern_indices__new,
                                                            (),
                                                            __new_row_ind,
                                                        );
                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                            &mut do_union_pattern_indices_0_new,
                                                            (__new_row.0.clone(),),
                                                            __new_row_ind,
                                                        );
                                                        _self.do_union_pattern.push(__new_row);
                                                        __changed = true;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment(
                        "do_union_pattern <-- rewrite_rule_indices_0_total, calc_expr_3_left_indices_1_delta, calc_expr_3_right_indices_0_1_total+delta, for_div_l_id, calc_expr_3_left_indices_0_1_total+delta, calc_expr_3_right_indices_0_1_total+delta, if let ⋯, if let ⋯, if let ⋯, let ⋯",
                    );
                    if let Some(__matching)
                        = rewrite_rule_indices_0_total
                            .index_get(&(("mul-comm-1").clone(),))
                    {
                        for __ind in __matching {
                            let __row = &_self.rewrite_rule[__ind].clone();
                            if let Some(__matching)
                                = calc_expr_3_left_indices_1_delta
                                    .index_get(&(("/").clone(),))
                            {
                                for __ind in __matching {
                                    let __row = &_self.calc_expr_3_left[__ind].clone();
                                    let div_e_id = &__row.0;
                                    let l_div_set = &__row.2;
                                    if let Some(__matching)
                                        = ascent::internal::RelIndexCombined::new(
                                                &calc_expr_3_right_indices_0_1_total,
                                                calc_expr_3_right_indices_0_1_delta,
                                            )
                                            .index_get(&(div_e_id.clone(), ("/").clone()))
                                    {
                                        for __ind in __matching {
                                            let __row = &_self.calc_expr_3_right[__ind].clone();
                                            let r_div_set = &__row.2;
                                            for div_l_id in l_div_set.iter() {
                                                if let Some(__matching)
                                                    = ascent::internal::RelIndexCombined::new(
                                                            &calc_expr_3_left_indices_0_1_total,
                                                            calc_expr_3_left_indices_0_1_delta,
                                                        )
                                                        .index_get(&(div_l_id.clone(), ("*").clone()))
                                                {
                                                    for __ind in __matching {
                                                        let __row = &_self.calc_expr_3_left[__ind].clone();
                                                        let l_mult_set = &__row.2;
                                                        if let Some(__matching)
                                                            = ascent::internal::RelIndexCombined::new(
                                                                    &calc_expr_3_right_indices_0_1_total,
                                                                    calc_expr_3_right_indices_0_1_delta,
                                                                )
                                                                .index_get(&(div_l_id.clone(), ("*").clone()))
                                                        {
                                                            for __ind in __matching {
                                                                let __row = &_self.calc_expr_3_right[__ind].clone();
                                                                let r_mult_set = &__row.2;
                                                                if let Some(l_mult_set_rep) = l_mult_set.first() {
                                                                    if let Some(r_mult_set_rep) = r_mult_set.first() {
                                                                        if let Some(r_div_set_rep) = r_div_set.first() {
                                                                            let new_pat_expr = Rc::new(
                                                                                Calc(
                                                                                    "*",
                                                                                    Rc::new(EClass(*l_mult_set_rep)),
                                                                                    Rc::new(
                                                                                        Calc(
                                                                                            "/",
                                                                                            Rc::new(EClass(*r_mult_set_rep)),
                                                                                            Rc::new(EClass(*r_div_set_rep)),
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            );
                                                                            let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                                                                ascent::internal::Convert::convert(new_pat_expr),
                                                                                ascent::internal::Convert::convert(div_e_id),
                                                                            );
                                                                            let __new_row_ind = _self.do_union_pattern.len();
                                                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                &do_union_pattern_indices_0_1_total,
                                                                                &__new_row,
                                                                            )
                                                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                    do_union_pattern_indices_0_1_delta,
                                                                                    &__new_row,
                                                                                )
                                                                                && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                                    &mut do_union_pattern_indices_0_1_new,
                                                                                    &__new_row,
                                                                                    __new_row_ind,
                                                                                )
                                                                            {
                                                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                                                    &mut do_union_pattern_indices__new,
                                                                                    (),
                                                                                    __new_row_ind,
                                                                                );
                                                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                                                    &mut do_union_pattern_indices_0_new,
                                                                                    (__new_row.0.clone(),),
                                                                                    __new_row_ind,
                                                                                );
                                                                                _self.do_union_pattern.push(__new_row);
                                                                                __changed = true;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment(
                        "do_union_pattern <-- rewrite_rule_indices_0_total, calc_expr_3_left_indices_1_total, calc_expr_3_right_indices_0_1_delta, for_div_l_id, calc_expr_3_left_indices_0_1_total+delta, calc_expr_3_right_indices_0_1_total+delta, if let ⋯, if let ⋯, if let ⋯, let ⋯",
                    );
                    if let Some(__matching)
                        = rewrite_rule_indices_0_total
                            .index_get(&(("mul-comm-1").clone(),))
                    {
                        for __ind in __matching {
                            let __row = &_self.rewrite_rule[__ind].clone();
                            if let Some(__matching)
                                = calc_expr_3_left_indices_1_total
                                    .index_get(&(("/").clone(),))
                            {
                                for __ind in __matching {
                                    let __row = &_self.calc_expr_3_left[__ind].clone();
                                    let div_e_id = &__row.0;
                                    let l_div_set = &__row.2;
                                    if let Some(__matching)
                                        = calc_expr_3_right_indices_0_1_delta
                                            .index_get(&(div_e_id.clone(), ("/").clone()))
                                    {
                                        for __ind in __matching {
                                            let __row = &_self.calc_expr_3_right[__ind].clone();
                                            let r_div_set = &__row.2;
                                            for div_l_id in l_div_set.iter() {
                                                if let Some(__matching)
                                                    = ascent::internal::RelIndexCombined::new(
                                                            &calc_expr_3_left_indices_0_1_total,
                                                            calc_expr_3_left_indices_0_1_delta,
                                                        )
                                                        .index_get(&(div_l_id.clone(), ("*").clone()))
                                                {
                                                    for __ind in __matching {
                                                        let __row = &_self.calc_expr_3_left[__ind].clone();
                                                        let l_mult_set = &__row.2;
                                                        if let Some(__matching)
                                                            = ascent::internal::RelIndexCombined::new(
                                                                    &calc_expr_3_right_indices_0_1_total,
                                                                    calc_expr_3_right_indices_0_1_delta,
                                                                )
                                                                .index_get(&(div_l_id.clone(), ("*").clone()))
                                                        {
                                                            for __ind in __matching {
                                                                let __row = &_self.calc_expr_3_right[__ind].clone();
                                                                let r_mult_set = &__row.2;
                                                                if let Some(l_mult_set_rep) = l_mult_set.first() {
                                                                    if let Some(r_mult_set_rep) = r_mult_set.first() {
                                                                        if let Some(r_div_set_rep) = r_div_set.first() {
                                                                            let new_pat_expr = Rc::new(
                                                                                Calc(
                                                                                    "*",
                                                                                    Rc::new(EClass(*l_mult_set_rep)),
                                                                                    Rc::new(
                                                                                        Calc(
                                                                                            "/",
                                                                                            Rc::new(EClass(*r_mult_set_rep)),
                                                                                            Rc::new(EClass(*r_div_set_rep)),
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            );
                                                                            let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                                                                ascent::internal::Convert::convert(new_pat_expr),
                                                                                ascent::internal::Convert::convert(div_e_id),
                                                                            );
                                                                            let __new_row_ind = _self.do_union_pattern.len();
                                                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                &do_union_pattern_indices_0_1_total,
                                                                                &__new_row,
                                                                            )
                                                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                    do_union_pattern_indices_0_1_delta,
                                                                                    &__new_row,
                                                                                )
                                                                                && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                                    &mut do_union_pattern_indices_0_1_new,
                                                                                    &__new_row,
                                                                                    __new_row_ind,
                                                                                )
                                                                            {
                                                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                                                    &mut do_union_pattern_indices__new,
                                                                                    (),
                                                                                    __new_row_ind,
                                                                                );
                                                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                                                    &mut do_union_pattern_indices_0_new,
                                                                                    (__new_row.0.clone(),),
                                                                                    __new_row_ind,
                                                                                );
                                                                                _self.do_union_pattern.push(__new_row);
                                                                                __changed = true;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment(
                        "do_union_pattern <-- rewrite_rule_indices_0_total, calc_expr_3_left_indices_1_total, calc_expr_3_right_indices_0_1_total, for_div_l_id, calc_expr_3_left_indices_0_1_delta, calc_expr_3_right_indices_0_1_total+delta, if let ⋯, if let ⋯, if let ⋯, let ⋯",
                    );
                    if let Some(__matching)
                        = rewrite_rule_indices_0_total
                            .index_get(&(("mul-comm-1").clone(),))
                    {
                        for __ind in __matching {
                            let __row = &_self.rewrite_rule[__ind].clone();
                            if let Some(__matching)
                                = calc_expr_3_left_indices_1_total
                                    .index_get(&(("/").clone(),))
                            {
                                for __ind in __matching {
                                    let __row = &_self.calc_expr_3_left[__ind].clone();
                                    let div_e_id = &__row.0;
                                    let l_div_set = &__row.2;
                                    if let Some(__matching)
                                        = calc_expr_3_right_indices_0_1_total
                                            .index_get(&(div_e_id.clone(), ("/").clone()))
                                    {
                                        for __ind in __matching {
                                            let __row = &_self.calc_expr_3_right[__ind].clone();
                                            let r_div_set = &__row.2;
                                            for div_l_id in l_div_set.iter() {
                                                if let Some(__matching)
                                                    = calc_expr_3_left_indices_0_1_delta
                                                        .index_get(&(div_l_id.clone(), ("*").clone()))
                                                {
                                                    for __ind in __matching {
                                                        let __row = &_self.calc_expr_3_left[__ind].clone();
                                                        let l_mult_set = &__row.2;
                                                        if let Some(__matching)
                                                            = ascent::internal::RelIndexCombined::new(
                                                                    &calc_expr_3_right_indices_0_1_total,
                                                                    calc_expr_3_right_indices_0_1_delta,
                                                                )
                                                                .index_get(&(div_l_id.clone(), ("*").clone()))
                                                        {
                                                            for __ind in __matching {
                                                                let __row = &_self.calc_expr_3_right[__ind].clone();
                                                                let r_mult_set = &__row.2;
                                                                if let Some(l_mult_set_rep) = l_mult_set.first() {
                                                                    if let Some(r_mult_set_rep) = r_mult_set.first() {
                                                                        if let Some(r_div_set_rep) = r_div_set.first() {
                                                                            let new_pat_expr = Rc::new(
                                                                                Calc(
                                                                                    "*",
                                                                                    Rc::new(EClass(*l_mult_set_rep)),
                                                                                    Rc::new(
                                                                                        Calc(
                                                                                            "/",
                                                                                            Rc::new(EClass(*r_mult_set_rep)),
                                                                                            Rc::new(EClass(*r_div_set_rep)),
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            );
                                                                            let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                                                                ascent::internal::Convert::convert(new_pat_expr),
                                                                                ascent::internal::Convert::convert(div_e_id),
                                                                            );
                                                                            let __new_row_ind = _self.do_union_pattern.len();
                                                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                &do_union_pattern_indices_0_1_total,
                                                                                &__new_row,
                                                                            )
                                                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                    do_union_pattern_indices_0_1_delta,
                                                                                    &__new_row,
                                                                                )
                                                                                && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                                    &mut do_union_pattern_indices_0_1_new,
                                                                                    &__new_row,
                                                                                    __new_row_ind,
                                                                                )
                                                                            {
                                                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                                                    &mut do_union_pattern_indices__new,
                                                                                    (),
                                                                                    __new_row_ind,
                                                                                );
                                                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                                                    &mut do_union_pattern_indices_0_new,
                                                                                    (__new_row.0.clone(),),
                                                                                    __new_row_ind,
                                                                                );
                                                                                _self.do_union_pattern.push(__new_row);
                                                                                __changed = true;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment(
                        "do_union_pattern <-- rewrite_rule_indices_0_total, calc_expr_3_left_indices_1_total, calc_expr_3_right_indices_0_1_total, for_div_l_id, calc_expr_3_left_indices_0_1_total, calc_expr_3_right_indices_0_1_delta, if let ⋯, if let ⋯, if let ⋯, let ⋯",
                    );
                    if let Some(__matching)
                        = rewrite_rule_indices_0_total
                            .index_get(&(("mul-comm-1").clone(),))
                    {
                        for __ind in __matching {
                            let __row = &_self.rewrite_rule[__ind].clone();
                            if let Some(__matching)
                                = calc_expr_3_left_indices_1_total
                                    .index_get(&(("/").clone(),))
                            {
                                for __ind in __matching {
                                    let __row = &_self.calc_expr_3_left[__ind].clone();
                                    let div_e_id = &__row.0;
                                    let l_div_set = &__row.2;
                                    if let Some(__matching)
                                        = calc_expr_3_right_indices_0_1_total
                                            .index_get(&(div_e_id.clone(), ("/").clone()))
                                    {
                                        for __ind in __matching {
                                            let __row = &_self.calc_expr_3_right[__ind].clone();
                                            let r_div_set = &__row.2;
                                            for div_l_id in l_div_set.iter() {
                                                if let Some(__matching)
                                                    = calc_expr_3_left_indices_0_1_total
                                                        .index_get(&(div_l_id.clone(), ("*").clone()))
                                                {
                                                    for __ind in __matching {
                                                        let __row = &_self.calc_expr_3_left[__ind].clone();
                                                        let l_mult_set = &__row.2;
                                                        if let Some(__matching)
                                                            = calc_expr_3_right_indices_0_1_delta
                                                                .index_get(&(div_l_id.clone(), ("*").clone()))
                                                        {
                                                            for __ind in __matching {
                                                                let __row = &_self.calc_expr_3_right[__ind].clone();
                                                                let r_mult_set = &__row.2;
                                                                if let Some(l_mult_set_rep) = l_mult_set.first() {
                                                                    if let Some(r_mult_set_rep) = r_mult_set.first() {
                                                                        if let Some(r_div_set_rep) = r_div_set.first() {
                                                                            let new_pat_expr = Rc::new(
                                                                                Calc(
                                                                                    "*",
                                                                                    Rc::new(EClass(*l_mult_set_rep)),
                                                                                    Rc::new(
                                                                                        Calc(
                                                                                            "/",
                                                                                            Rc::new(EClass(*r_mult_set_rep)),
                                                                                            Rc::new(EClass(*r_div_set_rep)),
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            );
                                                                            let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                                                                ascent::internal::Convert::convert(new_pat_expr),
                                                                                ascent::internal::Convert::convert(div_e_id),
                                                                            );
                                                                            let __new_row_ind = _self.do_union_pattern.len();
                                                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                &do_union_pattern_indices_0_1_total,
                                                                                &__new_row,
                                                                            )
                                                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                    do_union_pattern_indices_0_1_delta,
                                                                                    &__new_row,
                                                                                )
                                                                                && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                                    &mut do_union_pattern_indices_0_1_new,
                                                                                    &__new_row,
                                                                                    __new_row_ind,
                                                                                )
                                                                            {
                                                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                                                    &mut do_union_pattern_indices__new,
                                                                                    (),
                                                                                    __new_row_ind,
                                                                                );
                                                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                                                    &mut do_union_pattern_indices_0_new,
                                                                                    (__new_row.0.clone(),),
                                                                                    __new_row_ind,
                                                                                );
                                                                                _self.do_union_pattern.push(__new_row);
                                                                                __changed = true;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment(
                        "do_union_pattern <-- rewrite_rule_indices_0_total, e_node_indices__delta, calc_expr_3_left_indices__total+delta, if ⋯, if let ⋯, let ⋯",
                    );
                    if let Some(__matching)
                        = rewrite_rule_indices_0_total
                            .index_get(&(("expand-mul-1").clone(),))
                    {
                        for __ind in __matching {
                            let __row = &_self.rewrite_rule[__ind].clone();
                            if let Some(__matching)
                                = e_node_indices__delta.index_get(&())
                            {
                                for __ind in __matching {
                                    let __row = &_self.e_node[__ind].clone();
                                    let e_id = &__row.0;
                                    if let Some(__matching)
                                        = ascent::internal::RelIndexCombined::new(
                                                &calc_expr_3_left_indices__total,
                                                calc_expr_3_left_indices__delta,
                                            )
                                            .index_get(&())
                                    {
                                        for __ind in __matching {
                                            let __row = &_self.calc_expr_3_left[__ind].clone();
                                            let __1 = &__row.0;
                                            let __2 = &__row.1;
                                            let eq_set = &__row.2;
                                            if eq_set.contains(e_id) {
                                                if let Some(eq_representive_node) = eq_set.first() {
                                                    let new_pat_expr = Rc::new(
                                                        Calc(
                                                            "*",
                                                            Rc::new(EClass(*eq_representive_node)),
                                                            Rc::new(Num(1)),
                                                        ),
                                                    );
                                                    let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                                        ascent::internal::Convert::convert(new_pat_expr),
                                                        ascent::internal::Convert::convert(e_id),
                                                    );
                                                    let __new_row_ind = _self.do_union_pattern.len();
                                                    if !::ascent::internal::RelFullIndexRead::contains_key(
                                                        &do_union_pattern_indices_0_1_total,
                                                        &__new_row,
                                                    )
                                                        && !::ascent::internal::RelFullIndexRead::contains_key(
                                                            do_union_pattern_indices_0_1_delta,
                                                            &__new_row,
                                                        )
                                                        && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                            &mut do_union_pattern_indices_0_1_new,
                                                            &__new_row,
                                                            __new_row_ind,
                                                        )
                                                    {
                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                            &mut do_union_pattern_indices__new,
                                                            (),
                                                            __new_row_ind,
                                                        );
                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                            &mut do_union_pattern_indices_0_new,
                                                            (__new_row.0.clone(),),
                                                            __new_row_ind,
                                                        );
                                                        _self.do_union_pattern.push(__new_row);
                                                        __changed = true;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment(
                        "do_union_pattern <-- rewrite_rule_indices_0_total, e_node_indices__total, calc_expr_3_left_indices__delta, if ⋯, if let ⋯, let ⋯",
                    );
                    if let Some(__matching)
                        = rewrite_rule_indices_0_total
                            .index_get(&(("expand-mul-1").clone(),))
                    {
                        for __ind in __matching {
                            let __row = &_self.rewrite_rule[__ind].clone();
                            if let Some(__matching)
                                = e_node_indices__total.index_get(&())
                            {
                                for __ind in __matching {
                                    let __row = &_self.e_node[__ind].clone();
                                    let e_id = &__row.0;
                                    if let Some(__matching)
                                        = calc_expr_3_left_indices__delta.index_get(&())
                                    {
                                        for __ind in __matching {
                                            let __row = &_self.calc_expr_3_left[__ind].clone();
                                            let __1 = &__row.0;
                                            let __2 = &__row.1;
                                            let eq_set = &__row.2;
                                            if eq_set.contains(e_id) {
                                                if let Some(eq_representive_node) = eq_set.first() {
                                                    let new_pat_expr = Rc::new(
                                                        Calc(
                                                            "*",
                                                            Rc::new(EClass(*eq_representive_node)),
                                                            Rc::new(Num(1)),
                                                        ),
                                                    );
                                                    let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                                        ascent::internal::Convert::convert(new_pat_expr),
                                                        ascent::internal::Convert::convert(e_id),
                                                    );
                                                    let __new_row_ind = _self.do_union_pattern.len();
                                                    if !::ascent::internal::RelFullIndexRead::contains_key(
                                                        &do_union_pattern_indices_0_1_total,
                                                        &__new_row,
                                                    )
                                                        && !::ascent::internal::RelFullIndexRead::contains_key(
                                                            do_union_pattern_indices_0_1_delta,
                                                            &__new_row,
                                                        )
                                                        && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                            &mut do_union_pattern_indices_0_1_new,
                                                            &__new_row,
                                                            __new_row_ind,
                                                        )
                                                    {
                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                            &mut do_union_pattern_indices__new,
                                                            (),
                                                            __new_row_ind,
                                                        );
                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                            &mut do_union_pattern_indices_0_new,
                                                            (__new_row.0.clone(),),
                                                            __new_row_ind,
                                                        );
                                                        _self.do_union_pattern.push(__new_row);
                                                        __changed = true;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment(
                        "do_union_pattern <-- rewrite_rule_indices_0_total, calc_expr_3_left_indices_1_delta, calc_expr_3_right_indices_0_1_total+delta, if ⋯, let ⋯",
                    );
                    if let Some(__matching)
                        = rewrite_rule_indices_0_total
                            .index_get(&(("elim-div-1").clone(),))
                    {
                        for __ind in __matching {
                            let __row = &_self.rewrite_rule[__ind].clone();
                            if let Some(__matching)
                                = calc_expr_3_left_indices_1_delta
                                    .index_get(&(("/").clone(),))
                            {
                                for __ind in __matching {
                                    let __row = &_self.calc_expr_3_left[__ind].clone();
                                    let e_id = &__row.0;
                                    let l_set = &__row.2;
                                    if let Some(__matching)
                                        = ascent::internal::RelIndexCombined::new(
                                                &calc_expr_3_right_indices_0_1_total,
                                                calc_expr_3_right_indices_0_1_delta,
                                            )
                                            .index_get(&(e_id.clone(), ("/").clone()))
                                    {
                                        for __ind in __matching {
                                            let __row = &_self.calc_expr_3_right[__ind].clone();
                                            let r_set = &__row.2;
                                            if !l_set.is_disjoint(r_set) {
                                                let new_pat_expr = Rc::new(Num(1));
                                                let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                                    ascent::internal::Convert::convert(new_pat_expr),
                                                    ascent::internal::Convert::convert(e_id),
                                                );
                                                let __new_row_ind = _self.do_union_pattern.len();
                                                if !::ascent::internal::RelFullIndexRead::contains_key(
                                                    &do_union_pattern_indices_0_1_total,
                                                    &__new_row,
                                                )
                                                    && !::ascent::internal::RelFullIndexRead::contains_key(
                                                        do_union_pattern_indices_0_1_delta,
                                                        &__new_row,
                                                    )
                                                    && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                        &mut do_union_pattern_indices_0_1_new,
                                                        &__new_row,
                                                        __new_row_ind,
                                                    )
                                                {
                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                        &mut do_union_pattern_indices__new,
                                                        (),
                                                        __new_row_ind,
                                                    );
                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                        &mut do_union_pattern_indices_0_new,
                                                        (__new_row.0.clone(),),
                                                        __new_row_ind,
                                                    );
                                                    _self.do_union_pattern.push(__new_row);
                                                    __changed = true;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment(
                        "do_union_pattern <-- rewrite_rule_indices_0_total, calc_expr_3_left_indices_1_total, calc_expr_3_right_indices_0_1_delta, if ⋯, let ⋯",
                    );
                    if let Some(__matching)
                        = rewrite_rule_indices_0_total
                            .index_get(&(("elim-div-1").clone(),))
                    {
                        for __ind in __matching {
                            let __row = &_self.rewrite_rule[__ind].clone();
                            if let Some(__matching)
                                = calc_expr_3_left_indices_1_total
                                    .index_get(&(("/").clone(),))
                            {
                                for __ind in __matching {
                                    let __row = &_self.calc_expr_3_left[__ind].clone();
                                    let e_id = &__row.0;
                                    let l_set = &__row.2;
                                    if let Some(__matching)
                                        = calc_expr_3_right_indices_0_1_delta
                                            .index_get(&(e_id.clone(), ("/").clone()))
                                    {
                                        for __ind in __matching {
                                            let __row = &_self.calc_expr_3_right[__ind].clone();
                                            let r_set = &__row.2;
                                            if !l_set.is_disjoint(r_set) {
                                                let new_pat_expr = Rc::new(Num(1));
                                                let __new_row: (Rc<PatternExpr>, ENodeId) = (
                                                    ascent::internal::Convert::convert(new_pat_expr),
                                                    ascent::internal::Convert::convert(e_id),
                                                );
                                                let __new_row_ind = _self.do_union_pattern.len();
                                                if !::ascent::internal::RelFullIndexRead::contains_key(
                                                    &do_union_pattern_indices_0_1_total,
                                                    &__new_row,
                                                )
                                                    && !::ascent::internal::RelFullIndexRead::contains_key(
                                                        do_union_pattern_indices_0_1_delta,
                                                        &__new_row,
                                                    )
                                                    && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                        &mut do_union_pattern_indices_0_1_new,
                                                        &__new_row,
                                                        __new_row_ind,
                                                    )
                                                {
                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                        &mut do_union_pattern_indices__new,
                                                        (),
                                                        __new_row_ind,
                                                    );
                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                        &mut do_union_pattern_indices_0_new,
                                                        (__new_row.0.clone(),),
                                                        __new_row_ind,
                                                    );
                                                    _self.do_union_pattern.push(__new_row);
                                                    __changed = true;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment(
                        "do_add_new_expr, do_add_new_expr <-- do_union_pattern_indices__delta, if let ⋯",
                    );
                    if let Some(__matching)
                        = do_union_pattern_indices__delta.index_get(&())
                    {
                        for __ind in __matching {
                            let __row = &_self.do_union_pattern[__ind].clone();
                            let pat = &__row.0;
                            let __1 = &__row.1;
                            if let Calc(op, l, r) = pat.deref() {
                                let __new_row: (Rc<PatternExpr>,) = (
                                    ascent::internal::Convert::convert(l),
                                );
                                let __new_row_ind = _self.do_add_new_expr.len();
                                if !::ascent::internal::RelFullIndexRead::contains_key(
                                    &do_add_new_expr_indices_0_total,
                                    &__new_row,
                                )
                                    && !::ascent::internal::RelFullIndexRead::contains_key(
                                        do_add_new_expr_indices_0_delta,
                                        &__new_row,
                                    )
                                    && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                        &mut do_add_new_expr_indices_0_new,
                                        &__new_row,
                                        __new_row_ind,
                                    )
                                {
                                    ::ascent::internal::RelIndexWrite::index_insert(
                                        &mut do_add_new_expr_indices__new,
                                        (),
                                        __new_row_ind,
                                    );
                                    _self.do_add_new_expr.push(__new_row);
                                    __changed = true;
                                }
                                let __new_row: (Rc<PatternExpr>,) = (
                                    ascent::internal::Convert::convert(r),
                                );
                                let __new_row_ind = _self.do_add_new_expr.len();
                                if !::ascent::internal::RelFullIndexRead::contains_key(
                                    &do_add_new_expr_indices_0_total,
                                    &__new_row,
                                )
                                    && !::ascent::internal::RelFullIndexRead::contains_key(
                                        do_add_new_expr_indices_0_delta,
                                        &__new_row,
                                    )
                                    && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                        &mut do_add_new_expr_indices_0_new,
                                        &__new_row,
                                        __new_row_ind,
                                    )
                                {
                                    ::ascent::internal::RelIndexWrite::index_insert(
                                        &mut do_add_new_expr_indices__new,
                                        (),
                                        __new_row_ind,
                                    );
                                    _self.do_add_new_expr.push(__new_row);
                                    __changed = true;
                                }
                            }
                        }
                    }
                    ascent::internal::comment(
                        "do_uinon_id <-- do_union_pattern_indices_0_delta, e_node_match_indices_0_total+delta [SIMPLE JOIN]",
                    );
                    if do_union_pattern_indices_0_delta.len()
                        <= ascent::internal::RelIndexCombined::new(
                                &e_node_match_indices_0_total,
                                e_node_match_indices_0_delta,
                            )
                            .len()
                    {
                        for (__cl1_joined_columns, __cl1_tuple_indices) in do_union_pattern_indices_0_delta
                            .iter_all()
                        {
                            let pat = &__cl1_joined_columns.0;
                            if let Some(__matching)
                                = ascent::internal::RelIndexCombined::new(
                                        &e_node_match_indices_0_total,
                                        e_node_match_indices_0_delta,
                                    )
                                    .index_get(&(pat.clone(),))
                            {
                                for cl1_ind in __cl1_tuple_indices {
                                    let __row = &_self.do_union_pattern[cl1_ind].clone();
                                    let e_id = &__row.1;
                                    for __ind in __matching.clone() {
                                        let __row = &_self.e_node_match[__ind].clone();
                                        let new_e_id = &__row.1;
                                        let __new_row: (ENodeId, ENodeId) = (
                                            ascent::internal::Convert::convert(e_id),
                                            ascent::internal::Convert::convert(new_e_id),
                                        );
                                        let __new_row_ind = _self.do_uinon_id.len();
                                        if !::ascent::internal::RelFullIndexRead::contains_key(
                                            &do_uinon_id_indices_0_1_total,
                                            &__new_row,
                                        )
                                            && !::ascent::internal::RelFullIndexRead::contains_key(
                                                do_uinon_id_indices_0_1_delta,
                                                &__new_row,
                                            )
                                            && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                &mut do_uinon_id_indices_0_1_new,
                                                &__new_row,
                                                __new_row_ind,
                                            )
                                        {
                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                &mut do_uinon_id_indices__new,
                                                (),
                                                __new_row_ind,
                                            );
                                            _self.do_uinon_id.push(__new_row);
                                            __changed = true;
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        for (__cl1_joined_columns, __cl1_tuple_indices) in ascent::internal::RelIndexCombined::new(
                                &e_node_match_indices_0_total,
                                e_node_match_indices_0_delta,
                            )
                            .iter_all()
                        {
                            let pat = &__cl1_joined_columns.0;
                            if let Some(__matching)
                                = do_union_pattern_indices_0_delta
                                    .index_get(&(pat.clone(),))
                            {
                                for cl1_ind in __cl1_tuple_indices {
                                    let __row = &_self.e_node_match[cl1_ind].clone();
                                    let new_e_id = &__row.1;
                                    for __ind in __matching.clone() {
                                        let __row = &_self.do_union_pattern[__ind].clone();
                                        let e_id = &__row.1;
                                        let __new_row: (ENodeId, ENodeId) = (
                                            ascent::internal::Convert::convert(e_id),
                                            ascent::internal::Convert::convert(new_e_id),
                                        );
                                        let __new_row_ind = _self.do_uinon_id.len();
                                        if !::ascent::internal::RelFullIndexRead::contains_key(
                                            &do_uinon_id_indices_0_1_total,
                                            &__new_row,
                                        )
                                            && !::ascent::internal::RelFullIndexRead::contains_key(
                                                do_uinon_id_indices_0_1_delta,
                                                &__new_row,
                                            )
                                            && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                &mut do_uinon_id_indices_0_1_new,
                                                &__new_row,
                                                __new_row_ind,
                                            )
                                        {
                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                &mut do_uinon_id_indices__new,
                                                (),
                                                __new_row_ind,
                                            );
                                            _self.do_uinon_id.push(__new_row);
                                            __changed = true;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment(
                        "do_uinon_id <-- do_union_pattern_indices_0_total, e_node_match_indices_0_delta [SIMPLE JOIN]",
                    );
                    if do_union_pattern_indices_0_total.len()
                        <= e_node_match_indices_0_delta.len()
                    {
                        for (__cl1_joined_columns, __cl1_tuple_indices) in do_union_pattern_indices_0_total
                            .iter_all()
                        {
                            let pat = &__cl1_joined_columns.0;
                            if let Some(__matching)
                                = e_node_match_indices_0_delta.index_get(&(pat.clone(),))
                            {
                                for cl1_ind in __cl1_tuple_indices {
                                    let __row = &_self.do_union_pattern[cl1_ind].clone();
                                    let e_id = &__row.1;
                                    for __ind in __matching.clone() {
                                        let __row = &_self.e_node_match[__ind].clone();
                                        let new_e_id = &__row.1;
                                        let __new_row: (ENodeId, ENodeId) = (
                                            ascent::internal::Convert::convert(e_id),
                                            ascent::internal::Convert::convert(new_e_id),
                                        );
                                        let __new_row_ind = _self.do_uinon_id.len();
                                        if !::ascent::internal::RelFullIndexRead::contains_key(
                                            &do_uinon_id_indices_0_1_total,
                                            &__new_row,
                                        )
                                            && !::ascent::internal::RelFullIndexRead::contains_key(
                                                do_uinon_id_indices_0_1_delta,
                                                &__new_row,
                                            )
                                            && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                &mut do_uinon_id_indices_0_1_new,
                                                &__new_row,
                                                __new_row_ind,
                                            )
                                        {
                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                &mut do_uinon_id_indices__new,
                                                (),
                                                __new_row_ind,
                                            );
                                            _self.do_uinon_id.push(__new_row);
                                            __changed = true;
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        for (__cl1_joined_columns, __cl1_tuple_indices) in e_node_match_indices_0_delta
                            .iter_all()
                        {
                            let pat = &__cl1_joined_columns.0;
                            if let Some(__matching)
                                = do_union_pattern_indices_0_total
                                    .index_get(&(pat.clone(),))
                            {
                                for cl1_ind in __cl1_tuple_indices {
                                    let __row = &_self.e_node_match[cl1_ind].clone();
                                    let new_e_id = &__row.1;
                                    for __ind in __matching.clone() {
                                        let __row = &_self.do_union_pattern[__ind].clone();
                                        let e_id = &__row.1;
                                        let __new_row: (ENodeId, ENodeId) = (
                                            ascent::internal::Convert::convert(e_id),
                                            ascent::internal::Convert::convert(new_e_id),
                                        );
                                        let __new_row_ind = _self.do_uinon_id.len();
                                        if !::ascent::internal::RelFullIndexRead::contains_key(
                                            &do_uinon_id_indices_0_1_total,
                                            &__new_row,
                                        )
                                            && !::ascent::internal::RelFullIndexRead::contains_key(
                                                do_uinon_id_indices_0_1_delta,
                                                &__new_row,
                                            )
                                            && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                &mut do_uinon_id_indices_0_1_new,
                                                &__new_row,
                                                __new_row_ind,
                                            )
                                        {
                                            ::ascent::internal::RelIndexWrite::index_insert(
                                                &mut do_uinon_id_indices__new,
                                                (),
                                                __new_row_ind,
                                            );
                                            _self.do_uinon_id.push(__new_row);
                                            __changed = true;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        do_match_indices__delta,
                        &mut do_match_indices__total,
                    );
                    std::mem::swap(&mut do_match_indices__new, do_match_indices__delta);
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        do_match_indices_0_1_delta,
                        &mut do_match_indices_0_1_total,
                    );
                    std::mem::swap(
                        &mut do_match_indices_0_1_new,
                        do_match_indices_0_1_delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        num_indices_0_1_delta,
                        &mut num_indices_0_1_total,
                    );
                    std::mem::swap(&mut num_indices_0_1_new, num_indices_0_1_delta);
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        num_indices__delta,
                        &mut num_indices__total,
                    );
                    std::mem::swap(&mut num_indices__new, num_indices__delta);
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        root_indices_0_1_delta,
                        &mut root_indices_0_1_total,
                    );
                    std::mem::swap(&mut root_indices_0_1_new, root_indices_0_1_delta);
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        root_indices_0_delta,
                        &mut root_indices_0_total,
                    );
                    std::mem::swap(&mut root_indices_0_new, root_indices_0_delta);
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        root_indices__delta,
                        &mut root_indices__total,
                    );
                    std::mem::swap(&mut root_indices__new, root_indices__delta);
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_match_indices_0_1_delta,
                        &mut e_node_match_indices_0_1_total,
                    );
                    std::mem::swap(
                        &mut e_node_match_indices_0_1_new,
                        e_node_match_indices_0_1_delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_match_indices__delta,
                        &mut e_node_match_indices__total,
                    );
                    std::mem::swap(
                        &mut e_node_match_indices__new,
                        e_node_match_indices__delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_match_indices_0_delta,
                        &mut e_node_match_indices_0_total,
                    );
                    std::mem::swap(
                        &mut e_node_match_indices_0_new,
                        e_node_match_indices_0_delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        calc_expr_3_right_indices__delta,
                        &mut calc_expr_3_right_indices__total,
                    );
                    std::mem::swap(
                        &mut calc_expr_3_right_indices__new,
                        calc_expr_3_right_indices__delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        calc_expr_3_right_indices_0_1_delta,
                        &mut calc_expr_3_right_indices_0_1_total,
                    );
                    std::mem::swap(
                        &mut calc_expr_3_right_indices_0_1_new,
                        calc_expr_3_right_indices_0_1_delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        calc_expr_3_right_indices_0_delta,
                        &mut calc_expr_3_right_indices_0_total,
                    );
                    std::mem::swap(
                        &mut calc_expr_3_right_indices_0_new,
                        calc_expr_3_right_indices_0_delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        calc_expr_3_right_indices_0_1_2_delta,
                        &mut calc_expr_3_right_indices_0_1_2_total,
                    );
                    std::mem::swap(
                        &mut calc_expr_3_right_indices_0_1_2_new,
                        calc_expr_3_right_indices_0_1_2_delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        do_union_pattern_indices__delta,
                        &mut do_union_pattern_indices__total,
                    );
                    std::mem::swap(
                        &mut do_union_pattern_indices__new,
                        do_union_pattern_indices__delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        do_union_pattern_indices_0_1_delta,
                        &mut do_union_pattern_indices_0_1_total,
                    );
                    std::mem::swap(
                        &mut do_union_pattern_indices_0_1_new,
                        do_union_pattern_indices_0_1_delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        do_union_pattern_indices_0_delta,
                        &mut do_union_pattern_indices_0_total,
                    );
                    std::mem::swap(
                        &mut do_union_pattern_indices_0_new,
                        do_union_pattern_indices_0_delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        calc_expr_3_left_indices_0_1_delta,
                        &mut calc_expr_3_left_indices_0_1_total,
                    );
                    std::mem::swap(
                        &mut calc_expr_3_left_indices_0_1_new,
                        calc_expr_3_left_indices_0_1_delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        calc_expr_3_left_indices_0_delta,
                        &mut calc_expr_3_left_indices_0_total,
                    );
                    std::mem::swap(
                        &mut calc_expr_3_left_indices_0_new,
                        calc_expr_3_left_indices_0_delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        calc_expr_3_left_indices_1_delta,
                        &mut calc_expr_3_left_indices_1_total,
                    );
                    std::mem::swap(
                        &mut calc_expr_3_left_indices_1_new,
                        calc_expr_3_left_indices_1_delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        calc_expr_3_left_indices_0_1_2_delta,
                        &mut calc_expr_3_left_indices_0_1_2_total,
                    );
                    std::mem::swap(
                        &mut calc_expr_3_left_indices_0_1_2_new,
                        calc_expr_3_left_indices_0_1_2_delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        calc_expr_3_left_indices__delta,
                        &mut calc_expr_3_left_indices__total,
                    );
                    std::mem::swap(
                        &mut calc_expr_3_left_indices__new,
                        calc_expr_3_left_indices__delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        var_indices__delta,
                        &mut var_indices__total,
                    );
                    std::mem::swap(&mut var_indices__new, var_indices__delta);
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        var_indices_0_1_delta,
                        &mut var_indices_0_1_total,
                    );
                    std::mem::swap(&mut var_indices_0_1_new, var_indices_0_1_delta);
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_indices_0_delta,
                        &mut e_node_indices_0_total,
                    );
                    std::mem::swap(&mut e_node_indices_0_new, e_node_indices_0_delta);
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_indices__delta,
                        &mut e_node_indices__total,
                    );
                    std::mem::swap(&mut e_node_indices__new, e_node_indices__delta);
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        do_uinon_id_indices__delta,
                        &mut do_uinon_id_indices__total,
                    );
                    std::mem::swap(
                        &mut do_uinon_id_indices__new,
                        do_uinon_id_indices__delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        do_uinon_id_indices_0_1_delta,
                        &mut do_uinon_id_indices_0_1_total,
                    );
                    std::mem::swap(
                        &mut do_uinon_id_indices_0_1_new,
                        do_uinon_id_indices_0_1_delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        do_add_new_expr_indices_0_delta,
                        &mut do_add_new_expr_indices_0_total,
                    );
                    std::mem::swap(
                        &mut do_add_new_expr_indices_0_new,
                        do_add_new_expr_indices_0_delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        do_add_new_expr_indices__delta,
                        &mut do_add_new_expr_indices__total,
                    );
                    std::mem::swap(
                        &mut do_add_new_expr_indices__new,
                        do_add_new_expr_indices__delta,
                    );
                    if !__changed {
                        break;
                    }
                }
                _self.do_match_indices_ = do_match_indices__total;
                _self.do_match_indices_0_1 = do_match_indices_0_1_total;
                _self.num_indices_0_1 = num_indices_0_1_total;
                _self.num_indices_ = num_indices__total;
                _self.root_indices_0_1 = root_indices_0_1_total;
                _self.root_indices_0 = root_indices_0_total;
                _self.root_indices_ = root_indices__total;
                _self.e_node_match_indices_0_1 = e_node_match_indices_0_1_total;
                _self.e_node_match_indices_ = e_node_match_indices__total;
                _self.e_node_match_indices_0 = e_node_match_indices_0_total;
                _self.calc_expr_3_right_indices_ = calc_expr_3_right_indices__total;
                _self
                    .calc_expr_3_right_indices_0_1 = calc_expr_3_right_indices_0_1_total;
                _self.calc_expr_3_right_indices_0 = calc_expr_3_right_indices_0_total;
                _self
                    .calc_expr_3_right_indices_0_1_2 = calc_expr_3_right_indices_0_1_2_total;
                _self.do_union_pattern_indices_ = do_union_pattern_indices__total;
                _self.do_union_pattern_indices_0_1 = do_union_pattern_indices_0_1_total;
                _self.do_union_pattern_indices_0 = do_union_pattern_indices_0_total;
                _self.calc_expr_3_left_indices_0_1 = calc_expr_3_left_indices_0_1_total;
                _self.calc_expr_3_left_indices_0 = calc_expr_3_left_indices_0_total;
                _self.calc_expr_3_left_indices_1 = calc_expr_3_left_indices_1_total;
                _self
                    .calc_expr_3_left_indices_0_1_2 = calc_expr_3_left_indices_0_1_2_total;
                _self.calc_expr_3_left_indices_ = calc_expr_3_left_indices__total;
                _self.var_indices_ = var_indices__total;
                _self.var_indices_0_1 = var_indices_0_1_total;
                _self.e_node_indices_0 = e_node_indices_0_total;
                _self.e_node_indices_ = e_node_indices__total;
                _self.do_uinon_id_indices_ = do_uinon_id_indices__total;
                _self.do_uinon_id_indices_0_1 = do_uinon_id_indices_0_1_total;
                _self.do_add_new_expr_indices_0 = do_add_new_expr_indices_0_total;
                _self.do_add_new_expr_indices_ = do_add_new_expr_indices__total;
                _self.scc1_duration += _scc_start_time.elapsed();
            }
            ascent::internal::comment("scc 2");
            {
                let _scc_start_time = ::ascent::internal::Instant::now();
                #[allow(non_snake_case)]
                let node_need_merge_indices_0_delta: &mut ascent::internal::LatticeIndexType<
                    (BTreeSet<ENodeId>,),
                > = &mut _self.node_need_merge_indices_0;
                #[allow(non_snake_case)]
                let mut node_need_merge_indices_0_total: ascent::internal::LatticeIndexType<
                    (BTreeSet<ENodeId>,),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut node_need_merge_indices_0_new: ascent::internal::LatticeIndexType<
                    (BTreeSet<ENodeId>,),
                > = Default::default();
                #[allow(non_snake_case)]
                let node_need_merge_indices_0_1_delta: &mut ascent::internal::RelFullIndexType<
                    (BTreeSet<ENodeId>, Set<ENodeId>),
                > = &mut _self.node_need_merge_indices_0_1;
                #[allow(non_snake_case)]
                let mut node_need_merge_indices_0_1_total: ascent::internal::RelFullIndexType<
                    (BTreeSet<ENodeId>, Set<ENodeId>),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut node_need_merge_indices_0_1_new: ascent::internal::RelFullIndexType<
                    (BTreeSet<ENodeId>, Set<ENodeId>),
                > = Default::default();
                #[allow(non_snake_case)]
                let calc_expr_3_right_indices_0_total: &mut ascent::internal::LatticeIndexType<
                    (ENodeId,),
                > = &mut _self.calc_expr_3_right_indices_0;
                #[allow(non_snake_case)]
                let root_indices__total: &mut ascent::internal::LatticeIndexType<()> = &mut _self
                    .root_indices_;
                #[allow(non_snake_case)]
                let calc_expr_3_left_indices_0_total: &mut ascent::internal::LatticeIndexType<
                    (ENodeId,),
                > = &mut _self.calc_expr_3_left_indices_0;
                #[allow(unused_assignments, unused_variables)]
                {
                    let mut __changed = false;
                    ascent::internal::comment(
                        "node_need_merge, node_need_merge <-- root_indices__total, for_n1, for_n2, if ⋯, calc_expr_3_left_indices_0_total, calc_expr_3_left_indices_0_total, if ⋯, calc_expr_3_right_indices_0_total, calc_expr_3_right_indices_0_total, if ⋯",
                    );
                    if let Some(__matching) = root_indices__total.index_get(&()) {
                        for __ind in __matching {
                            let __row = &_self.root[__ind].clone();
                            let __1 = &__row.0;
                            let eq_set = &__row.1;
                            for n1 in eq_set.iter() {
                                for n2 in eq_set.iter() {
                                    if n1 != n2 {
                                        if let Some(__matching)
                                            = calc_expr_3_left_indices_0_total.index_get(&(n1.clone(),))
                                        {
                                            for __ind in __matching {
                                                let __row = &_self.calc_expr_3_left[__ind].clone();
                                                let __2 = &__row.1;
                                                let n1_l_eq = &__row.2;
                                                if let Some(__matching)
                                                    = calc_expr_3_left_indices_0_total.index_get(&(n2.clone(),))
                                                {
                                                    for __ind in __matching {
                                                        let __row = &_self.calc_expr_3_left[__ind].clone();
                                                        let __3 = &__row.1;
                                                        let n2_l_eq = &__row.2;
                                                        if !n1_l_eq.is_disjoint(n2_l_eq) {
                                                            if let Some(__matching)
                                                                = calc_expr_3_right_indices_0_total
                                                                    .index_get(&(n1.clone(),))
                                                            {
                                                                for __ind in __matching {
                                                                    let __row = &_self.calc_expr_3_right[__ind].clone();
                                                                    let __4 = &__row.1;
                                                                    let n1_r_eq = &__row.2;
                                                                    if let Some(__matching)
                                                                        = calc_expr_3_right_indices_0_total
                                                                            .index_get(&(n2.clone(),))
                                                                    {
                                                                        for __ind in __matching {
                                                                            let __row = &_self.calc_expr_3_right[__ind].clone();
                                                                            let __5 = &__row.1;
                                                                            let n2_r_eq = &__row.2;
                                                                            if !n1_r_eq.is_disjoint(n2_r_eq) {
                                                                                let __new_row: (BTreeSet<ENodeId>, Set<ENodeId>) = (
                                                                                    eq_set.0.clone(),
                                                                                    Set::singleton(*n1),
                                                                                );
                                                                                let __lattice_key = (__new_row.0.clone(),);
                                                                                if let Some(__existing_ind)
                                                                                    = node_need_merge_indices_0_new
                                                                                        .get(&__lattice_key)
                                                                                        .or_else(|| {
                                                                                            node_need_merge_indices_0_delta.get(&__lattice_key)
                                                                                        })
                                                                                        .or_else(|| {
                                                                                            node_need_merge_indices_0_total.get(&__lattice_key)
                                                                                        })
                                                                                {
                                                                                    let __existing_ind = *__existing_ind.iter().next().unwrap();
                                                                                    let __lat_changed = ::ascent::Lattice::join_mut(
                                                                                        &mut _self.node_need_merge[__existing_ind].1,
                                                                                        __new_row.1.clone(),
                                                                                    );
                                                                                    if __lat_changed {
                                                                                        let __new_row_ind = __existing_ind;
                                                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                                                            &mut node_need_merge_indices_0_new,
                                                                                            (__new_row.0.clone(),),
                                                                                            __new_row_ind,
                                                                                        );
                                                                                        __changed = true;
                                                                                    }
                                                                                } else {
                                                                                    let __new_row_ind = _self.node_need_merge.len();
                                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                                        &mut node_need_merge_indices_0_new,
                                                                                        (__new_row.0.clone(),),
                                                                                        __new_row_ind,
                                                                                    );
                                                                                    _self.node_need_merge.push(__new_row);
                                                                                    __changed = true;
                                                                                }
                                                                                let __new_row: (BTreeSet<ENodeId>, Set<ENodeId>) = (
                                                                                    eq_set.0.clone(),
                                                                                    Set::singleton(*n2),
                                                                                );
                                                                                let __lattice_key = (__new_row.0.clone(),);
                                                                                if let Some(__existing_ind)
                                                                                    = node_need_merge_indices_0_new
                                                                                        .get(&__lattice_key)
                                                                                        .or_else(|| {
                                                                                            node_need_merge_indices_0_delta.get(&__lattice_key)
                                                                                        })
                                                                                        .or_else(|| {
                                                                                            node_need_merge_indices_0_total.get(&__lattice_key)
                                                                                        })
                                                                                {
                                                                                    let __existing_ind = *__existing_ind.iter().next().unwrap();
                                                                                    let __lat_changed = ::ascent::Lattice::join_mut(
                                                                                        &mut _self.node_need_merge[__existing_ind].1,
                                                                                        __new_row.1.clone(),
                                                                                    );
                                                                                    if __lat_changed {
                                                                                        let __new_row_ind = __existing_ind;
                                                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                                                            &mut node_need_merge_indices_0_new,
                                                                                            (__new_row.0.clone(),),
                                                                                            __new_row_ind,
                                                                                        );
                                                                                        __changed = true;
                                                                                    }
                                                                                } else {
                                                                                    let __new_row_ind = _self.node_need_merge.len();
                                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                                        &mut node_need_merge_indices_0_new,
                                                                                        (__new_row.0.clone(),),
                                                                                        __new_row_ind,
                                                                                    );
                                                                                    _self.node_need_merge.push(__new_row);
                                                                                    __changed = true;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        node_need_merge_indices_0_delta,
                        &mut node_need_merge_indices_0_total,
                    );
                    std::mem::swap(
                        &mut node_need_merge_indices_0_new,
                        node_need_merge_indices_0_delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        node_need_merge_indices_0_1_delta,
                        &mut node_need_merge_indices_0_1_total,
                    );
                    std::mem::swap(
                        &mut node_need_merge_indices_0_1_new,
                        node_need_merge_indices_0_1_delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        node_need_merge_indices_0_delta,
                        &mut node_need_merge_indices_0_total,
                    );
                    std::mem::swap(
                        &mut node_need_merge_indices_0_new,
                        node_need_merge_indices_0_delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        node_need_merge_indices_0_1_delta,
                        &mut node_need_merge_indices_0_1_total,
                    );
                    std::mem::swap(
                        &mut node_need_merge_indices_0_1_new,
                        node_need_merge_indices_0_1_delta,
                    );
                }
                _self.node_need_merge_indices_0 = node_need_merge_indices_0_total;
                _self.node_need_merge_indices_0_1 = node_need_merge_indices_0_1_total;
                _self.scc2_duration += _scc_start_time.elapsed();
            }
            ascent::internal::comment("scc 3");
            {
                let _scc_start_time = ::ascent::internal::Instant::now();
                #[allow(non_snake_case)]
                let node_need_merge_indices_0_delta: &mut ascent::internal::LatticeIndexType<
                    (BTreeSet<ENodeId>,),
                > = &mut _self.node_need_merge_indices_0;
                #[allow(non_snake_case)]
                let mut node_need_merge_indices_0_total: ascent::internal::LatticeIndexType<
                    (BTreeSet<ENodeId>,),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut node_need_merge_indices_0_new: ascent::internal::LatticeIndexType<
                    (BTreeSet<ENodeId>,),
                > = Default::default();
                #[allow(non_snake_case)]
                let node_need_merge_indices_0_1_delta: &mut ascent::internal::RelFullIndexType<
                    (BTreeSet<ENodeId>, Set<ENodeId>),
                > = &mut _self.node_need_merge_indices_0_1;
                #[allow(non_snake_case)]
                let mut node_need_merge_indices_0_1_total: ascent::internal::RelFullIndexType<
                    (BTreeSet<ENodeId>, Set<ENodeId>),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut node_need_merge_indices_0_1_new: ascent::internal::RelFullIndexType<
                    (BTreeSet<ENodeId>, Set<ENodeId>),
                > = Default::default();
                #[allow(non_snake_case)]
                let calc_expr_3_right_indices_0_total: &mut ascent::internal::LatticeIndexType<
                    (ENodeId,),
                > = &mut _self.calc_expr_3_right_indices_0;
                #[allow(non_snake_case)]
                let calc_expr_3_left_indices__total: &mut ascent::internal::LatticeIndexType<
                    (),
                > = &mut _self.calc_expr_3_left_indices_;
                #[allow(non_snake_case)]
                let calc_expr_3_left_indices_0_total: &mut ascent::internal::LatticeIndexType<
                    (ENodeId,),
                > = &mut _self.calc_expr_3_left_indices_0;
                #[allow(unused_assignments, unused_variables)]
                {
                    let mut __changed = false;
                    ascent::internal::comment(
                        "node_need_merge, node_need_merge <-- calc_expr_3_left_indices__total, for_n1, for_n2, if ⋯, calc_expr_3_left_indices_0_total, calc_expr_3_left_indices_0_total, if ⋯, calc_expr_3_right_indices_0_total, calc_expr_3_right_indices_0_total, if ⋯",
                    );
                    if let Some(__matching)
                        = calc_expr_3_left_indices__total.index_get(&())
                    {
                        for __ind in __matching {
                            let __row = &_self.calc_expr_3_left[__ind].clone();
                            let __1 = &__row.0;
                            let __2 = &__row.1;
                            let eq_set = &__row.2;
                            for n1 in eq_set.iter() {
                                for n2 in eq_set.iter() {
                                    if n1 != n2 {
                                        if let Some(__matching)
                                            = calc_expr_3_left_indices_0_total.index_get(&(n1.clone(),))
                                        {
                                            for __ind in __matching {
                                                let __row = &_self.calc_expr_3_left[__ind].clone();
                                                let __3 = &__row.1;
                                                let n1_l_eq = &__row.2;
                                                if let Some(__matching)
                                                    = calc_expr_3_left_indices_0_total.index_get(&(n2.clone(),))
                                                {
                                                    for __ind in __matching {
                                                        let __row = &_self.calc_expr_3_left[__ind].clone();
                                                        let __4 = &__row.1;
                                                        let n2_l_eq = &__row.2;
                                                        if !n1_l_eq.is_disjoint(n2_l_eq) {
                                                            if let Some(__matching)
                                                                = calc_expr_3_right_indices_0_total
                                                                    .index_get(&(n1.clone(),))
                                                            {
                                                                for __ind in __matching {
                                                                    let __row = &_self.calc_expr_3_right[__ind].clone();
                                                                    let __5 = &__row.1;
                                                                    let n1_r_eq = &__row.2;
                                                                    if let Some(__matching)
                                                                        = calc_expr_3_right_indices_0_total
                                                                            .index_get(&(n2.clone(),))
                                                                    {
                                                                        for __ind in __matching {
                                                                            let __row = &_self.calc_expr_3_right[__ind].clone();
                                                                            let __6 = &__row.1;
                                                                            let n2_r_eq = &__row.2;
                                                                            if !n1_r_eq.is_disjoint(n2_r_eq) {
                                                                                let __new_row: (BTreeSet<ENodeId>, Set<ENodeId>) = (
                                                                                    eq_set.0.clone(),
                                                                                    Set::singleton(*n1),
                                                                                );
                                                                                let __lattice_key = (__new_row.0.clone(),);
                                                                                if let Some(__existing_ind)
                                                                                    = node_need_merge_indices_0_new
                                                                                        .get(&__lattice_key)
                                                                                        .or_else(|| {
                                                                                            node_need_merge_indices_0_delta.get(&__lattice_key)
                                                                                        })
                                                                                        .or_else(|| {
                                                                                            node_need_merge_indices_0_total.get(&__lattice_key)
                                                                                        })
                                                                                {
                                                                                    let __existing_ind = *__existing_ind.iter().next().unwrap();
                                                                                    let __lat_changed = ::ascent::Lattice::join_mut(
                                                                                        &mut _self.node_need_merge[__existing_ind].1,
                                                                                        __new_row.1.clone(),
                                                                                    );
                                                                                    if __lat_changed {
                                                                                        let __new_row_ind = __existing_ind;
                                                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                                                            &mut node_need_merge_indices_0_new,
                                                                                            (__new_row.0.clone(),),
                                                                                            __new_row_ind,
                                                                                        );
                                                                                        __changed = true;
                                                                                    }
                                                                                } else {
                                                                                    let __new_row_ind = _self.node_need_merge.len();
                                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                                        &mut node_need_merge_indices_0_new,
                                                                                        (__new_row.0.clone(),),
                                                                                        __new_row_ind,
                                                                                    );
                                                                                    _self.node_need_merge.push(__new_row);
                                                                                    __changed = true;
                                                                                }
                                                                                let __new_row: (BTreeSet<ENodeId>, Set<ENodeId>) = (
                                                                                    eq_set.0.clone(),
                                                                                    Set::singleton(*n2),
                                                                                );
                                                                                let __lattice_key = (__new_row.0.clone(),);
                                                                                if let Some(__existing_ind)
                                                                                    = node_need_merge_indices_0_new
                                                                                        .get(&__lattice_key)
                                                                                        .or_else(|| {
                                                                                            node_need_merge_indices_0_delta.get(&__lattice_key)
                                                                                        })
                                                                                        .or_else(|| {
                                                                                            node_need_merge_indices_0_total.get(&__lattice_key)
                                                                                        })
                                                                                {
                                                                                    let __existing_ind = *__existing_ind.iter().next().unwrap();
                                                                                    let __lat_changed = ::ascent::Lattice::join_mut(
                                                                                        &mut _self.node_need_merge[__existing_ind].1,
                                                                                        __new_row.1.clone(),
                                                                                    );
                                                                                    if __lat_changed {
                                                                                        let __new_row_ind = __existing_ind;
                                                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                                                            &mut node_need_merge_indices_0_new,
                                                                                            (__new_row.0.clone(),),
                                                                                            __new_row_ind,
                                                                                        );
                                                                                        __changed = true;
                                                                                    }
                                                                                } else {
                                                                                    let __new_row_ind = _self.node_need_merge.len();
                                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                                        &mut node_need_merge_indices_0_new,
                                                                                        (__new_row.0.clone(),),
                                                                                        __new_row_ind,
                                                                                    );
                                                                                    _self.node_need_merge.push(__new_row);
                                                                                    __changed = true;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        node_need_merge_indices_0_delta,
                        &mut node_need_merge_indices_0_total,
                    );
                    std::mem::swap(
                        &mut node_need_merge_indices_0_new,
                        node_need_merge_indices_0_delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        node_need_merge_indices_0_1_delta,
                        &mut node_need_merge_indices_0_1_total,
                    );
                    std::mem::swap(
                        &mut node_need_merge_indices_0_1_new,
                        node_need_merge_indices_0_1_delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        node_need_merge_indices_0_delta,
                        &mut node_need_merge_indices_0_total,
                    );
                    std::mem::swap(
                        &mut node_need_merge_indices_0_new,
                        node_need_merge_indices_0_delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        node_need_merge_indices_0_1_delta,
                        &mut node_need_merge_indices_0_1_total,
                    );
                    std::mem::swap(
                        &mut node_need_merge_indices_0_1_new,
                        node_need_merge_indices_0_1_delta,
                    );
                }
                _self.node_need_merge_indices_0 = node_need_merge_indices_0_total;
                _self.node_need_merge_indices_0_1 = node_need_merge_indices_0_1_total;
                _self.scc3_duration += _scc_start_time.elapsed();
            }
            ascent::internal::comment("scc 4");
            {
                let _scc_start_time = ::ascent::internal::Instant::now();
                #[allow(non_snake_case)]
                let node_need_merge_indices_0_1_delta: &mut ascent::internal::RelFullIndexType<
                    (BTreeSet<ENodeId>, Set<ENodeId>),
                > = &mut _self.node_need_merge_indices_0_1;
                #[allow(non_snake_case)]
                let mut node_need_merge_indices_0_1_total: ascent::internal::RelFullIndexType<
                    (BTreeSet<ENodeId>, Set<ENodeId>),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut node_need_merge_indices_0_1_new: ascent::internal::RelFullIndexType<
                    (BTreeSet<ENodeId>, Set<ENodeId>),
                > = Default::default();
                #[allow(non_snake_case)]
                let node_need_merge_indices_0_delta: &mut ascent::internal::LatticeIndexType<
                    (BTreeSet<ENodeId>,),
                > = &mut _self.node_need_merge_indices_0;
                #[allow(non_snake_case)]
                let mut node_need_merge_indices_0_total: ascent::internal::LatticeIndexType<
                    (BTreeSet<ENodeId>,),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut node_need_merge_indices_0_new: ascent::internal::LatticeIndexType<
                    (BTreeSet<ENodeId>,),
                > = Default::default();
                #[allow(non_snake_case)]
                let calc_expr_3_right_indices__total: &mut ascent::internal::LatticeIndexType<
                    (),
                > = &mut _self.calc_expr_3_right_indices_;
                #[allow(non_snake_case)]
                let calc_expr_3_right_indices_0_total: &mut ascent::internal::LatticeIndexType<
                    (ENodeId,),
                > = &mut _self.calc_expr_3_right_indices_0;
                #[allow(non_snake_case)]
                let calc_expr_3_left_indices_0_total: &mut ascent::internal::LatticeIndexType<
                    (ENodeId,),
                > = &mut _self.calc_expr_3_left_indices_0;
                #[allow(unused_assignments, unused_variables)]
                {
                    let mut __changed = false;
                    ascent::internal::comment(
                        "node_need_merge, node_need_merge <-- calc_expr_3_right_indices__total, for_n1, for_n2, if ⋯, calc_expr_3_left_indices_0_total, calc_expr_3_left_indices_0_total, if ⋯, calc_expr_3_right_indices_0_total, calc_expr_3_right_indices_0_total, if ⋯",
                    );
                    if let Some(__matching)
                        = calc_expr_3_right_indices__total.index_get(&())
                    {
                        for __ind in __matching {
                            let __row = &_self.calc_expr_3_right[__ind].clone();
                            let __1 = &__row.0;
                            let __2 = &__row.1;
                            let eq_set = &__row.2;
                            for n1 in eq_set.iter() {
                                for n2 in eq_set.iter() {
                                    if n1 != n2 {
                                        if let Some(__matching)
                                            = calc_expr_3_left_indices_0_total.index_get(&(n1.clone(),))
                                        {
                                            for __ind in __matching {
                                                let __row = &_self.calc_expr_3_left[__ind].clone();
                                                let __3 = &__row.1;
                                                let n1_l_eq = &__row.2;
                                                if let Some(__matching)
                                                    = calc_expr_3_left_indices_0_total.index_get(&(n2.clone(),))
                                                {
                                                    for __ind in __matching {
                                                        let __row = &_self.calc_expr_3_left[__ind].clone();
                                                        let __4 = &__row.1;
                                                        let n2_l_eq = &__row.2;
                                                        if !n1_l_eq.is_disjoint(n2_l_eq) {
                                                            if let Some(__matching)
                                                                = calc_expr_3_right_indices_0_total
                                                                    .index_get(&(n1.clone(),))
                                                            {
                                                                for __ind in __matching {
                                                                    let __row = &_self.calc_expr_3_right[__ind].clone();
                                                                    let __5 = &__row.1;
                                                                    let n1_r_eq = &__row.2;
                                                                    if let Some(__matching)
                                                                        = calc_expr_3_right_indices_0_total
                                                                            .index_get(&(n2.clone(),))
                                                                    {
                                                                        for __ind in __matching {
                                                                            let __row = &_self.calc_expr_3_right[__ind].clone();
                                                                            let __6 = &__row.1;
                                                                            let n2_r_eq = &__row.2;
                                                                            if !n1_r_eq.is_disjoint(n2_r_eq) {
                                                                                let __new_row: (BTreeSet<ENodeId>, Set<ENodeId>) = (
                                                                                    eq_set.0.clone(),
                                                                                    Set::singleton(*n1),
                                                                                );
                                                                                let __lattice_key = (__new_row.0.clone(),);
                                                                                if let Some(__existing_ind)
                                                                                    = node_need_merge_indices_0_new
                                                                                        .get(&__lattice_key)
                                                                                        .or_else(|| {
                                                                                            node_need_merge_indices_0_delta.get(&__lattice_key)
                                                                                        })
                                                                                        .or_else(|| {
                                                                                            node_need_merge_indices_0_total.get(&__lattice_key)
                                                                                        })
                                                                                {
                                                                                    let __existing_ind = *__existing_ind.iter().next().unwrap();
                                                                                    let __lat_changed = ::ascent::Lattice::join_mut(
                                                                                        &mut _self.node_need_merge[__existing_ind].1,
                                                                                        __new_row.1.clone(),
                                                                                    );
                                                                                    if __lat_changed {
                                                                                        let __new_row_ind = __existing_ind;
                                                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                                                            &mut node_need_merge_indices_0_new,
                                                                                            (__new_row.0.clone(),),
                                                                                            __new_row_ind,
                                                                                        );
                                                                                        __changed = true;
                                                                                    }
                                                                                } else {
                                                                                    let __new_row_ind = _self.node_need_merge.len();
                                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                                        &mut node_need_merge_indices_0_new,
                                                                                        (__new_row.0.clone(),),
                                                                                        __new_row_ind,
                                                                                    );
                                                                                    _self.node_need_merge.push(__new_row);
                                                                                    __changed = true;
                                                                                }
                                                                                let __new_row: (BTreeSet<ENodeId>, Set<ENodeId>) = (
                                                                                    eq_set.0.clone(),
                                                                                    Set::singleton(*n2),
                                                                                );
                                                                                let __lattice_key = (__new_row.0.clone(),);
                                                                                if let Some(__existing_ind)
                                                                                    = node_need_merge_indices_0_new
                                                                                        .get(&__lattice_key)
                                                                                        .or_else(|| {
                                                                                            node_need_merge_indices_0_delta.get(&__lattice_key)
                                                                                        })
                                                                                        .or_else(|| {
                                                                                            node_need_merge_indices_0_total.get(&__lattice_key)
                                                                                        })
                                                                                {
                                                                                    let __existing_ind = *__existing_ind.iter().next().unwrap();
                                                                                    let __lat_changed = ::ascent::Lattice::join_mut(
                                                                                        &mut _self.node_need_merge[__existing_ind].1,
                                                                                        __new_row.1.clone(),
                                                                                    );
                                                                                    if __lat_changed {
                                                                                        let __new_row_ind = __existing_ind;
                                                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                                                            &mut node_need_merge_indices_0_new,
                                                                                            (__new_row.0.clone(),),
                                                                                            __new_row_ind,
                                                                                        );
                                                                                        __changed = true;
                                                                                    }
                                                                                } else {
                                                                                    let __new_row_ind = _self.node_need_merge.len();
                                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                                        &mut node_need_merge_indices_0_new,
                                                                                        (__new_row.0.clone(),),
                                                                                        __new_row_ind,
                                                                                    );
                                                                                    _self.node_need_merge.push(__new_row);
                                                                                    __changed = true;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        node_need_merge_indices_0_1_delta,
                        &mut node_need_merge_indices_0_1_total,
                    );
                    std::mem::swap(
                        &mut node_need_merge_indices_0_1_new,
                        node_need_merge_indices_0_1_delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        node_need_merge_indices_0_delta,
                        &mut node_need_merge_indices_0_total,
                    );
                    std::mem::swap(
                        &mut node_need_merge_indices_0_new,
                        node_need_merge_indices_0_delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        node_need_merge_indices_0_1_delta,
                        &mut node_need_merge_indices_0_1_total,
                    );
                    std::mem::swap(
                        &mut node_need_merge_indices_0_1_new,
                        node_need_merge_indices_0_1_delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        node_need_merge_indices_0_delta,
                        &mut node_need_merge_indices_0_total,
                    );
                    std::mem::swap(
                        &mut node_need_merge_indices_0_new,
                        node_need_merge_indices_0_delta,
                    );
                }
                _self.node_need_merge_indices_0_1 = node_need_merge_indices_0_1_total;
                _self.node_need_merge_indices_0 = node_need_merge_indices_0_total;
                _self.scc4_duration += _scc_start_time.elapsed();
            }
            ascent::internal::comment("scc 5");
            {
                let _scc_start_time = ::ascent::internal::Instant::now();
                #[allow(non_snake_case)]
                let new_expr_indices_0_delta: &mut ascent::internal::RelFullIndexType<
                    (Rc<PatternExpr>,),
                > = &mut _self.new_expr_indices_0;
                #[allow(non_snake_case)]
                let mut new_expr_indices_0_total: ascent::internal::RelFullIndexType<
                    (Rc<PatternExpr>,),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut new_expr_indices_0_new: ascent::internal::RelFullIndexType<
                    (Rc<PatternExpr>,),
                > = Default::default();
                #[allow(non_snake_case)]
                let do_add_new_expr_indices__total: &mut ascent::internal::RelIndexType<
                    (),
                > = &mut _self.do_add_new_expr_indices_;
                #[allow(non_snake_case)]
                let e_node_match_indices_0_total: &mut ascent::internal::RelIndexType<
                    (Rc<PatternExpr>,),
                > = &mut _self.e_node_match_indices_0;
                #[allow(unused_assignments, unused_variables)]
                {
                    let mut __changed = false;
                    ascent::internal::comment(
                        "new_expr <-- do_add_new_expr_indices__total, agg e_node_match_indices_0",
                    );
                    if let Some(__matching)
                        = do_add_new_expr_indices__total.index_get(&())
                    {
                        for __ind in __matching {
                            let __row = &_self.do_add_new_expr[__ind].clone();
                            let pat = &__row.0;
                            let __matching = e_node_match_indices_0_total
                                .index_get(&(pat.clone(),));
                            let __agregated_rel = &_self.e_node_match;
                            let __agg_args = __matching
                                .into_iter()
                                .flatten()
                                .map(|__ind| {
                                    let __row = &__agregated_rel[__ind];
                                    ()
                                });
                            for () in not(__agg_args) {
                                let __new_row: (Rc<PatternExpr>,) = (
                                    ascent::internal::Convert::convert(pat),
                                );
                                let __new_row_ind = _self.new_expr.len();
                                if !::ascent::internal::RelFullIndexRead::contains_key(
                                    &new_expr_indices_0_total,
                                    &__new_row,
                                )
                                    && !::ascent::internal::RelFullIndexRead::contains_key(
                                        new_expr_indices_0_delta,
                                        &__new_row,
                                    )
                                    && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                        &mut new_expr_indices_0_new,
                                        &__new_row,
                                        __new_row_ind,
                                    )
                                {
                                    _self.new_expr.push(__new_row);
                                    __changed = true;
                                }
                            }
                        }
                    }
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        new_expr_indices_0_delta,
                        &mut new_expr_indices_0_total,
                    );
                    std::mem::swap(
                        &mut new_expr_indices_0_new,
                        new_expr_indices_0_delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        new_expr_indices_0_delta,
                        &mut new_expr_indices_0_total,
                    );
                    std::mem::swap(
                        &mut new_expr_indices_0_new,
                        new_expr_indices_0_delta,
                    );
                }
                _self.new_expr_indices_0 = new_expr_indices_0_total;
                _self.scc5_duration += _scc_start_time.elapsed();
            }
        }
        fn update_indices_priv(&mut self) {
            for (i, tuple) in self.var.iter().enumerate() {
                let selection_tuple = ();
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.var_indices_,
                    selection_tuple,
                    i,
                );
                let selection_tuple = (tuple.0.clone(), tuple.1.clone());
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.var_indices_0_1,
                    selection_tuple,
                    i,
                );
            }
            for (i, tuple) in self.do_add_new_expr.iter().enumerate() {
                let selection_tuple = (tuple.0.clone(),);
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.do_add_new_expr_indices_0,
                    selection_tuple,
                    i,
                );
                let selection_tuple = ();
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.do_add_new_expr_indices_,
                    selection_tuple,
                    i,
                );
            }
            for (i, tuple) in self.assign_new_expr_id.iter().enumerate() {
                let selection_tuple = ();
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.assign_new_expr_id_indices_,
                    selection_tuple,
                    i,
                );
                let selection_tuple = (tuple.0.clone(),);
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.assign_new_expr_id_indices_0,
                    selection_tuple,
                    i,
                );
                let selection_tuple = (tuple.0.clone(), tuple.1.clone());
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.assign_new_expr_id_indices_0_1,
                    selection_tuple,
                    i,
                );
            }
            for (i, tuple) in self.rewrite_rule.iter().enumerate() {
                let selection_tuple = (tuple.0.clone(),);
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.rewrite_rule_indices_0,
                    selection_tuple,
                    i,
                );
            }
            for (i, tuple) in self.do_union_pattern.iter().enumerate() {
                let selection_tuple = (tuple.0.clone(),);
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.do_union_pattern_indices_0,
                    selection_tuple,
                    i,
                );
                let selection_tuple = (tuple.0.clone(), tuple.1.clone());
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.do_union_pattern_indices_0_1,
                    selection_tuple,
                    i,
                );
                let selection_tuple = ();
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.do_union_pattern_indices_,
                    selection_tuple,
                    i,
                );
            }
            for (i, tuple) in self.new_expr.iter().enumerate() {
                let selection_tuple = (tuple.0.clone(),);
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.new_expr_indices_0,
                    selection_tuple,
                    i,
                );
            }
            for (i, tuple) in self.root.iter().enumerate() {
                let selection_tuple = (tuple.0.clone(),);
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.root_indices_0,
                    selection_tuple,
                    i,
                );
                let selection_tuple = ();
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.root_indices_,
                    selection_tuple,
                    i,
                );
                let selection_tuple = (tuple.0.clone(), tuple.1.clone());
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.root_indices_0_1,
                    selection_tuple,
                    i,
                );
            }
            for (i, tuple) in self.e_node_match.iter().enumerate() {
                let selection_tuple = (tuple.0.clone(), tuple.1.clone());
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.e_node_match_indices_0_1,
                    selection_tuple,
                    i,
                );
                let selection_tuple = (tuple.0.clone(),);
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.e_node_match_indices_0,
                    selection_tuple,
                    i,
                );
                let selection_tuple = ();
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.e_node_match_indices_,
                    selection_tuple,
                    i,
                );
            }
            for (i, tuple) in self.calc_expr_3_left.iter().enumerate() {
                let selection_tuple = ();
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.calc_expr_3_left_indices_,
                    selection_tuple,
                    i,
                );
                let selection_tuple = (tuple.0.clone(), tuple.1.clone());
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.calc_expr_3_left_indices_0_1,
                    selection_tuple,
                    i,
                );
                let selection_tuple = (
                    tuple.0.clone(),
                    tuple.1.clone(),
                    tuple.2.clone(),
                );
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.calc_expr_3_left_indices_0_1_2,
                    selection_tuple,
                    i,
                );
                let selection_tuple = (tuple.0.clone(),);
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.calc_expr_3_left_indices_0,
                    selection_tuple,
                    i,
                );
                let selection_tuple = (tuple.1.clone(),);
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.calc_expr_3_left_indices_1,
                    selection_tuple,
                    i,
                );
            }
            for (i, tuple) in self.do_uinon_id.iter().enumerate() {
                let selection_tuple = (tuple.0.clone(), tuple.1.clone());
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.do_uinon_id_indices_0_1,
                    selection_tuple,
                    i,
                );
                let selection_tuple = ();
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.do_uinon_id_indices_,
                    selection_tuple,
                    i,
                );
            }
            for (i, tuple) in self.num.iter().enumerate() {
                let selection_tuple = ();
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.num_indices_,
                    selection_tuple,
                    i,
                );
                let selection_tuple = (tuple.0.clone(), tuple.1.clone());
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.num_indices_0_1,
                    selection_tuple,
                    i,
                );
            }
            for (i, tuple) in self.do_match.iter().enumerate() {
                let selection_tuple = ();
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.do_match_indices_,
                    selection_tuple,
                    i,
                );
                let selection_tuple = (tuple.0.clone(), tuple.1.clone());
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.do_match_indices_0_1,
                    selection_tuple,
                    i,
                );
            }
            for (i, tuple) in self.e_node.iter().enumerate() {
                let selection_tuple = ();
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.e_node_indices_,
                    selection_tuple,
                    i,
                );
                let selection_tuple = (tuple.0.clone(),);
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.e_node_indices_0,
                    selection_tuple,
                    i,
                );
            }
            for (i, tuple) in self.node_need_merge.iter().enumerate() {
                let selection_tuple = (tuple.0.clone(),);
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.node_need_merge_indices_0,
                    selection_tuple,
                    i,
                );
                let selection_tuple = (tuple.0.clone(), tuple.1.clone());
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.node_need_merge_indices_0_1,
                    selection_tuple,
                    i,
                );
            }
            for (i, tuple) in self.calc_expr_3_right.iter().enumerate() {
                let selection_tuple = (tuple.0.clone(),);
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.calc_expr_3_right_indices_0,
                    selection_tuple,
                    i,
                );
                let selection_tuple = (tuple.0.clone(), tuple.1.clone());
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.calc_expr_3_right_indices_0_1,
                    selection_tuple,
                    i,
                );
                let selection_tuple = (
                    tuple.0.clone(),
                    tuple.1.clone(),
                    tuple.2.clone(),
                );
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.calc_expr_3_right_indices_0_1_2,
                    selection_tuple,
                    i,
                );
                let selection_tuple = ();
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.calc_expr_3_right_indices_,
                    selection_tuple,
                    i,
                );
            }
        }
        #[deprecated = "Explicit call to update_indices not required anymore."]
        pub fn update_indices(&mut self) {
            self.update_indices_priv();
        }
        #[allow(unused_imports)]
        fn type_constaints() {
            let _type_constraints: ascent::internal::TypeConstraints<ENodeId>;
            let _type_constraints: ascent::internal::TypeConstraints<Sym>;
            let _type_constraints: ascent::internal::TypeConstraints<Rc<PatternExpr>>;
            let _type_constraints: ascent::internal::LatTypeConstraints<Set<ENodeId>>;
            let _type_constraints: ascent::internal::TypeConstraints<i32>;
            let _type_constraints: ascent::internal::TypeConstraints<BTreeSet<ENodeId>>;
        }
        pub fn summary() -> &'static str {
            "scc 0, is_looping: false:\n  e_node_match <-- assign_new_expr_id_indices__total\n  dynamic relations: e_node_match\nscc 1, is_looping: true:\n  calc_expr_3_left <-- calc_expr_3_left_indices__delta, do_uinon_id_indices__total+delta, if ⋯\n  calc_expr_3_left <-- calc_expr_3_left_indices__total, do_uinon_id_indices__delta, if ⋯\n  e_node <-- calc_expr_3_left_indices__delta\n  calc_expr_3_left, calc_expr_3_right <-- do_add_new_expr_indices_0_delta, assign_new_expr_id_indices_0_total, if let ⋯, e_node_match_indices_0_total+delta, e_node_match_indices_0_total+delta [SIMPLE JOIN]\n  calc_expr_3_left, calc_expr_3_right <-- do_add_new_expr_indices_0_total, assign_new_expr_id_indices_0_total, if let ⋯, e_node_match_indices_0_delta, e_node_match_indices_0_total+delta [SIMPLE JOIN]\n  calc_expr_3_left, calc_expr_3_right <-- do_add_new_expr_indices_0_total, assign_new_expr_id_indices_0_total, if let ⋯, e_node_match_indices_0_total, e_node_match_indices_0_delta [SIMPLE JOIN]\n  var <-- do_add_new_expr_indices_0_delta, assign_new_expr_id_indices_0_total, if let ⋯ [SIMPLE JOIN]\n  e_node <-- var_indices__delta\n  num <-- do_add_new_expr_indices_0_delta, assign_new_expr_id_indices_0_total, if let ⋯ [SIMPLE JOIN]\n  e_node <-- num_indices__delta\n  root <-- root_indices__delta, do_uinon_id_indices__total+delta, if ⋯\n  root <-- root_indices__total, do_uinon_id_indices__delta, if ⋯\n  e_node <-- root_indices__delta\n  do_match, do_match <-- do_match_indices__delta, if let ⋯, calc_expr_3_left_indices_0_1_total+delta, calc_expr_3_right_indices_0_1_total+delta, for_l_eq, for_r_eq\n  do_match, do_match <-- do_match_indices__total, if let ⋯, calc_expr_3_left_indices_0_1_delta, calc_expr_3_right_indices_0_1_total+delta, for_l_eq, for_r_eq\n  do_match, do_match <-- do_match_indices__total, if let ⋯, calc_expr_3_left_indices_0_1_total, calc_expr_3_right_indices_0_1_delta, for_l_eq, for_r_eq\n  do_match <-- do_add_new_expr_indices__delta, e_node_indices__total+delta [SIMPLE JOIN]\n  do_match <-- do_add_new_expr_indices__total, e_node_indices__delta [SIMPLE JOIN]\n  calc_expr_3_right <-- calc_expr_3_right_indices__delta, do_uinon_id_indices__total+delta, if ⋯\n  calc_expr_3_right <-- calc_expr_3_right_indices__total, do_uinon_id_indices__delta, if ⋯\n  e_node_match <-- do_match_indices__delta, if let ⋯, num_indices_0_1_total+delta\n  e_node_match <-- do_match_indices__total, if let ⋯, num_indices_0_1_delta\n  e_node_match <-- do_match_indices__delta, if let ⋯, var_indices_0_1_total+delta\n  e_node_match <-- do_match_indices__total, if let ⋯, var_indices_0_1_delta\n  e_node_match <-- do_match_indices__delta, root_indices_0_total+delta, for_root_eq_id, e_node_match_indices_0_1_total+delta\n  e_node_match <-- do_match_indices__total, root_indices_0_delta, for_root_eq_id, e_node_match_indices_0_1_total+delta\n  e_node_match <-- do_match_indices__total, root_indices_0_total, for_root_eq_id, e_node_match_indices_0_1_delta\n  e_node_match <-- do_match_indices__delta, if let ⋯, calc_expr_3_right_indices__total+delta, if ⋯, if ⋯\n  e_node_match <-- do_match_indices__total, if let ⋯, calc_expr_3_right_indices__delta, if ⋯, if ⋯\n  e_node_match <-- e_node_match_indices__delta, root_indices__total+delta, if ⋯, for_e_id\n  e_node_match <-- e_node_match_indices__total, root_indices__delta, if ⋯, for_e_id\n  e_node_match <-- do_match_indices__delta, if let ⋯, calc_expr_3_left_indices__total+delta, if ⋯, if ⋯\n  e_node_match <-- do_match_indices__total, if let ⋯, calc_expr_3_left_indices__delta, if ⋯, if ⋯\n  e_node_match <-- e_node_match_indices__delta, calc_expr_3_right_indices__total+delta, if ⋯, for_e_id\n  e_node_match <-- e_node_match_indices__total, calc_expr_3_right_indices__delta, if ⋯, for_e_id\n  e_node_match <-- e_node_match_indices__delta, calc_expr_3_left_indices__total+delta, if ⋯, for_e_id\n  e_node_match <-- e_node_match_indices__total, calc_expr_3_left_indices__delta, if ⋯, for_e_id\n  e_node_match <-- do_match_indices__delta, if let ⋯, root_indices__total+delta, if ⋯, if ⋯\n  e_node_match <-- do_match_indices__total, if let ⋯, root_indices__delta, if ⋯, if ⋯\n  e_node_match <-- do_match_indices__delta, if let ⋯, calc_expr_3_left_indices_0_1_total+delta, calc_expr_3_right_indices_0_1_total+delta, e_node_match_indices_0_total+delta, e_node_match_indices_0_total+delta, if ⋯, if ⋯\n  e_node_match <-- do_match_indices__total, if let ⋯, calc_expr_3_left_indices_0_1_delta, calc_expr_3_right_indices_0_1_total+delta, e_node_match_indices_0_total+delta, e_node_match_indices_0_total+delta, if ⋯, if ⋯\n  e_node_match <-- do_match_indices__total, if let ⋯, calc_expr_3_left_indices_0_1_total, calc_expr_3_right_indices_0_1_delta, e_node_match_indices_0_total+delta, e_node_match_indices_0_total+delta, if ⋯, if ⋯\n  e_node_match <-- do_match_indices__total, if let ⋯, calc_expr_3_left_indices_0_1_total, calc_expr_3_right_indices_0_1_total, e_node_match_indices_0_delta, e_node_match_indices_0_total+delta, if ⋯, if ⋯\n  e_node_match <-- do_match_indices__total, if let ⋯, calc_expr_3_left_indices_0_1_total, calc_expr_3_right_indices_0_1_total, e_node_match_indices_0_total, e_node_match_indices_0_delta, if ⋯, if ⋯\n  e_node_match <-- do_match_indices__delta, if let ⋯\n  do_union_pattern <-- rewrite_rule_indices_0_total, e_node_indices__delta, calc_expr_3_right_indices__total+delta, if ⋯, if let ⋯, let ⋯\n  do_union_pattern <-- rewrite_rule_indices_0_total, e_node_indices__total, calc_expr_3_right_indices__delta, if ⋯, if let ⋯, let ⋯\n  do_add_new_expr <-- do_union_pattern_indices__delta\n  do_union_pattern <-- rewrite_rule_indices_0_total, e_node_indices__delta, root_indices__total+delta, if ⋯, if let ⋯, let ⋯\n  do_union_pattern <-- rewrite_rule_indices_0_total, e_node_indices__total, root_indices__delta, if ⋯, if let ⋯, let ⋯\n  do_union_pattern <-- rewrite_rule_indices_0_total, calc_expr_3_left_indices_1_delta, calc_expr_3_right_indices_0_1_total+delta, for_div_l_id, calc_expr_3_left_indices_0_1_total+delta, calc_expr_3_right_indices_0_1_total+delta, if let ⋯, if let ⋯, if let ⋯, let ⋯\n  do_union_pattern <-- rewrite_rule_indices_0_total, calc_expr_3_left_indices_1_total, calc_expr_3_right_indices_0_1_delta, for_div_l_id, calc_expr_3_left_indices_0_1_total+delta, calc_expr_3_right_indices_0_1_total+delta, if let ⋯, if let ⋯, if let ⋯, let ⋯\n  do_union_pattern <-- rewrite_rule_indices_0_total, calc_expr_3_left_indices_1_total, calc_expr_3_right_indices_0_1_total, for_div_l_id, calc_expr_3_left_indices_0_1_delta, calc_expr_3_right_indices_0_1_total+delta, if let ⋯, if let ⋯, if let ⋯, let ⋯\n  do_union_pattern <-- rewrite_rule_indices_0_total, calc_expr_3_left_indices_1_total, calc_expr_3_right_indices_0_1_total, for_div_l_id, calc_expr_3_left_indices_0_1_total, calc_expr_3_right_indices_0_1_delta, if let ⋯, if let ⋯, if let ⋯, let ⋯\n  do_union_pattern <-- rewrite_rule_indices_0_total, e_node_indices__delta, calc_expr_3_left_indices__total+delta, if ⋯, if let ⋯, let ⋯\n  do_union_pattern <-- rewrite_rule_indices_0_total, e_node_indices__total, calc_expr_3_left_indices__delta, if ⋯, if let ⋯, let ⋯\n  do_union_pattern <-- rewrite_rule_indices_0_total, calc_expr_3_left_indices_1_delta, calc_expr_3_right_indices_0_1_total+delta, if ⋯, let ⋯\n  do_union_pattern <-- rewrite_rule_indices_0_total, calc_expr_3_left_indices_1_total, calc_expr_3_right_indices_0_1_delta, if ⋯, let ⋯\n  do_add_new_expr, do_add_new_expr <-- do_union_pattern_indices__delta, if let ⋯\n  do_uinon_id <-- do_union_pattern_indices_0_delta, e_node_match_indices_0_total+delta [SIMPLE JOIN]\n  do_uinon_id <-- do_union_pattern_indices_0_total, e_node_match_indices_0_delta [SIMPLE JOIN]\n  dynamic relations: do_match, num, root, e_node_match, calc_expr_3_right, do_union_pattern, calc_expr_3_left, var, e_node, do_uinon_id, do_add_new_expr\nscc 2, is_looping: false:\n  node_need_merge, node_need_merge <-- root_indices__total, for_n1, for_n2, if ⋯, calc_expr_3_left_indices_0_total, calc_expr_3_left_indices_0_total, if ⋯, calc_expr_3_right_indices_0_total, calc_expr_3_right_indices_0_total, if ⋯\n  dynamic relations: node_need_merge\nscc 3, is_looping: false:\n  node_need_merge, node_need_merge <-- calc_expr_3_left_indices__total, for_n1, for_n2, if ⋯, calc_expr_3_left_indices_0_total, calc_expr_3_left_indices_0_total, if ⋯, calc_expr_3_right_indices_0_total, calc_expr_3_right_indices_0_total, if ⋯\n  dynamic relations: node_need_merge\nscc 4, is_looping: false:\n  node_need_merge, node_need_merge <-- calc_expr_3_right_indices__total, for_n1, for_n2, if ⋯, calc_expr_3_left_indices_0_total, calc_expr_3_left_indices_0_total, if ⋯, calc_expr_3_right_indices_0_total, calc_expr_3_right_indices_0_total, if ⋯\n  dynamic relations: node_need_merge\nscc 5, is_looping: false:\n  new_expr <-- do_add_new_expr_indices__total, agg e_node_match_indices_0\n  dynamic relations: new_expr\n"
        }
        pub fn relation_sizes_summary(&self) -> String {
            use std::fmt::Write;
            let mut res = String::new();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["", " size: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"var"),
                            ::core::fmt::ArgumentV1::new_display(&self.var.len()),
                        ],
                    ),
                )
                .unwrap();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["", " size: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"do_add_new_expr"),
                            ::core::fmt::ArgumentV1::new_display(
                                &self.do_add_new_expr.len(),
                            ),
                        ],
                    ),
                )
                .unwrap();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["", " size: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"assign_new_expr_id"),
                            ::core::fmt::ArgumentV1::new_display(
                                &self.assign_new_expr_id.len(),
                            ),
                        ],
                    ),
                )
                .unwrap();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["", " size: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"rewrite_rule"),
                            ::core::fmt::ArgumentV1::new_display(
                                &self.rewrite_rule.len(),
                            ),
                        ],
                    ),
                )
                .unwrap();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["", " size: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"do_union_pattern"),
                            ::core::fmt::ArgumentV1::new_display(
                                &self.do_union_pattern.len(),
                            ),
                        ],
                    ),
                )
                .unwrap();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["", " size: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"new_expr"),
                            ::core::fmt::ArgumentV1::new_display(&self.new_expr.len()),
                        ],
                    ),
                )
                .unwrap();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["", " size: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"root"),
                            ::core::fmt::ArgumentV1::new_display(&self.root.len()),
                        ],
                    ),
                )
                .unwrap();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["", " size: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"e_node_match"),
                            ::core::fmt::ArgumentV1::new_display(
                                &self.e_node_match.len(),
                            ),
                        ],
                    ),
                )
                .unwrap();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["", " size: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"calc_expr_3_left"),
                            ::core::fmt::ArgumentV1::new_display(
                                &self.calc_expr_3_left.len(),
                            ),
                        ],
                    ),
                )
                .unwrap();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["", " size: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"do_uinon_id"),
                            ::core::fmt::ArgumentV1::new_display(&self.do_uinon_id.len()),
                        ],
                    ),
                )
                .unwrap();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["", " size: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"num"),
                            ::core::fmt::ArgumentV1::new_display(&self.num.len()),
                        ],
                    ),
                )
                .unwrap();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["", " size: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"do_match"),
                            ::core::fmt::ArgumentV1::new_display(&self.do_match.len()),
                        ],
                    ),
                )
                .unwrap();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["", " size: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"e_node"),
                            ::core::fmt::ArgumentV1::new_display(&self.e_node.len()),
                        ],
                    ),
                )
                .unwrap();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["", " size: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"node_need_merge"),
                            ::core::fmt::ArgumentV1::new_display(
                                &self.node_need_merge.len(),
                            ),
                        ],
                    ),
                )
                .unwrap();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["", " size: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"calc_expr_3_right"),
                            ::core::fmt::ArgumentV1::new_display(
                                &self.calc_expr_3_right.len(),
                            ),
                        ],
                    ),
                )
                .unwrap();
            res
        }
        pub fn scc_times_summary(&self) -> String {
            use std::fmt::Write;
            let mut res = String::new();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["scc ", " time: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"0"),
                            ::core::fmt::ArgumentV1::new_debug(&self.scc0_duration),
                        ],
                    ),
                )
                .unwrap();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["scc ", " time: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"1"),
                            ::core::fmt::ArgumentV1::new_debug(&self.scc1_duration),
                        ],
                    ),
                )
                .unwrap();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["scc ", " time: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"2"),
                            ::core::fmt::ArgumentV1::new_debug(&self.scc2_duration),
                        ],
                    ),
                )
                .unwrap();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["scc ", " time: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"3"),
                            ::core::fmt::ArgumentV1::new_debug(&self.scc3_duration),
                        ],
                    ),
                )
                .unwrap();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["scc ", " time: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"4"),
                            ::core::fmt::ArgumentV1::new_debug(&self.scc4_duration),
                        ],
                    ),
                )
                .unwrap();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["scc ", " time: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"5"),
                            ::core::fmt::ArgumentV1::new_debug(&self.scc5_duration),
                        ],
                    ),
                )
                .unwrap();
            res
        }
    }
    impl Default for EGraphRewrite {
        fn default() -> Self {
            let mut _self = EGraphRewrite {
                var: Default::default(),
                var_indices_: Default::default(),
                var_indices_0_1: Default::default(),
                do_add_new_expr: Default::default(),
                do_add_new_expr_indices_0: Default::default(),
                do_add_new_expr_indices_: Default::default(),
                assign_new_expr_id: Default::default(),
                assign_new_expr_id_indices_: Default::default(),
                assign_new_expr_id_indices_0: Default::default(),
                assign_new_expr_id_indices_0_1: Default::default(),
                rewrite_rule: Default::default(),
                rewrite_rule_indices_0: Default::default(),
                do_union_pattern: Default::default(),
                do_union_pattern_indices_0: Default::default(),
                do_union_pattern_indices_0_1: Default::default(),
                do_union_pattern_indices_: Default::default(),
                new_expr: Default::default(),
                new_expr_indices_0: Default::default(),
                root: Default::default(),
                root_indices_0: Default::default(),
                root_indices_: Default::default(),
                root_indices_0_1: Default::default(),
                e_node_match: Default::default(),
                e_node_match_indices_0_1: Default::default(),
                e_node_match_indices_0: Default::default(),
                e_node_match_indices_: Default::default(),
                calc_expr_3_left: Default::default(),
                calc_expr_3_left_indices_: Default::default(),
                calc_expr_3_left_indices_0_1: Default::default(),
                calc_expr_3_left_indices_0_1_2: Default::default(),
                calc_expr_3_left_indices_0: Default::default(),
                calc_expr_3_left_indices_1: Default::default(),
                do_uinon_id: Default::default(),
                do_uinon_id_indices_0_1: Default::default(),
                do_uinon_id_indices_: Default::default(),
                num: Default::default(),
                num_indices_: Default::default(),
                num_indices_0_1: Default::default(),
                do_match: Default::default(),
                do_match_indices_: Default::default(),
                do_match_indices_0_1: Default::default(),
                e_node: Default::default(),
                e_node_indices_: Default::default(),
                e_node_indices_0: Default::default(),
                node_need_merge: Default::default(),
                node_need_merge_indices_0: Default::default(),
                node_need_merge_indices_0_1: Default::default(),
                calc_expr_3_right: Default::default(),
                calc_expr_3_right_indices_0: Default::default(),
                calc_expr_3_right_indices_0_1: Default::default(),
                calc_expr_3_right_indices_0_1_2: Default::default(),
                calc_expr_3_right_indices_: Default::default(),
                scc0_duration: std::time::Duration::ZERO,
                scc1_duration: std::time::Duration::ZERO,
                scc2_duration: std::time::Duration::ZERO,
                scc3_duration: std::time::Duration::ZERO,
                scc4_duration: std::time::Duration::ZERO,
                scc5_duration: std::time::Duration::ZERO,
            };
            _self
        }
    }
    fn merge_node(g: &mut EGraphRewrite) -> EGraphRewrite {
        for (n_class, n_eqs) in g.node_need_merge.iter() {
            {
                ::std::io::_print(
                    ::core::fmt::Arguments::new_v1(
                        &["Node needs merge: ", " in set ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_debug(&n_eqs.deref()),
                            ::core::fmt::ArgumentV1::new_debug(&n_class),
                        ],
                    ),
                );
            };
            if let Some(selected) = n_eqs.first() {
                let delete_set: BTreeSet<ENodeId> = n_eqs
                    .deref()
                    .clone()
                    .into_iter()
                    .filter(|n_eq| n_eq != selected)
                    .collect();
                {
                    ::std::io::_print(
                        ::core::fmt::Arguments::new_v1(
                            &["", "\n"],
                            &[::core::fmt::ArgumentV1::new_debug(&delete_set)],
                        ),
                    );
                };
                let filtered_root = g
                    .root
                    .iter()
                    .filter_map(|(a, a_eq_set)| {
                        if delete_set.contains(a) {
                            None
                        } else {
                            let filted_set: Set<ENodeId> = Set(
                                a_eq_set
                                    .deref()
                                    .clone()
                                    .into_iter()
                                    .filter(|re| !delete_set.contains(re))
                                    .collect(),
                            );
                            Some((*a, filted_set))
                        }
                    })
                    .collect();
                let filtered_new_calc_expr_3_left = g
                    .calc_expr_3_left
                    .iter()
                    .filter_map(|(a, op, a_eq_set)| {
                        if delete_set.contains(a) {
                            None
                        } else {
                            let filted_set: Set<ENodeId> = Set(
                                a_eq_set
                                    .deref()
                                    .clone()
                                    .into_iter()
                                    .filter(|re| !delete_set.contains(re))
                                    .collect(),
                            );
                            Some((*a, *op, filted_set))
                        }
                    })
                    .collect();
                let filtered_calc_expr_3_right = g
                    .calc_expr_3_right
                    .iter()
                    .filter_map(|(a, op, a_eq_set)| {
                        if delete_set.contains(a) {
                            None
                        } else {
                            let filted_set: Set<ENodeId> = Set(
                                a_eq_set
                                    .deref()
                                    .clone()
                                    .into_iter()
                                    .filter(|re| !delete_set.contains(re))
                                    .collect(),
                            );
                            Some((*a, *op, filted_set))
                        }
                    })
                    .collect();
                g.root = filtered_root;
                g.calc_expr_3_left = filtered_new_calc_expr_3_left;
                g.calc_expr_3_right = filtered_calc_expr_3_right;
            }
        }
        let mut new_rw = EGraphRewrite::default();
        new_rw
            .rewrite_rule = <[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([
                ("elim-div-1",),
                ("expand-mul-1",),
                ("mul-comm-1",),
            ]),
        );
        new_rw.num = g.num.clone();
        new_rw.var = g.var.clone();
        new_rw.root = g.root.clone();
        new_rw.calc_expr_3_left = g.calc_expr_3_left.clone();
        new_rw.calc_expr_3_right = g.calc_expr_3_right.clone();
        new_rw
    }
    pub fn e_saturate(g: &EGraphData, max_iteration: usize) -> EGraphData {
        let mut rw_g = EGraphRewrite::default();
        rw_g
            .rewrite_rule = <[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([
                ("elim-div-1",),
                ("expand-mul-1",),
                ("mul-comm-1",),
            ]),
        );
        rw_g.root = g.root.clone();
        rw_g.calc_expr_3_left = g.calc_expr_3_left.clone();
        rw_g.calc_expr_3_right = g.calc_expr_3_right.clone();
        rw_g.var = g.var.clone();
        rw_g.num = g.num.clone();
        rw_g.e_node = g.e_node.clone();
        let mut cnt = 0;
        loop {
            rw_g.run();
            {
                ::std::io::_print(
                    ::core::fmt::Arguments::new_v1(
                        &["", " \n ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(
                                &rw_g.scc_times_summary(),
                            ),
                            ::core::fmt::ArgumentV1::new_display(
                                &rw_g.relation_sizes_summary(),
                            ),
                        ],
                    ),
                );
            };
            let new_expr = rw_g.new_expr.clone();
            if new_expr.is_empty() {
                break;
            }
            rw_g = merge_node(&mut rw_g);
            for (ne,) in new_expr.iter() {
                let new_id = match ne.deref() {
                    Var(_) => gen_id("Var"),
                    Num(_) => gen_id("Num"),
                    Calc(_, _, _) => gen_id("Calc"),
                    _ => gen_id("None"),
                };
                {
                    ::std::io::_print(
                        ::core::fmt::Arguments::new_v1(
                            &["Assign Id ", " to new expression >> ", "\n"],
                            &[
                                ::core::fmt::ArgumentV1::new_debug(&new_id),
                                ::core::fmt::ArgumentV1::new_debug(&ne),
                            ],
                        ),
                    );
                };
                rw_g.assign_new_expr_id.push((ne.clone(), new_id));
            }
            {
                ::std::io::_print(
                    ::core::fmt::Arguments::new_v1(
                        &["Iteration ", "\n"],
                        &[::core::fmt::ArgumentV1::new_debug(&cnt)],
                    ),
                );
            };
            if cnt >= max_iteration {
                break;
            }
            cnt = cnt + 1;
        }
        let mut saturated_graph = EGraphData::default();
        saturated_graph.root = rw_g.root;
        saturated_graph.calc_expr_3_left = rw_g.calc_expr_3_left;
        saturated_graph.calc_expr_3_right = rw_g.calc_expr_3_right;
        saturated_graph.var = rw_g.var;
        saturated_graph.num = rw_g.num;
        saturated_graph.e_node = rw_g.e_node;
        saturated_graph
    }
}
pub mod graph {
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
    pub enum ENodeId {
        Id(Sym, usize),
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ENodeId {
        #[inline]
        fn clone(&self) -> ENodeId {
            let _: ::core::clone::AssertParamIsClone<Sym>;
            let _: ::core::clone::AssertParamIsClone<usize>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for ENodeId {}
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ENodeId {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ENodeId {
        #[inline]
        fn eq(&self, other: &ENodeId) -> bool {
            match (self, other) {
                (ENodeId::Id(__self_0, __self_1), ENodeId::Id(__arg1_0, __arg1_1)) => {
                    *__self_0 == *__arg1_0 && *__self_1 == *__arg1_1
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for ENodeId {}
    #[automatically_derived]
    impl ::core::cmp::Eq for ENodeId {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<Sym>;
            let _: ::core::cmp::AssertParamIsEq<usize>;
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ENodeId {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                ENodeId::Id(__self_0, __self_1) => {
                    ::core::fmt::Formatter::debug_tuple_field2_finish(
                        f,
                        "Id",
                        &__self_0,
                        &__self_1,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for ENodeId {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match self {
                ENodeId::Id(__self_0, __self_1) => {
                    ::core::hash::Hash::hash(__self_0, state);
                    ::core::hash::Hash::hash(__self_1, state)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for ENodeId {
        #[inline]
        fn partial_cmp(
            &self,
            other: &ENodeId,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match (self, other) {
                (ENodeId::Id(__self_0, __self_1), ENodeId::Id(__arg1_0, __arg1_1)) => {
                    match ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0) {
                        ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                            ::core::cmp::PartialOrd::partial_cmp(__self_1, __arg1_1)
                        }
                        cmp => cmp,
                    }
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for ENodeId {
        #[inline]
        fn cmp(&self, other: &ENodeId) -> ::core::cmp::Ordering {
            match (self, other) {
                (ENodeId::Id(__self_0, __self_1), ENodeId::Id(__arg1_0, __arg1_1)) => {
                    match ::core::cmp::Ord::cmp(__self_0, __arg1_0) {
                        ::core::cmp::Ordering::Equal => {
                            ::core::cmp::Ord::cmp(__self_1, __arg1_1)
                        }
                        cmp => cmp,
                    }
                }
            }
        }
    }
    pub enum PatternExpr {
        Var(Sym),
        Num(i32),
        Calc(Sym, Rc<PatternExpr>, Rc<PatternExpr>),
        WildCard(Sym),
        EClass(ENodeId),
    }
    #[automatically_derived]
    impl ::core::clone::Clone for PatternExpr {
        #[inline]
        fn clone(&self) -> PatternExpr {
            match self {
                PatternExpr::Var(__self_0) => {
                    PatternExpr::Var(::core::clone::Clone::clone(__self_0))
                }
                PatternExpr::Num(__self_0) => {
                    PatternExpr::Num(::core::clone::Clone::clone(__self_0))
                }
                PatternExpr::Calc(__self_0, __self_1, __self_2) => {
                    PatternExpr::Calc(
                        ::core::clone::Clone::clone(__self_0),
                        ::core::clone::Clone::clone(__self_1),
                        ::core::clone::Clone::clone(__self_2),
                    )
                }
                PatternExpr::WildCard(__self_0) => {
                    PatternExpr::WildCard(::core::clone::Clone::clone(__self_0))
                }
                PatternExpr::EClass(__self_0) => {
                    PatternExpr::EClass(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for PatternExpr {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for PatternExpr {
        #[inline]
        fn eq(&self, other: &PatternExpr) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
                && match (self, other) {
                    (PatternExpr::Var(__self_0), PatternExpr::Var(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (PatternExpr::Num(__self_0), PatternExpr::Num(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (
                        PatternExpr::Calc(__self_0, __self_1, __self_2),
                        PatternExpr::Calc(__arg1_0, __arg1_1, __arg1_2),
                    ) => {
                        *__self_0 == *__arg1_0 && *__self_1 == *__arg1_1
                            && *__self_2 == *__arg1_2
                    }
                    (
                        PatternExpr::WildCard(__self_0),
                        PatternExpr::WildCard(__arg1_0),
                    ) => *__self_0 == *__arg1_0,
                    (PatternExpr::EClass(__self_0), PatternExpr::EClass(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() }
                }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for PatternExpr {}
    #[automatically_derived]
    impl ::core::cmp::Eq for PatternExpr {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<Sym>;
            let _: ::core::cmp::AssertParamIsEq<i32>;
            let _: ::core::cmp::AssertParamIsEq<Rc<PatternExpr>>;
            let _: ::core::cmp::AssertParamIsEq<Rc<PatternExpr>>;
            let _: ::core::cmp::AssertParamIsEq<ENodeId>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for PatternExpr {
        #[inline]
        fn partial_cmp(
            &self,
            other: &PatternExpr,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            match ::core::cmp::PartialOrd::partial_cmp(&__self_tag, &__arg1_tag) {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                    match (self, other) {
                        (PatternExpr::Var(__self_0), PatternExpr::Var(__arg1_0)) => {
                            ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                        }
                        (PatternExpr::Num(__self_0), PatternExpr::Num(__arg1_0)) => {
                            ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                        }
                        (
                            PatternExpr::Calc(__self_0, __self_1, __self_2),
                            PatternExpr::Calc(__arg1_0, __arg1_1, __arg1_2),
                        ) => {
                            match ::core::cmp::PartialOrd::partial_cmp(
                                __self_0,
                                __arg1_0,
                            ) {
                                ::core::option::Option::Some(
                                    ::core::cmp::Ordering::Equal,
                                ) => {
                                    match ::core::cmp::PartialOrd::partial_cmp(
                                        __self_1,
                                        __arg1_1,
                                    ) {
                                        ::core::option::Option::Some(
                                            ::core::cmp::Ordering::Equal,
                                        ) => {
                                            ::core::cmp::PartialOrd::partial_cmp(__self_2, __arg1_2)
                                        }
                                        cmp => cmp,
                                    }
                                }
                                cmp => cmp,
                            }
                        }
                        (
                            PatternExpr::WildCard(__self_0),
                            PatternExpr::WildCard(__arg1_0),
                        ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
                        (
                            PatternExpr::EClass(__self_0),
                            PatternExpr::EClass(__arg1_0),
                        ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
                        _ => unsafe { ::core::intrinsics::unreachable() }
                    }
                }
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for PatternExpr {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                PatternExpr::Var(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Var",
                        &__self_0,
                    )
                }
                PatternExpr::Num(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Num",
                        &__self_0,
                    )
                }
                PatternExpr::Calc(__self_0, __self_1, __self_2) => {
                    ::core::fmt::Formatter::debug_tuple_field3_finish(
                        f,
                        "Calc",
                        &__self_0,
                        &__self_1,
                        &__self_2,
                    )
                }
                PatternExpr::WildCard(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "WildCard",
                        &__self_0,
                    )
                }
                PatternExpr::EClass(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "EClass",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for PatternExpr {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_tag, state);
            match self {
                PatternExpr::Var(__self_0) => ::core::hash::Hash::hash(__self_0, state),
                PatternExpr::Num(__self_0) => ::core::hash::Hash::hash(__self_0, state),
                PatternExpr::Calc(__self_0, __self_1, __self_2) => {
                    ::core::hash::Hash::hash(__self_0, state);
                    ::core::hash::Hash::hash(__self_1, state);
                    ::core::hash::Hash::hash(__self_2, state)
                }
                PatternExpr::WildCard(__self_0) => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
                PatternExpr::EClass(__self_0) => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
            }
        }
    }
    use crate::graph::ENodeId::*;
    static GLOBAL_CALC_COUNT: AtomicUsize = AtomicUsize::new(0);
    static GLOBAL_NUM_COUNT: AtomicUsize = AtomicUsize::new(0);
    static GLOBAL_VAR_COUNT: AtomicUsize = AtomicUsize::new(0);
    static GLOBAL_ROOT_COUNT: AtomicUsize = AtomicUsize::new(0);
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
    pub struct EGraphData {
        pub e_node: Vec<(ENodeId,)>,
        #[allow(non_snake_case)]
        pub e_node_indices_0: ascent::internal::RelFullIndexType<(ENodeId,)>,
        pub root: Vec<(ENodeId, Set<ENodeId>)>,
        #[allow(non_snake_case)]
        pub root_indices_: ascent::internal::LatticeIndexType<()>,
        #[allow(non_snake_case)]
        pub root_indices_0: ascent::internal::LatticeIndexType<(ENodeId,)>,
        #[allow(non_snake_case)]
        pub root_indices_0_1: ascent::internal::RelFullIndexType<
            (ENodeId, Set<ENodeId>),
        >,
        pub var: Vec<(ENodeId, Sym)>,
        #[allow(non_snake_case)]
        pub var_indices_0_1: ascent::internal::RelFullIndexType<(ENodeId, Sym)>,
        #[allow(non_snake_case)]
        pub var_indices_: ascent::internal::RelIndexType<()>,
        pub e_classes: Vec<(Set<ENodeId>,)>,
        #[allow(non_snake_case)]
        pub e_classes_indices_0: ascent::internal::RelFullIndexType<(Set<ENodeId>,)>,
        pub num: Vec<(ENodeId, i32)>,
        #[allow(non_snake_case)]
        pub num_indices_: ascent::internal::RelIndexType<()>,
        #[allow(non_snake_case)]
        pub num_indices_0_1: ascent::internal::RelFullIndexType<(ENodeId, i32)>,
        pub calc_expr_3_left: Vec<(ENodeId, Sym, Set<ENodeId>)>,
        #[allow(non_snake_case)]
        pub calc_expr_3_left_indices_: ascent::internal::LatticeIndexType<()>,
        #[allow(non_snake_case)]
        pub calc_expr_3_left_indices_0_1: ascent::internal::LatticeIndexType<
            (ENodeId, Sym),
        >,
        #[allow(non_snake_case)]
        pub calc_expr_3_left_indices_0_1_2: ascent::internal::RelFullIndexType<
            (ENodeId, Sym, Set<ENodeId>),
        >,
        pub calc_expr_3_right: Vec<(ENodeId, Sym, Set<ENodeId>)>,
        #[allow(non_snake_case)]
        pub calc_expr_3_right_indices_: ascent::internal::LatticeIndexType<()>,
        #[allow(non_snake_case)]
        pub calc_expr_3_right_indices_0_1: ascent::internal::LatticeIndexType<
            (ENodeId, Sym),
        >,
        #[allow(non_snake_case)]
        pub calc_expr_3_right_indices_0_1_2: ascent::internal::RelFullIndexType<
            (ENodeId, Sym, Set<ENodeId>),
        >,
        pub do_rebuild: Vec<(bool,)>,
        #[allow(non_snake_case)]
        pub do_rebuild_indices_0: ascent::internal::RelFullIndexType<(bool,)>,
        pub scc0_duration: std::time::Duration,
        pub scc1_duration: std::time::Duration,
        pub scc2_duration: std::time::Duration,
        pub scc3_duration: std::time::Duration,
        pub scc4_duration: std::time::Duration,
        pub scc5_duration: std::time::Duration,
        pub scc6_duration: std::time::Duration,
        pub scc7_duration: std::time::Duration,
    }
    impl EGraphData {
        #[allow(unused_imports)]
        ///Runs the Ascent program to a fixed point.
        pub fn run(&mut self) {
            use core::cmp::PartialEq;
            use ascent::internal::RelIndexRead;
            use ascent::internal::RelIndexReadAll;
            self.update_indices_priv();
            let _self = self;
            ascent::internal::comment("scc 0");
            {
                let _scc_start_time = ::ascent::internal::Instant::now();
                #[allow(non_snake_case)]
                let calc_expr_3_left_indices_0_1_2_delta: &mut ascent::internal::RelFullIndexType<
                    (ENodeId, Sym, Set<ENodeId>),
                > = &mut _self.calc_expr_3_left_indices_0_1_2;
                #[allow(non_snake_case)]
                let mut calc_expr_3_left_indices_0_1_2_total: ascent::internal::RelFullIndexType<
                    (ENodeId, Sym, Set<ENodeId>),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut calc_expr_3_left_indices_0_1_2_new: ascent::internal::RelFullIndexType<
                    (ENodeId, Sym, Set<ENodeId>),
                > = Default::default();
                #[allow(non_snake_case)]
                let calc_expr_3_left_indices_0_1_delta: &mut ascent::internal::LatticeIndexType<
                    (ENodeId, Sym),
                > = &mut _self.calc_expr_3_left_indices_0_1;
                #[allow(non_snake_case)]
                let mut calc_expr_3_left_indices_0_1_total: ascent::internal::LatticeIndexType<
                    (ENodeId, Sym),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut calc_expr_3_left_indices_0_1_new: ascent::internal::LatticeIndexType<
                    (ENodeId, Sym),
                > = Default::default();
                #[allow(non_snake_case)]
                let calc_expr_3_left_indices__delta: &mut ascent::internal::LatticeIndexType<
                    (),
                > = &mut _self.calc_expr_3_left_indices_;
                #[allow(non_snake_case)]
                let mut calc_expr_3_left_indices__total: ascent::internal::LatticeIndexType<
                    (),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut calc_expr_3_left_indices__new: ascent::internal::LatticeIndexType<
                    (),
                > = Default::default();
                #[allow(non_snake_case)]
                let do_rebuild_indices_0_total: &mut ascent::internal::RelFullIndexType<
                    (bool,),
                > = &mut _self.do_rebuild_indices_0;
                #[allow(non_snake_case)]
                let calc_expr_3_right_indices__total: &mut ascent::internal::LatticeIndexType<
                    (),
                > = &mut _self.calc_expr_3_right_indices_;
                #[allow(non_snake_case)]
                let root_indices__total: &mut ascent::internal::LatticeIndexType<()> = &mut _self
                    .root_indices_;
                #[allow(unused_assignments, unused_variables)]
                loop {
                    let mut __changed = false;
                    ascent::internal::comment(
                        "calc_expr_3_left <-- do_rebuild_indices_0_total, calc_expr_3_left_indices__delta, calc_expr_3_right_indices__total, if ⋯",
                    );
                    if let Some(__matching)
                        = do_rebuild_indices_0_total.index_get(&((true).clone(),))
                    {
                        for __ind in __matching {
                            let __row = &_self.do_rebuild[__ind].clone();
                            if let Some(__matching)
                                = calc_expr_3_left_indices__delta.index_get(&())
                            {
                                for __ind in __matching {
                                    let __row = &_self.calc_expr_3_left[__ind].clone();
                                    let e_id = &__row.0;
                                    let op = &__row.1;
                                    let eq1_set = &__row.2;
                                    if let Some(__matching)
                                        = calc_expr_3_right_indices__total.index_get(&())
                                    {
                                        for __ind in __matching {
                                            let __row = &_self.calc_expr_3_right[__ind].clone();
                                            let __1 = &__row.0;
                                            let __2 = &__row.1;
                                            let eq2_set = &__row.2;
                                            if !eq1_set.is_disjoint(eq2_set) {
                                                let __new_row: (ENodeId, Sym, Set<ENodeId>) = (
                                                    ascent::internal::Convert::convert(e_id),
                                                    ascent::internal::Convert::convert(op),
                                                    ascent::internal::Convert::convert(eq2_set),
                                                );
                                                let __lattice_key = (
                                                    __new_row.0.clone(),
                                                    __new_row.1.clone(),
                                                );
                                                if let Some(__existing_ind)
                                                    = calc_expr_3_left_indices_0_1_new
                                                        .get(&__lattice_key)
                                                        .or_else(|| {
                                                            calc_expr_3_left_indices_0_1_delta.get(&__lattice_key)
                                                        })
                                                        .or_else(|| {
                                                            calc_expr_3_left_indices_0_1_total.get(&__lattice_key)
                                                        })
                                                {
                                                    let __existing_ind = *__existing_ind.iter().next().unwrap();
                                                    let __lat_changed = ::ascent::Lattice::join_mut(
                                                        &mut _self.calc_expr_3_left[__existing_ind].2,
                                                        __new_row.2.clone(),
                                                    );
                                                    if __lat_changed {
                                                        let __new_row_ind = __existing_ind;
                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                            &mut calc_expr_3_left_indices_0_1_new,
                                                            (__new_row.0.clone(), __new_row.1.clone()),
                                                            __new_row_ind,
                                                        );
                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                            &mut calc_expr_3_left_indices__new,
                                                            (),
                                                            __new_row_ind,
                                                        );
                                                        __changed = true;
                                                    }
                                                } else {
                                                    let __new_row_ind = _self.calc_expr_3_left.len();
                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                        &mut calc_expr_3_left_indices_0_1_new,
                                                        (__new_row.0.clone(), __new_row.1.clone()),
                                                        __new_row_ind,
                                                    );
                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                        &mut calc_expr_3_left_indices__new,
                                                        (),
                                                        __new_row_ind,
                                                    );
                                                    _self.calc_expr_3_left.push(__new_row);
                                                    __changed = true;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment(
                        "calc_expr_3_left <-- do_rebuild_indices_0_total, calc_expr_3_left_indices__delta, root_indices__total, if ⋯",
                    );
                    if let Some(__matching)
                        = do_rebuild_indices_0_total.index_get(&((true).clone(),))
                    {
                        for __ind in __matching {
                            let __row = &_self.do_rebuild[__ind].clone();
                            if let Some(__matching)
                                = calc_expr_3_left_indices__delta.index_get(&())
                            {
                                for __ind in __matching {
                                    let __row = &_self.calc_expr_3_left[__ind].clone();
                                    let e_id = &__row.0;
                                    let op = &__row.1;
                                    let eq1_set = &__row.2;
                                    if let Some(__matching) = root_indices__total.index_get(&())
                                    {
                                        for __ind in __matching {
                                            let __row = &_self.root[__ind].clone();
                                            let __1 = &__row.0;
                                            let eq2_set = &__row.1;
                                            if !eq1_set.is_disjoint(eq2_set) {
                                                let __new_row: (ENodeId, Sym, Set<ENodeId>) = (
                                                    ascent::internal::Convert::convert(e_id),
                                                    ascent::internal::Convert::convert(op),
                                                    ascent::internal::Convert::convert(eq2_set),
                                                );
                                                let __lattice_key = (
                                                    __new_row.0.clone(),
                                                    __new_row.1.clone(),
                                                );
                                                if let Some(__existing_ind)
                                                    = calc_expr_3_left_indices_0_1_new
                                                        .get(&__lattice_key)
                                                        .or_else(|| {
                                                            calc_expr_3_left_indices_0_1_delta.get(&__lattice_key)
                                                        })
                                                        .or_else(|| {
                                                            calc_expr_3_left_indices_0_1_total.get(&__lattice_key)
                                                        })
                                                {
                                                    let __existing_ind = *__existing_ind.iter().next().unwrap();
                                                    let __lat_changed = ::ascent::Lattice::join_mut(
                                                        &mut _self.calc_expr_3_left[__existing_ind].2,
                                                        __new_row.2.clone(),
                                                    );
                                                    if __lat_changed {
                                                        let __new_row_ind = __existing_ind;
                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                            &mut calc_expr_3_left_indices_0_1_new,
                                                            (__new_row.0.clone(), __new_row.1.clone()),
                                                            __new_row_ind,
                                                        );
                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                            &mut calc_expr_3_left_indices__new,
                                                            (),
                                                            __new_row_ind,
                                                        );
                                                        __changed = true;
                                                    }
                                                } else {
                                                    let __new_row_ind = _self.calc_expr_3_left.len();
                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                        &mut calc_expr_3_left_indices_0_1_new,
                                                        (__new_row.0.clone(), __new_row.1.clone()),
                                                        __new_row_ind,
                                                    );
                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                        &mut calc_expr_3_left_indices__new,
                                                        (),
                                                        __new_row_ind,
                                                    );
                                                    _self.calc_expr_3_left.push(__new_row);
                                                    __changed = true;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment(
                        "calc_expr_3_left <-- do_rebuild_indices_0_total, calc_expr_3_left_indices__delta, calc_expr_3_left_indices__total+delta, if ⋯",
                    );
                    if let Some(__matching)
                        = do_rebuild_indices_0_total.index_get(&((true).clone(),))
                    {
                        for __ind in __matching {
                            let __row = &_self.do_rebuild[__ind].clone();
                            if let Some(__matching)
                                = calc_expr_3_left_indices__delta.index_get(&())
                            {
                                for __ind in __matching {
                                    let __row = &_self.calc_expr_3_left[__ind].clone();
                                    let e_id = &__row.0;
                                    let op = &__row.1;
                                    let eq1_set = &__row.2;
                                    if let Some(__matching)
                                        = ascent::internal::RelIndexCombined::new(
                                                &calc_expr_3_left_indices__total,
                                                calc_expr_3_left_indices__delta,
                                            )
                                            .index_get(&())
                                    {
                                        for __ind in __matching {
                                            let __row = &_self.calc_expr_3_left[__ind].clone();
                                            let __1 = &__row.0;
                                            let __2 = &__row.1;
                                            let eq2_set = &__row.2;
                                            if !eq1_set.is_disjoint(eq2_set) {
                                                let __new_row: (ENodeId, Sym, Set<ENodeId>) = (
                                                    ascent::internal::Convert::convert(e_id),
                                                    ascent::internal::Convert::convert(op),
                                                    ascent::internal::Convert::convert(eq2_set),
                                                );
                                                let __lattice_key = (
                                                    __new_row.0.clone(),
                                                    __new_row.1.clone(),
                                                );
                                                if let Some(__existing_ind)
                                                    = calc_expr_3_left_indices_0_1_new
                                                        .get(&__lattice_key)
                                                        .or_else(|| {
                                                            calc_expr_3_left_indices_0_1_delta.get(&__lattice_key)
                                                        })
                                                        .or_else(|| {
                                                            calc_expr_3_left_indices_0_1_total.get(&__lattice_key)
                                                        })
                                                {
                                                    let __existing_ind = *__existing_ind.iter().next().unwrap();
                                                    let __lat_changed = ::ascent::Lattice::join_mut(
                                                        &mut _self.calc_expr_3_left[__existing_ind].2,
                                                        __new_row.2.clone(),
                                                    );
                                                    if __lat_changed {
                                                        let __new_row_ind = __existing_ind;
                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                            &mut calc_expr_3_left_indices_0_1_new,
                                                            (__new_row.0.clone(), __new_row.1.clone()),
                                                            __new_row_ind,
                                                        );
                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                            &mut calc_expr_3_left_indices__new,
                                                            (),
                                                            __new_row_ind,
                                                        );
                                                        __changed = true;
                                                    }
                                                } else {
                                                    let __new_row_ind = _self.calc_expr_3_left.len();
                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                        &mut calc_expr_3_left_indices_0_1_new,
                                                        (__new_row.0.clone(), __new_row.1.clone()),
                                                        __new_row_ind,
                                                    );
                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                        &mut calc_expr_3_left_indices__new,
                                                        (),
                                                        __new_row_ind,
                                                    );
                                                    _self.calc_expr_3_left.push(__new_row);
                                                    __changed = true;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ascent::internal::comment(
                        "calc_expr_3_left <-- do_rebuild_indices_0_total, calc_expr_3_left_indices__total, calc_expr_3_left_indices__delta, if ⋯",
                    );
                    if let Some(__matching)
                        = do_rebuild_indices_0_total.index_get(&((true).clone(),))
                    {
                        for __ind in __matching {
                            let __row = &_self.do_rebuild[__ind].clone();
                            if let Some(__matching)
                                = calc_expr_3_left_indices__total.index_get(&())
                            {
                                for __ind in __matching {
                                    let __row = &_self.calc_expr_3_left[__ind].clone();
                                    let e_id = &__row.0;
                                    let op = &__row.1;
                                    let eq1_set = &__row.2;
                                    if let Some(__matching)
                                        = calc_expr_3_left_indices__delta.index_get(&())
                                    {
                                        for __ind in __matching {
                                            let __row = &_self.calc_expr_3_left[__ind].clone();
                                            let __1 = &__row.0;
                                            let __2 = &__row.1;
                                            let eq2_set = &__row.2;
                                            if !eq1_set.is_disjoint(eq2_set) {
                                                let __new_row: (ENodeId, Sym, Set<ENodeId>) = (
                                                    ascent::internal::Convert::convert(e_id),
                                                    ascent::internal::Convert::convert(op),
                                                    ascent::internal::Convert::convert(eq2_set),
                                                );
                                                let __lattice_key = (
                                                    __new_row.0.clone(),
                                                    __new_row.1.clone(),
                                                );
                                                if let Some(__existing_ind)
                                                    = calc_expr_3_left_indices_0_1_new
                                                        .get(&__lattice_key)
                                                        .or_else(|| {
                                                            calc_expr_3_left_indices_0_1_delta.get(&__lattice_key)
                                                        })
                                                        .or_else(|| {
                                                            calc_expr_3_left_indices_0_1_total.get(&__lattice_key)
                                                        })
                                                {
                                                    let __existing_ind = *__existing_ind.iter().next().unwrap();
                                                    let __lat_changed = ::ascent::Lattice::join_mut(
                                                        &mut _self.calc_expr_3_left[__existing_ind].2,
                                                        __new_row.2.clone(),
                                                    );
                                                    if __lat_changed {
                                                        let __new_row_ind = __existing_ind;
                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                            &mut calc_expr_3_left_indices_0_1_new,
                                                            (__new_row.0.clone(), __new_row.1.clone()),
                                                            __new_row_ind,
                                                        );
                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                            &mut calc_expr_3_left_indices__new,
                                                            (),
                                                            __new_row_ind,
                                                        );
                                                        __changed = true;
                                                    }
                                                } else {
                                                    let __new_row_ind = _self.calc_expr_3_left.len();
                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                        &mut calc_expr_3_left_indices_0_1_new,
                                                        (__new_row.0.clone(), __new_row.1.clone()),
                                                        __new_row_ind,
                                                    );
                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                        &mut calc_expr_3_left_indices__new,
                                                        (),
                                                        __new_row_ind,
                                                    );
                                                    _self.calc_expr_3_left.push(__new_row);
                                                    __changed = true;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        calc_expr_3_left_indices_0_1_2_delta,
                        &mut calc_expr_3_left_indices_0_1_2_total,
                    );
                    std::mem::swap(
                        &mut calc_expr_3_left_indices_0_1_2_new,
                        calc_expr_3_left_indices_0_1_2_delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        calc_expr_3_left_indices_0_1_delta,
                        &mut calc_expr_3_left_indices_0_1_total,
                    );
                    std::mem::swap(
                        &mut calc_expr_3_left_indices_0_1_new,
                        calc_expr_3_left_indices_0_1_delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        calc_expr_3_left_indices__delta,
                        &mut calc_expr_3_left_indices__total,
                    );
                    std::mem::swap(
                        &mut calc_expr_3_left_indices__new,
                        calc_expr_3_left_indices__delta,
                    );
                    if !__changed {
                        break;
                    }
                }
                _self
                    .calc_expr_3_left_indices_0_1_2 = calc_expr_3_left_indices_0_1_2_total;
                _self.calc_expr_3_left_indices_0_1 = calc_expr_3_left_indices_0_1_total;
                _self.calc_expr_3_left_indices_ = calc_expr_3_left_indices__total;
                _self.scc0_duration += _scc_start_time.elapsed();
            }
            ascent::internal::comment("scc 1");
            {
                let _scc_start_time = ::ascent::internal::Instant::now();
                #[allow(non_snake_case)]
                let e_node_indices_0_delta: &mut ascent::internal::RelFullIndexType<
                    (ENodeId,),
                > = &mut _self.e_node_indices_0;
                #[allow(non_snake_case)]
                let mut e_node_indices_0_total: ascent::internal::RelFullIndexType<
                    (ENodeId,),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut e_node_indices_0_new: ascent::internal::RelFullIndexType<
                    (ENodeId,),
                > = Default::default();
                #[allow(non_snake_case)]
                let calc_expr_3_left_indices__total: &mut ascent::internal::LatticeIndexType<
                    (),
                > = &mut _self.calc_expr_3_left_indices_;
                #[allow(unused_assignments, unused_variables)]
                {
                    let mut __changed = false;
                    ascent::internal::comment(
                        "e_node <-- calc_expr_3_left_indices__total",
                    );
                    if let Some(__matching)
                        = calc_expr_3_left_indices__total.index_get(&())
                    {
                        for __ind in __matching {
                            let __row = &_self.calc_expr_3_left[__ind].clone();
                            let e_id = &__row.0;
                            let __1 = &__row.1;
                            let __2 = &__row.2;
                            let __new_row: (ENodeId,) = (
                                ascent::internal::Convert::convert(e_id),
                            );
                            let __new_row_ind = _self.e_node.len();
                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                &e_node_indices_0_total,
                                &__new_row,
                            )
                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                    e_node_indices_0_delta,
                                    &__new_row,
                                )
                                && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                    &mut e_node_indices_0_new,
                                    &__new_row,
                                    __new_row_ind,
                                )
                            {
                                _self.e_node.push(__new_row);
                                __changed = true;
                            }
                        }
                    }
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_indices_0_delta,
                        &mut e_node_indices_0_total,
                    );
                    std::mem::swap(&mut e_node_indices_0_new, e_node_indices_0_delta);
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_indices_0_delta,
                        &mut e_node_indices_0_total,
                    );
                    std::mem::swap(&mut e_node_indices_0_new, e_node_indices_0_delta);
                }
                _self.e_node_indices_0 = e_node_indices_0_total;
                _self.scc1_duration += _scc_start_time.elapsed();
            }
            ascent::internal::comment("scc 2");
            {
                let _scc_start_time = ::ascent::internal::Instant::now();
                #[allow(non_snake_case)]
                let e_classes_indices_0_delta: &mut ascent::internal::RelFullIndexType<
                    (Set<ENodeId>,),
                > = &mut _self.e_classes_indices_0;
                #[allow(non_snake_case)]
                let mut e_classes_indices_0_total: ascent::internal::RelFullIndexType<
                    (Set<ENodeId>,),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut e_classes_indices_0_new: ascent::internal::RelFullIndexType<
                    (Set<ENodeId>,),
                > = Default::default();
                #[allow(non_snake_case)]
                let calc_expr_3_left_indices__total: &mut ascent::internal::LatticeIndexType<
                    (),
                > = &mut _self.calc_expr_3_left_indices_;
                #[allow(unused_assignments, unused_variables)]
                {
                    let mut __changed = false;
                    ascent::internal::comment(
                        "e_classes <-- calc_expr_3_left_indices__total",
                    );
                    if let Some(__matching)
                        = calc_expr_3_left_indices__total.index_get(&())
                    {
                        for __ind in __matching {
                            let __row = &_self.calc_expr_3_left[__ind].clone();
                            let __1 = &__row.0;
                            let __2 = &__row.1;
                            let eqs = &__row.2;
                            let __new_row: (Set<ENodeId>,) = (
                                ascent::internal::Convert::convert(eqs),
                            );
                            let __new_row_ind = _self.e_classes.len();
                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                &e_classes_indices_0_total,
                                &__new_row,
                            )
                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                    e_classes_indices_0_delta,
                                    &__new_row,
                                )
                                && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                    &mut e_classes_indices_0_new,
                                    &__new_row,
                                    __new_row_ind,
                                )
                            {
                                _self.e_classes.push(__new_row);
                                __changed = true;
                            }
                        }
                    }
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_classes_indices_0_delta,
                        &mut e_classes_indices_0_total,
                    );
                    std::mem::swap(
                        &mut e_classes_indices_0_new,
                        e_classes_indices_0_delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_classes_indices_0_delta,
                        &mut e_classes_indices_0_total,
                    );
                    std::mem::swap(
                        &mut e_classes_indices_0_new,
                        e_classes_indices_0_delta,
                    );
                }
                _self.e_classes_indices_0 = e_classes_indices_0_total;
                _self.scc2_duration += _scc_start_time.elapsed();
            }
            ascent::internal::comment("scc 3");
            {
                let _scc_start_time = ::ascent::internal::Instant::now();
                #[allow(non_snake_case)]
                let e_node_indices_0_delta: &mut ascent::internal::RelFullIndexType<
                    (ENodeId,),
                > = &mut _self.e_node_indices_0;
                #[allow(non_snake_case)]
                let mut e_node_indices_0_total: ascent::internal::RelFullIndexType<
                    (ENodeId,),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut e_node_indices_0_new: ascent::internal::RelFullIndexType<
                    (ENodeId,),
                > = Default::default();
                #[allow(non_snake_case)]
                let var_indices__total: &mut ascent::internal::RelIndexType<()> = &mut _self
                    .var_indices_;
                #[allow(unused_assignments, unused_variables)]
                {
                    let mut __changed = false;
                    ascent::internal::comment("e_node <-- var_indices__total");
                    if let Some(__matching) = var_indices__total.index_get(&()) {
                        for __ind in __matching {
                            let __row = &_self.var[__ind].clone();
                            let e_id = &__row.0;
                            let __1 = &__row.1;
                            let __new_row: (ENodeId,) = (
                                ascent::internal::Convert::convert(e_id),
                            );
                            let __new_row_ind = _self.e_node.len();
                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                &e_node_indices_0_total,
                                &__new_row,
                            )
                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                    e_node_indices_0_delta,
                                    &__new_row,
                                )
                                && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                    &mut e_node_indices_0_new,
                                    &__new_row,
                                    __new_row_ind,
                                )
                            {
                                _self.e_node.push(__new_row);
                                __changed = true;
                            }
                        }
                    }
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_indices_0_delta,
                        &mut e_node_indices_0_total,
                    );
                    std::mem::swap(&mut e_node_indices_0_new, e_node_indices_0_delta);
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_indices_0_delta,
                        &mut e_node_indices_0_total,
                    );
                    std::mem::swap(&mut e_node_indices_0_new, e_node_indices_0_delta);
                }
                _self.e_node_indices_0 = e_node_indices_0_total;
                _self.scc3_duration += _scc_start_time.elapsed();
            }
            ascent::internal::comment("scc 4");
            {
                let _scc_start_time = ::ascent::internal::Instant::now();
                #[allow(non_snake_case)]
                let e_node_indices_0_delta: &mut ascent::internal::RelFullIndexType<
                    (ENodeId,),
                > = &mut _self.e_node_indices_0;
                #[allow(non_snake_case)]
                let mut e_node_indices_0_total: ascent::internal::RelFullIndexType<
                    (ENodeId,),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut e_node_indices_0_new: ascent::internal::RelFullIndexType<
                    (ENodeId,),
                > = Default::default();
                #[allow(non_snake_case)]
                let num_indices__total: &mut ascent::internal::RelIndexType<()> = &mut _self
                    .num_indices_;
                #[allow(unused_assignments, unused_variables)]
                {
                    let mut __changed = false;
                    ascent::internal::comment("e_node <-- num_indices__total");
                    if let Some(__matching) = num_indices__total.index_get(&()) {
                        for __ind in __matching {
                            let __row = &_self.num[__ind].clone();
                            let e_id = &__row.0;
                            let __1 = &__row.1;
                            let __new_row: (ENodeId,) = (
                                ascent::internal::Convert::convert(e_id),
                            );
                            let __new_row_ind = _self.e_node.len();
                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                &e_node_indices_0_total,
                                &__new_row,
                            )
                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                    e_node_indices_0_delta,
                                    &__new_row,
                                )
                                && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                    &mut e_node_indices_0_new,
                                    &__new_row,
                                    __new_row_ind,
                                )
                            {
                                _self.e_node.push(__new_row);
                                __changed = true;
                            }
                        }
                    }
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_indices_0_delta,
                        &mut e_node_indices_0_total,
                    );
                    std::mem::swap(&mut e_node_indices_0_new, e_node_indices_0_delta);
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_indices_0_delta,
                        &mut e_node_indices_0_total,
                    );
                    std::mem::swap(&mut e_node_indices_0_new, e_node_indices_0_delta);
                }
                _self.e_node_indices_0 = e_node_indices_0_total;
                _self.scc4_duration += _scc_start_time.elapsed();
            }
            ascent::internal::comment("scc 5");
            {
                let _scc_start_time = ::ascent::internal::Instant::now();
                #[allow(non_snake_case)]
                let e_node_indices_0_delta: &mut ascent::internal::RelFullIndexType<
                    (ENodeId,),
                > = &mut _self.e_node_indices_0;
                #[allow(non_snake_case)]
                let mut e_node_indices_0_total: ascent::internal::RelFullIndexType<
                    (ENodeId,),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut e_node_indices_0_new: ascent::internal::RelFullIndexType<
                    (ENodeId,),
                > = Default::default();
                #[allow(non_snake_case)]
                let root_indices__total: &mut ascent::internal::LatticeIndexType<()> = &mut _self
                    .root_indices_;
                #[allow(unused_assignments, unused_variables)]
                {
                    let mut __changed = false;
                    ascent::internal::comment("e_node <-- root_indices__total");
                    if let Some(__matching) = root_indices__total.index_get(&()) {
                        for __ind in __matching {
                            let __row = &_self.root[__ind].clone();
                            let e_id = &__row.0;
                            let __1 = &__row.1;
                            let __new_row: (ENodeId,) = (
                                ascent::internal::Convert::convert(e_id),
                            );
                            let __new_row_ind = _self.e_node.len();
                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                &e_node_indices_0_total,
                                &__new_row,
                            )
                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                    e_node_indices_0_delta,
                                    &__new_row,
                                )
                                && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                    &mut e_node_indices_0_new,
                                    &__new_row,
                                    __new_row_ind,
                                )
                            {
                                _self.e_node.push(__new_row);
                                __changed = true;
                            }
                        }
                    }
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_indices_0_delta,
                        &mut e_node_indices_0_total,
                    );
                    std::mem::swap(&mut e_node_indices_0_new, e_node_indices_0_delta);
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_node_indices_0_delta,
                        &mut e_node_indices_0_total,
                    );
                    std::mem::swap(&mut e_node_indices_0_new, e_node_indices_0_delta);
                }
                _self.e_node_indices_0 = e_node_indices_0_total;
                _self.scc5_duration += _scc_start_time.elapsed();
            }
            ascent::internal::comment("scc 6");
            {
                let _scc_start_time = ::ascent::internal::Instant::now();
                #[allow(non_snake_case)]
                let e_classes_indices_0_delta: &mut ascent::internal::RelFullIndexType<
                    (Set<ENodeId>,),
                > = &mut _self.e_classes_indices_0;
                #[allow(non_snake_case)]
                let mut e_classes_indices_0_total: ascent::internal::RelFullIndexType<
                    (Set<ENodeId>,),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut e_classes_indices_0_new: ascent::internal::RelFullIndexType<
                    (Set<ENodeId>,),
                > = Default::default();
                #[allow(non_snake_case)]
                let root_indices__total: &mut ascent::internal::LatticeIndexType<()> = &mut _self
                    .root_indices_;
                #[allow(unused_assignments, unused_variables)]
                {
                    let mut __changed = false;
                    ascent::internal::comment("e_classes <-- root_indices__total");
                    if let Some(__matching) = root_indices__total.index_get(&()) {
                        for __ind in __matching {
                            let __row = &_self.root[__ind].clone();
                            let __1 = &__row.0;
                            let eqs = &__row.1;
                            let __new_row: (Set<ENodeId>,) = (
                                ascent::internal::Convert::convert(eqs),
                            );
                            let __new_row_ind = _self.e_classes.len();
                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                &e_classes_indices_0_total,
                                &__new_row,
                            )
                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                    e_classes_indices_0_delta,
                                    &__new_row,
                                )
                                && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                    &mut e_classes_indices_0_new,
                                    &__new_row,
                                    __new_row_ind,
                                )
                            {
                                _self.e_classes.push(__new_row);
                                __changed = true;
                            }
                        }
                    }
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_classes_indices_0_delta,
                        &mut e_classes_indices_0_total,
                    );
                    std::mem::swap(
                        &mut e_classes_indices_0_new,
                        e_classes_indices_0_delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_classes_indices_0_delta,
                        &mut e_classes_indices_0_total,
                    );
                    std::mem::swap(
                        &mut e_classes_indices_0_new,
                        e_classes_indices_0_delta,
                    );
                }
                _self.e_classes_indices_0 = e_classes_indices_0_total;
                _self.scc6_duration += _scc_start_time.elapsed();
            }
            ascent::internal::comment("scc 7");
            {
                let _scc_start_time = ::ascent::internal::Instant::now();
                #[allow(non_snake_case)]
                let e_classes_indices_0_delta: &mut ascent::internal::RelFullIndexType<
                    (Set<ENodeId>,),
                > = &mut _self.e_classes_indices_0;
                #[allow(non_snake_case)]
                let mut e_classes_indices_0_total: ascent::internal::RelFullIndexType<
                    (Set<ENodeId>,),
                > = Default::default();
                #[allow(non_snake_case)]
                let mut e_classes_indices_0_new: ascent::internal::RelFullIndexType<
                    (Set<ENodeId>,),
                > = Default::default();
                #[allow(non_snake_case)]
                let calc_expr_3_right_indices__total: &mut ascent::internal::LatticeIndexType<
                    (),
                > = &mut _self.calc_expr_3_right_indices_;
                #[allow(unused_assignments, unused_variables)]
                {
                    let mut __changed = false;
                    ascent::internal::comment(
                        "e_classes <-- calc_expr_3_right_indices__total",
                    );
                    if let Some(__matching)
                        = calc_expr_3_right_indices__total.index_get(&())
                    {
                        for __ind in __matching {
                            let __row = &_self.calc_expr_3_right[__ind].clone();
                            let __1 = &__row.0;
                            let __2 = &__row.1;
                            let eqs = &__row.2;
                            let __new_row: (Set<ENodeId>,) = (
                                ascent::internal::Convert::convert(eqs),
                            );
                            let __new_row_ind = _self.e_classes.len();
                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                &e_classes_indices_0_total,
                                &__new_row,
                            )
                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                    e_classes_indices_0_delta,
                                    &__new_row,
                                )
                                && ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                    &mut e_classes_indices_0_new,
                                    &__new_row,
                                    __new_row_ind,
                                )
                            {
                                _self.e_classes.push(__new_row);
                                __changed = true;
                            }
                        }
                    }
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_classes_indices_0_delta,
                        &mut e_classes_indices_0_total,
                    );
                    std::mem::swap(
                        &mut e_classes_indices_0_new,
                        e_classes_indices_0_delta,
                    );
                    ::ascent::internal::RelIndexWrite::move_index_contents(
                        e_classes_indices_0_delta,
                        &mut e_classes_indices_0_total,
                    );
                    std::mem::swap(
                        &mut e_classes_indices_0_new,
                        e_classes_indices_0_delta,
                    );
                }
                _self.e_classes_indices_0 = e_classes_indices_0_total;
                _self.scc7_duration += _scc_start_time.elapsed();
            }
        }
        fn update_indices_priv(&mut self) {
            for (i, tuple) in self.e_node.iter().enumerate() {
                let selection_tuple = (tuple.0.clone(),);
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.e_node_indices_0,
                    selection_tuple,
                    i,
                );
            }
            for (i, tuple) in self.root.iter().enumerate() {
                let selection_tuple = ();
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.root_indices_,
                    selection_tuple,
                    i,
                );
                let selection_tuple = (tuple.0.clone(),);
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.root_indices_0,
                    selection_tuple,
                    i,
                );
                let selection_tuple = (tuple.0.clone(), tuple.1.clone());
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.root_indices_0_1,
                    selection_tuple,
                    i,
                );
            }
            for (i, tuple) in self.var.iter().enumerate() {
                let selection_tuple = (tuple.0.clone(), tuple.1.clone());
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.var_indices_0_1,
                    selection_tuple,
                    i,
                );
                let selection_tuple = ();
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.var_indices_,
                    selection_tuple,
                    i,
                );
            }
            for (i, tuple) in self.e_classes.iter().enumerate() {
                let selection_tuple = (tuple.0.clone(),);
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.e_classes_indices_0,
                    selection_tuple,
                    i,
                );
            }
            for (i, tuple) in self.num.iter().enumerate() {
                let selection_tuple = ();
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.num_indices_,
                    selection_tuple,
                    i,
                );
                let selection_tuple = (tuple.0.clone(), tuple.1.clone());
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.num_indices_0_1,
                    selection_tuple,
                    i,
                );
            }
            for (i, tuple) in self.calc_expr_3_left.iter().enumerate() {
                let selection_tuple = ();
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.calc_expr_3_left_indices_,
                    selection_tuple,
                    i,
                );
                let selection_tuple = (tuple.0.clone(), tuple.1.clone());
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.calc_expr_3_left_indices_0_1,
                    selection_tuple,
                    i,
                );
                let selection_tuple = (
                    tuple.0.clone(),
                    tuple.1.clone(),
                    tuple.2.clone(),
                );
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.calc_expr_3_left_indices_0_1_2,
                    selection_tuple,
                    i,
                );
            }
            for (i, tuple) in self.calc_expr_3_right.iter().enumerate() {
                let selection_tuple = ();
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.calc_expr_3_right_indices_,
                    selection_tuple,
                    i,
                );
                let selection_tuple = (tuple.0.clone(), tuple.1.clone());
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.calc_expr_3_right_indices_0_1,
                    selection_tuple,
                    i,
                );
                let selection_tuple = (
                    tuple.0.clone(),
                    tuple.1.clone(),
                    tuple.2.clone(),
                );
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.calc_expr_3_right_indices_0_1_2,
                    selection_tuple,
                    i,
                );
            }
            for (i, tuple) in self.do_rebuild.iter().enumerate() {
                let selection_tuple = (tuple.0.clone(),);
                ascent::internal::RelIndexWrite::index_insert(
                    &mut self.do_rebuild_indices_0,
                    selection_tuple,
                    i,
                );
            }
        }
        #[deprecated = "Explicit call to update_indices not required anymore."]
        pub fn update_indices(&mut self) {
            self.update_indices_priv();
        }
        #[allow(unused_imports)]
        fn type_constaints() {
            let _type_constraints: ascent::internal::TypeConstraints<ENodeId>;
            let _type_constraints: ascent::internal::LatTypeConstraints<Set<ENodeId>>;
            let _type_constraints: ascent::internal::TypeConstraints<Sym>;
            let _type_constraints: ascent::internal::TypeConstraints<Set<ENodeId>>;
            let _type_constraints: ascent::internal::TypeConstraints<i32>;
            let _type_constraints: ascent::internal::TypeConstraints<bool>;
        }
        pub fn summary() -> &'static str {
            "scc 0, is_looping: true:\n  calc_expr_3_left <-- do_rebuild_indices_0_total, calc_expr_3_left_indices__delta, calc_expr_3_right_indices__total, if ⋯\n  calc_expr_3_left <-- do_rebuild_indices_0_total, calc_expr_3_left_indices__delta, root_indices__total, if ⋯\n  calc_expr_3_left <-- do_rebuild_indices_0_total, calc_expr_3_left_indices__delta, calc_expr_3_left_indices__total+delta, if ⋯\n  calc_expr_3_left <-- do_rebuild_indices_0_total, calc_expr_3_left_indices__total, calc_expr_3_left_indices__delta, if ⋯\n  dynamic relations: calc_expr_3_left\nscc 1, is_looping: false:\n  e_node <-- calc_expr_3_left_indices__total\n  dynamic relations: e_node\nscc 2, is_looping: false:\n  e_classes <-- calc_expr_3_left_indices__total\n  dynamic relations: e_classes\nscc 3, is_looping: false:\n  e_node <-- var_indices__total\n  dynamic relations: e_node\nscc 4, is_looping: false:\n  e_node <-- num_indices__total\n  dynamic relations: e_node\nscc 5, is_looping: false:\n  e_node <-- root_indices__total\n  dynamic relations: e_node\nscc 6, is_looping: false:\n  e_classes <-- root_indices__total\n  dynamic relations: e_classes\nscc 7, is_looping: false:\n  e_classes <-- calc_expr_3_right_indices__total\n  dynamic relations: e_classes\n"
        }
        pub fn relation_sizes_summary(&self) -> String {
            use std::fmt::Write;
            let mut res = String::new();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["", " size: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"e_node"),
                            ::core::fmt::ArgumentV1::new_display(&self.e_node.len()),
                        ],
                    ),
                )
                .unwrap();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["", " size: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"root"),
                            ::core::fmt::ArgumentV1::new_display(&self.root.len()),
                        ],
                    ),
                )
                .unwrap();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["", " size: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"var"),
                            ::core::fmt::ArgumentV1::new_display(&self.var.len()),
                        ],
                    ),
                )
                .unwrap();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["", " size: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"e_classes"),
                            ::core::fmt::ArgumentV1::new_display(&self.e_classes.len()),
                        ],
                    ),
                )
                .unwrap();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["", " size: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"num"),
                            ::core::fmt::ArgumentV1::new_display(&self.num.len()),
                        ],
                    ),
                )
                .unwrap();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["", " size: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"calc_expr_3_left"),
                            ::core::fmt::ArgumentV1::new_display(
                                &self.calc_expr_3_left.len(),
                            ),
                        ],
                    ),
                )
                .unwrap();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["", " size: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"calc_expr_3_right"),
                            ::core::fmt::ArgumentV1::new_display(
                                &self.calc_expr_3_right.len(),
                            ),
                        ],
                    ),
                )
                .unwrap();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["", " size: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"do_rebuild"),
                            ::core::fmt::ArgumentV1::new_display(&self.do_rebuild.len()),
                        ],
                    ),
                )
                .unwrap();
            res
        }
        pub fn scc_times_summary(&self) -> String {
            use std::fmt::Write;
            let mut res = String::new();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["scc ", " time: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"0"),
                            ::core::fmt::ArgumentV1::new_debug(&self.scc0_duration),
                        ],
                    ),
                )
                .unwrap();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["scc ", " time: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"1"),
                            ::core::fmt::ArgumentV1::new_debug(&self.scc1_duration),
                        ],
                    ),
                )
                .unwrap();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["scc ", " time: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"2"),
                            ::core::fmt::ArgumentV1::new_debug(&self.scc2_duration),
                        ],
                    ),
                )
                .unwrap();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["scc ", " time: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"3"),
                            ::core::fmt::ArgumentV1::new_debug(&self.scc3_duration),
                        ],
                    ),
                )
                .unwrap();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["scc ", " time: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"4"),
                            ::core::fmt::ArgumentV1::new_debug(&self.scc4_duration),
                        ],
                    ),
                )
                .unwrap();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["scc ", " time: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"5"),
                            ::core::fmt::ArgumentV1::new_debug(&self.scc5_duration),
                        ],
                    ),
                )
                .unwrap();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["scc ", " time: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"6"),
                            ::core::fmt::ArgumentV1::new_debug(&self.scc6_duration),
                        ],
                    ),
                )
                .unwrap();
            (&mut res)
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["scc ", " time: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"7"),
                            ::core::fmt::ArgumentV1::new_debug(&self.scc7_duration),
                        ],
                    ),
                )
                .unwrap();
            res
        }
    }
    impl Default for EGraphData {
        fn default() -> Self {
            let mut _self = EGraphData {
                e_node: Default::default(),
                e_node_indices_0: Default::default(),
                root: Default::default(),
                root_indices_: Default::default(),
                root_indices_0: Default::default(),
                root_indices_0_1: Default::default(),
                var: Default::default(),
                var_indices_0_1: Default::default(),
                var_indices_: Default::default(),
                e_classes: Default::default(),
                e_classes_indices_0: Default::default(),
                num: Default::default(),
                num_indices_: Default::default(),
                num_indices_0_1: Default::default(),
                calc_expr_3_left: Default::default(),
                calc_expr_3_left_indices_: Default::default(),
                calc_expr_3_left_indices_0_1: Default::default(),
                calc_expr_3_left_indices_0_1_2: Default::default(),
                calc_expr_3_right: Default::default(),
                calc_expr_3_right_indices_: Default::default(),
                calc_expr_3_right_indices_0_1: Default::default(),
                calc_expr_3_right_indices_0_1_2: Default::default(),
                do_rebuild: Default::default(),
                do_rebuild_indices_0: Default::default(),
                scc0_duration: std::time::Duration::ZERO,
                scc1_duration: std::time::Duration::ZERO,
                scc2_duration: std::time::Duration::ZERO,
                scc3_duration: std::time::Duration::ZERO,
                scc4_duration: std::time::Duration::ZERO,
                scc5_duration: std::time::Duration::ZERO,
                scc6_duration: std::time::Duration::ZERO,
                scc7_duration: std::time::Duration::ZERO,
            };
            _self
        }
    }
    pub fn rebuild(g: &EGraphData) -> EGraphData {
        let mut rebuilt_g = EGraphData::default();
        rebuilt_g.root = g.root.clone();
        rebuilt_g.calc_expr_3_left = g.calc_expr_3_left.clone();
        rebuilt_g.calc_expr_3_right = g.calc_expr_3_right.clone();
        rebuilt_g.var = g.var.clone();
        rebuilt_g.num = g.num.clone();
        rebuilt_g
            .do_rebuild = <[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([(true,)]),
        );
        rebuilt_g.run();
        rebuilt_g
    }
    pub fn graph_to_dot(g: &EGraphData) -> String {
        let mut dot_str = String::new();
        dot_str.push_str("digraph egraph {\n");
        dot_str.push_str("  compound=true\n");
        dot_str.push_str("  clusterrank=local\n");
        let mut class_map: HashMap<Vec<ENodeId>, String> = HashMap::new();
        let mut eclass_cnt = 0;
        for (n,) in g.e_classes.iter() {
            let n_list: Vec<ENodeId> = n.deref().clone().into_iter().collect();
            if class_map.contains_key(&n_list) {
                continue;
            }
            class_map
                .insert(
                    n_list,
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_display(&eclass_cnt)],
                            ),
                        );
                        res
                    },
                );
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
        fn in_class_index(
            m: &HashMap<Vec<ENodeId>, String>,
            e: ENodeId,
        ) -> Option<usize> {
            for (nodes, _) in m.iter() {
                if nodes.contains(&e) {
                    return nodes.iter().position(|&x| x == e);
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
            dot_str
                .push_str(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &["  subgraph cluster_", " {\n"],
                                &[::core::fmt::ArgumentV1::new_display(&c_id)],
                            ),
                        );
                        res
                    }
                        .as_str(),
                );
            dot_str.push_str("    style=dotted\n");
            for (i, e_id) in c.iter().enumerate() {
                if let Some(sym) = node_sym_map.get(e_id) {
                    dot_str
                        .push_str(
                            {
                                let res = ::alloc::fmt::format(
                                    ::core::fmt::Arguments::new_v1(
                                        &["    ", ".", "[label = \"", "\"]\n"],
                                        &[
                                            ::core::fmt::ArgumentV1::new_display(&c_id),
                                            ::core::fmt::ArgumentV1::new_display(&i),
                                            ::core::fmt::ArgumentV1::new_display(&sym),
                                        ],
                                    ),
                                );
                                res
                            }
                                .as_str(),
                        );
                }
            }
            dot_str.push_str("  }\n");
        }
        for (e_id, op, to_eq_set) in g.calc_expr_3_left.iter() {
            let to_eq_list: Vec<ENodeId> = to_eq_set
                .deref()
                .clone()
                .into_iter()
                .collect();
            if let Some(to_c_id) = class_map.get(&to_eq_list) {
                let from_c_id = in_class(&class_map, *e_id);
                if let Some(sub_id) = in_class_index(&class_map, *e_id) {
                    dot_str
                        .push_str(
                            {
                                let res = ::alloc::fmt::format(
                                    ::core::fmt::Arguments::new_v1(
                                        &["  ", ".", "", " -> ", ".0 [lhead = cluster_", ", ]\n"],
                                        &[
                                            ::core::fmt::ArgumentV1::new_display(&from_c_id),
                                            ::core::fmt::ArgumentV1::new_display(&sub_id),
                                            ::core::fmt::ArgumentV1::new_display(&":sw"),
                                            ::core::fmt::ArgumentV1::new_display(&to_c_id),
                                            ::core::fmt::ArgumentV1::new_display(&to_c_id),
                                        ],
                                    ),
                                );
                                res
                            }
                                .as_str(),
                        );
                }
            }
        }
        for (e_id, op, to_eq_set) in g.calc_expr_3_right.iter() {
            let to_eq_list: Vec<ENodeId> = to_eq_set
                .deref()
                .clone()
                .into_iter()
                .collect();
            if let Some(to_c_id) = class_map.get(&to_eq_list) {
                let from_c_id = in_class(&class_map, *e_id);
                if let Some(sub_id) = in_class_index(&class_map, *e_id) {
                    dot_str
                        .push_str(
                            {
                                let res = ::alloc::fmt::format(
                                    ::core::fmt::Arguments::new_v1(
                                        &["  ", ".", "", " -> ", ".0 [lhead = cluster_", ", ]\n"],
                                        &[
                                            ::core::fmt::ArgumentV1::new_display(&from_c_id),
                                            ::core::fmt::ArgumentV1::new_display(&sub_id),
                                            ::core::fmt::ArgumentV1::new_display(&":se"),
                                            ::core::fmt::ArgumentV1::new_display(&to_c_id),
                                            ::core::fmt::ArgumentV1::new_display(&to_c_id),
                                        ],
                                    ),
                                );
                                res
                            }
                                .as_str(),
                        );
                }
            }
        }
        dot_str.push_str("}");
        dot_str
    }
}
use ascent::lattice::set::Set;
use graph::*;
use graph::ENodeId::*;
use graph::PatternExpr::*;
use ematch::*;
use erewrite::*;
fn init_test_egraph() -> EGraphData {
    let mut g = EGraphData::default();
    let calc_id_1 = gen_id("Calc");
    let calc_id_2 = gen_id("Calc");
    let var_id_1 = gen_id("Var");
    let num_id_1 = gen_id("Num");
    let num_id_2 = gen_id("Num");
    g
        .calc_expr_3_left = <[_]>::into_vec(
        #[rustc_box]
        ::alloc::boxed::Box::new([
            (calc_id_1, "*", Set::singleton(var_id_1)),
            (calc_id_2, "/", Set::singleton(calc_id_1)),
        ]),
    );
    g
        .calc_expr_3_right = <[_]>::into_vec(
        #[rustc_box]
        ::alloc::boxed::Box::new([
            (calc_id_1, "*", Set::singleton(num_id_1)),
            (calc_id_2, "/", Set::singleton(num_id_1)),
        ]),
    );
    g
        .root = <[_]>::into_vec(
        #[rustc_box]
        ::alloc::boxed::Box::new([(Id("Root", 1), Set::singleton(calc_id_2))]),
    );
    g.var = <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([(var_id_1, "a")]));
    g
        .num = <[_]>::into_vec(
        #[rustc_box]
        ::alloc::boxed::Box::new([(num_id_1, 2), (num_id_2, 1)]),
    );
    g.run();
    g
}
fn run_egraph_test() {
    let mut test_g = init_test_egraph();
    let test_match_pat_1 = Rc::new(
        Calc("*", Rc::new(EClass(Id("Calc", 0))), Rc::new(Num(1))),
    );
    let matched_test_1 = e_match(&test_g, &test_match_pat_1);
    {
        ::std::io::_print(
            ::core::fmt::Arguments::new_v1(
                &["Match ", " get res ", "\n"],
                &[
                    ::core::fmt::ArgumentV1::new_debug(&test_match_pat_1),
                    ::core::fmt::ArgumentV1::new_debug(&matched_test_1),
                ],
            ),
        );
    };
    test_g = e_saturate(&test_g, 10);
    let test_match_pat_2 = Rc::new(
        Calc(
            "*",
            Rc::new(Calc("*", Rc::new(Var("a")), Rc::new(Num(1)))),
            Rc::new(Num(1)),
        ),
    );
    let matched_test_2 = e_match(&test_g, &test_match_pat_2);
    {
        ::std::io::_print(
            ::core::fmt::Arguments::new_v1(
                &["Vars: ", "\n"],
                &[::core::fmt::ArgumentV1::new_debug(&test_g.var)],
            ),
        );
    };
    {
        ::std::io::_print(
            ::core::fmt::Arguments::new_v1(
                &["Nums: ", "\n"],
                &[::core::fmt::ArgumentV1::new_debug(&test_g.num)],
            ),
        );
    };
    let mut node_eq_sets: BTreeSet<BTreeSet<ENodeId>> = BTreeSet::new();
    for (e_id, op, eq_set) in &test_g.calc_expr_3_left {
        node_eq_sets.insert(eq_set.deref().clone());
        {
            ::std::io::_print(
                ::core::fmt::Arguments::new_v1(
                    &["left set ", " ", " >>> ", "\n"],
                    &[
                        ::core::fmt::ArgumentV1::new_debug(&e_id),
                        ::core::fmt::ArgumentV1::new_debug(&op),
                        ::core::fmt::ArgumentV1::new_debug(&eq_set.deref()),
                    ],
                ),
            );
        };
    }
    for (e_id, op, eq_set) in &test_g.calc_expr_3_right {
        node_eq_sets.insert(eq_set.deref().clone());
        {
            ::std::io::_print(
                ::core::fmt::Arguments::new_v1(
                    &["right set ", " ", " >>> ", "\n"],
                    &[
                        ::core::fmt::ArgumentV1::new_debug(&e_id),
                        ::core::fmt::ArgumentV1::new_debug(&op),
                        ::core::fmt::ArgumentV1::new_debug(&eq_set.deref()),
                    ],
                ),
            );
        };
    }
    for (e_id, eq_set) in &test_g.root {
        node_eq_sets.insert(eq_set.deref().clone());
        {
            ::std::io::_print(
                ::core::fmt::Arguments::new_v1(
                    &["root set ", " >>> ", "\n"],
                    &[
                        ::core::fmt::ArgumentV1::new_debug(&e_id),
                        ::core::fmt::ArgumentV1::new_debug(&eq_set.deref()),
                    ],
                ),
            );
        };
    }
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(&["Saturated!\n"], &[]));
    };
    let matched_test_1 = e_match(&test_g, &test_match_pat_1);
    {
        ::std::io::_print(
            ::core::fmt::Arguments::new_v1(
                &["Match ", " get res ", "\n"],
                &[
                    ::core::fmt::ArgumentV1::new_debug(&test_match_pat_1),
                    ::core::fmt::ArgumentV1::new_debug(&matched_test_1),
                ],
            ),
        );
    };
    {
        ::std::io::_print(
            ::core::fmt::Arguments::new_v1(
                &["Match ", " got : ", "\n"],
                &[
                    ::core::fmt::ArgumentV1::new_debug(&test_match_pat_2),
                    ::core::fmt::ArgumentV1::new_debug(&matched_test_2),
                ],
            ),
        );
    };
    test_g.run();
    {
        ::std::io::_print(
            ::core::fmt::Arguments::new_v1(
                &["DOT: \n", "\n"],
                &[::core::fmt::ArgumentV1::new_display(&graph_to_dot(&test_g))],
            ),
        );
    };
}
fn main() {
    run_egraph_test();
}
