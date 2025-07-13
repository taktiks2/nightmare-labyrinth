//! ゲームエラー型定義
//!
//! anyhowを使用したシンプルなエラーハンドリング
//! Bevy 0.16のベストプラクティスに従ったエラーハンドリング

use anyhow::{Context, Result};

/// ゲーム専用Result型
///
/// anyhowのResultを使用してエラーハンドリングを簡素化
pub type GameResult<T> = Result<T>;

/// ファイル操作でのエラーハンドリングを簡素化するヘルパー関数
pub fn file_context(path: &str, operation: &str) -> String {
    format!("{}の{}に失敗", path, operation)
}

/// JSONファイル操作でのエラーハンドリングを簡素化するヘルパー関数
pub fn json_context(path: &str, operation: &str) -> String {
    format!("JSONファイル{}の{}に失敗", path, operation)
}

/// エンティティ操作でのエラーハンドリングを簡素化するヘルパー関数
pub fn entity_context(entity: &str, operation: &str) -> String {
    format!("エンティティ{}の{}に失敗", entity, operation)
}

/// エラーにユーザーフレンドリーなコンテキストを追加するトレイト拡張
pub trait GameContextExt<T> {
    fn with_file_context(self, path: &str, operation: &str) -> GameResult<T>;
    fn with_json_context(self, path: &str, operation: &str) -> GameResult<T>;
    fn with_entity_context(self, entity: &str, operation: &str) -> GameResult<T>;
}

impl<T, E> GameContextExt<T> for Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn with_file_context(self, path: &str, operation: &str) -> GameResult<T> {
        self.with_context(|| file_context(path, operation))
    }

    fn with_json_context(self, path: &str, operation: &str) -> GameResult<T> {
        self.with_context(|| json_context(path, operation))
    }

    fn with_entity_context(self, entity: &str, operation: &str) -> GameResult<T> {
        self.with_context(|| entity_context(entity, operation))
    }
}
