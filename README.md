# rust-try

## コマンド

```sh
https://www.rust-lang.org

#インストール
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

#マシン再起動
reboot

#アップデート
rustup update
```

```sh
#パッケージ作成
cargo new rust-try --bin
```

```sh
#デバックビルド
cargo build

#実行
cargo run
```

```sh
#リリースビルド
cargo build --release

#実行
./target/release/rust-try
```

## WEBサーバ実装

Rustの日本語ドキュメント/Japanese Docs for Rust  
https://doc.rust-jp.rs//

シングルスレッドのWebサーバを構築する  
https://doc.rust-jp.rs/book-ja/ch20-01-single-threaded.html
