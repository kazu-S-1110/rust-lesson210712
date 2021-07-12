pub mod sub_a; //サブモジュールを親でも使いたい場合はpubをつける
mod sub_b;

pub fn run() {
  println!("Here is vars module");
  sub_a::func_a();
  sub_b::func_b();
}
