#!/bin/bash

for PKG in hypertask_sync_js_daemon ; do 
	for PROFILE in debug ; do 
		echo "build npm $PKG $PROFILE"
		wasm-pack build "--$PROFILE" --scope freddieridell --out-name "index_$PROFILE" "rust/$PKG" || exit 1
	done
done
