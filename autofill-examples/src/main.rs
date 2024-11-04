fn main() {
    println!("factorial 5 = {}", factorial(6));

    let u = (0.0, 1.0);
    let v = (5.0, 5.0);
    println!("projection test: {:?} * {:?} = {:?}", u, v, project(u, v));
}

autofill::autofill! {

    /// < 0 should return 1
    fn factorial(i: i32) -> i32 { todo!() }

    /// project u onto v
    fn project(u: (f64, f64), v: (f64, f64)) -> (f64, f64) { todo!() }
}
