
use crate::vec::*;
use crate::num::*;

// 00 01 02 03
// 04 05 06 07
// 08 09 10 11
// 12 13 14 15

const fn _create_identity<T: Number>(num_rows: usize, num_cols: usize, zero: T, one: T) -> [T; 16] {
    let mut m = [zero; 16];
    let mut r = 0;
    loop {
        let mut c = 0;
        loop {
            if r == c {
                m[r * num_rows + c] = one;
            }
            else {
                m[r * num_rows + c] = zero;
            }
            c += 1;
            if c == num_cols {
                break;
            }
        }
        r += 1;
        if r == num_rows {
            break;
        }
    }
    m
}

macro_rules! mat_impl {
    ($MatN:ident, $rows:expr, $cols:expr, $elems:expr, 
        $RowVecN:ident { $($row_field:ident, $row_field_index:expr),* },
        $ColVecN:ident { $($col_field:ident, $col_field_index:expr),* } ) => {
        #[derive(Debug, Copy, Clone)]
        pub struct $MatN<T> {
            pub m: [T; $elems]
        }

        impl<T> $MatN<T> where T: Number {
            /// initialise matrix to all zero's
            pub fn zero() -> $MatN<T> {
                $MatN {
                    m: [T::zero(); $elems]
                }
            }

            /// initialise matrix to identity
            pub fn identity() -> $MatN<T> {
                let mut mat = Self::zero();
                for r in 0..$rows {
                    for c in 0..$cols {
                        if c == r {
                            mat.set(r, c, T::one());
                        }
                    }
                }
                mat
            }

            /// get single element from the matrix at row, column index
            pub fn at(self, row: u32, column: u32) -> T {
                let urow = row as usize;
                let ucol = column as usize;
                self.m[urow * $cols + ucol]
            }

            /// set a single element of the matrix at row, column index
            pub fn set(&mut self, row: u32, column: u32, value: T) {
                let urow = row as usize;
                let ucol = column as usize;
                self.m[urow * $cols + ucol] = value
            }

            /// gets a single row of the matrix in n sized vec where in is the column count of the matrix
            pub fn get_row(&self, row: u32) -> $RowVecN<T> {
                let urow = row as usize;
                $RowVecN {
                    $($row_field: self.m[urow * $cols + $row_field_index],)+
                }
            }

            // set row

            /// gets a single column of the matrix in n sized vec where in is the row count of the matrix
            pub fn get_column(&self, column: u32) -> $ColVecN<T> {
                let ucol = column as usize;
                $ColVecN {
                    $($col_field: self.m[$col_field_index * $cols + ucol],)+
                }
            }

            // set column
        }
    }
}

impl<T> std::ops::Mul<Self> for Mat4<T> where T: Number {
    type Output = Self;
    fn mul(self, rhs: Mat4<T>) -> Mat4<T> {
        Mat4 {
            m: [
                self.m[0] * rhs.m[0] + self.m[1] * rhs.m[4] + self.m[2] * rhs.m[8] + self.m[3] * rhs.m[12],
                self.m[0] * rhs.m[1] + self.m[1] * rhs.m[5] + self.m[2] * rhs.m[9] + self.m[3] * rhs.m[13],
                self.m[0] * rhs.m[2] + self.m[1] * rhs.m[6] + self.m[2] * rhs.m[10] + self.m[3] * rhs.m[14],
                self.m[0] * rhs.m[3] + self.m[1] * rhs.m[7] + self.m[2] * rhs.m[11] + self.m[3] * rhs.m[15],

                self.m[4] * rhs.m[0] + self.m[5] * rhs.m[4] + self.m[6] * rhs.m[8] + self.m[7] * rhs.m[12],
                self.m[4] * rhs.m[1] + self.m[5] * rhs.m[5] + self.m[6] * rhs.m[9] + self.m[7] * rhs.m[13],
                self.m[4] * rhs.m[2] + self.m[5] * rhs.m[6] + self.m[6] * rhs.m[10] + self.m[7] * rhs.m[14],
                self.m[4] * rhs.m[3] + self.m[5] * rhs.m[7] + self.m[6] * rhs.m[11] + self.m[7] * rhs.m[15],

                // ..
                self.m[0] * rhs.m[0] + self.m[1] * rhs.m[4] + self.m[2] * rhs.m[8] + self.m[3] * rhs.m[12],
                self.m[0] * rhs.m[0] + self.m[1] * rhs.m[4] + self.m[2] * rhs.m[8] + self.m[3] * rhs.m[12],
                self.m[0] * rhs.m[0] + self.m[1] * rhs.m[4] + self.m[2] * rhs.m[8] + self.m[3] * rhs.m[12],
                self.m[0] * rhs.m[0] + self.m[1] * rhs.m[4] + self.m[2] * rhs.m[8] + self.m[3] * rhs.m[12],

                self.m[0] * rhs.m[0] + self.m[1] * rhs.m[4] + self.m[2] * rhs.m[8] + self.m[3] * rhs.m[12],
                self.m[0] * rhs.m[0] + self.m[1] * rhs.m[4] + self.m[2] * rhs.m[8] + self.m[3] * rhs.m[12],
                self.m[0] * rhs.m[0] + self.m[1] * rhs.m[4] + self.m[2] * rhs.m[8] + self.m[3] * rhs.m[12],
                self.m[0] * rhs.m[0] + self.m[1] * rhs.m[4] + self.m[2] * rhs.m[8] + self.m[3] * rhs.m[12],
            ]
        }
    }
}

//mat_impl!(Mat2, 2, 2, 4, Vec2 {x, 0, y, 1});
//mat_impl!(Mat3, 3, 3, 9, Vec3 {x, 0, y, 1, z, 2});

mat_impl!(Mat4, 4, 4, 16, Vec4 {x, 0, y, 1, z, 2, w, 3}, Vec4 {x, 0, y, 1, z, 2, w, 3});
mat_impl!(Mat34, 3, 4, 12, Vec4 {x, 0, y, 1, z, 2, w, 3}, Vec3 {x, 0, y, 1, z, 2});

// construct
// x identity
// x zero
// translation
// scale
// rotation

// mul 
// mul assign
// add / sub

// transpose
// inverse
// det

// korneker
// from