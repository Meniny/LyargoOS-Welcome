-include config.mk

SUBDIRS = data

CARGO ?= cargo
CARGO_OPTS ?=

ifeq ($(findstring --release,$(CARGO_OPTS)),--release)
BIN ?= target/release/lyargoos-welcome
else
BIN ?= target/debug/lyargoos-welcome
endif

SCRIPTS = \
	system_update \
	virt_manager \
	common

.PHONY: build
build:
	@for dir in $(SUBDIRS); do \
		$(MAKE) -C $$dir ; \
	done
	$(CARGO) build $(CARGO_OPTS)

.PHONY: install
install: build
	install -Dm 755 $(BIN) $(DESTDIR)$(BINDIR)/lyargoos-welcome
	install -d $(DESTDIR)$(LIBEXECDIR)/lyargoos-welcome/scripts
	@for script in $(SCRIPTS); do \
		install -m755 scripts/$$script.sh $(DESTDIR)$(LIBEXECDIR)/lyargoos-welcome/scripts; \
	done
	@for dir in $(SUBDIRS); do \
		$(MAKE) install -C $$dir ; \
	done

.PHONY: uninstall
uninstall:
	rm -f $(DESTDIR)$(BINDIR)/lyargoos-welcome
	rm -rf $(DESTDIR)$(LIBEXECDIR)/lyargoos-welcome
	rm -rf $(DESTDIR)$(SHAREDIR)/lyargoos-welcome
	@for dir in $(SUBDIRS); do \
		$(MAKE) uninstall -C $$dir ; \
	done

.PHONY: run
run: build
	GTK_THEME=Adwaita-dark $(CARGO) run $(CARGO_OPTS)

.PHONY: clean
clean:
	@for dir in $(SUBDIRS); do \
		$(MAKE) clean -C $$dir ; \
	done
	rm -f src/config.rs
	rm -f config.mk
	@for script in $(SCRIPTS); do \
		rm -f scripts/$$script.sh; \
	done
	$(CARGO) clean
