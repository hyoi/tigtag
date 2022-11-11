Note: Japanese text only.

# ぴこげー: TigTag
「tig」も「tag」も和訳は「鬼ごっこ」だそうです。  
逃げ回ってドットをすべて拾ったらステージクリアなゲーム。(よくあるヤツ)  
昔のベーマガみたいなピコゲーを作りたかったのです。  
逆襲なし、追手は重なるとスピードアップするマゾ仕様。  
たいへん耳障りなSEが実装されています。ボリューム上げるな要注意っ (≧ω≦;)
## WASM版
https://hyoi.github.io/tigtag/
## 操作方法
`⇧` `⇩` `⇦` `⇨` キーで上下左右に移動。   
`Esc`キーで一時停止(Pause)。   
`Space`キーでゲーム開始など。  
`Alt`＋`Enter`でフルスクリーンとウインドウモード切替（デスクトップアプリ）。
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
  - [bevy_kira_audio](https://github.com/NiklasEi/bevy_kira_audio)
  - [Unofficial Bevy Cheat Book](https://bevy-cheatbook.github.io/)
- [Google Fonts](https://fonts.google.com/)
  - [Press Start 2P](https://fonts.google.com/specimen/Press+Start+2P)
  - [Orbitron](https://fonts.google.com/specimen/Orbitron)
  - [Reggae One](https://fonts.google.com/specimen/Reggae+One)
  - [BIZ UDPGothic](https://fonts.google.com/specimen/BIZ+UDPGothic)
- [ドット絵ダウンロードサイト DOTOWN](https://dotown.maeda-design-room.net/)
  - Rustだから蟹 <img src="./assets/sprites/kani_DOTOWN.png" width="22" height="16" style="vertical-align: bottom;">
## 宿題
- [x] (v0.2.2)音を鳴らしたい。beep音でいいから
- [x] (v0.5.0)全部 なおしたい なおしたい 病（リファクタリングにいたる病）
- [x] (v0.5.1)demoで稀に面クリするくらいに自機の逃走アルゴリズムを鍛えたい
  - [ ] もちょっと鍛える余地がありそう。アイディアはあるんだが～
- [x] (v0.5.3)自機がドットのある道を通る時、WASMだとFPSが駄々下がるのをナントカしないと‥‥
- [ ] パッドで操作できるようにしたい
- [ ] タッチ操作できたら、ブラウザ＆WASMでスマホ上で遊べる‥‥？
