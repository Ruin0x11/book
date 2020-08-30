# The Rust Programming Language

[まえがき](foreword.md)
[導入](ch00-00-introduction.md)

## Getting started

- [事始め](ch01-00-getting-started.md)
    - [インストール](ch01-01-installation.md)
    - [Hello, World!](ch01-02-hello-world.md)
    - [Hello, Cargo!](ch01-03-hello-cargo.md)

- [数当てゲームをプログラムする](ch02-00-guessing-game-tutorial.md)

- [普遍的なプログラミング概念](ch03-00-common-programming-concepts.md)
    - [変数と可変性](ch03-01-variables-and-mutability.md)
    - [データ型](ch03-02-data-types.md)
    - [関数の動作法](ch03-03-how-functions-work.md)
    - [コメント](ch03-04-comments.md)
    - [制御フロー](ch03-05-control-flow.md)

- [所有権を理解する](ch04-00-understanding-ownership.md)
    - [所有権とは？](ch04-01-what-is-ownership.md)
    - [参照と借用](ch04-02-references-and-borrowing.md)
    - [スライス](ch04-03-slices.md)

- [構造体を使用して関連のあるデータを構造化する](ch05-00-structs.md)
    - [構造体を定義し、インスタンス化する](ch05-01-defining-structs.md)
    - [構造体を使用したプログラム例](ch05-02-example-structs.md)
    - [メソッド記法](ch05-03-method-syntax.md)

- [Enum とパターンマッチング](ch06-00-enums.md)
    - [Enum を定義する](ch06-01-defining-an-enum.md)
    - [match 制御フロー演算子](ch06-02-match.md)
    - [if let で簡潔な制御フロー](ch06-03-if-let.md)

## Basic Rust Literacy

- [Managing Growing Projects with Packages, Crates, and Modules](ch07-00-managing-growing-projects-with-packages-crates-and-modules.md)
    - [Packages and Crates](ch07-01-packages-and-crates.md)
    - [Defining Modules to Control Scope and Privacy](ch07-02-defining-modules-to-control-scope-and-privacy.md)
    - [Paths for Referring to an Item in the Module Tree](ch07-03-paths-for-referring-to-an-item-in-the-module-tree.md)
    - [Bringing Paths Into Scope with the `use` Keyword](ch07-04-bringing-paths-into-scope-with-the-use-keyword.md)
    - [Separating Modules into Different Files](ch07-05-separating-modules-into-different-files.md)

- [一般的なコレクション](ch08-00-common-collections.md)
    - [ベクタ型](ch08-01-vectors.md)
    - [文字列型](ch08-02-strings.md)
    - [ハッシュマップ](ch08-03-hash-maps.md)

- [エラー処理](ch09-00-error-handling.md)
    - [panic!で回復不能なエラー](ch09-01-unrecoverable-errors-with-panic.md)
    - [Result で回復可能なエラー](ch09-02-recoverable-errors-with-result.md)
    - [panic!すべきかするまいか](ch09-03-to-panic-or-not-to-panic.md)

- [ジェネリック型、トレイト、ライフタイム](ch10-00-generics.md)
    - [ジェネリックなデータ型](ch10-01-syntax.md)
    - [トレイト: 共通の振る舞いを定義する](ch10-02-traits.md)
    - [ライフタイムで参照を有効化する](ch10-03-lifetime-syntax.md)

- [テスト](ch11-00-testing.md)
    - [テストを書く](ch11-01-writing-tests.md)
    - [テストを走らせる](ch11-02-running-tests.md)
    - [テストの体系化](ch11-03-test-organization.md)

- [入出力プロジェクト: コマンドラインプログラムを構築する](ch12-00-an-io-project.md)
    - [コマンドライン引数を受け付ける](ch12-01-accepting-command-line-arguments.md)
    - [ファイルを読み込む](ch12-02-reading-a-file.md)
    - [リファクタリングしてモジュール性の向上とエラー処理](ch12-03-improving-error-handling-and-modularity.md)
    - [テスト駆動開発でライブラリの機能を開発する](ch12-04-testing-the-librarys-functionality.md)
    - [環境変数を取り扱う](ch12-05-working-with-environment-variables.md)
    - [標準出力ではなく標準エラーにエラーメッセージを書き込む](ch12-06-writing-to-stderr-instead-of-stdout.md)

