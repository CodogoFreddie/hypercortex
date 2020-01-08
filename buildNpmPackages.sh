#!/bin/bash

for PKG in hypertask_client_js hypertask_sync_js_daemon ; do 
	for PROFILE in dev profiling release ; do 
		echo "build npm $PKG $PROFILE"
		wasm-pack build "--$PROFILE" --scope freddieridell --out-name "index_$PROFILE" "rust/$PKG" 2> /dev/null
	done
done
