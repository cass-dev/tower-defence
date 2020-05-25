set -ex

RUSTFLAGS='-C target-feature=+atomics,+bulk-memory' \
  cargo build --target wasm32-unknown-unknown -Z build-std=std,panic_abort \
  --features "wasm,gl" --release

wasm-bindgen target/wasm32-unknown-unknown/release/tower-defence.wasm \
  --out-dir pkg --no-modules

# worker.js crashes because it does not have `AudioContext` / `webkitAudioContext` in scope.
# This prevents it from crashing.
audio_context_workaround="const lAudioContext = (typeof AudioContext !== 'undefined' ? AudioContext : typeof webkitAudioContext !== 'undefined' ? webkitAudioContext : null)"

sed -i "s/const lAudioContext.\+\$/${audio_context_workaround}/" 'pkg/tower-defence.js'
