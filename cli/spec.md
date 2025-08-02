# Specification
# CLI Specification

This document outlines the specification for the CLI of the project.

## Commands
1. agentctl config add
2. agentctl config show
3. agentctl env show
4. agentctl env set <profile id>
5. agentctl describe <id>
6. agentctl deploy <id>
7. agentctl invoke <id> --input="<content>" 
8. agentctl list <id>

The `config` commands read and write a TOML file located at `~/.agentctl/config.toml`.


All commands have the following flags:
1. --output / -o json/toml/yaml/csv
