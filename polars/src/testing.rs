use crate::prelude::*;

impl Series {
    pub fn series_equal(&self, other: &Series) -> bool {
        match self.eq(other).sum() {
            None => false,
            Some(sum) => sum as usize == self.len(),
        }
    }
}

impl DataFrame {
    pub fn frame_equal(&self, other: &DataFrame) -> bool {
        self.columns()
            .iter()
            .zip(other.columns().iter())
            .map(|(a, b)| a.series_equal(b))
            .all(|v| v)
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::*;

    #[test]
    fn test_series_equal() {
        let a = Series::new("a", [1, 2, 3].as_ref());
        let b = Series::new("b", [1, 2, 3].as_ref());
        assert!(a.series_equal(&b))
    }

    #[test]
    fn test_df_equal() {
        let a = Series::new("a", [1, 2, 3].as_ref());
        let b = Series::new("b", [1, 2, 3].as_ref());

        let df1 = DataFrame::new(vec![a, b]).unwrap();
        let df2 = df1.clone();
        assert!(df1.frame_equal(&df2))
    }
}
