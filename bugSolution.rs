fn main() {
    let mut x = 5;
    { //Create a new scope to avoid double mutable borrow
        let y = &mut x; 
        *y += 1;
    }
    {
        let z = &mut x;
        *z += 1;
    }
    println!("{}", x);
}