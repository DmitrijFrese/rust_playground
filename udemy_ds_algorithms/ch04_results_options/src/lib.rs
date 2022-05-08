// #[derive(Debug)]
// pub enum Result<T, E> {
//     Thing(T),
//     Error(E),
// }

pub enum Option<T> {
    Some(T),
    None,
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Result::Err("Cannot Divide by zero".to_string());
    }

    Result::Ok(a / b)
}

pub fn test_results_options() {
    let a = divide(12, 4);
    let b = divide(4, 0);
    match a {
        Result::Ok(v) => println!("val = {}", v),
        _ => {}
    }
    if let Result::Ok(v) = a {
        println!("val = {}", v)
    }
    println!("{:#?}, {:#?}", a, b);
}