fn main() {
    let number = 7;

    println!("Tell me about {}", number);
    match number {
        // 単一の値とのマッチをチェック
        1 => println!("One!"),
        // いくつかの値とのマッチをチェック
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // 特定の範囲の値とのマッチをチェック
        13..=19 => println!("A teen"),
        // その他の場合の処理
        _ => println!("Ain't special"),
    }
}