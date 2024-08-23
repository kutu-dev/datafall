# Default recipe of the justfile
default: run-debug

# Run the program in debug mode
run-debug:
  cargo run

# Fix the formatting and fixable linting warnings
check:
  cargo fix
  addlicense -l mpl .

# Show this info message
help:
  just --list

# Update and relock the inputs of the flake
update-flake:
  nix flake update --commit-lock-file

# See all the things that need to be done
todo:
  rg TODO:
