#!/usr/bin/env bash

PROG=target/debug/termecho

function run() {
	echo "$@"
	cargo build && sudo ./"$PROG" "$@"
}

function usage() {
	cat << EOF
Build application with 'cargo build' and execute resulting binary using sudo.

USAGE:
    ./run_sudo.sh [FLAGS] [--] <prog_args>

FLAGS:
    -h    Prints help information
ARGS:
    <prog_args>...    Arguments passed through to application binary.
EOF
}

function main() {
	while getopts ":h" option; do
		case $option in
				h)
					usage
					exit;;
		esac
	done

	run "$@"
}

main "$@"
