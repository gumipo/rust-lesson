// Boxポインターを使用にして可変長型にすることでコンパイルエラーを解消できる
enum List {
    Node(i32, Box<List>),
    Nil,
}

pub fn run() {
    // let a1: [u8; 9000000] = [1; 9000000]; // 7MBのデータ 8MBだとStack Over fllowするよ

    // vector型 配列の要素を動的に変更する際に使用
    let mut v1 = vec![1, 2, 3, 4];
    let v2 = vec![5, 6, 7, 8];
    let mut v3 = vec![9, 10];

    println!("Stack address of v1 is {:p}", &v1); // 0x7ff7baf0bc58　v２のアドレスを見ると 112 - 88 = 24 バイトの差
    println!("Stack address of v2 is {:p}", &v2); // 0x7ff7baf0bc70

    println!("Heap address of v1 is {:?}", v1.as_ptr()); // 0x7f82fa904250
    println!("Length  of v1 is {}", v1.len()); // 0x7f82fa904250
    println!("capacity  of v1 is {}", v1.len()); // 0x7f82fa904250

    // 第一引数が代入先のindex 第２引数が値
    v1.insert(1, 10);

    println!("{:?}", v1); // [1, 10, 2, 3, 4]

    // 引数のindexの要素をremoveする
    v1.remove(0);
    println!("{:?}", v1); // [10, 2, 3, 4]

    // 配列を連結する
    v1.append(&mut v3);
    println!("{:?}", v1); // [10, 2, 3, 4, 9, 10]

    // Box Pointer スタックに存在するデータをHeapに移動させてBox Pointerという領域に格納する
    let t1: (i64, String) = (10, String::from("hello"));
    println!("Stack address of t1 is {:p}", &t1); // 0x7ff7b2d30bd0
    println!("Heap address of t1 is {:?}", t1.1.as_ptr()); // 0x7fca38f05d20
    println!("Len address of t1 is {}", t1.1.len()); // 0x7fca38f05d20
    println!("capacity address of t1 is {}", t1.1.capacity()); // 0x7fca38f05d20

    // Box Pointerを作る
    let mut b1 = Box::new(t1);

    // * で参照外し
    (*b1).0 += 2;
    (*b1).1 += "world";

    println!("{} {}", b1.0, b1.1);
    println!("Stack address of b1 is {:p}", &b1); // Box Pointer Stackのアドレス　=>  0x7ff7b3e81aa8
    println!("Heap address of b1 is {:p}", b1); // Heapの先頭アドレス　=>  0x7f8a24705db0
}
