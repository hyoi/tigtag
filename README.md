Note: Japanese text only.

# ぴこげー: TigTag
「tig」も「tag」も和訳は「鬼ごっこ」だそうです。  
よくある逃げ回ってドットをすべて拾ったらステージクリアなゲーム。  
昔のベーマガみたいなピコゲーを作ってみたかったのです。  
なお、逆襲なし、追手は重なるとスピードアップするマゾ仕様。
## 操作方法
カーソルキーで上下左右に移動。Escで一時停止。   
スペースキーでゲーム開始など。  
Alt＋Enterでフルスクリーンとウインドウモード切替（Not WASM版）。
## WASM版
https://hyoi.github.io/tigtag/
## Rustのコンパイル版
[bevy_webgl2_app_template](https://github.com/mrk-its/bevy_webgl2_app_template)をお借りしたので、cargo-makeを使います。   
```
cargo make --profile release run    
```
WASM版の場合は、
```
cargo make --profile release serve
```
※事前にRustのtargetの追加とか必要です、たぶんきっとおそらく
## お世話になりました
- [bevy](https://bevyengine.org/)と[その仲間たち](https://crates.io/search?q=bevy)
  - [bevy_webgl2_app_template](https://github.com/mrk-its/bevy_webgl2_app_template)
  - [bevy_prototype_lyon](https://github.com/Nilirad/bevy_prototype_lyon/)
  - [Unofficial Bevy Cheat Book](https://bevy-cheatbook.github.io/)
- [Google Fonts](https://fonts.google.com/)
  - [Press Start 2P](https://fonts.google.com/specimen/Press+Start+2P)
  - [Orbitron](https://fonts.google.com/specimen/Orbitron)
## 宿題
- Demoを見てると、すり抜けてしまう瞬間があるみたい…。再現性がはっきりするなら治したい
- 音を鳴らしたい。beep音でいいから。
- 環境によってメモリリークするかも。bevyの依存crateでGPU関係が怪しそう。治せるかどうかは未知数。  
もしかしたらbevy0.5に移行したので大丈夫になったかも。
