use std::env;

/// クレートを使用しないでコマンドライン引数を扱うサンプル
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}

