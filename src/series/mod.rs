#![allow(dead_code)]
use chrono::prelude::*;
use std::any::Any;

pub enum Dtype {
    STRING,
    FLOAT,
    INTEGER,
    DATETIME,
}

#[allow(type_alias_bounds)]
type UnderlyingStorage<T: Any> = Vec<T>;

pub struct Series<T: Any + Clone, K: Any + Clone + From<usize>> {
    data: UnderlyingStorage<T>,
    pub dtype: Dtype,
    pub index: UnderlyingStorage<K>,
    pub index_dtype: Dtype,
    pub name: String,
}

impl<T: Any + Clone, K: Any + Clone + From<usize>> Series<T, K> {
    #[allow(clippy::redundant_closure, dead_code)]
    fn new(
        array: &[T],
        dtype: Dtype,
        index: Option<(&[K], Dtype)>,
        name: Option<String>,
    ) -> Option<Self> {
        let idx: UnderlyingStorage<K>;
        let idx_dtype: Dtype;
        if let Some((i, d)) = index {
            idx = UnderlyingStorage::from(i);
            idx_dtype = d;
        } else {
            idx = (0..array.len())
                .map(|x: usize| K::from(x))
                .collect::<UnderlyingStorage<K>>();
            idx_dtype = Dtype::INTEGER;
        };
        if idx.len() == array.len() {
            Some(Self {
                data: array.to_vec(),
                dtype,
                index: idx.to_vec(),
                index_dtype: idx_dtype,
                name: name.unwrap_or_else(|| String::from("")),
            })
        } else {
            None
        }
    }
}
