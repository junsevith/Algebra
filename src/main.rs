use crate::gaussian::Gaussian;
use crate::polynomial::Polynomial;

mod gaussian;
mod polynomial;

fn main() {
    println!("Liczby Gaussa:");
    let num = Gaussian::new(3, 4);
    let num2 = Gaussian::new(1, 3);
    println!("+ | {}", num + num2);
    println!("- | {}", num - num2);
    println!("* | {}", num * num2);
    println!("/ | {}", num / num2);
    println!("% | {}", num % num2);
    println!("gcd | {}", Gaussian::gcd(&num, &num2));
    println!("lcm | {}", Gaussian::lcm(&num, &num2));

    println!("Wielomiany:");
    println!();
    let num = Polynomial::new(&[1, 0, 1]);
    let num2 = Polynomial::new(&[1, 2, 1]);
    println!("{}", num);
    println!("{}", num2);
    println!("+ | {}", num.clone() + num2.clone());
    println!("- | {}", num.clone() - num2.clone());
    println!("* | {}", num.clone() * num2.clone());
    println!("% | {}", num.clone() % num2.clone());
    println!("gcd | {}", Polynomial::gcd(&num, &num2));
    println!("lcm | {}", Polynomial::lcm(&num, &num2));
}
