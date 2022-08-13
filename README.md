# このリポジトリの概要

twitterで見かけたプログラミング問題たちの私なりの回答たちです.  
問題提起を行っているtweetを参考にしたり、誰かがある問題をプログラムで解いているのを見て真似したりしています。  

## ドドスコスコスコドドスコスコスコドドスコスコスコラブ注入♡

を実現するプログラムを作成しました.  

実行コマンドは以下です.

`--is-new-line`フラグを指定することで、一行ごとにドドスコしてくれます.

```bash
cd ../src_rust
cargo run --release --bin dodosuko [-- --is-new-line]
```

<blockquote class="twitter-tweet"><p lang="ja" dir="ltr">【問題】配列{&quot;ドド&quot;,&quot;スコ&quot;}からランダムに要素を標準出力し続け、『その並びが「ドドスコスコスコ」を3回繰り返したもの』に一致したときに「ラブ注入♡」と標準出力して終了するプログラムを作成せよ(配点:5点)</p>&mdash; ((🐑++)) (@Sheeeeepla) <a href="https://twitter.com/Sheeeeepla/status/1554028833942441984?ref_src=twsrc%5Etfw">August 1, 2022</a></blockquote>

### 追記

Python版も実装しました.

```bash
cd ../src_py
python dodosuko.py
```

## いーち、にぃー、さぁぁぁぁん！！！？？？

を実現するプログラムを作成しました.  

実行コマンドは以下です.

```bash
cd ../src_rust
cargo run --release --bin nabe_of_world [-- --limit-of-world N]
```

`--limit-of-world`の後ろに数字を指定することで、その数まで愚者を数えさせることができます.

<blockquote class="twitter-tweet"><p lang="ja" dir="ltr">【問題】1から順に数値をインクリメントし続けて標準出力する。ただし3の倍数と3の付く値だけアホになるプログラムを作成せよ。(最大値、アホになる際の出力は任意とする。)</p>&mdash; ((🐑++)) (@Sheeeeepla) <a href="https://twitter.com/Sheeeeepla/status/1554415675212693504?ref_src=twsrc%5Etfw">August 2, 2022</a></blockquote>

## モンティホール問題

かの有名な、メーメー問題です。  
<https://ja.wikipedia.org/wiki/%E3%83%A2%E3%83%B3%E3%83%86%E3%82%A3%E3%83%BB%E3%83%9B%E3%83%BC%E3%83%AB%E5%95%8F%E9%A1%8C>  
過去に数学者達が扉を変えた方が確率が高くなるという直観に反する結果になることをなかなか気づけず、最初に正しい答えを導き出すことができた人を叩いていたようですが、
プログラムでシミュレーションし、理論(扉を変えた方が良い)と結果が一致することを確かめました。  
十分な試行回数を設ければ、理論通り、正解の扉を選択できる確率は、扉を変えた方が二倍高い結果が得られます.  

実行コマンドは以下です.

```bash
cd ../src_rust
cargo run --release --bin monty_hall_meme [-- --n-trials N]
```

`--n-trials`の後ろに数字を指定することで、シミュレーション実験の試行回数を設定できます.

<blockquote class="twitter-tweet"><p lang="ja" dir="ltr">ついに！！<br><br>社長からの挑戦状「モンティ・ホール問題」<br>プログラムでシミュレーションできました😭✨<br><br>辛かった。できなすぎて。<br>諦めそうでした。<br>みなさんのヒントをかき集めて悩みました😌<br><br>結果「扉は変えた方が正解する可能性が高い」<br>でした！！！！！！🥳<a href="https://twitter.com/hashtag/%E9%A7%86%E3%81%91%E5%87%BA%E3%81%97%E3%82%A8%E3%83%B3%E3%82%B8%E3%83%8B%E3%82%A2%E3%81%A8%E7%B9%8B%E3%81%8C%E3%82%8A%E3%81%9F%E3%81%84?src=hash&amp;ref_src=twsrc%5Etfw">#駆け出しエンジニアと繋がりたい</a> <a href="https://t.co/Ll3bgr9uMx">pic.twitter.com/Ll3bgr9uMx</a></p>&mdash; みずほ｜社長秘書エンジニア🥳 (@tip1q) <a href="https://twitter.com/tip1q/status/1553050374764122112?ref_src=twsrc%5Etfw">July 29, 2022</a></blockquote>
