bin:
	cargo build --release
clean:
	cargo clean --release -p icrc32
clean_all:
	cargo clean
fmt:
	cargo fmt
	git status
lint:
	cargo clippy --release --fix --all
	git status
plint:
	cargo clippy --release --fix --all -- -W clippy::pedantic
	git status
test:
	cargo test --release
test_debug:
	cargo test --release -- --nocapture

xxx:	fmt lint test
