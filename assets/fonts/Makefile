FONTS := promptfont

.PHONY: all $(FONTS)

all: $(FONTS)

$(FONTS):
	@fontforge -lang=ff -c 'Open("$@/$@.sfd"); Generate("$@.ttf")' || $(MAKE) error_msg

error_msg:
	@echo "Building fonts failed, make sure fontforge is installed"
	@echo "- sudo apt install fontforge"