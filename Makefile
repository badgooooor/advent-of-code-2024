run-test:
	cd src/$(day) \
		&& cargo build \
		&& ./target/debug/$(day) ./test-file.txt

run-real:
	cd src/$(day) \
		&& cargo build \
		&& ./target/debug/$(day) ./real-file.txt