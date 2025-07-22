// [[1,2],[1,2],[1,2]] => [1, 2, 1, 2, 1, 2]
// 扁平化后，效率更高，对比二维数组的双指针结构

use crate::vector::{Msg, MsgInput, MsgOutput};

use super::vector::{dot_product, Vector};
use std::{
    fmt::{self, Debug, Display},
    ops::{Add, AddAssign, Mul},
    sync::mpsc,
    thread,
};

const NUM_THREADS: usize = 2;

pub struct Matrix<T: Debug> {
    data: Vec<T>,
    row: usize,
    col: usize,
}

impl<T: Debug> Matrix<T> {
    pub fn new(data: impl Into<Vec<T>>, row: usize, col: usize) -> Self {
        Self {
            data: data.into(),
            row,
            col,
        }
    }
}

impl<T> fmt::Display for Matrix<T>
where
    T: Debug,
{
    // fmt {1 2, 3 4, 5 6} for matrix [[1, 2], [3, 4], [5, 6]]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ ")?;
        for i in 0..self.row {
            for j in 0..self.col {
                write!(f, "{:?} ", self.data[i * self.col + j])?;
                if j != self.col - 1 {
                    write!(f, " ")?;
                }
            }
            if i != self.row - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl<T> fmt::Debug for Matrix<T>
where
    T: Display + Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Matrix {{ row: {}, col: {}, {} }}",
            self.row, self.col, self
        )
    }
}

pub fn mutiply_matrix<T>(a: &Matrix<T>, b: &Matrix<T>) -> anyhow::Result<Matrix<T>>
where
    T: Debug + Mul<Output = T> + Add<Output = T> + AddAssign + Copy + Default + Send + 'static,
{
    if a.col != b.row {
        return Err(anyhow::Error::msg("维度不匹配"));
    }
    let matrix_len = a.row * b.col;

    // multiple threading
    let senders = (0..NUM_THREADS)
        .map(|_| {
            let (tx, rx) = mpsc::channel::<Msg<T>>();
            thread::spawn(move || {
                for msg in rx {
                    let value = dot_product(msg.input.row, msg.input.col)?;
                    if let Err(e) = msg.sender.send(MsgOutput {
                        idx: msg.input.idx,
                        value,
                    }) {
                        eprintln!("Sender error: {}", e.to_string())
                    }
                }
                Ok::<_, anyhow::Error>(())
            });
            tx
        })
        .collect::<Vec<_>>();
    let mut receivers = Vec::with_capacity(matrix_len);

    let mut data = vec![T::default(); matrix_len];
    for i in 0..a.row {
        for j in 0..b.col {
            let row = Vector::new(&a.data[i * a.col..(i + 1) * a.col]);
            let col_data = b.data[j..]
                .iter()
                .step_by(b.col)
                .copied()
                .collect::<Vec<_>>();
            let col = Vector::new(col_data);
            // send to thread
            let idx = i * b.col + j;
            let input = MsgInput::new(idx, row, col);
            // ?
            let (tx, rx) = oneshot::channel();
            let msg = Msg::new(input, tx);
            if let Err(e) = senders[idx % NUM_THREADS].send(msg) {
                eprintln!("Sender error: {}", e.to_string());
            }
            receivers.push(rx);
        }
    }

    for rx in receivers {
        let ret = rx.recv()?;
        data[ret.idx] = ret.value;
    }

    let result = Matrix {
        data: data,
        row: a.row,
        col: b.col,
    };
    anyhow::Ok(result)
}

impl<T> Mul for Matrix<T>
where
    T: Debug + Mul<Output = T> + Add<Output = T> + AddAssign + Copy + Default + Send + 'static,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        mutiply_matrix(&self, &rhs).expect("Matrix multiply error")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mutiply() -> anyhow::Result<()> {
        let a = Matrix::new(vec![1, 2, 3, 4], 2, 2);
        let b = Matrix::new(vec![5, 6, 7, 8], 2, 2);
        let c = mutiply_matrix(&a, &b)?;
        assert_eq!(
            format!("{c:?}"),
            "Matrix { row: 2, col: 2, { 19  22 , 43  50 } }"
        );

        Ok(())
    }

    #[test]
    fn test_mutiply_with_symbol() -> anyhow::Result<()> {
        let a = Matrix::new(vec![1, 2, 3, 4], 2, 2);
        let b = Matrix::new(vec![5, 6, 7, 8], 2, 2);
        let c = a * b;
        assert_eq!(
            format!("{c:?}"),
            "Matrix { row: 2, col: 2, { 19  22 , 43  50 } }"
        );

        Ok(())
    }

    #[test]
    fn test_a_can_not_mutiply_b() {
        let a = Matrix::new(vec![1, 2, 3, 4], 2, 2);
        let b = Matrix::new(vec![5, 6, 7], 3, 1);
        let c = mutiply_matrix(&a, &b);
        assert!(c.is_err());
    }

    #[test]
    #[should_panic(expected = "维度不匹配")]
    fn test_a_can_not_mutiply_panic() {
        let a = Matrix::new(vec![1, 2, 3, 4], 2, 2);
        let b = Matrix::new(vec![5, 6, 7], 3, 1);
        let _c = a * b;
    }
}
