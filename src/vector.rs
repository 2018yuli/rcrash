use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Deref, Mul},
};

#[derive()]
pub struct Vector<T: Debug> {
    data: Vec<T>,
}

impl<T: Debug> Vector<T> {
    pub fn new(data: impl Into<Vec<T>>) -> Self {
        Self { data: data.into() }
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }
}

impl<T: Debug> Deref for Vector<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

pub fn dot_matrix<T>(a: Vector<T>, b: Vector<T>) -> anyhow::Result<Vector<T>>
where
    T: Debug + Mul<Output = T> + Add<Output = T> + AddAssign + Copy + Default,
{
    if a.len() != b.len() {
        return Err(anyhow::Error::msg("维度不匹配"));
    }
    let mut sum = vec![T::default(); a.data.len()];
    for i in 0..a.data.len() {
        sum[i] += a.data[i] * b.data[i];
    }

    Ok(Vector { data: sum })
}
