fn main() {
    let result = call(-2, 4);
    match result {  
        Ok(value) => println!("除法的平方根为: {}", value),  
        Err(MatError::DivisionByZero) => println!("等于0"),  
        Err(MatError::NegativeSquareRoot) => println!("参数不能是负数"),  
    }  
}

fn call(a:i32,b:i32)->Result<f64,MatError> {
    let r =divide(a, b).ok_or(MatError::DivisionByZero)?;
    sqrt(r)
}

fn divide(a:i32,b:i32) -> Option<f64>{
    if b!=0 {
        Some(a as f64 / b as f64)
    }else{
        None
    }
}

pub enum MatError {
    DivisionByZero,
    NegativeSquareRoot,
}

fn sqrt(x:f64)->Result<f64,MatError>{
    if x<0.0{
        Err(MatError::NegativeSquareRoot)
    }else {
        Ok(x.sqrt())
    }
}