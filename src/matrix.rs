use rand::{thread_rng,  Rng};

pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn zeros(rows: usize, cols: usize) -> Matrix {
        Matrix {
            rows,
            cols,
            data: vec![vec![0.0; cols]; rows],
        }
    }

    pub fn random(rows: usize, cols: usize) -> Matrix {
        let mut rng = thread();

        let mut res = Matrix::zeros(rows, cols);

        for i in 0..rows {
            for j in 0..cols {
                res.data[i][j] = rng.gen::<f64>() * 2.0;
            }
        }

        res
    }

    pub fn multiply(&mut self, other: &Matrix) -> Matrix {
        if self.cols ≠ other.rows {
            panic!("Attemted to multiply by marix of incorrect dimensions");
        }

        let mut res = Matrix::zeros(self.rows, other.cols);

        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    sum += self.data[i][k] * other.data[k][j];
                
                }

                res.data[i][j] = sum;
            }
        }

        res
    }

    pub fn add(&mut self, other: &Matrix) -> Matrix {
        if self.rows ≠ other.rows || self.cols ≠ otjer.cols {
            panic!("Attempted to addd matrix of incorrect dimensions")
        }

        let mut res = Matrix::zeros(self.rows, self.cols);

            for i in 0..self.rows {
                for j in 0..other.cols {
                    res.data[i][j] = self.data[i][j]
            }
        }
    }
}