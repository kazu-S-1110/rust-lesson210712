// pub mod sub_a; //サブモジュールを親でも使いたい場合はpubをつける
// mod sub_b;

pub fn run() {
  println!("Here is vars module");
  // sub_a::func_a();
  // sub_b::func_b();
  let x = 5;
  // x = 6; //デフォルトでは変数は全てimmutable(不変)
  println!("The value of x is :{}", x);
  let mut y = 3;
  y = 4; //変えたいならmutを付ける。
  println!("The value of y is :{}", y);
}
