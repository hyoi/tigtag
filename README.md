Note: Japanese text only.

# ぴこげー: TigTag
「tig」も「tag」も和訳は「鬼ごっこ」だそうです。  
逃げ回ってドットをすべて拾ったらステージクリアなゲーム。(よくあるヤツ)  
昔のベーマガみたいなピコゲーを作りたかったのです。  
逆襲なし、追手は重なるとスピードアップするマゾ仕様。
## 操作方法
カーソルキーで上下左右に移動。Escで一時停止。   
スペースキーでゲーム開始など。  
Alt＋Enterでフルスクリーンとウインドウモード切替（Not WASM版）。
## ~~WASM版~~
まだ正常に動きませんでした‥‥  
~~https://hyoi.github.io/tigtag/~~
## Rustのコンパイル版
```
cargo run --release    
```
~~WASM版 (brvy0.6からbevy_webgl2に頼らなくても良くなりました)~~
```
cargo build --target wasm32-unknown-unknown
```
※事前にRustのtargetの追加とかwasm-bindgenとか必要です。たぶんきっとおそらく
## お世話になりました
- [bevy](https://bevyengine.org/)と[その仲間たち](https://crates.io/search?q=bevy)
  - [bevy_prototype_lyon](https://github.com/Nilirad/bevy_prototype_lyon/)
  - [Unofficial Bevy Cheat Book](https://bevy-cheatbook.github.io/)
- [Google Fonts](https://fonts.google.com/)
  - [Press Start 2P](https://fonts.google.com/specimen/Press+Start+2P)
  - [Orbitron](https://fonts.google.com/specimen/Orbitron)
  - [Reggae One](https://fonts.google.com/specimen/Reggae+One?subset=japanese)
## 宿題
- 音を鳴らしたい。beep音でいいから。
