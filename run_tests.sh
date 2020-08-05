#!/bin/sh
# firefox currently not working, safari gets focused :-(
wasm-pack test --chrome --headless -- --features "popup redirect"