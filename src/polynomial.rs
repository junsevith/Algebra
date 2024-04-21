use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Neg, Rem, Shl, Sub};

use fmtastic::Superscript;
use num_traits::Num;

pub struct Polynomial {
    coeffs: Vec<f64>,
}

impl Polynomial {
    pub fn new<T: Into<f64> + Clone>(coefficients: &[T]) -> Self {
        let mut vec = vec![];
        for i in coefficients {
            vec.push(T::into(i.clone()))
        };

        Polynomial {
            coeffs: vec
        }
    }

    pub fn norm(&self) -> usize {
        self.coeffs.len()
    }

    pub fn get(&self, index: usize) -> f64 {
        return if index < self.coeffs.len() {
            self.coeffs[index]
        } else {
            0.0
        };
    }

    pub fn leading(&self) -> f64 {
        self.coeffs.last().unwrap_or(&0.0).clone()
    }

    ///Removes all coefficients that are equal to 0 from the beginning of the polynomial
    fn trim(&mut self) {
        while self.coeffs.last() == Some(&0.0) {
            self.coeffs.pop();
        }
    }


    pub fn gcd(first: Self, second: Self) -> Self {
        let (mut higher, mut lower) = if first.norm() > second.norm() {
            (first, second)
        } else {
            (second, first)
        };
        while lower.norm() > 0 {
            let rem = higher.div_rem(&lower).1;
            higher = lower;
            lower = rem;
        }
        higher
    }

    pub fn lcm(first: Self, second: Self) -> Self {
        (&first * &second) / Self::gcd(first, second)
    }

    pub fn scale(mut self, coefficient: f64) -> Self {
        for i in self.coeffs.iter_mut() {
            *i *= coefficient;
        }
        self
    }

    pub fn div_rem(self, divisor: &Self) -> (Self, Self) {
        let mian_norm = divisor.norm();
        let mut out = vec![0.0; self.norm() - mian_norm + 1];
        let mut dividend = self;
        let mian_lead = divisor.leading();

        while dividend.norm() >= mian_norm {
            let factor = dividend.leading() / mian_lead;
            let num = divisor.clone().scale(factor) << (dividend.norm() - mian_norm);
            out[dividend.norm() - mian_norm] = factor;
            dividend = dividend - num;
        }
        (Self {
            coeffs: out
        }, dividend)
    }

    pub fn extended_gcd(a: Self, b: Self) -> (Self, Self, Self) {
        return if b.norm() == 0 {
            (a, Self::new(&[1]), Self::new(&[0]))
        } else {
            let (quotient, remainder) = a.div_rem(&b);
            let (gcd, x, y) = Self::extended_gcd(b, remainder);
            let next = x - (&quotient * &y);
            (gcd, y, next)
        }
    }
}

impl Add for Polynomial {
    type Output = Polynomial;

    fn add(self, rhs: Self) -> Self::Output {
        let (smaller, mut bigger) = if self.norm() < rhs.norm() {
            (self, rhs)
        } else {
            (rhs, self)
        };

        for i in 0..smaller.norm() {
            bigger.coeffs[i] += smaller.coeffs[i];
        }
        bigger.trim();
        bigger
    }
}

impl Neg for Polynomial {
    type Output = Polynomial;

    fn neg(self) -> Self::Output {
        self.scale(-1.0)
    }
}

impl Sub for Polynomial {
    type Output = Polynomial;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}


impl Mul for Polynomial {
    type Output = Polynomial;

    ///Multiplication of two polynomials
    ///
    /// Because the used algorithm doesn't need to consume values you can instead use references:
    /// ```
    /// &first * &second
    /// ```

    fn mul(self, rhs: Self) -> Self::Output {
        &self * &rhs
    }
}

impl Mul for &Polynomial {
    type Output = Polynomial;

    ///Multiplication of two polynomials
    ///
    /// Because the used algorithm doesn't need to consume values you can use references:
    fn mul(self, rhs: Self) -> Self::Output {
        let mut out = vec![0.0; self.norm() + rhs.norm() - 1];
        for i in 0..self.norm() {
            let first = self.coeffs[i];
            if first != 0.0 {
                for j in 0..rhs.norm() {
                    out[i + j] += first * rhs.coeffs[j];
                }
            }
        }
        Self::Output {
            coeffs: out
        }
    }
}

impl Div for Polynomial {
    type Output = Polynomial;

    fn div(self, rhs: Self) -> Self::Output {
        self.div_rem(&rhs).0
    }
}

impl Rem for Polynomial {
    type Output = Polynomial;

    fn rem(self, rhs: Self) -> Self::Output {
        self.div_rem(&rhs).1
    }
}

impl Shl<usize> for Polynomial {
    type Output = Polynomial;

    fn shl(self, rhs: usize) -> Self::Output {
        let mut out = vec![0.0; self.norm() + rhs];
        for i in 0..self.norm() {
            out[i + rhs] = self.coeffs[i];
        }
        Self::Output {
            coeffs: out
        }
    }
}

impl Clone for Polynomial {
    fn clone(&self) -> Self {
        Self {
            coeffs: self.coeffs.clone()
        }
    }
}


impl Display for Polynomial {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let len = self.coeffs.len();
        if len == 0 {
            return write!(f, "0");
        } else if len == 1 {
            return write!(f, "{}", self.coeffs[0]);
        }

        let mut out = String::new();
        for i in (0..len).rev() {
            let mut coeff = self.coeffs[i];
            let mut var = "x";
            let mut sep = "+";
            let mut pow = Superscript(i).to_string();
            if i == len - 1 {
                sep = ""
            }
            if i == 0 {
                var = "";
                pow = String::new();
            } else if i == 1 {
                pow = String::new();
            }
            if coeff < 0f64 {
                sep = "-";
                coeff = coeff.abs();
            } else if coeff == 0f64 {
                continue;
            }

            let coeff = if coeff == 1f64 && i != 0 {
                String::new()
            } else {
                coeff.to_string()
            };

            out = format!(" {} {} {}{}{}", out, sep, coeff, var, pow);
        }
        write!(f, "{}", out.trim())
    }
}