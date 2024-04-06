Note: Japanese text only.

# ぴこげー: TigTag
「tig」も「tag」も和訳は「鬼ごっこ」だそうです。  
逃げ回ってドットをすべて拾ったらステージクリアなゲーム。(よくあるヤツ)  
昔のベーマガみたいなピコゲーを作りたかったのです。  
逆襲なし、追手は重なるとスピードアップするマゾ仕様。  
SEが実装されています。音量要注意っ (≧ω≦;)

## WASM版
[https://hyoi.github.io/tigtag/](https://hyoi.github.io/tigtag/)  

## 操作方法

### キーボード⌨️
- `↑` `↓` `←` `→` キーで上下左右に移動  
- `Esc`キーで一時停止(Pause)  
  - デスクトップアプリのみ`EXIT`メニューあり  
`↑` `↓`で選択、`Space`か`Enter`で決定
- `Alt`＋`Enter`でフルスクリーンとウインドウモード切替（デスクトップアプリのみ）  
- `Hit ANY key!`が表示されてる時、一部のキーを除き何か押せば反応するハズ  

### ゲームパッド🎮
- 十字ボタンで上下左右に移動。  
- ps4 PADの`SHARE`で一時停止(Pause)  
  - デスクトップアプリのみ`EXIT`メニューあり  
`↑` `↓`で選択、`◯`で決定
- ps4 PADの`OPTIOS`でフルスクリーンとウインドウモード切替（デスクトップアプリのみ）  
※所有してないので他のPADでボタンがどれにアサインされるか分からない‥‥

## コンパイル方法
- デスクトップアプリにするなら`cargo run -r`でOK。   
※`cargo run`だとデバッグモード。いろいろ余計に表示されます
```
cargo run -r    
```
- WASMの場合は`--target`を指定してコンパイル後、`wasm-bindgen`で環境を整えます。   
※コマンドライン引数に書くディレクトリーは作業環境に合わせてください   
```
cargo build -r --target wasm32-unknown-unknown
wasm-bindgen --out-dir .\wasm --target web --no-typescript .\target\wasm32-unknown-unknown\release\tigtag.wasm
```
- WASMのコンパイルには事前にRustのtarget追加とwasm-bindgenのインストールが必要です  
- wasm-bindgenを実行するとバージョン違いの警告が出ることがあります。その時はバージョン上げましょう  
- [Unofficial Bevy Cheat Book - 13.5. Browser (WebAssembly)](https://bevy-cheatbook.github.io/platforms/wasm.html)をご参考に   
```
rustup target install wasm32-unknown-unknown
cargo install -f wasm-bindgen-cli
```


## お世話になりました
- [bevy](https://bevyengine.org/)と[その仲間たち](https://crates.io/search?q=bevy)
  - [Unofficial Bevy Cheat Book](https://bevy-cheatbook.github.io/)
- [Google Fonts](https://fonts.google.com/)
  - [Press Start 2P](https://fonts.google.com/specimen/Press+Start+2P)
  - [Orbitron](https://fonts.google.com/specimen/Orbitron)
  - [Reggae One](https://fonts.google.com/specimen/Reggae+One)
- [ドット絵ダウンロードサイト DOTOWN](https://dotown.maeda-design-room.net/)
  - Rustだから蟹 <img src="./assets/image/sprite/kani_DOTOWN.png" width="22" height="16" style="vertical-align: bottom;">  

## 宿題
- [ ] スマホでプレーできるようにしたい。
  - [ ] スマホのブラウザでちゃんと表示されるようにしたい
  - [ ] タッチ操作できたらWASMでスマホ上で遊べるかも？
