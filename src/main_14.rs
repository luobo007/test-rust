  
fn main() {  
    let start_alloc = Instant::now();  
    let large_vec: Vec<i32> = vec![0; 1_000_000];  
    let end_alloc = Instant::now();  
  
    let start_access = Instant::now();  
    let _ = large_vec[100_000]; 
    let end_access = Instant::now();  
  
    println!("Allocation time: {:?}", end_alloc - start_alloc);  
    println!("Access time: {:?}", end_access - start_access);  
  
}