pub mod sub_a;
pub mod sub_b;

pub fn run() {
    println!("Hello, Vars module!");
    // sub_a::fnc_a();
    // sub_b::fnc_b();
    let mut x = 5;
    println!("The value of x is:{}", x);
    x = 6;
    println!("The value of x is:{}", x);

    let i1 = 0.12;
}
