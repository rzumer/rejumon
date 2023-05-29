# Reじゅもん

ドラゴンクエスト　ふっかつのじゅもん　修正ツール

このツールは、単なるパスワード生成ツールではなく、自動的に誤字を検出して修正するツールです。
紙に書いたパスワードが誤っていた場合などに役立つように作りました。

※誤字は１つまで検出する<br>
※DQ2の形式はまだ実装されていない

## 使用方法

`rejumon [--name <name>] [--flags <flags>] [--keep-checksum] <input>`

* --name <name>: 名前を指定して固定します。
* --flags <flags>: フラグを指定して固定します。フラグの形式は5桁のバイナリ（例：10111）となります。各フラグの意味については、dq1.rs の PROGRESS_FLAG_TABLE か以下のスクリーンショットを参照してください。
* --keep-checksum: チェックサム（チェック値）を固定します。

例:
```sh
cargo run きへづみやしやねふりたすちなのへむびおの
```
![image](https://user-images.githubusercontent.com/7488362/235271126-4193bfba-dcb8-4c77-99fe-95a3133ed325.png)
