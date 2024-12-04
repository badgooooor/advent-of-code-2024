run-test:
	cd src/$(day)
	rustc src/$(day)/main.rs
	./main src/$(day)/test-file.txt

run-real:
	cd src/$(day)
	rustc src/$(day)/main.rs
	./main src/$(day)/real-file.txt