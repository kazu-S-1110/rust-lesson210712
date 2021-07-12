// pub mod sub_a; //サブモジュールを親でも使いたい場合はpubをつける
// mod sub_b;

// 定数はconstで宣言が可能,定数名は全て大文字で書く。
const MAX_POINT: u32 = 100_000;
// 変数はスコープの外では宣言が不可能。
// let z = 1; <- error

pub fn run() {
  println!("Here is vars module");
  // sub_a::func_a();
  // sub_b::func_b();
  let x = 5;
  // x = 6; //デフォルトでは変数は全てimmutable(不変)
  println!("The value of x is :{}", x);
  let mut y = 3;
  println!("The value of y is :{}", y);
  y = 4; //変えたいならmutを付ける。
  println!("The value of y is :{}", y);

  // 使っていない変数に対して先頭に_を付けるとワーニングを回避できる。
  //型の宣言はtypescriptと同じ（型推論も有効）
  let _i1: i32 = 3;
  let _f1: f64 = 0.41;

  println!("{}", usize::BITS); //環境のサイズを出力してくれる！
  println!("Memory address of const is :{:p}", &MAX_POINT); //メモリアドレスを取得するには&を先頭に付ける。:pとすることでポインタの形で出力。

  let i2: i64 = 1;
  let i3: i64 = 2;
  println!("Stack address of i2 is :{:p}", &i2);
  println!("Stack address of i3 is :{:p}", &i3); //メモリアドレスが定数と違うところにあることを確認。
}
