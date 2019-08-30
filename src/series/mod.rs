#![allow(dead_code)]
use chrono::prelude::*;
use std::any::Any;
use std::collections::HashSet;
use std::hash::Hash;
use std::iter::FromIterator;

#[derive(Copy, Clone, Debug)]
pub enum Dtype {
    STRING,
    FLOAT,
    INTEGER,
    DATETIME,
}

#[allow(type_alias_bounds)]
type UnderlyingStorage<T: Any> = Vec<T>;

pub struct Series<T: Any + Clone, K: Any + Clone + From<usize> + std::cmp::Eq + Hash> {
    data: UnderlyingStorage<T>,
    pub dtype: Dtype,
    pub index_array: UnderlyingStorage<K>,
    pub index_dtype: Dtype,
    pub name: String,
}

impl<T: Any + Clone, K: Any + Clone + From<usize> + std::cmp::Eq + Hash> Series<T, K> {
    fn iloc(&self, idx: &K) -> Option<T> {
        let pos = self.index_array.iter().position(|x| *x == *idx);
        if let Some(p) = pos {
            Some(self.data[p].clone())
        } else {
            None
        }
    }

    fn iloc_vec(&self, idx: &[K]) -> Option<Self> {
        let pos = self
            .index_array
            .iter()
            .enumerate()
            .flat_map(|(i, x)| {
                if idx.iter().find(|y| x == *y) == None {
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
                dtype: self.dtype,
                index_array: UnderlyingStorage::from(idx),
                index_dtype: self.index_dtype,
                name: self.name.clone(),
            })
        } else {
            None
        }
    }

    fn loc(&self, idx: &K) -> Option<T> {
        let pos = self.index_array.iter().position(|x| *x == *idx);
        if let Some(p) = pos {
            Some(self.data[p].clone())
        } else {
            None
        }
    }

    fn loc_vec(&self, idx: &[K]) -> Option<Self> {
        let pos = self
            .index_array
            .iter()
            .enumerate()
            .flat_map(|(i, x)| {
                if idx.iter().find(|y| x == *y) == None {
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
                dtype: self.dtype,
                index_array: UnderlyingStorage::from(idx),
                index_dtype: self.index_dtype,
                name: self.name.clone(),
            })
        } else {
            None
        }
    }

    #[allow(clippy::redundant_closure, dead_code)]
    fn new(
        array: &[T],
        dtype: Dtype,
        index: Option<(&[K], Dtype)>,
        name: Option<String>,
    ) -> Option<Self> {
        let idx: UnderlyingStorage<K>;
        let idx_dtype: Dtype;
        let length: usize;
        if let Some((i, d)) = index {
            length = i.len();
            idx = UnderlyingStorage::from(i);
            let hash_idx: std::collections::HashSet<K> = HashSet::from_iter(idx.clone());
            if length != hash_idx.len() {
                return None;
            }
            idx_dtype = d;
        } else {
            idx = (0..array.len())
                .map(|x: usize| K::from(x))
                .collect::<UnderlyingStorage<K>>();
            length = array.len();
            idx_dtype = Dtype::INTEGER;
        };
        if length == array.len() {
            Some(Self {
                data: array.to_vec(),
                dtype,
                index_array: idx,
                index_dtype: idx_dtype,
                name: name.unwrap_or_else(|| String::from("")),
            })
        } else {
            None
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
