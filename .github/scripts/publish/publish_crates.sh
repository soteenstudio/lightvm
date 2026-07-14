if [[ "$INPUT_VERSION" == *"nightly"* ]]; then
  cargo publish --token "$TOKEN" --allow-dirty --no-verify
else
  cargo publish --token "$TOKEN" --allow-dirty
fi