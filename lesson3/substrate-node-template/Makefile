restart: build purge run
rerun: purge run

expand:
	BUILD_DUMMY_WASM_BINARY=1 cargo expand -p node-template > expanded-node.rs
    BUILD_DUMMY_WASM_BINARY=1 cargo expand -p node-template-runtime > expanded-runtime.rs

build-run: build run

run:
	-pkill node-template
	target/debug/node-template --dev -d target/substrate --execution=Native --ws-external -lruntime=debug

purge:
	target/debug/node-template purge-chain --dev -y -d target/substrate

build:
	WASM_BUILD_TYPE=release cargo build

check:
	SKIP_WASM_BUILD= cargo check
check-wasm:
	WASM_BUILD_TYPE=release cargo check

check-debug:
	RUSTFLAGS="-Z external-macro-backtrace" BUILD_DUMMY_WASM_BINARY= cargo +nightly check

check-dummy:
	BUILD_DUMMY_WASM_BINARY= cargo check

