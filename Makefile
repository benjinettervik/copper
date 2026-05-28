.PHONY: performance demo

performance:
	set RUSTFLAGS=-Awarnings&& cargo run --bin performance_test_game --quiet

demo:
	set RUSTFLAGS=-Awarnings&& cargo run --bin demo