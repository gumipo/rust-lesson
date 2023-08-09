# rust-lesson

環境構築
$ cargo init

Build & run 
$ cargo run

静的チェック
$ cargo check

ライブラリクレートの作成
$cargo new --lib ${app-name}


## Crateとmodule

- Crateは1つ以上はいる lib crate or bin crate
- libCrrateは一つまで bin Crateは何個でも作成可能

公開されたlib crate
https://crates.io/
## アプリケーションメモリ
- Heap 可変長データ
- Stack サイズが決まった変数や配列など
- Static  const,文字列リテラルの実態
- Text（コード） 


## StackとHeap

Stackのもつデータ
8byte Heapの先頭アドレス
8byte Length 
8byte Capacity   

Heap は日データをの持つ

## 所有権と借用
let str  = String::from("hello");
let s = &str  // 借用

所有権はstrのままで借用という形でsからstrを参照できる

- Vector型の24バイト Heap領域に入る
- Box Pointerの使用例

## ライフタイムとダングリングポインタ