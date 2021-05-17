%: src/bin/%.rs
	cargo run --bin $@

clean:
	rm *.exe*