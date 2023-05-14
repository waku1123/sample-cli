# samplecli

逆ポーランド記法の数式を計算するサンプルcliアプリケーション

## 仕様

- 扱える演算子は `+ - * / %` のみ
- 扱える数値は32bit整数のみ

```bash
$ rpncalc
#> 1 1 +  # 入力
#> 2

#> 12 + 34 + *  # 入力
#> 21
```

`-v` オプションを付けて実行することで、入力から答えを算出する過程も出力できるようにする
```bash
$ rpncalc
#> 1 1 +
#> ["+", "1"] [1]
#> ["+"] [1, 1]
#> [] [2]
#> 2
```

## 実行

```bash
$ cargo run
```
