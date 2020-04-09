<!-- -*- coding:utf-8-unix -*- -->

# AtCoder Rust Base (`ja`)

このリポジトリには[AtCoder][atcoder]コンテスト（競技プログラミング）にRustで参加するためのCargoパッケージテンプレートが用意されています。
パッケージは[cargo-generate][cargo-generate-crate]で作成します。

**この`README.md`では`ja`テンプレートの内容について説明します**。
他のテンプレートについては[こちら][list-of-templates]をご覧ください。

[atcoder]: https://atcoder.jp
[cargo-generate-crate]: https://crates.io/crates/cargo-generate
[list-of-templates]: https://github.com/rust-lang-ja/atcoder-rust-base/blob/master/README.md#用意されているテンプレート


## `ja`テンプレートの内容

- AtCoder 2019年言語アップデート後の環境向け
- Rust 1.42.0
  - `rustup`でインストールされていることを前提にしている

**TODO** もう少し詳しく書く


## 使いかた

### 準備：cargo-generateのインストール

以下のコマンドでcargo-generateをインストールします。

#### Linux (Ubuntu 18.04)

```console
$ sudo apt install libssl-dev pkg-config
$ cargo install cargo-generate
```

#### Windows 10 MSVC

```console
$ cargo install cargo-generate
```

#### macOS Mojave 10.14

cargo-generateが依存しているopenss-sysクレートは、macOSに元から入っているOpenSSLライブラリのバージョンに対応していません。
（OpenSSLのバージョンが低すぎる）　
そのOpenSSLライブラリを使おうとすると以下のようなエラーになります。

```console
error failed to run custom build command for `openssl-sys v0.9.47`
...

It looks like you're compiling on macOS, where the system contains a version of
OpenSSL 0.9.8. This crate no longer supports OpenSSL 0.9.8.

As a consumer of this crate, you can fix this error by using Homebrew to
install the `openssl` package, ...
```

エラーメッセージのおすすめにしたがって、[Homebrew][homebrew]で新しいバージョンのOpenSSLライブラリをインストールします。

```console
$ brew install openssl
```

cargo-generateをインストールします。

```console
$ cargo install cargo-generate
```

[homebrew]: https://brew.sh/


### パッケージの生成

`cargo generate`コマンドでパッケージを生成します。

```console
$ cargo generate --name abc086c \
    --git https://github.com/rust-lang-ja/atcoder-rust-base \
    --branch ja
```

- `--name`: これから作成するパッケージの名前。好きな名前が付けられる。例：`abc086c`
- `--branch`: このテンプレートリポジトリのブランチ名。`ja`テンプレートを使うなら`ja`を指定する


### 解答となるプログラムの作成

1. 使用するクレートの選択
   - [`Cargo.toml`][cargo-toml-file]ファイルを開き`[dependencies]`セクションに書かれているクレートのなかで必要なものがあればコメントを外します。
   - 注意：AtCoderの環境では、これら以外のクレートは使用できません。またバージョンも固定されています。

1. 使用するクレートのドキュメントの生成
   - 必須ではありませんが、以下のコマンドで依存クレートのドキュメントをビルドし、Webブラウザで開いておくと便利でしょう。

      ```console
      $ cargo doc --open   # ドキュメントのビルドし、ビルドできたらWebブラウザで開く
      # または
      $ cargo doc          # ドキュメントのビルドのみ行う
      ```

1. テストケースの作成
   - [`tests/sample_inputs.rs`](./tests/sample_inputs.rs)ファイルには、ひな型となるテストケースが用意されています。
   - AtCoderの問題文に書かれているサンプル入出力をこのファイルに書き写します。
     これにより`cargo test`でプログラムの動作が確認できるようになります（後述）。

1. プログラムの作成
   - [`src/main.rs`](./src/main.rs)に解答となるプログラムを書きます。

1. テストケースの実行
   - 以下のコマンドでテストケースを実行し、テストにパスすることを確認します。

      ```console
      $ cargo test -j 1
      ```

      **実行例**

      ```console
      $ cargo test -j 1
          ...
          Finished dev [unoptimized + debuginfo] target(s) in 25.31s
           Running target/debug/deps/main-aae3efe8c7e14c29

      running 0 tests

      test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

           Running target/debug/deps/sample_inputs-946c74199de6e6a4

      running 3 tests
      No
      test sample2 ... ok
      Yes
      test sample1 ... ok
      No
      test sample3 ... ok

      test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
      ```

   - `-j`オプションはテストケース実行の並列数を指定し、デフォルト値はCPUの論理コア数です。
     `-j 1`を指定すると、テストケースが複数あるときに、それらを1つずつ順番に実行するようになります。
   - 上の例では`No`や`Yes`のようにプログラムからの標準出力を表示しています。
     もしテストケースが並列に実行されると複数のテストケースからの標準出力が混ざって分かりにくくなります。
     `-j 1`の指定は、このようなときに便利です。


1. プログラムの提出
   - プログラムが完成したら`src/main.rs`の内容をAtCoderに提出します。
     `AC`を目指して頑張ってください。

[cargo-toml-file]: ./Cargo.toml


## 使用可能なクレート

AtCoderの環境では、[`Cargo.toml`][cargo-toml-file]にあらかじめ書かれているクレートのみが使用できます。
それら以外のクレートを追加すると、手元ではコンパイルできてもAtCoderの環境ではコンパイルエラーになりますので注意してください。
またクレートのバージョンも固定されており、変更できません。


## ライセンス / License

本リポジトリの内容は **MITライセンス** のもとで公開されています。
詳しくは[LICENSE][license-file]ファイルを参照してください。

[license-file]: ./LICENSE
