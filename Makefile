hl.so: hl.rs plugin.rs
	rustc --edition=2018 --crate-type dylib hl.rs -o hl.so

install: hl.so
	mkdir -p $$HOME/.weechat/plugins
	cp hl.so $$HOME/.weechat/plugins

libcstr.so: cstr.rs
	rustc --edition=2018 --crate-type proc-macro cstr.rs

example.so: example.o
	$(CC) -shared -fPIC -o example.so example.o

example.o: example.c weechat-plugin.h
	$(CC) -fPIC -Wall -c example.c

uninstall:
	rm -rf $$HOME/.weechat/plugins/hl.so

.PHONY: install uninstall

