.PHONY: performance demo

performance: 
	cargo run --bin performance_test_game

demo:
	cargo run --bin demo