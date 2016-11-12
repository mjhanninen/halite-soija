#!/bin/sh
test -d dist || mkdir dist
test -e dist/pod.zip && rm dist/pod.zip
git ls-files | xargs zip dist/pod.zip
