fn main() {

    println!("{}",std::mem::size_of::<MyEnum>());
    println!("{}", std::mem::size_of::<EnumA>());  
    println!("{}", std::mem::size_of::<EnumAB>()); 
}

enum MyEnum {A(u8,u8),B,C {},}
enum EnumA {A= 255,}
enum EnumAB {A = 255,B,}