# Nightmare Labyrinth (悪夢の迷路)

## ゲーム概要

悪夢から抜け出せなくなってしまった少女を救い出す。
ホラー風ローグライク
悪夢を見せるクトゥルフ神話の神に立ち向かう話にする？
ナイアーラトテップかクトゥルフ？

## ゲームシステム

- ランダム生成のマップ
- 主人公は基本戦闘をしない
- 主人公が動くまでは時間が動かない
- 満腹度の代わりに恐怖パラメータを採用
    - ゲージが満タンになるとゲームオーバー or 視界を狭くする？
    - 恐怖パラメータとは別に体力ゲージを設けるか？
- アイテムは敵から見つかりにくくなったりするもの

## スプライトシートの取り込み方

asepriteの境界で形状を1にせっていする。スプライトとスプライトの間に1ピクセルの余白を入れる。外枠には入れない。

以下でlayoutとして取り込むとピッタリ切り取れる。

```rs
let layout = TextureAtlasLayout::from_grid(
    UVec2::splat(SPRITE_SIZE as u32),
    4,
    3,
    Some(UVec2::splat(1)),
    None,
);
```

https://github.com/bevyengine/bevy/discussions/4424

## スケーリングしたときにぼやけさせない方法

Appに以下の設定を追加

```rs
.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
```

https://stackoverflow.com/a/78945047

## デバッグモードでの起動方法

```sh
cargo run --features dev
```
