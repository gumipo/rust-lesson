pub fn run() {
    let s1 = String::from("hello");
    let s2 = s1;
    // s1の所有権がs2になるので参照できなくなる
    // 二重開放エラーを防ぐために所有権のMoveが起きるためs1には所有権がないため参照できない
    println!("{}", s2);

    // 静的領域を参照するStackは所有権の移動はなくStackがコピーされる
    let i1 = 1;
    let i2 = i1;

    println!("{} {}", i1, i2);
    println!("Stack address of i1 is: {:p}", &i1); // 0x7ff7b33f4be8
    println!("Stack address of i1 is: {:p}", &i2); // 0x7ff7b33f4bec

    // 文字列スライス
    let sl1 = "literal";
    let sl2 = sl1;
    println!("{} {}", sl1, sl2);
    println!("Stack address of sl1 is: {:p}", &sl1); // 0x7ff7b8984be0
    println!("Stack address of sl2 is: {:p}", &sl2); // 0x7ff7b8984bf0

    // Deep Copy : Heap領域もコピーして新しくStackとHeapを生成する
    let s3 = String::from("deep copy");
    let s4 = s3.clone();

    // 所有権のMoveが起きないので両方参照できる
    println!("{} {}", s3, s4);
    println!("Stack address of s3 is: {:p}", &s3); // 0x7ff7b61bdac0
    println!("Stack address of s4 is: {:p}", &s4); // 0x7ff7b61bdad8

    //Heapも別のアドレスに格納されている
    println!("Heap memory address  of deep copy: {:?}", s3.as_ptr()); // 0x7fef26705d70
    println!("Heap memory address  of deep copy: {:?}", s4.as_ptr()); // 0x7fef26705d80

    let s5 = String::from("hello");
    println!("Stack address of s5 is: {:p}", &s5); // 0x7ff7b0104988
    println!("Heap memory address  of hello: {:?}", s5.as_ptr()); // 0x7f8f3e705d90
    println!("Len  of s5 is: {}", s5.len()); // 5
    println!("capacity of s5 is: {}", s5.capacity()); // 5
    take_oenership(s5);

    // 所有権が関数の引数に移動するため参照できない
    // print!("{}", s5);

    let s6 = String::from("hello");
    println!("Stack address of s6 is: {:p}", &s6);
    println!("Heap memory address  of hello: {:?}", s6.as_ptr());
    println!("Len  of s6 is: {}", s6.len()); // 5

    //引数に渡して所有権がsに移動してから返り値に所有権が移る
    let s7 = take_giveback_ownership(s6);
    println!("Stack address of s7 is: {:p}", &s7);
    println!("Heap memory address  of hello: {:?}", s7.as_ptr());
    println!("Len  of s7 is: {}", s7.len()); // 5

    let s8 = String::from("hello");
    let len = calculate_length(&s8);
    println!("value is: {} {}", s8, len);

    let mut s9 = String::from("hello");
    change(&mut s9);
    println!("value is: {}", s9);

    let s10 = String::from("hello");
    let r1 = &s10;
    let r2 = &s10;
    println!("value is: {} {}", r1, r2);

    let mut s11 = String::from("hello");
    let r1 = &mut s11;
    // println!("{}", s11); // &mut のリファレンスが有効な場合は読み込みできない
    // println!("{}", r1);
    println!("{}", r1);
    println!("{}", s11); // r１の有効期間が終わりs11にアクセスできる;

    let mut s12 = String::from("hello");
    let r1 = &s12;
    let r2 = &s12;

    println!("{} and {}", r1, r2);

    let r3 = &mut s12;
    *r3 = String::from("hello_world");

    println!("{}", s12);
}

fn take_oenership(s: String) {
    println!("Stack address of s5 is: {:p}", &s); // 0x7ff7baf61ac0
    println!("Heap memory address  of hello: {:?}", s.as_ptr()); // 0x7f8f3e705d90
    println!("Len  of s5 is: {}", s.len()); // 5
    println!("capacity of s5 is: {}", s.capacity()); // 5
    print!("{}", s)
}

fn take_giveback_ownership(s: String) -> String {
    return s;
}

fn calculate_length(s: &String) -> usize {
    return s.len();
}

fn change(s: &mut String) {
    s.push_str("_world")
}
