mod vars; //別モジュールからのインポート

fn main() {
    // println!("Hello, world!");
    vars::run(); //コロン２つでそのモジュールの階層を移動できる。また、モジュールはデフォルトでプライベートになっているのでパブリックにする必要がある。

    // vars::sub_a::func_a();
}
