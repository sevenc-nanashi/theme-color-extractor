# theme-color-extractor / 良い感じにテーマカラーを抽出するツール

画像からテーマカラーを抽出するライブラリです。
彩度、透明度、明度の重み月平均で行なっています。

## 開発

```bash
# テスト
$ cargo run -r --example cli -- /path/to/image.jpg

# ビルド
$ wasm-pack build --target no-modules
```

## ライセンス

MIT Licenseで公開しています。
