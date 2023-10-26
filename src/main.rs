use std::env;

fn sum(x: f64, y: f64) -> f64 {
    return x + y;
}

fn subtraction(x: f64, y: f64) -> f64 {
    return x - y;
}

fn division(x: f64, y: f64) -> f64 {
    const EPSILON: f64 = 1e-15;

    if (y).abs() <= EPSILON {
        panic!("division by 0");
    }
    return x/y;
}

fn multiplication(x: f64, y: f64) -> f64 {
    return x*y;
}

fn main() {
    
    let args: Vec<String> = env::args().skip(1).collect();
    let x: f64 = args[0].parse().unwrap();
    let op  = &args[1];
    let y: f64 = args[2].parse().unwrap();
    let resutl = match op.as_str() {
        "+" => sum(x, y),
        "-" => subtraction(x, y),
        "/" => division(x, y),
        "*" => multiplication(x, y),
        &_ => todo!()
    };

    println!("x: {} \ny: {}", x, y);


    println!("x {} y = {}", op, resutl);
    
}
