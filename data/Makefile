-include ../config.mk

BLP_FILES = \
	window.blp \
	stack/main.blp \
	stack/log.blp
UI_FILES = $(BLP_FILES:.blp=.ui)

resources/ui/%.ui: resources/ui/%.blp
	blueprint-compiler compile --output $@ $<

resources/resources.gresource: resources/resources.gresource.xml $(addprefix resources/ui/, $(UI_FILES))
	glib-compile-resources --sourcedir resources \
		resources/resources.gresource.xml \
		--target resources/resources.gresource

.PHONY: install
install:
	install -Dm 644 org.lyargoos.Welcome.desktop \
		$(DESTDIR)$(SHAREDIR)/applications/org.lyargoos.Welcome.desktop
	install -Dm 644 org.lyargoos.Welcome.metainfo.xml \
		$(DESTDIR)$(SHAREDIR)/metainfo/org.lyargoos.Welcome.metainfo.xml
	install -Dm 644 resources/resources.gresource \
		$(DESTDIR)$(SHAREDIR)/lyargoos-welcome/resources.gresource
	install -Dm 644 icons/org.lyargoos.Welcome.png \
		$(DESTDIR)$(SHAREDIR)/icons/hicolor/256x256/apps/org.lyargoos.Welcome.png

.PHONY: uninstall
uninstall:
	rm -f $(DESTDIR)$(SHAREDIR)/applications/org.lyargoos.Welcome.desktop
	rm -f $(DESTDIR)$(SHAREDIR)/metainfo/org.lyargoos.Welcome.metainfo.xml
	rm -f $(DESTDIR)$(SHAREDIR)/lyargoos-welcome/resources.gresource
	rm -f $(DESTDIR)$(SHAREDIR)/icons/hicolor/256x256/apps/org.lyargoos.Welcome.png

.PHONY: clean
clean:
	rm -f $(addprefix resources/ui/, $(UI_FILES))
	rm -rf data/resources/resources.gresource
