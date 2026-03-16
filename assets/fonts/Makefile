FONTS := promptfont

.PHONY: all $(FONTS)

all: $(FONTS)

$(FONTS):
	@fontforge -lang=ff -c 'Open("$@/$@.sfd"); Generate("$@.ttf")' 