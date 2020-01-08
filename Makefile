
all: npmPkgs/js_sync_daemon

npmPkgs/js_sync_daemon: \
	rust/hypertask_client_js/pkg/index_dev.js \
	rust/hypertask_client_js/pkg/index_profiling.js \
	rust/hypertask_client_js/pkg/index_release.js 
	cp -r rust/hypertask_sync_js_daemon/pkg/ npmPkgs/js_sync_daemon

npmPkgs/js_sync_daemon: \
	rust/hypertask_sync_js_daemon/pkg/index_dev.js \
	rust/hypertask_sync_js_daemon/pkg/index_profiling.js \
	rust/hypertask_sync_js_daemon/pkg/index_release.js 
	cp -r rust/hypertask_client_js/pkg/ npmPkgs/js_client

rust/%/pkg/index_dev.js: rust/% rust/%/Cargo.toml
	wasm-pack build --dev --scope freddieridell --out-name index_dev $<
rust/%/pkg/index_profiling.js: rust/% rust/%/Cargo.toml
	wasm-pack build --profiling --scope freddieridell --out-name index_profiling $<
rust/%/pkg/index_release.js: rust/% rust/%/Cargo.toml
	wasm-pack build --release --scope freddieridell --out-name index_release $<

clean: 
	rm -rf npmPkgs rust/**/pkg
