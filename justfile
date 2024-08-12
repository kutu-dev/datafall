# Show this info message
help:
  just --list

# Update and relock the inputs of the flake
update-flake:
  nix flake update --commit-lock-file

# See all the things that need to be done
todo:
  rg TODO:
