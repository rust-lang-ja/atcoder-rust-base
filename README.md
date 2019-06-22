<!-- -*- coding:utf-8-unix -*- -->

# AtCoder Rust Base

このリポジトリは[AtCoder][atcoder]コンテスト（競技プログラミング）にRustで参加するためのCargoパッケージテンプレートです。
パッケージの作成は[cargo-generate][cargo-generate-crate]で行います。

[atcoder]: https://atcoder.jp
[cargo-generate-crate]: https://crates.io/crates/cargo-generate


## 用意されているテンプレート

以下のテンプレートが用意されています。

| 名前 | Rustバージョン | 内容 |
|:-- |:--:|:-- |
| [ja][ja-branch] | 1.35.0 | 標準的な内容のテンプレートに日本語のソースコードコメントを付けたもの。注意：2019年言語アップデート後の環境向け。Rust 1.15.1の環境では使用できない |
| [vendor-ja][vendor-ja-branch] | 1.35.0 | jaをベースに、依存するクレートのソースコードを`vendor`ディレクトリ配下に展開したもの。AtCoderの運営者が環境構築に使用できる。注意：Rust 1.15.1の環境では使用できない |

If you want a template with English source code comments, please request it to us by filing [a GitHub issue][gh-issue].

[ja-branch]: https://github.com/rust-lang-ja/atcoder-rust-base/tree/ja
[vendor-ja-branch]: https://github.com/rust-lang-ja/atcoder-rust-base/tree/vendor-ja
[gh-issue]: https://github.com/rust-lang-ja/atcoder-rust-base/issues


## 使いかた

### 準備：cargo-generateのインストール

**TODO**

### パッケージの生成

1. `cargo generate`コマンドでパッケージを生成します。

   ```console
   $ cargo generate --name abc086c --git https://github.com/rust-lang-ja/atcoder-rust-base --branch ja
   ```

   - `--name`: パッケージの名前。例：`abc086c`
   - `--branch`: このテンプレートリポジトリのブランチ名。`ja`テンプレートを使いたいなら`ja`を指定する

1. `Cargo.toml`ファイルを開き`[dependencies]`セクションにあるクレートのなかから、必要なものをコメントアウトします。

1. `tests/sample_inputs.rs`ファイルを開き、テストケースを追加します。

1. `src/main.rs`に解答となるプログラムを書きます。

1. 以下のコマンドでテストケースを実行し、テストにパスするか確認します。

   ```console
   $ cargo test -j 1
   ```

1. プログラムが完成したら、AtCoderに`src/main.rs`の内容を提出します。


## 使用可能なクレート

**TODO**


## ライセンス / License

本リポジトリの内容は **MITライセンス** のもとで公開されています。
詳しくは[LICENSE][license-file]ファイルを参照してください。

[license-file]: ./LICENSE
