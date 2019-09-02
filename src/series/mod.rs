#![allow(dead_code)]
use crate::array::UnderlyingStorage;
use crate::indexer::{Index, Indexer};
use chrono::prelude::*;
use std::any::Any;
use std::collections::HashSet;
use std::hash::Hash;
use std::iter::FromIterator;

#[derive(Clone, Debug)]
pub enum DtypeVector {
    STRING(Vec<String>),
    FLOAT(Vec<f64>),
    INTEGER(Vec<i64>),
    DATETIME(Vec<DateTime<Utc>>),
}

#[derive(Debug, Clone)]
pub enum Dtype {
    STRING,
    FLOAT,
    INTEGER,
    DATETIME,
}

pub struct Series<T: Any + Clone, K: Index> {
    data: UnderlyingStorage<T>,
    pub index_array: Option<Indexer<K>>,
    pub name: String,
}

impl<T: Any + Clone, K: Index> Series<T, K> {
    fn iloc(&self, idx: &K) -> Option<T> {
        if let Some(index_array) = self.index_array.to_owned() {
            let pos = index_array.into_iter().position(|x| x == *idx);
            if let Some(p) = pos {
                Some(self.data[p].clone())
            } else {
                None
            }
        } else {
            None
        }
    }

    fn iloc_vec(&self, idx: Indexer<K>) -> Option<Self> {
        if let Some(index_array) = self.index_array.clone() {
            let pos = index_array
                .into_iter()
                .enumerate()
                .flat_map(|(i, x)| {
                    if idx.clone().into_iter().find(|y| x == *y) == None {
                        None
                    } else {
                        Some(i)
                    }
                })
                .collect::<Vec<_>>();
            if pos.len() == idx.len() {
                Some(Self {
                    data: pos
                        .iter()
                        .map(|x| self.data[*x].clone())
                        .collect::<UnderlyingStorage<T>>(),
                    index_array: Some(idx),
                    name: self.name.clone(),
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    fn loc(&self, idx: &K) -> Option<T> {
        if let Some(index_array) = self.index_array.clone() {
            let pos = index_array.into_iter().position(|x| x == *idx);
            if let Some(p) = pos {
                Some(self.data[p].clone())
            } else {
                None
            }
        } else {
            None
        }
    }

    fn loc_vec(&self, idx: Indexer<K>) -> Option<Self> {
        if let Some(index_array) = self.index_array.clone() {
            let pos = index_array
                .into_iter()
                .enumerate()
                .flat_map(|(i, x)| {
                    if idx.clone().into_iter().find(|y| x == *y) == None {
                        None
                    } else {
                        Some(i)
                    }
                })
                .collect::<Vec<_>>();
            if pos.len() == idx.len() {
                Some(Self {
                    data: pos
                        .iter()
                        .map(|x| self.data[*x].clone())
                        .collect::<UnderlyingStorage<T>>(),
                    index_array: Some(idx),
                    name: self.name.clone(),
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    #[allow(clippy::redundant_closure, dead_code)]
    fn new_with_index(array: &[T], index: &[K], name: Option<String>) -> Option<Self> {
        let idx = Indexer::new(index)?;
        Some(Self {
            data: array.to_vec(),
            index_array: Some(idx),
            name: name.unwrap_or_else(|| String::from("")),
        })
    }

    #[allow(clippy::new_ret_no_self)]
    fn new(array: &[T], name: Option<String>) -> Series<T, usize> {
        let idx = Indexer::<usize>::from_range(0, array.len());
        Series {
            data: UnderlyingStorage::from(array),
            index_array: Some(idx),
            name: name.unwrap_or_else(|| String::from("")),
        }
    }
}

// impl<T: Any + Clone, K: Any + Clone + From<usize> + Hash + std::cmp::Eq> std::ops::Index<K>  for Series<T, K> {
//     type Output = Option<T>;
//     fn index(&self, idx: K) -> &<Self as std::ops::Index<K>>::Output {
//         let pos = self.index_array.iter().position(|x| *x == idx);
//         if let Some(p) = pos {
//             let out = self.data[p].clone();
//             Some(out)
//         } else {
//             &None
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mod_test() {}
}
