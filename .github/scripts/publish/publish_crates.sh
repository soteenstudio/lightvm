if [[ "${{ github.event.inputs.version_override }}" == *"nightly"* ]]; then
  cargo publish --token ${{ secrets.CRATES_IO_TOKEN }} --allow-dirty --no-verify
else
  cargo publish --token ${{ secrets.CRATES_IO_TOKEN }} --allow-dirty
fi