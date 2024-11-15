use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Index, IndexMut, Mul, Sub};

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix4x4 {
    data: [[f64; 4]; 4],
}

impl Matrix4x4 {
    /// Creates a new 4x4 matrix initialized to zero.
    pub fn new() -> Self {
        Matrix4x4 {
            data: [[0.0; 4]; 4],
        }
    }

    /// Creates a 4x4 matrix from a 2D array.
    pub fn from_array(arr: [[f64; 4]; 4]) -> Self {
        Matrix4x4 { data: arr }
    }

    /// Creates an identity matrix.
    pub fn identity() -> Self {
        let mut mat = Matrix4x4::new();
        for i in 0..4 {
            mat.data[i][i] = 1.0;
        }
        mat
    }

    /// Transposes the matrix.
    pub fn transpose(&self) -> Self {
        let mut transposed = Matrix4x4::new();
        for i in 0..4 {
            for j in 0..4 {
                transposed.data[j][i] = self.data[i][j];
            }
        }
        transposed
    }

    /// Calculates the determinant of the matrix.
    pub fn determinant(&self) -> f64 {
        let m = &self.data;
        m[0][0] * self.minor(0, 0) - m[0][1] * self.minor(0, 1) + m[0][2] * self.minor(0, 2)
            - m[0][3] * self.minor(0, 3)
    }

    /// Calculates the minor of the matrix at a given row and column.
    fn minor(&self, row: usize, col: usize) -> f64 {
        let mut minor = [[0.0; 3]; 3];
        let mut mi = 0;
        for i in 0..4 {
            if i == row {
                continue;
            }
            let mut mj = 0;
            for j in 0..4 {
                if j == col {
                    continue;
                }
                minor[mi][mj] = self.data[i][j];
                mj += 1;
            }
            mi += 1;
        }
        // Calculate determinant of 3x3 minor
        minor[0][0] * (minor[1][1] * minor[2][2] - minor[1][2] * minor[2][1])
            - minor[0][1] * (minor[1][0] * minor[2][2] - minor[1][2] * minor[2][0])
            + minor[0][2] * (minor[1][0] * minor[2][1] - minor[1][1] * minor[2][0])
    }

    /// Calculates the inverse of the matrix, if it exists.
    pub fn inverse(&self) -> Option<Self> {
        let det = self.determinant();
        if det.abs() < 1e-10 {
            return None; // Singular matrix
        }

        let mut cofactors = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                let minor = self.minor(i, j);
                cofactors[i][j] = ((i + j) % 2 == 0) as i32 as f64 * minor
                    - ((i + j) % 2 != 0) as i32 as f64 * minor;
                cofactors[i][j] = self.minor(i, j) * if (i + j) % 2 == 0 { 1.0 } else { -1.0 };
            }
        }

        let adjugate = Matrix4x4::from_array(cofactors).transpose();
        Some(adjugate.scalar_mul(1.0 / det))
    }

    /// Multiplies the matrix by a scalar.
    pub fn scalar_mul(&self, scalar: f64) -> Self {
        let mut result = Matrix4x4::new();
        for i in 0..4 {
            for j in 0..4 {
                result.data[i][j] = self.data[i][j] * scalar;
            }
        }
        result
    }
}

// Implement Indexing
impl Index<usize> for Matrix4x4 {
    type Output = [f64; 4];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Matrix4x4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

// Implement Display
impl Display for Matrix4x4 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in &self.data {
            writeln!(
                f,
                "| {:>8.3} {:>8.3} {:>8.3} {:>8.3} |",
                row[0], row[1], row[2], row[3]
            )?;
        }
        Ok(())
    }
}

// Implement Add
impl Add for Matrix4x4 {
    type Output = Matrix4x4;

    fn add(self, other: Matrix4x4) -> Matrix4x4 {
        let mut result = Matrix4x4::new();
        for i in 0..4 {
            for j in 0..4 {
                result.data[i][j] = self.data[i][j] + other.data[i][j];
            }
        }
        result
    }
}

// Implement Sub
impl Sub for Matrix4x4 {
    type Output = Matrix4x4;

    fn sub(self, other: Matrix4x4) -> Matrix4x4 {
        let mut result = Matrix4x4::new();
        for i in 0..4 {
            for j in 0..4 {
                result.data[i][j] = self.data[i][j] - other.data[i][j];
            }
        }
        result
    }
}

// Implement Mul for Matrix4x4 * Matrix4x4
impl Mul for Matrix4x4 {
    type Output = Matrix4x4;

    fn mul(self, other: Matrix4x4) -> Matrix4x4 {
        let mut result = Matrix4x4::new();
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    result.data[i][j] += self.data[i][k] * other.data[k][j];
                }
            }
        }
        result
    }
}

// Implement Mul for Matrix4x4 * f64
impl Mul<f64> for Matrix4x4 {
    type Output = Matrix4x4;

    fn mul(self, scalar: f64) -> Matrix4x4 {
        self.scalar_mul(scalar)
    }
}

// Implement Mul for f64 * Matrix4x4
impl Mul<Matrix4x4> for f64 {
    type Output = Matrix4x4;

    fn mul(self, matrix: Matrix4x4) -> Matrix4x4 {
        matrix.scalar_mul(self)
    }
}
