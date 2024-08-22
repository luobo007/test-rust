
fn productExceptSelf(nums: Vec<i32>) -> Vec<f64> {  
    let n = nums.len();  
    let mut left_prod: Vec<f64> = vec![1.0; n]; // 左侧乘积，初始化为1  
    let mut right_prod: Vec<f64> = vec![1.0; n]; // 右侧乘积，初始化为1  
    let mut result: Vec<f64> = Vec::with_capacity(n);  
  
    // 计算左侧乘积  
    let mut prod = 1.0;  
    for i in 0..n {  
        left_prod[i] = prod;  
        prod *= nums[i] as f64;  
    }  
  
    // 计算右侧乘积，并计算最终结果  
    prod = 1.0;  
    for i in (0..n).rev() {  
        right_prod[i] = prod;  
        prod *= nums[i] as f64;  
        result.push(left_prod[i] * right_prod[i]);  
    }  
  
    // 因为我们是从右向左遍历的，所以result需要反转以得到正确的顺序  
    result.reverse();  
  
    result  
}  
  
fn main() {  
    let nums = vec![1, 2, 3, 4];  
    println!("{:?}", productExceptSelf(nums)); // 输出：[24.0, 12.0, 8.0, 6.0]  
  
    let nums_with_floats = vec![1, 2, 3.0 as i32, 4];  
    println!("{:?}", productExceptSelf(nums_with_floats)); // 输出：[24.0, 12.0, 8.0, 6.0] 注意Rust的浮点数处理  
  
    let nums_with_zeros = vec![-1, 1, 0, -3, 3];  
    println!("{:?}", productExceptSelf(nums_with_zeros)); // 输出：[0.0, 0.0, 9.0, 0.0, 0.0]  
}