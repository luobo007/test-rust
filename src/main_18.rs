macro_rules! repeat {
    ($val:expr,$count:expr) => {
        std::iter::repeat($val).take($count).collect::<String>()
    };
}

macro_rules! sum {
    ($($x:expr),*) => {
        0$(+$x)*
    };
}

macro_rules! max_value {
    ($x:expr) =>($x); 
        ($x:expr,$($xs:expr),+)=>{
            std::cmp::max($x,max_value!($($xs),+))
      
    };
}

fn main() {
    assert_eq!(repeat!("x",3),"xxx");
    assert_eq!(sum!(1,2,3,4,5),15);
    assert_eq!(max_value!(1,8,9),9);
}