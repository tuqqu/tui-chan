command_path = /usr/local/bin/tui-chan
bin_path = ./target/release/tui-chan

.PHONY: install
install:
	@sudo cp -f $(bin_path) $(command_path)
	@echo "Command 'tui-chan' has been installed."

.PHONY: uninstall
uninstall:
	@sudo rm -f $(command_path)
	@echo "Command 'tui-chan' has been uninstalled."