#!/usr/bin/env bash

pushd $(mktemp -d 2>/dev/null || mktemp -d -t 'tmpdir') &>/dev/null; python3 -m http.server --bind 0.0.0.0 5000; popd
