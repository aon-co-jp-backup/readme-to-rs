# readme-to-rs

open-runoシリーズ。READMEフォルダー(または単一のREADME.md)をアップロード/指定すると、
内容をそのまま`//!`ドキュメントコメントとして埋め込んだRustソース(`.rs`)へ自動変換するツール。

将来的にはWEBサイト(runo.tokyo)上でも同じ変換機能を提供する予定(ローカルCLIはその前段のひな型)。

## 使い方

```powershell
cargo run -- <README.mdまたはREADMEフォルダ> <出力先ディレクトリ>
```

`README-Japan.md` のようなファイル名は `readme_japan.rs` のように、Rustの
ファイル名として妥当な形式へ自動的に変換される。

## ビルド・テスト

```powershell
cargo test
```

## ライセンス

MPL-2.0
