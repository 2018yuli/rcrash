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

// 自动拆箱
impl<T: Debug> Deref for Vector<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

pub fn dot_product<T>(a: Vector<T>, b: Vector<T>) -> anyhow::Result<T>
where
    T: Debug + Mul<Output = T> + Add<Output = T> + AddAssign + Copy + Default,
{
    if a.len() != b.len() {
        return Err(anyhow::Error::msg("维度不匹配"));
    }
    let mut sum = T::default();
    for i in 0..a.len() {
        sum += a[i] * b[i];
    }
    anyhow::Ok(sum)
}

pub struct MsgInput<T: Debug> {
    pub idx: usize,
    pub row: Vector<T>,
    pub col: Vector<T>,
}
impl<T: Debug> MsgInput<T> {
    pub fn new(idx: usize, row: Vector<T>, col: Vector<T>) -> Self {
        Self { idx, row, col }
    }
}
pub struct MsgOutput<T: Debug> {
    pub idx: usize,
    pub value: T,
}

pub struct Msg<T: Debug> {
    pub input: MsgInput<T>,
    pub sender: oneshot::Sender<MsgOutput<T>>,
}
impl<T: Debug> Msg<T> {
    pub fn new(input: MsgInput<T>, sender: oneshot::Sender<MsgOutput<T>>) -> Self {
        Self { input, sender }
    }
}
