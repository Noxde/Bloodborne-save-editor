export $(cat ../.env)
cargo tarpaulin --target-dir target/tarpaulin/artifacts --skip-clean  --exclude-files src/main.rs "target/debug/**/*" "target/release/**/*" --ciserver github-ci --coveralls $COVERALLS_REPO_TOKEN
