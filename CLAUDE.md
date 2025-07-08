# CLAUDE.md

このファイルは、このリポジトリでのコード作業時にClaude Code (claude.ai/code) にガイダンスを提供します。

## コマンド

### 開発・ビルド
```bash
# 開発用起動（デバッグ機能付き）
cargo run --features dev

# Web向けビルド・起動
trunk serve

# 通常のビルド
cargo build

# リリースビルド
cargo build --release
```

### Web配信
- Trunkを使用してWebAssemblyビルドを行う
- `trunk serve`でローカルサーバーを起動し、ブラウザで開発可能

## アーキテクチャ

### 全体構造
Bevy 0.16.1を使用したゲームエンジンベースのプロジェクト。モジュール式の設計でプラグイン構造を採用。

### 主要モジュール
- **AppPlugin**: メインエントリーポイント（lib.rs）
- **game/**: ゲーム本体のロジック
  - **screens/**: ゲーム画面（title, loading, home）
  - **states**: ゲーム状態管理  
  - **assets**: アセット管理
  - **components/events/resources**: Bevyのコンポーネント設計
  - **turn_base**: ターン制システム
- **camera**: カメラ制御
- **mesh**: メッシュ管理
- **dev_tools**: 開発用ツール（devフィーチャー有効時のみ）

### ゲーム仕様
- **ジャンル**: ホラー風ローグライク
- **コンセプト**: 悪夢から抜け出せない少女を救出
- **システム**: 
  - ランダム生成マップ
  - 主人公は非戦闘
  - ターン制（主人公が動くまで時間停止）
  - 恐怖パラメータ（満腹度の代替）

### スプライト処理
- Asepriteファイルを使用（bevy_aseprite_ultra）
- ピクセルアート用に`ImagePlugin::default_nearest()`を設定
- スプライト間は1ピクセルの余白で区切り

### アセット管理
- JSON/RONファイルサポート（bevy_common_assets）
- 動的アセットローダー（bevy_asset_loader）
- 日本語フォント（NotoSansJP）を含む

## 開発フロー

### devフィーチャー
開発時は`--features dev`を使用：
- Bevyの動的リンクで高速コンパイル
- 開発ツール（bevy-inspector-egui）有効化
- `bevy_dev_tools`プラグイン有効化
