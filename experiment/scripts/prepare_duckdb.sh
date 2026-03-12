#!/bin/bash
set -eu
set -o pipefail

workspace=$(realpath $(dirname $0)/../)
cd $workspace

os=$(uname -s)

case $os in
    "Linux")
        arch=$(uname -m)
        case $arch in
            "x86_64")
                wget https://github.com/duckdb/duckdb/releases/download/v0.10.2/duckdb_cli-linux-amd64.zip
                unzip -o duckdb_cli-linux-amd64.zip
                rm duckdb_cli-linux-amd64.zip
            ;;
            "aarch64")
                wget https://github.com/duckdb/duckdb/releases/download/v0.10.2/duckdb_cli-linux-aarch64.zip
                unzip -o duckdb_cli-linux-aarch64.zip
                rm duckdb_cli-linux-aarch64.zip
            ;;
            *)
                echo "ERROR: unsupported arch: $arch" 1>&2
                exit 1
            ;;
        esac
    ;;
    "Darwin")
        wget https://github.com/duckdb/duckdb/releases/download/v0.10.2/duckdb_cli-osx-universal.zip
        unzip -o duckdb_cli-osx-universal.zip
        rm duckdb_cli-osx-universal.zip
    ;;
    *)
        echo "ERROR: unsupported os: $os" 1>&2
        exit 1
    ;;
esac