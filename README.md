<div id="top"></div>

<p style="display: inline">
<img src="https://img.shields.io/badge/-rust-000000.svg?logo=next.js" alt="https://www.rust-lang.org/">
</p>

## プロジェクトについて

Garoon APIからスケジュールを取得してGoogleカレンダーに転写(tensha)する

**Googleカレンダーにインポートする機能は未実装**

## 環境変数

実行ファイルと同階層に`.env`ファイルを作成し、以下の環境変数を設定する

```env
GAROON_BASE_URL=https://example.cybozu.com/scripts/cbgrn/grn.exe  # ポータル画面を開いた際の/portalより前のURL
GAROON_USER_ID=username
GAROON_PASSWORD=password
```

## 使い方

実行ファイルに実行権限があることを確認して以下のコマンドを実行する

```shell
./tensha
```

Googleカレンダーにインポートできる形式のCSVファイル `events.csv` が、tenshaと同階層に生成される。

ブラウザ版Googleカレンダーから上記CSVをインポートする
