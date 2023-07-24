run:
	@cargo run -- -d

build:
	@cargo build --release

buildrun:
	@cargo build --release
	@./target/release/$(NAME) -d