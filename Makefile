# Nightmare Labyrinth - Makefile
# ゲーム開発用便利コマンド集

.PHONY: help dev build build-release web test clean title playing menu loading

# デフォルトターゲット：ヘルプ表示
help:
	@echo "Nightmare Labyrinth - 開発コマンド"
	@echo ""
	@echo "基本コマンド:"
	@echo "  dev          - 開発モードでゲーム起動"
	@echo "  build        - デバッグビルド"
	@echo "  build-release- リリースビルド"
	@echo "  web          - Web版サーバー起動"
	@echo "  test         - テスト実行"
	@echo "  clean        - ビルド成果物削除"
	@echo ""
	@echo "デバッグコマンド（画面直接遷移）:"
	@echo "  title        - タイトル画面で起動"
	@echo "  playing      - ゲームプレイ画面で起動"
	@echo "  menu         - メニュー画面で起動"
	@echo "  loading      - ローディング画面で起動"

# 開発モードでゲーム起動
dev:
	cargo run --features dev

# デバッグビルド
build:
	cargo build --features dev

# リリースビルド
build-release:
	cargo build --release

# Web版サーバー起動
web:
	trunk serve

# テスト実行
test:
	cargo test

# ビルド成果物削除
clean:
	cargo clean

# デバッグ用画面直接遷移コマンド
title:
	cargo run --features dev -- --debug-state title

playing:
	cargo run --features dev -- --debug-state playing

menu:
	cargo run --features dev -- --debug-state menu

loading:
	cargo run --features dev -- --debug-state loading
