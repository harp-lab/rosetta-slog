use std::cmp::Ordering;
use std::collections::BTreeSet;

use ascent::Lattice;

#[derive(PartialEq, Clone, Default, Hash, Eq, Debug)]
pub struct DisjoinedSet<T>(pub BTreeSet<BTreeSet<T>>);

impl <T: Ord+Copy> DisjoinedSet<T> {
    pub fn singleton(e: BTreeSet<T>) -> Self{
        Self([e].into_iter().collect())
    }
}

impl <T: Ord+Copy> PartialOrd for DisjoinedSet<T> where T: Ord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let mut self_elemet_set: BTreeSet<T> = BTreeSet::new();
        self.0.clone().into_iter().for_each(|mut self_set| {
            self_elemet_set.append(&mut self_set);
        });
        let mut other_elemet_set: BTreeSet<T> = BTreeSet::new();
        for mut other_set in other.0.clone() {
            other_elemet_set.append(&mut other_set);
        }
        if self_elemet_set == other_elemet_set {
            Some(Ordering::Equal)
        } else if self_elemet_set.is_superset(&other_elemet_set) {
            Some(Ordering::Greater)
        } else if self_elemet_set.is_subset(&other_elemet_set) {
            Some(Ordering::Less)
        } else {
            None
        }

    }
}

impl <T: Ord+Copy> Lattice for DisjoinedSet<T> {
    fn meet(self, other: Self) -> Self {
        !todo!()
    }
    fn join(self, mut other: Self) -> Self {
        let mut fusion_set: BTreeSet<BTreeSet<T>> = BTreeSet::new();
        // for 
        Self(fusion_set)
    }

//     fn meet_mut(&mut self, other: Self) -> bool {
//       let res = !(*self <= other);
//       crate::util::update(self, |x| x.meet(other));
//       res
//    }

//     fn join_mut(&mut self, other: Self) -> bool {
//       let res = !(*self >= other);
//       crate::util::update(self, |x| x.join(other));
//       res
//    }
}
