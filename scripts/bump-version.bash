#!/bin/bash
set -euo pipefail
cd "${BASH_SOURCE[0]%/*}"/..

# How to use
# ./scripts/bump-version.bash
# or if you want to specify the version
# VERSION="0.2.1" ./scripts/bump-version.bash

# 環境変数でバージョンが指定されたとき、Cargo.tomlを上書き
if [ -n "${VERSION:-}" ]; then
  echo "Bump Cargo.toml: ${VERSION}"
  TARGET=("Cargo.toml")
  sed -i -e "/version/s/\"[0-9]\+\.[0-9]\+\.[0-9]\+.*\"/\"${VERSION}\"/" "${TARGET[@]}"
fi

# TARGETのversionを更新する
TARGET=("README.md" "src/lib.rs")
# Cargo.toml のバージョンを取得
VERSION=$(cargo read-manifest | jq -r .version)
echo "Version: ${VERSION:?"cann't read version."}, Target: ${TARGET[*]}"
# 正規表現でバージョンを置換
# "0.1.12-alpha" のようなバージョン表記を"${VERSION}"に置換する
sed -i -e "/advanced-pid/s/\"[0-9]\+\.[0-9]\+\.[0-9]\+.*\"/\"${VERSION}\"/" "${TARGET[@]}"
