# Wasmtime `examples/min-platform` integration note

このリポジトリに `wasmtime/examples/min-platform` を取り込むための
再実行可能な手順を追加しました。

## 追加内容

- `scripts/import_wasmtime_min_platform.sh`
  - upstream (`bytecodealliance/wasmtime`) を shallow clone して
    `examples/min-platform` を `third_party/wasmtime-min-platform` へコピーします。
- `third_party/wasmtime-min-platform/.gitkeep`
  - 取り込み先ディレクトリを事前作成。

## 実行方法

```bash
./scripts/import_wasmtime_min_platform.sh
```

## 補足

この環境では GitHub へのアクセスが `403` で失敗する場合があります。
その場合はネットワーク到達可能な環境で上記スクリプトを実行してください。
