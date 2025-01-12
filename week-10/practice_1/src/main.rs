fn main() {

    let v = vec![101, 250, 330, 400];
    // vector v owns the object in heap

    //only a single variable owns the heap memory at any given time
    let v2 = v.clone();
    //tow pointers to the same content is not allowed in rus

    //Rust is very smart in terms of memory access , so it detects a race ondition
    // as two variables point to the same heap

    println!("{:?}",v2);
}
