use core::fmt;
use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Mul},
};

use anyhow::{anyhow, Result};

// martix example:
// [[1, 2], [1, 2], [1, 2]] => [1, 2, 1, 2, 1, 2]

pub struct Matrix<T> {
    data: Vec<T>,
    row: usize,
    col: usize,
}

pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: Debug + Copy + Default + Add<Output = T> + AddAssign + Mul<Output = T>,
{
    if a.col != b.row {
        return Err(anyhow!("Matrix a's col must be equal to matrix b's row"));
    }

    let mut data = vec![T::default(); a.row * b.col];
    for i in 0..a.row {
        for j in 0..b.col {
            for k in 0..a.col {
                data[i * b.col + j] += a.data[i * a.col + k] * b.data[k * b.col + j];
            }
        }
    }

    Ok(Matrix {
        data,
        row: a.row,
        col: b.col,
    })
}

impl<T: fmt::Debug> Matrix<T> {
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
    T: fmt::Debug,
{
    // display a 2x3 as "{1 2 3, 4 5 6}" 3x2 as "{1 2, 3 4, 5 6}"
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        for i in 0..self.row {
            for j in 0..self.col {
                write!(f, "{:?} ", self.data[i * self.col + j])?;
            }
            if i != self.row - 1 {
                write!(f, ",")?;
            }
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl<T> fmt::Debug for Matrix<T>
where
    T: fmt::Display + Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Matrix(row={}, col={}, {})", self.row, self.col, self)
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Ok;

    use super::*;

    #[test]
    fn test_multiply_matrix() -> Result<()> {
        let a = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new(vec![1, 2, 3, 4, 5, 6], 3, 2);
        println!("{:?}", a);
        println!("{:?}", b);
        let c = multiply(&a, &b)?;
        println!("{:?}", c);
        assert!(c.data == vec![22, 28, 49, 64]);
        Ok(())
    }

    #[test]
    fn test_matrix_new() -> Result<()> {
        let a = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        println!("{:?}", a);
        assert!(a.data == vec![1, 2, 3, 4, 5, 6]);
        Ok(())
    }
}
