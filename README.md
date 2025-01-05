# Nightmare Labyrinth

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
