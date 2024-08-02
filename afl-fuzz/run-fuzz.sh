#!/bin/bash

# build w cargo afl build
cargo afl fuzz -o out -i in target/debug/afl-fuzz
