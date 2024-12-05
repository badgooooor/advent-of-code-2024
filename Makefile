run-test:
	cd src/$(day) \
		&& cargo run main.rs \
		&& ./target/debug/$(day) ./test-file.txt

run-real:
	cd src/$(day) \
		&& cargo run main.rs \
		&& ./target/debug/$(day) ./real-file.txt