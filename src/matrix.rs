use crate::{dot_product, Vector};
use anyhow::{anyhow, Result};
use core::fmt;
use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Mul},
    sync::mpsc,
    thread,
};

const NUM_THREAD: usize = 4;

// martix example:
// [[1, 2], [1, 2], [1, 2]] => [1, 2, 1, 2, 1, 2]

pub struct Matrix<T> {
    data: Vec<T>,
    row: usize,
    col: usize,
}

pub struct MsgInput<T> {
    idx: usize,
    row: Vector<T>,
    col: Vector<T>,
}

pub struct MsgOutput<T> {
    idx: usize,
    result: T,
}

pub struct Msg<T> {
    input: MsgInput<T>,
    // sender to send the result back, use oneshot channel
    sender: oneshot::Sender<MsgOutput<T>>,
}

pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: Debug + Copy + Default + Add<Output = T> + AddAssign + Mul<Output = T> + Send + 'static,
{
    // Check matrix dimensions to ensure they are compatible for multiplication
    if a.col != b.row {
        return Err(anyhow!(
            "Matrix a's column count must be equal to matrix b's row count."
        ));
    }

    // Create channels and spawn worker threads for parallel processing
    let senders = (0..NUM_THREAD)
        .map(|_| {
            let (tx, rx) = mpsc::channel::<Msg<T>>();
            thread::spawn(move || {
                for msg in rx {
                    let value = dot_product(msg.input.row, msg.input.col)?;
                    if let Err(e) = msg.sender.send(MsgOutput {
                        idx: msg.input.idx,
                        result: value,
                    }) {
                        eprintln!("Error sending result: {}", e);
                    }
                }
                Ok::<_, anyhow::Error>(())
            });
            tx
        })
        .collect::<Vec<_>>();

    // Prepare containers for the results and receivers to collect outputs
    let matrix_len = a.row * b.col;
    let mut data = vec![T::default(); matrix_len];
    let mut receivers = Vec::with_capacity(matrix_len);

    // Distribute tasks among workers
    for i in 0..a.row {
        for j in 0..b.col {
            let row = Vector::new(&a.data[i * a.col..(i + 1) * a.col]);
            let col_data = b.data[j..]
                .iter()
                .step_by(b.col)
                .copied()
                .collect::<Vec<_>>();
            let col = Vector::new(col_data);
            let idx = i * b.col + j;
            let input = MsgInput::new(idx, row, col);
            let (oneshot_tx, oneshot_rx) = oneshot::channel();
            let msg = Msg::new(input, oneshot_tx);
            if let Err(e) = senders[idx % NUM_THREAD].send(msg) {
                eprintln!("Error dispatching task: {}", e);
            }
            receivers.push(oneshot_rx);
        }
    }

    // Collect results from worker threads
    for rx in receivers {
        let output = rx.recv()?;
        data[output.idx] = output.result;
    }

    // Construct the resulting matrix
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

impl<T> Mul for Matrix<T>
where
    T: Debug + Copy + Default + Add<Output = T> + AddAssign + Mul<Output = T> + Send + 'static,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        multiply(&self, &rhs).expect("Failed to multiply matrices")
    }
}

impl<T> MsgInput<T> {
    pub fn new(idx: usize, row: Vector<T>, col: Vector<T>) -> Self {
        Self { idx, row, col }
    }
}

impl<T> Msg<T> {
    pub fn new(input: MsgInput<T>, sender: oneshot::Sender<MsgOutput<T>>) -> Self {
        Self { input, sender }
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
        let c = a * b;
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

    #[test]
    fn test_a_can_not_multiply_b() {
        let a = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new(vec![1, 2, 3, 4, 5, 6], 3, 3);
        let c = multiply(&a, &b);
        assert!(c.is_err());
    }

    #[test]
    #[should_panic]
    fn text_a_can_not_multiply_b_panic() {
        let a = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        let _ = a * b;
    }
}
