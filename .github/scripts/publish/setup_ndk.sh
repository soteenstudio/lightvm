#!/bin/bash

if [ -z "$ANDROID_NDK_HOME" ]; then
  export ANDROID_NDK_HOME=$ANDROID_NDK_LATEST_HOME
fi

echo "Use NDK from: $ANDROID_NDK_HOME"
