.PHONY: all
all:
	# nothing to do, use build, strip and install targets instead

.PHONY: build
build:
	cargo update
	cargo build --release

.PHONY: strip
strip: build
	strip target/release/check_fs_object

.PHONY: create-bin
create-bin:
	mkdir -p -m 0755 $(DESTDIR)/usr/lib64/nagios/plugins

.PHONY: install
install: strip create-bin
	install -m 0755 target/release/check_fs_object $(DESTDIR)/usr/lib64/nagios/plugins

.PHONY: uninstall
uninstall:
	rm -f $(DESTDIR)/usr/lib64/nagios/plugins/check_fs_object

.PHONY: clean
clean:
	cargo clean

