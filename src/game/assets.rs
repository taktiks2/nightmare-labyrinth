//! ゲームアセット管理システム
//!
//! ゲームで使用する各種アセット（スプライト、フォント、レベルデータなど）の
//! ロードと管理を担当するシステム
//!
//! # サポートするアセットタイプ
//! - **Asepriteファイル**: ピクセルアートのスプライトアニメーション
//! - **JSONファイル**: レベルデータやゲーム設定
//! - **フォント**: 日本語フォントのサポート
//! - **動的アセット**: RONファイルで設定されたアセットの動的ロード
//!
//! # ローディングフロー
//! 1. **Loading状態**: ローディング画面を表示し、アセットを非同期でロード
//! 2. **アセットコレクション**: 必要なアセットを一括でロード
//! 3. **Title状態**: ロード完了後、自動的にタイトル画面に遷移

use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;

use crate::game::{resources, states};

/// アセット管理プラグイン
///
/// ゲームで使用する各種アセットのローダーとローディングシステムを初期化
/// アセットのロードが完了するまでLoading状態を維持し、
/// 完了後にTitle状態に自動遷移する
pub(super) fn plugin(app: &mut App) {
    app
        // JSONアセットプラグインを追加（レベルデータ用）
        // .json拡張子のファイルをresources::Level型として読み込み可能にする
        .add_plugins(JsonAssetPlugin::<resources::Level>::new(&["json"]))
        // Asepriteファイルサポートプラグインを追加
        // ピクセルアートのスプライトとアニメーションをサポート
        .add_plugins(AsepriteUltraPlugin)
        // アセットローディング状態を設定
        .add_loading_state(
            LoadingState::new(states::GameState::Loading)
                // アセットロード完了後の遷移先状態を指定
                .continue_to_state(states::GameState::Title)
                // 動的アセット設定ファイルをロード
                // RONファイルでアセットのパスや設定を定義
                .with_dynamic_assets_file::<StandardDynamicAssetCollection>(
                    "dynamic_asset.assets.ron",
                )
                // ゲームアセットコレクションをロード
                // resources::GameAssetsで定義されたすべてのアセットを一括ロード
                .load_collection::<resources::GameAssets>(),
        );
}
