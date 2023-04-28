# Reじゅもん

ドラゴンクエスト　ふっかつのじゅもん　修正ツール

このツールは、単なるパスワード生成ツールではなく、自動的に誤字を検出して修正するツールです。
紙に書いたパスワードが誤っていた場合などに役立つように作りました。

※誤字は１つまで検出する<br>
※DQ2の形式はまだ実装されていない

## 使い方
例:
```sh
>cargo run きへづみやしやねふりたすちなのへむびおの

Found 7 substitutions:
れへづみやしやねふりたすちなのへむびおの
GameData { name: ['ら', 'は', 'え', '２'], experience: 139, gold: 73, weapon: 7, armor: 1, shield: 2, herbs: 0, keys: 0, items: [0, 4, 0, 0, 0, 0, 0, 0], progress_flags: [false, false, true, false, false], encryption_key: 3, checksum: 229 }

きへづみやぎやねふりたすちなのへむびおの
GameData { name: ['ら', 'は', 'え', '２'], experience: 132, gold: 33, weapon: 7, armor: 1, shield: 2, herbs: 0, keys: 0, items: [0, 4, 0, 8, 0, 0, 0, 0], progress_flags: [false, false, true, true, false], encryption_key: 3, checksum: 130 }

きへづみやしやねふれたすちなのへむびおの
GameData { name: ['ら', 'は', 'え', 'の'], experience: 132, gold: 73, weapon: 7, armor: 1, shield: 2, herbs: 0, keys: 0, items: [0, 4, 0, 0, 0, 0, 0, 8], progress_flags: [false, false, false, false, false], encryption_key: 3, checksum: 130 }

きへづみやしやねふぎたすちなのへむびおの
GameData { name: ['ら', 'は', 'え', 'は'], experience: 132, gold: 73, weapon: 7, armor: 1, shield: 1, herbs: 0, keys: 0, items: [0, 4, 0, 0, 0, 0, 0, 8], progress_flags: [false, false, true, false, false], encryption_key: 7, checksum: 130 }

きへづみやしやねふりだすちなのへむびおの
GameData { name: ['ら', 'は', 'え', 'る'], experience: 132, gold: 73, weapon: 2, armor: 2, shield: 0, herbs: 0, keys: 0, items: [0, 4, 0, 0, 0, 0, 0, 0], progress_flags: [false, false, false, false, false], encryption_key: 7, checksum: 130 }

きへづみやしやねふりたすちなのへかびおの
GameData { name: ['ら', 'へ', 'え', '２'], experience: 58756, gold: 73, weapon: 7, armor: 1, shield: 2, herbs: 0, keys: 0, items: [0, 4, 0, 0, 0, 0, 0, 0], progress_flags: [false, false, true, false, false], encryption_key: 3, checksum: 130 }

きへづみやしやねふりたすちなのへむろおの
GameData { name: ['ら', 'ら', 'え', '２'], experience: 32900, gold: 73, weapon: 7, armor: 1, shield: 2, herbs: 0, keys: 0, items: [1, 4, 0, 0, 0, 0, 0, 0], progress_flags: [false, true, true, false, false], encryption_key: 3, checksum: 130 }
```
