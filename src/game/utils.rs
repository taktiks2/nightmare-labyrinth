use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fs::File;
use std::io::{BufReader, Write};
use std::ops::Neg;

use crate::{
    game::{components, resources, states},
    globals,
};

/// 指定位置にスプライトエンティティを生成
///
/// Asepriteアセットを使用してスプライトを生成し、位置・スケール・状態を設定
/// ゲーム内オブジェクトの基本的な生成処理を担当
pub fn spawn_sprite_at(
    commands: &mut Commands,
    game_assets: &Res<resources::GameAssets>,
    tile: resources::Tile,
    position: IVec2,
    z: Option<f32>,
    name: Option<String>,
) -> Entity {
    let mut entity_commands =
        commands.spawn((
            // Asepriteスライスの設定
            AseSpriteSlice {
                name: tile.as_string(),
                aseprite: game_assets.aseprite.clone(),
            },
            // 位置とスケールの設定
            Transform::from_translation(position_to_translation(position, z))
                .with_scale(Vec3::new(globals::SPRITE_SCALE, globals::SPRITE_SCALE, 1.)),
            // ゲーム状態に応じた自動削除の設定
            StateScoped(states::GameState::Playing),
            // ゲーム内座標の設定
            components::Position(IVec2::new(position.x, position.y)),
        ));

    // 名前が指定されている場合は設定
    if let Some(name) = name {
        entity_commands.insert(Name::new(name));
    }

    entity_commands.id()
}

/// ゲーム座標をワールド座標に変換
///
/// グリッドベースのゲーム座標（IVec2）を画面上の実際の座標（Vec3）に変換
/// Y軸を反転させてピクセルアートの標準的な座標系に対応
pub fn position_to_translation(position: IVec2, z: Option<f32>) -> Vec3 {
    Vec3::new(
        // X座標：スプライトサイズとスケールを考慮
        position.x as f32 * globals::SPRITE_SIZE * globals::SPRITE_SCALE,
        // Y座標：画面座標系に合わせて反転
        position.y.neg() as f32 * globals::SPRITE_SIZE * globals::SPRITE_SCALE,
        // Z座標：深度情報（デフォルトは0.0）
        z.unwrap_or_default(),
    )
}

/// JSONファイルからRust構造体へのデシリアライゼーション
///
/// 指定されたパスのJSONファイルを読み込み、型Tの構造体に変換
/// セーブデータやゲーム設定の読み込みに使用
pub fn deserialize_json<T>(path: &str) -> Result<T, serde_json::Error>
where
    T: DeserializeOwned,
{
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let target: T = serde_json::from_reader(reader)?;
    Ok(target)
}

/// Rust構造体からJSONファイルへのシリアライゼーション
///
/// 型Tの構造体をJSONファイルとして指定パスに保存
/// セーブデータやゲーム設定の書き込みに使用
pub fn serialize_json<T>(data: &T, path: &str) -> Result<(), serde_json::Error>
where
    T: Serialize,
{
    // 読みやすい形式でJSON文字列を生成
    let json_string = serde_json::to_string_pretty(data)?;

    // ファイルを作成して書き込み
    let mut file = File::create(path).expect("Failed to create file");
    file.write_all(json_string.as_bytes())
        .expect("Failed to write to file");

    Ok(())
}
