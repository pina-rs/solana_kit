#!/usr/bin/env bash
# One CI-owned shell retains its installed app, Appium PID and cleanup trap.
set -euo pipefail

controller="$(jq -sre '[.[] | select(.reason == "compiler-artifact" and
  .target.name == "solana_kit_installed_wallet_journey") |
  .executable | select(. != null)] |
  if length == 1 then .[0] else error("expected one installed controller") end' \
  "$RUNNER_TEMP/installed-controller.jsonl")"
adb="$ANDROID_HOME/platform-tools/adb"
device=emulator-5554
package=com.leptoskit.solanawalletapp

"$adb" -s "$device" install \
  "$RUNNER_TEMP/wallet-rpc-app/android/app/build/outputs/apk/debug/app-debug.apk"
server=

cleanup() {
  status=$?

  if [ -n "$server" ]; then
    kill "$server" 2>/dev/null || true
    wait "$server" 2>/dev/null || true
  fi
  "$adb" -s "$device" uninstall "$package" || status=1
  exit "$status"
}
trap cleanup EXIT

driver_port=14731
command -v lsof > /dev/null

if lsof -nP -iTCP:"$driver_port" -sTCP:LISTEN; then
  echo 'Dedicated Android UI driver port is occupied' >&2
  exit 1
fi
(
  cd framework/setup/native-ui
  exec env -u APPIUM_HOME node node_modules/appium/build/lib/main.js \
    --address 127.0.0.1 --port "$driver_port" --use-drivers uiautomator2 --log-no-colors
) > "$RUNNER_TEMP/installed-appium.log" 2>&1 &
server=$!
deadline=$((SECONDS + 60))

until curl --silent --show-error --fail --user-agent solana-kit-verification \
  "http://127.0.0.1:$driver_port/status" > "$RUNNER_TEMP/installed-driver-status.json"; do
  kill -0 "$server"

  if [ "$SECONDS" -ge "$deadline" ]; then
    echo 'Owned Android UI driver startup deadline' >&2
    exit 1
  fi
  sleep 1
done
kill -0 "$server"

env -u HTTP_PROXY -u HTTPS_PROXY -u ALL_PROXY \
  -u http_proxy -u https_proxy -u all_proxy \
  "$controller" android "http://127.0.0.1:$driver_port" "$package" \
  "$RUNNER_TEMP/android-config" "$RUNNER_TEMP/installed-wallet-run" \
  "$RUNNER_TEMP/validator/solana-release/bin/solana-test-validator" "$device" "$adb"
