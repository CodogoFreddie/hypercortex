#!/bin/sh

for PROFILE in dev profiling release ; do 
	wasm-pack build "--$PROFILE" --scope freddieridell --out-name "index_$PROFILE" ;
done
