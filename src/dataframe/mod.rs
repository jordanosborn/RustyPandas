#![allow(dead_code)]
use crate::array::UnderlyingStorage;
use crate::indexer::{Index, Indexer};
use crate::series::{Dtype, Series};
use nalgebra;
use std::any::Any;

struct DataFrame<T: Any + Clone, K: Index> {
    data: UnderlyingStorage<Series<T, K>>,
    index: Indexer<K>,
    pub columns: Vec<String>,
}

impl<T: Any + Clone, K: Index> DataFrame<T, K> {
    pub fn new() -> Self {
        Self {
            data: vec![],
            index: Indexer::new(&[]).unwrap(),
            columns: vec![],
        }
    }
}
