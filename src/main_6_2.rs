//进阶:你能以O(1)额外空间复杂度解决这个问题?(输出数组不算作空间复杂度分析的额外空间。)
fn main() {  
    let nums = vec![1, 2, 3, 4];  
    println!("{:?}", productExceptSelf(nums)); 
  
    let nums2 = vec![-1, 1, 0, -3, 3];  
    println!("{:?}", productExceptSelf(nums2)); 
}


fn productExceptSelf(nums: Vec<i32>) -> Vec<i32> {  
    let n = nums.len();  
    let mut left_products = vec![1; n]; // 存储从左到右的乘积  
    let mut right_products = vec![1; n]; // 存储从右到左的乘积  
    let mut answer = vec![0; n];  
  
    // 计算从左到右的乘积  
    for i in 1..n {  
        left_products[i] = left_products[i - 1] * nums[i - 1];  
    }  
  
    // 计算从右到左的乘积  
    for i in (0..n - 1).rev() {  
        right_products[i] = right_products[i + 1] * nums[i + 1];  
    }  
  
    // 构建答案数组  
    for i in 0..n {  
        answer[i] = left_products[i] * right_products[i];  
    }  
  
    answer  
}  
  