## Thinking in Rust

- [関数型言語の機能: イテレータとクロージャ](ch13-00-functional-features.md)
    - [クロージャ: 環境をキャプチャできる匿名関数](ch13-01-closures.md)
    - [一連の要素をイテレータで処理する](ch13-02-iterators.md)
    - [入出力プロジェクトを改善する](ch13-03-improving-our-io-project.md)
    - [パフォーマンス比較: ループ VS イテレータ](ch13-04-performance.md)

- [Cargo と Crates.io についてより詳しく](ch14-00-more-about-cargo.md)
    - [リリースプロファイルでビルドをカスタマイズする](ch14-01-release-profiles.md)
    - [Crates.io にクレートを公開する](ch14-02-publishing-to-crates-io.md)
    - [Cargo のワークスペース](ch14-03-cargo-workspaces.md)
    - [cargo install で Crates.io からバイナリをインストールする](ch14-04-installing-binaries.md)
    - [独自のコマンドで Cargo で拡張する](ch14-05-extending-cargo.md)

- [スマートポインタ](ch15-00-smart-pointers.md)
    - [Box はヒープのデータを指し、既知のサイズである](ch15-01-box.md)
    - [Deref トレイトにより、参照を通してデータにアクセスできる](ch15-02-deref.md)
    - [Drop トレイトにより、片付けの時にコードを実行する](ch15-03-drop.md)
    - [Rc は、参照カウント方式のスマートポインタ](ch15-04-rc.md)
    - [RefCell と内部可変性パターン](ch15-05-interior-mutability.md)
    - [循環参照し、メモリをリークするのは安全である](ch15-06-reference-cycles.md)

- [恐れるな！並行性](ch16-00-concurrency.md)
    - [スレッド](ch16-01-threads.md)
    - [メッセージ受け渡し](ch16-02-message-passing.md)
    - [状態共有](ch16-03-shared-state.md)
    - [拡張可能な並行性: Sync と Send](ch16-04-extensible-concurrency-sync-and-send.md)

- [Rust のオブジェクト指向プログラミング機能](ch17-00-oop.md)
    - [オブジェクト指向言語の特徴](ch17-01-what-is-oo.md)
    - [トレイトオブジェクトで異なる型の値を許容する](ch17-02-trait-objects.md)
    - [オブジェクト指向デザインパターンを実装する](ch17-03-oo-design-patterns.md)

## Advanced Topics

- [パターンは値の構造に合致する](ch18-00-patterns.md)
    - [パターンが使用されるかもしれない箇所全部](ch18-01-all-the-places-for-patterns.md)
    - [論駁可能性: パターンが合致しないか](ch18-02-refutability.md)
    - [パターン記法全部](ch18-03-pattern-syntax.md)

- [高度な機能](ch19-00-advanced-features.md)
    - [Unsafe Rust](ch19-01-unsafe-rust.md)
    - [高度なライフタイム](ch19-02-advanced-lifetimes.md)
    - [高度なトレイト](ch19-03-advanced-traits.md)
    - [高度な型](ch19-04-advanced-types.md)
    - [高度な関数とクロージャ](ch19-05-advanced-functions-and-closures.md)

- [最後のプロジェクト: マルチスレッドの Web サーバを構築する](ch20-00-final-project-a-web-server.md)
    - [シングルスレッドの Web サーバ](ch20-01-single-threaded.md)
    - [シングルスレッドのサーバをマルチスレッド化する](ch20-02-multithreaded.md)
    - [正常なシャットダウンとお片付け](ch20-03-graceful-shutdown-and-cleanup.md)

- [付録](appendix-00.md)
    - [A - キーワード](appendix-01-keywords.md)
    - [B - 演算子とシンボル](appendix-02-operators.md)
    - [C - 導出可能なトレイト](appendix-03-derivable-traits.md)
    - [D - Useful Development Tools](appendix-04-useful-development-tools.md)
    - [E - Editions](appendix-05-editions.md)
    - [F - 本の翻訳](appendix-06-translation.md){.active}
    - [G - Rust の作られ方と"Nightly Rust"](appendix-07-nightly-rust.md)
