# kal

Command Abstraction Layer primarily for `bot-any` packages.

## Core Concepts

- Command Fragment : `Select` subcommand or `Execute` current command with arguments supplied.
- Command Spec : The specification of command mainly used for documentation or registration.
- Command : A trait providing Command Spec and Command Fragments parser.
- Command Group : A macro for merging bunch of commands to manipulating them easily.
