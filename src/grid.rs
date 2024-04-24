use std::cmp::Ordering;
use std::fmt::{Debug, Display};
use num_traits::{Num, NumAssign};

pub struct Point<T, const DIMS: usize> {
    pub coords: [T; DIMS],
}


impl<T, const DIMS: usize> Point<T, DIMS> {
    pub fn new(point: [T; DIMS]) -> Self {
        Self {
            coords: point
        }
    }
}

impl<T: Clone + Num, const DIMS: usize> Point<T, DIMS> {
    pub fn norm(&self) -> T {
        let mut sum = T::zero();
        for i in &self.coords {
            sum = sum + (i.clone() * i.clone());
        }
        sum
    }
}

impl<T: Eq, const DIMS: usize> PartialEq<Self> for Point<T, DIMS> {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..DIMS {
            if self.coords[i] != other.coords[i] {
                return false;
            }
        }
        true
    }
}

impl<T: Ord, const DIMS: usize> PartialOrd for Point<T, DIMS> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let mut smaller = false;
        let mut greater = false;

        for i in 0..DIMS {
            match self.coords[i].cmp(&other.coords[i]) {
                Ordering::Less => smaller = true,
                Ordering::Greater => greater = true,
                Ordering::Equal => {}
            }
            if smaller && greater { break; }
        }

        return match (smaller, greater) {
            (true, true) => None,
            (false, true) => Some(Ordering::Greater),
            (true, false) => Some(Ordering::Less),
            (false, false) => Some(Ordering::Equal),
        };
    }
}

impl<T: Clone, const DIMS: usize> Clone for Point<T, DIMS> {
    fn clone(&self) -> Self {
        Self {
            coords: self.coords.clone()
        }
    }
}

impl<T: Debug, const DIMS: usize> Debug for Point<T, DIMS> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.coords)
    }
}

pub fn min<T: PartialOrd>(set: &[T]) -> Vec<&T> {
    let mut mins: Vec<&T> = vec![];
    for p in set {
        let mut is_min = true;
        let mut i = 0;
        while i < mins.len() {
            match p.partial_cmp(mins[i]) {
                Some(Ordering::Less) => {
                    mins.remove(i);
                }
                Some(Ordering::Greater) => {
                    is_min = false;
                    break;
                }
                Some(Ordering::Equal) => {
                    is_min = false;
                    break;
                }
                None => {
                    i += 1;
                }
            }
        }
        if is_min {
            mins.push(p);
        }
    }
    mins
}

pub fn gen_set<T: NumAssign + Ord + Clone + Debug, const DIMS: usize>(lower_bound: &Point<T, DIMS>, upper_bound: &Point<T, DIMS>, function: fn(&Point<T, DIMS>) -> bool) -> Vec<Point<T, DIMS>> {
    let mut set = vec![];
    propagate_set(&mut set, function, &lower_bound, &upper_bound, 0);
    set
}

fn propagate_set<T: NumAssign + Ord + Clone + Debug, const DIMS: usize>(set: &mut Vec<Point<T, DIMS>>, function: fn(&Point<T, DIMS>) -> bool, start_point: &Point<T, DIMS>, upper_bound: &Point<T, DIMS>, dim_start: usize) {
    if start_point <= upper_bound {
        if function(&start_point) {
            set.push(start_point.clone());
        }
        for dim in dim_start..DIMS {
            let mut new_point = start_point.clone();
            new_point.coords[dim] += T::one();
            propagate_set(set, function, &new_point, upper_bound, dim);
        }
    }
}