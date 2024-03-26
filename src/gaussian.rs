use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Div, Mul, Rem, Sub};

#[derive(Copy)]
pub struct Gaussian {
    real: isize,
    imag: isize,
}

const ZERO: Gaussian = Gaussian { real: 0, imag: 0 };
const UNITS: [Gaussian; 4] = [
    Gaussian { real: 1, imag: 0 },
    Gaussian { real: -1, imag: 0 },
    Gaussian { real: 0, imag: 1 },
    Gaussian { real: 0, imag: -1 },
];

impl Gaussian {
    pub fn new(real: isize, imag: isize) -> Self {
        Self {
            real,
            imag,
        }
    }
    pub fn add(&self, second: &Self) -> Self {
        Self {
            real: self.real + second.real,
            imag: self.imag + second.imag,
        }
    }

    pub fn subtract(&self, second: &Self) -> Self {
        Self {
            real: self.real - second.real,
            imag: self.imag - second.imag,
        }
    }

    pub fn multiply(&self, second: &Self) -> Self {
        Self {
            real: (self.real * second.real) - (self.imag * second.imag),
            imag: (self.real * second.imag) + (self.imag * second.real),
        }
    }

    pub fn norm(&self) -> isize {
        self.real.pow(2) + self.imag.pow(2)
    }

    pub fn float_div(&self, second: &Self) -> (f64, f64) {
        let real = ((self.real * second.real) + (self.imag * second.imag)) as f64;
        let imag = ((self.imag * second.real) - (self.real * second.imag)) as f64;
        let norm = second.norm() as f64;
        (real / norm, imag / norm)
    }

    pub fn div_rem(&self, second: &Self) -> (Self, Self) {
        let div = self.float_div(second);
        let q = Self::new(div.0.round() as isize, div.1.round() as isize);
        let r = self.subtract(&q.multiply(second));
        (q, r)
    }

    pub fn gcd(first: &Self, second: &Self) -> Self {
        let (mut higher, mut lower) = if first > second {
            (first.clone(), second.clone())
        } else {
            (second.clone(), first.clone())
        };
        while lower != ZERO {
            let rem = higher % lower;
            higher = lower;
            lower = rem;
        }
        if UNITS.contains(&higher) {
            higher = UNITS[0];
        }
        higher
    }

    pub fn lcm(first: &Self, second: &Self) -> Self {
         (first.clone() * second.clone()) / Self::gcd(first, second)
    }


    pub fn check_round(&self, second: &Self) {
        let div = self.float_div(second);

        let q = Self::new(div.0.floor() as isize, div.1.floor() as isize);
        let r = self.subtract(&q.multiply(second));
        let modu = r.norm();
        println!("Floor Floor {} {} {}", q, r, modu);

        let q = Self::new(div.0.floor() as isize, div.1.ceil() as isize);
        let r = self.subtract(&q.multiply(second));
        let modu = r.norm();
        println!("Floor Ceil {} {} {}", q, r, modu);

        let q = Self::new(div.0.ceil() as isize, div.1.ceil() as isize);
        let r = self.subtract(&q.multiply(second));
        let modu = r.norm();
        println!("Ceil Ceil {} {} {}", q, r, modu);

        let q = Self::new(div.0.ceil() as isize, div.1.floor() as isize);
        let r = self.subtract(&q.multiply(second));
        let modu = r.norm();
        println!("Ceil Floor {} {} {}", q, r, modu);
    }
}
impl Add for Gaussian {
    type Output = Self;

    fn add(self, second: Self) -> Self::Output {
        Self {
            real: self.real + second.real,
            imag: self.imag + second.imag,
        }
    }
}

impl Sub for Gaussian {
    type Output = Self;

    fn sub(self, second: Self) -> Self::Output {
        Self {
            real: self.real - second.real,
            imag: self.imag - second.imag,
        }
    }
}
impl Mul for Gaussian {
    type Output = Self;

    fn mul(self, second: Self) -> Self::Output {
        Self {
            real: (self.real * second.real) - (self.imag * second.imag),
            imag: (self.real * second.imag) + (self.imag * second.real),
        }
    }
}

impl Div for Gaussian {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let (real, imag) = self.float_div(&rhs);
        Gaussian{
            real: real.round() as isize,
            imag: imag.round() as isize
        }
    }
}

impl Rem for Gaussian {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        self - ((self / rhs) * rhs)
    }
}

impl Debug for Gaussian {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

impl Display for Gaussian {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

impl Clone for Gaussian {
    fn clone(&self) -> Self {
        Self {
            real: self.real,
            imag: self.imag,
        }
    }
}

impl PartialEq for Gaussian {
    fn eq(&self, other: &Self) -> bool {
        self.real == other.real && self.imag == other.imag
    }
}

impl PartialOrd for Gaussian {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.norm().cmp(&other.norm()))
    }
}