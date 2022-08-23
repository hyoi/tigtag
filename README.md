Note: Japanese text only.

# ぴこげー: TigTag
「tig」も「tag」も和訳は「鬼ごっこ」だそうです。  
逃げ回ってドットをすべて拾ったらステージクリアなゲーム。(よくあるヤツ)  
昔のベーマガみたいなピコゲーを作りたかったのです。  
逆襲なし、追手は重なるとスピードアップするマゾ仕様。  
取り急ぎたいへん耳障りなSEが実装されています。ボリューム上げるな要注意っ (≧ω≦;)
## WASM版
https://hyoi.github.io/tigtag/
## 操作方法
`⇧` `⇩` `⇦` `⇨` キーで上下左右に移動。   
`Esc`キーで一時停止(Pause)。   
`Space`キーでゲーム開始など。  
`Alt`＋`Enter`でフルスクリーンとウインドウモード切替（デスクトップアプリ）。
## コンパイル方法
デスクトップアプリにするなら `cargo run`でOK。
```
cargo run -r    
```
WASMの場合は、bevy 0.6 から bevy_webgl2 に頼らなくても良くなりました。
```
cargo build -r --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./target --target web --no-typescript ./target/wasm32-unknown-unknown/release/tigtag.wasm
```
※`wasm-bindgen`コマンドの各ディレクトリーは作業環境に合わせてください。   
※WASMのコンパイルには事前にRustのtargetの追加とwasm-bindgenのインストールが必要です。  
※wasm-bindgenを実行するとバージョン違いで警告が出ることがあります。その時は素直にバージョン上げましょう。  
```
rustup target install wasm32-unknown-unknown
cargo install -f wasm-bindgen-cli
```
　[Unofficial Bevy Cheat Book - 13.5. Browser (WebAssembly)](https://bevy-cheatbook.github.io/platforms/wasm.html)をご参考に。   
### `bevy_kira_audio`が出力するloggingを止めたい時
環境変数`RUST_LOG`に`OFF`をセットすると止まるようです。   
設定方法はOS/shellに依存するのでお手元の環境を調べてください。PowerShellなら‥‥   
```
$env:RUST_LOG = "OFF" 
```
## お世話になりました
- [bevy](https://bevyengine.org/)と[その仲間たち](https://crates.io/search?q=bevy)
  - [bevy_kira_audio](https://github.com/NiklasEi/bevy_kira_audio)
  - [bevy-web-resizer](https://github.com/frewsxcv/bevy-web-resizer)
  - [Unofficial Bevy Cheat Book](https://bevy-cheatbook.github.io/)
- [Google Fonts](https://fonts.google.com/)
  - [Press Start 2P](https://fonts.google.com/specimen/Press+Start+2P)
  - [Orbitron](https://fonts.google.com/specimen/Orbitron)
  - [Reggae One](https://fonts.google.com/specimen/Reggae+One?subset=japanese)
## 宿題
- ~~音を鳴らしたい。beep音でいいから。~~
- 全部 なおしたい なおしたい 病（リファクタリングにいたる病）