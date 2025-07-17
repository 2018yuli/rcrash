// [[1,2],[1,2],[1,2]] => [1, 2, 1, 2, 1, 2]
// 扁平化后，效率更高，对比二维数组的双指针结构

use std::{
    fmt::{self, Debug, Display},
    ops::{Add, AddAssign, Mul},
};

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
    T: Debug + Mul<Output = T> + Add<Output = T> + AddAssign + Copy + Default,
{
    if a.col != b.row {
        return Err(anyhow::Error::msg("维度不匹配"));
    }
    let mut data = vec![T::default(); a.row * b.col];
    for i in 0..a.row {
        for j in 0..b.col {
            for k in 0..a.col {
                data[i * b.col + j] += a.data[i * a.col + k] * b.data[k * b.col + j];
            }
        }
    }

    let result = Matrix {
        data: data,
        row: a.row,
        col: b.col,
    };
    Ok(result)
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
}
