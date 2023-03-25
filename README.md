Note: Japanese text only.

# ぴこげー: TigTag
「tig」も「tag」も和訳は「鬼ごっこ」だそうです。  
逃げ回ってドットをすべて拾ったらステージクリアなゲーム。(よくあるヤツ)  
昔のベーマガみたいなピコゲーを作りたかったのです。  
逆襲なし、追手は重なるとスピードアップするマゾ仕様。  
たいへん耳障りなSEが実装されています。ボリューム上げるな要注意っ (≧ω≦;)
## WASM版
[https://hyoi.github.io/tigtag/tigtag/](https://hyoi.github.io/tigtag/tigtag/) (workspaceを使用したらディレクトリが一段下がった‥‥)
## 操作方法
### キーボード
- `⇧` `⇩` `⇦` `⇨` キーで上下左右に移動。
- `Space`キーでゲーム開始など。
- `Esc`キーで一時停止(Pause)。
- `Alt`＋`Enter`でフルスクリーンとウインドウモード切替（デスクトップアプリ）。
### ゲームパッド🎮
- 十字ボタンで上下左右に移動。
- 東ボタン(Ａ／◯等)でゲーム開始など。
- 北ボタン(Ｙ／△等)で一時停止(Pause)。※WASMで西ボタンがPauseになる不具合確認
- 西ボタン(Ｘ／▢等)でフルスクリーンとウインドウモード切替（デスクトップアプリ）。
## コンパイル方法
デスクトップアプリにするなら`cargo run -r`でOK。   
※`cargo run`だとデバッグモード。(升目が表示されたり)
```
cargo run -r    
```
WASMの場合は`--target`を指定してコンパイル後、`wasm-bindgen`で整えます。
```
cargo build -r --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./target --target web --no-typescript ./target/wasm32-unknown-unknown/release/tigtag.wasm
```
※`wasm-bindgen`コマンドの各ディレクトリーは作業環境に合わせてください   
※WASMのコンパイルには事前にRustのtarget追加とwasm-bindgenのインストールが必要です  
※wasm-bindgenを実行するとバージョン違いで警告が出ることがあります。その時は素直にバージョン上げましょう  
```
rustup target install wasm32-unknown-unknown
cargo install -f wasm-bindgen-cli
```
　[Unofficial Bevy Cheat Book - 13.5. Browser (WebAssembly)](https://bevy-cheatbook.github.io/platforms/wasm.html)をご参考に。   
## お世話になりました
- [bevy](https://bevyengine.org/)と[その仲間たち](https://crates.io/search?q=bevy)
  - [Unofficial Bevy Cheat Book](https://bevy-cheatbook.github.io/)
- [Google Fonts](https://fonts.google.com/)
  - [Press Start 2P](https://fonts.google.com/specimen/Press+Start+2P)
  - [Orbitron](https://fonts.google.com/specimen/Orbitron)
  - [Reggae One](https://fonts.google.com/specimen/Reggae+One)
  - [BIZ UDPGothic](https://fonts.google.com/specimen/BIZ+UDPGothic)
- [ドット絵ダウンロードサイト DOTOWN](https://dotown.maeda-design-room.net/)
  - Rustだから蟹 <img src="./assets/sprites/kani_DOTOWN.png" width="22" height="16" style="vertical-align: bottom;">  
## 宿題
- [x] bevyの最新リリースに対応させる。v0.10 対応完了。
- [ ] Schedule v3へ最適化したい。
- [ ] スマホでプレーできるようにしたい。
  - [ ] タッチ操作できたらブラウザ＆WASMでスマホ上で遊べるかも？
