fn main() {
    println!("-- main --");
    move_ownership();
    //reference_check_2();
}

fn move_ownership() {
    println!("-- move_ownership --");
    // 文字列リテラルで宣言すると値はスタックに積まれるため、変数には参照になる。
    let a = String::from("hello");
    // これを有効にすると、a -> b に所有権が移動する。
    //let b = a;
    println!("{}", a);
}

fn reference_check() {
    println!("-- reference_check --");
    let a = 10;               // immutable object
    let aref1 = &a;           // reference
    let aref2 = &a;           // reference
    println!("{}, {}, {}", a, aref1, aref2); // borrow check!! - OK
}

fn reference_check_2() {
    println!("-- reference_check_2 --");
    let mut a = 10;           // mutable object
    let a_ref1 = &a;          // reference
    let a_mut_ref1 = &mut a;  // mutable reference
    let a_mut_ref2 = &mut a;  // mutable refernece
    *a_mut_ref2 = 20;         // assign
    println!("{}", a);        // borrow check!! - OK
}
