use std::{mem::MaybeUninit, ops::{Add, Index, IndexMut, Mul}};

#[derive(Clone, PartialEq, Debug)]
pub struct VecN<const N: usize>([f32; N]);

impl<const N: usize> VecN<N> {
    pub const fn zero() -> Self {
        Self([0.0; N])
    }

    pub fn from_fn<F: FnMut(usize) -> f32>(mut init: F) -> Self {
        Self(core::array::from_fn(|row| {
            init(row)
        }))
    }
}

impl<const N: usize> Index<usize> for VecN<N> {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const N: usize> IndexMut<usize> for VecN<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<const N: usize> Add for VecN<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from_fn(|el| self[el] + rhs[el])
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Matrix<const N: usize> ([VecN<N>; N]);

impl<const N: usize> Matrix<N> {
    pub fn ident() -> Self {
        let mut ret = Self::zero();
        for row in 0..N {
            ret[row][row] = 1.;
        }

        return ret;
    }

    pub const fn zero() -> Self {
        // unsafe:
        // this is sound because it's just a bunch of f32's. They are allowed
        // to be zero, that's valid.
        // apparently you can't do [VecN::zero(); N] without making VecN Copy
        // which is yucky because it should definitely not be Copy.
        unsafe {
            MaybeUninit::zeroed().assume_init()
        }
    }

    pub fn from_fn<F: FnMut((usize, usize)) -> f32>(mut init: F) -> Self {
        Self(core::array::from_fn(|row| {
            VecN::from_fn(|col| {
                init((row, col))
            })
        }))
    }
}

impl<const N: usize> Index<usize> for Matrix<N> {
    type Output = VecN<N>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const N: usize> IndexMut<usize> for Matrix<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<const N: usize> Add for &Matrix<N> {
    type Output = Matrix<N>;

    fn add(self, rhs: Self) -> Self::Output {
        Matrix::from_fn(|(row, col)| {
            self.0[row][col] + rhs.0[row][col]
        })
    }
}

impl<const N: usize> Add for Matrix<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        (&self).add(&rhs)
    }
}

impl<const N: usize> Mul for &Matrix<N> {
    type Output = Matrix<N>;

    fn mul(self, rhs: Self) -> Self::Output {
        Matrix::from_fn(|(row, col)| {
            (0..N).map(|idx| self[row][idx] * rhs[idx][col]).sum()
        })
    }
}

impl<const N: usize> Mul for Matrix<N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        (&self).mul(&rhs)
    }
}
