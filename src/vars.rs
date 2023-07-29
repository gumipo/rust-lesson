pub mod sub_a;
pub mod sub_b;

const MAX_POINTS: u32 = 100_000;

pub fn run() {
    println!("Hello, Vars module!");
    // sub_a::fnc_a();
    // sub_b::fnc_b();
    let mut x = 5;
    println!("The value of x is:{}", x);
    x = 6;
    println!("The value of x is:{}", x);

    // _　で意図的に使用しないとできる
    let _i1 = 3;
    let _f1 = 0.1;

    println!("{}", usize::BITS);

    // メモリが格納されている番地を取得
    println!("Memory address of const is : {:p}", &MAX_POINTS); // 0x103dc9d8c

    let i2: i64 = 1;
    let i3: i64 = 2;

    println!("Stack address of is is : {:p}", &i2); // 0x7ff7bf007c20
    println!("Stack address of is is : {:p}", &i3); // 0x7ff7bf007c28

    let y = 5;
    println!("Stack address of y is : {:p}", &y); // 0x7ff7b49c1b14

    let y = y + 1;
    println!("Stack address of y is : {:p}", &y); // 0x7ff7b49c1b64

    let y = y * 2;
    println!("Stack address of y is : {:p}", &y); // 0x7ff7b49c1bb4
    println!("Value of y is : {}", y); // 12

    {
        let y = 0;
        println!("Value of y is : {}", y);
    }

    println!("Value of y is : {}", y); // 12

    // タプル型
    let t1 = (500, 6.4, "dummy");
    let (_x, _y, _z) = t1;

    println!("Value of y is : {} {} {}", t1.0, t1.1, t1.2); // 12

    let mut t2 = ((0, 1), (2, 3));
    let ((ref mut x1_ptr, ref mut y1_ptr), _) = t2;
    *x1_ptr = 5;
    *y1_ptr = -5;

    //複雑なデータが型の出力は　{:?}
    println!("{:?}", t2);

    // 配列

    let a1 = [1, 2, 3, 4, 5];
    let a2 = [0; 10];
    println!("{:?} {:?} {} {}", a1, a2, a1[2], a1[3]);

    //文字列型
    let s1 = "helloこんにちは挨拶"; // 26bytes  アプファベット１バイト 日本語は1文字3バイト
    let s2 = "hello";

    println!("Stack address of s1 is {:p}", &s1); // 0x7ff7bb4629e0
    println!("Stack address of s2 is {:p}", &s2); // 0x7ff7bb4629f0
    println!("Stack memory of s1 is {:p}", s1.as_ptr()); // 0x10493e1e0
    println!("Stack memory of s2 is {:p}", s2.as_ptr()); // 0x10493e1fa

    println!("len of s1 is {}", s1.len()); // 26
    println!("len of s1 is {}", s2.len()); // 5

    let mut s1 = String::from("hello");
    let mut s2 = String::from("helloworld");

    println!("Stack address of s1 is {:p}", &s1); // 0x7ff7b98bc8e0
    println!("Stack address of s2 is {:p}", &s2); // 0x7ff7b98bc8f8 24バイト

    println!("Heap memory address of s1 is {:?}", s1.as_ptr()); // 0x7f917ff05d60
    println!("Heap memory address of s2 is {:?}", s2.as_ptr()); // 0x7f917ff05d70

    println!("len of s1 is {}", s1.len()); // 5
    println!("len of s2 is {}", s2.len()); // 10

    println!("capacity of s1 is {}", s1.capacity()); // 5
    println!("capacity of s2 is {}", s2.capacity()); // 10

    s1.push_str("_new1");
    s2.push_str("_new2");

    println!("{} {}", s1, s2);
}
