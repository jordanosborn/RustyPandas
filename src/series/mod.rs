use chrono::prelude::*;
use std::any::Any;

pub enum Dtype {
    STRING,
    FLOAT,
    INTEGER,
    DATETIME
}

type UnderlyingStorage<T: Any> = Vec<T>;

pub struct Series<T: Any, K: Any> {
    data: UnderlyingStorage<T>,
    pub dtype: Dtype,
    pub index: UnderlyingStorage<T>,
    pub index_dtype: Dtype,
    pub name: String
}

pub impl<T: Any> Series<T> {
    fn new<T>(array: &[T], dtype: Dtype, index: Option<(&[T], Dtype)>, name: Option<String>) -> Option<Self> {
        let index, index_dtype = index.unwrap_or_default(((0..array.len()).collect::<UnderlyingStorage>(), Dtype::INTEGER));
        if index.len() != array.len() {
            None
        }
        Some(
            Series {
                data: UnderlyingStorage<T>::new(array),
                dtype,
                index,
                index_dtype
                name: name.unwrap_or_default(String::from(""))
            }
        )
    }
}
