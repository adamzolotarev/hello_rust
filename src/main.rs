use std::mem;

fn main() {
    // println!("Hello, world!");
    // variable bindings are immutable unless specified mut
    data_types();
    operators();
}

fn data_types() {
    let a:u8 = 123; // unsigned immutable 8 bit
    println!("a = {}", a);

    // mutable
    let mut b:i8 = 0; // signed mutable
    println!("a = {}", b);
    b = 42;
    println!("a = {}", b);

    let mut c = 123456789;
    println!("c={}, size={} bytes", c, mem::size_of_val(&c));
    c = -1;
    println!("c={}, size={} bytes", c, mem::size_of_val(&c));
    // i8 u8 i16 u16 i32 u32 i64 u64

    let z:isize = 123; //isize/usize
    let size_of_z = mem::size_of_val(&z);
    println!("z={}, takes up {}, {}-bit OS", z, size_of_z, size_of_z * 8);

    let d:char = 'x';
    println!("d={}, takes up {} bytes", d, mem::size_of_val(&d));

    let e = 2.5; // double-precision, 8 bytes or 64 bits, f64;
    println!("e={}, takes up {} bytes", e, mem::size_of_val(&e));

    // boolean
    let g = false;
    println!("g={}, takes up {} bytes", g, mem::size_of_val(&g));
}

fn operators(){
    // +-*/ all follow precedence
    // rust does not support ++ or --
    let mut a =2+3*4;
    println!("{}", a);
    a -=2;
    println!("{}", a);

    println!("remainder of {} / {} = {}", a, 5, (a%5));

    // power
    let a_cubed = i32::pow(2, 3);
    println!("2 cubed {}", a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3); //more efficient integer power
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    println!("b cubed = {}, b to pi = {}", b_cubed, b_to_pi);

    // bitwise operators:
    
}