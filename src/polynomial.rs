use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Neg, Rem, Shl, Sub};

use fmtastic::Superscript;

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

    fn trim(&mut self) {
        while self.coeffs.last() == Some(&0.0) {
            self.coeffs.pop();
        }
    }


    pub fn gcd(first: &Self, second: &Self) -> Self {
        let (mut higher, mut lower) = if first.norm() > second.norm() {
            (first.clone(), second.clone())
        } else {
            (second.clone(), first.clone())
        };
        while lower.norm() > 0 {
            let rem = higher % lower.clone();
            higher = lower;
            lower = rem;
        }
        higher
    }

    pub fn lcm(first: &Self, second: &Self) -> Self {
        (first.clone() * second.clone()) / Self::gcd(first, second)
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
        let mut new = self;
        for x in new.coeffs.iter_mut() {
            *x *= -1.0;
        }
        new
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

    fn mul(self, rhs: Self) -> Self::Output {
        let mut out = vec![0.0; self.norm() + rhs.norm() - 1];
        for i in 0..self.norm() {
            for j in 0..rhs.norm() {
                out[i + j] += self.coeffs[i] * rhs.coeffs[j];
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
        let mut out = vec![0.0; self.norm() - rhs.norm() + 1];
        let mut dividend = self;
        while dividend.norm() >= rhs.norm() {
            let factor = dividend.leading() / rhs.leading();
            let num = Polynomial::new(&[factor]) << (dividend.norm() - rhs.norm());
            out[dividend.norm() - rhs.norm()] = factor;
            dividend = dividend - (rhs.clone() * num);
            dividend.trim();
        }
        Self::Output {
            coeffs: out
        }
    }
}

impl Rem for Polynomial {
    type Output = Polynomial;

    fn rem(self, rhs: Self) -> Self::Output {
        let mut out = self;
        while out.norm() >= rhs.norm() {
            let factor = out.leading() / rhs.leading();
            let num = Polynomial::new(&[factor]) << (out.norm() - rhs.norm());
            out = out - (rhs.clone() * num);
            out.trim();
            // println!("{}", copy)
        }
        out
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
        let mut out = String::new();
        let len = self.coeffs.len();
        if len == 0 {
            return write!(f, "0");
        }
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