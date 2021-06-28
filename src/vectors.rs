// Vectors are part of rust standard library called collections
// Collections contain multiple values


pub fn conn(){
    // let mut v: Vec<i64> = Vec::new();
    // v.push(1);
    // v.push(2);
    // v.push(3);
    // v.push(4);
    // v.push(5);
    // println!("{:?}",v);

    // ----------------Better way of creating vectors---------------------
    //Rust can infer the type being used when creating a vector
    let y = vec![5,6,7,8,9,];
    //println!("{:?}", y);
    //----------------------Reading elements of vectors-----------------------
    let third = &y[2];  // using indexing syntax
    // This syntax if best used when you want the program to panic and fail if we try to access
    // items beyond the scope of the vector
    println!("{}",third);
    let second = y.get(2); // Using the y.get method
    // When the get method is passed an index that is outside the vector, it
    // returns None without panicking.
    println!("{:?}",second);
    //-------------------------Iterating element in a vector--------------------------
    for i in &y{ // mutable items iteration
        print!(" {}", i);
    }
    let mut v = vec![6,7,8,9,11,12];
    for j in &mut v{ // Iteration over mutable vectors
        *j += 10; // the * is used to dereference j
        println!("{}", j);
    }

    //--------------------------Using enum to store different types because
    // vectors is only limited to one type --------------------------------------------------
    // Rust needs to know what types will be in the vector at compile time so it
    // knows exactly how much memory on the heap will be needed to store each
    // element
    #[derive(Debug)]
    enum Sheet{
        Int(i64),
        Float(f64),
        Text(String),
    }
    let row = vec![
        Sheet::Float(1.5),
        Sheet::Int(14),
        Sheet::Text(String::from("Secret"))
    ];
    for i in row{
        println!("{:?}",i)
    }
} // <- v and y go out of scope and is freed here
