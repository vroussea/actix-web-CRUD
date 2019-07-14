#!/usr/bin/env bash
cargo install watch
cargo install diesel_cli --no-default-features --features sqlite
echo "DATABASE_URL=test.db" > .env
diesel migration run