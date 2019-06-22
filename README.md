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

使いかたについては、テンプレートの名前をクリックして表示されたREADMEを参照してください。

If you want a template with English source code comments, please request it to us by filing [a GitHub issue][gh-issue].

[ja-branch]: ./tree/ja
[vendor-ja-branch]: ./tree/vendor-ja
[gh-issue]: ./issues


## ライセンス / License

本リポジトリの内容は **MITライセンス** のもとで公開されています。
詳しくは[LICENSE][license-file]ファイルを参照してください。

[license-file]: ./blob/master/LICENSE
