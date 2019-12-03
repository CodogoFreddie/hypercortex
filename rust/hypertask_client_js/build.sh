#!/bin/bash

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )";

pushd $DIR;

for PROFILE in dev profiling release ; do 
	wasm-pack build "--$PROFILE" --scope freddieridell --out-name "index_$PROFILE" ;
done

popd;
