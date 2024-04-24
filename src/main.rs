use crate::gaussian::Gaussian;
use crate::polynomial::Polynomial;

mod gaussian;
mod polynomial;
mod grid;

fn main() {
    // println!("Liczby Gaussa:");
    // let num = Gaussian::new(3, 4);
    // let num2 = Gaussian::new(1, 3);
    // println!("+ | {}", num + num2);
    // println!("- | {}", num - num2);
    // println!("* | {}", num * num2);
    // println!("/ | {}", num / num2);
    // println!("% | {}", num % num2);
    // println!("gcd | {}", Gaussian::gcd(&num, &num2));
    // println!("lcm | {}", Gaussian::lcm(&num, &num2));

    println!();
    println!("Wielomiany:");
    let num = Polynomial::new(&[1, 0, 1]);
    let num2 = Polynomial::new(&[1, 2, 1]);
    println!("{}", num);
    println!("{}", num2);
    println!("+ | {}", num.clone() + num2.clone());
    println!("- | {}", num.clone() - num2.clone());
    println!("* | {}", &num * &num2);
    println!("/ | {}", num.clone() / num2.clone());
    println!("% | {}", num.clone() % num2.clone());
    println!("gcd | {}", Polynomial::gcd(num.clone(), num2.clone()));
    println!("lcm | {}", Polynomial::lcm(num.clone(), num2.clone()));
    let res = Polynomial::extended_gcd(num.clone(), num2.clone());
    println!("ext_gcd | {} = ({})*({}) + ({})*({})", res.0, num, res.1, num2, res.2);


    println!();
    let point = grid::Point::new([0, 0]);
    let point2 = grid::Point::new([15, 15]);
    let set = grid::gen_set(&point, &point2,|point| point.coords[0] * point.coords[1] >= 11);
    let set = grid::min(&set);
    println!("{:?}", set);

    let point = grid::Point::new([5i32, 5]);
    let point2 = grid::Point::new([15, 15]);
    let set = grid::gen_set(&point, &point2,|point| (point.coords[0] - 10).pow(2) + (point.coords[1] - 10).pow(2) <= 25);
    let set = grid::min(&set);
    println!("{:?}", set);
}
