use crate::array::UnderlyingStorage;
use std::collections::HashSet;
use std::hash::Hash;
use std::iter::FromIterator;

pub trait Index: Clone + std::cmp::Eq + Hash {}
impl Index for usize {}
impl Index for String {}

#[derive(Debug, Clone)]
pub struct Indexer<K: Index> {
    index: UnderlyingStorage<K>,
}

impl<K: Index> Indexer<K> {
    pub fn new(index: &[K]) -> Option<Self> {
        let set: std::collections::HashSet<&K> = HashSet::from_iter(index);
        if set.len() == index.len() {
            Some(Indexer {
                index: index.to_vec(),
            })
        } else {
            None
        }
    }

    fn new_no_check(index: &[K]) -> Self {
        Indexer {
            index: index.to_vec(),
        }
    }

    pub fn len(&self) -> usize {
        self.index.len()
    }

    pub fn is_empty(&self) -> bool {
        self.index.len() == 0
    }

    pub fn from_range(a: usize, b: usize) -> Indexer<usize> {
        let v = (a..b).collect::<Vec<usize>>();
        Indexer::new_no_check(v.as_slice())
    }
}

// impl<K: Index> std::ops::Index<usize> for Indexer<K> {
//     type Output = Option<K>;
//     fn index(&self, idx: usize) -> &Self::Output {
//         let a = self.index.get(idx);

//     }
// }

impl<K: Index> IntoIterator for Indexer<K> {
    type Item = K;
    type IntoIter = ::std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.index.into_iter()
    }
}
