.PHONY: performance demo

performance_concurrent_interactive_%:
	set RUSTFLAGS=-Awarnings&& cargo run --bin performance_test --quiet --release -- true true $*

performance_nonconcurrent_interactive_%:
	set RUSTFLAGS=-Awarnings&& cargo run --bin performance_test --quiet --release -- false true $*

performance_noninteractive_%:
	set RUSTFLAGS=-Awarnings&& cargo run --bin performance_test --quiet --release -- true false $*

demo:
	set RUSTFLAGS=-Awarnings&& cargo run --bin demo --release
