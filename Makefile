release:
	cargo build --release --offline

test:
	cargo test --offline --all-features

clippy:
	cargo clippy --offline

clean:
	rm -r target; rm Cargo.lock
