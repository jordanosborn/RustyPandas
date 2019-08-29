use nalgebra;
use crate::series;

struct DataFrame {
    data: Vec<Series>
    index: Series
    pub columns: Vec<String>
}

impl DataFrame