example.so: example.o
	$(CC) -shared -fPIC -o example.so example.o

example.o: example.c weechat-plugin.h
	$(CC) -fPIC -Wall -c example.c

install: example.so
	mkdir -p $$HOME/.weechat/plugins
	cp example.so $$HOME/.weechat/plugins

uninstall:
	rm -rf $$HOME/.weechat/plugins/example.so

.PHONY: install uninstall

