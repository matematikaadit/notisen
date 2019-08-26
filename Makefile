hl.so: hl.rs plugin.rs
	rustc --crate-type dylib hl.rs -o hl.so

install: hl.so
	mkdir -p $$HOME/.weechat/plugins
	cp hl.so $$HOME/.weechat/plugins

example.so: example.o
	$(CC) -shared -fPIC -o example.so example.o

example.o: example.c weechat-plugin.h
	$(CC) -fPIC -Wall -c example.c

uninstall:
	rm -rf $$HOME/.weechat/plugins/example.so

.PHONY: install uninstall

