fn  main() {
        let mut my_vec:Vec<&i32>=vec![];
        let vec1=3;
        let vec2=4;
        insert_vlue(&mut my_vec, &vec1);
        println!("{:?}",my_vec);
        insert_vlue(&mut my_vec, &vec2); 
        println!("{:?}",my_vec);

}

fn insert_vlue<'r,'v>(my_vec:&'r mut Vec<&'v i32>,value: &'v i32) {
    my_vec.push(value)
}
