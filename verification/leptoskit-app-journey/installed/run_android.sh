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
receipt_limit=16384

"$adb" -s "$device" install \
  "$RUNNER_TEMP/wallet-rpc-app/android/app/build/outputs/apk/debug/app-debug.apk"
server=

valid_receipt() {
  local receipt=$1
  local bytes

  if ! bytes=$(wc -c <"$receipt"); then
    return 1
  fi

  if [ "$bytes" -gt "$receipt_limit" ]; then
    return 1
  fi

  awk '
      BEGIN {
        actions = "(create|load|restore|save|backup|review|approve|reject|cancel|disconnect|android)"
        errors = "(ok|Unavailable|PermissionDenied|InvalidArgument|Busy|TooLarge|Cancelled|Native|"
        errors = errors "InvalidCall|Exhausted)"
        category = "(bridge|mobile|entropy|storage|worker|json)"
        os_code = "(None|Some\\(-?[[:digit:]]+\\))"
        io = "(None|Some\\(\\([[:alpha:]]+, " os_code "\\)\\))"
        published = "(None|Some\\((true|false)\\))"
        stages = "(CheckPlatform|ValidateDestination|GenerateStagingName|CreateParentDirectory|"
        stages = stages "CreateStagingDirectory|OpenStagingFile|WriteStagingFile|SyncStagingFile|"
        stages = stages "PublishDestination|Cleanup)"
        stage = "(None|Some\\(" stages "\\))"
      }
      /^press node=-?[[:digit:]]+ handled=[01]$/ { next }
      $0 ~ "^action " actions "$" { next }
      /^guest result (ok|bridge-native|bridge-cancelled|other)$/ { next }
      /^save (received|admitted)$/ { next }
      $0 ~ "^save (completed|rejected) " errors "$" { next }
      /^(lease ended|trace overflow|receipt overflow|invalid-receipt)$/ { next }
      $0 ~ "^failure " category " io=" io " published=" published " stage=" stage "$" { next }
      { exit 1 }
    ' "$receipt"
}

retain_receipt() {
  local receipt="$RUNNER_TEMP/installed-wallet-run/android-wallet-diagnostics.txt"

  if ! mkdir -p "$(dirname "$receipt")"; then
    return 1
  fi

  if ! timeout -k 5s 20s "$adb" -s "$device" exec-out run-as "$package" \
    cat files/journey-diagnostics.txt >"$receipt" 2>/dev/null; then
    printf '%s\n' 'receipt unavailable in Android app' >"$receipt"

    return
  fi

  if valid_receipt "$receipt"; then
    return 0
  fi

  rm -f "$receipt" || echo 'Could not remove invalid Android receipt' >&2
  echo 'Android wallet diagnostic receipt failed validation' >&2

  return 1
}

retain_logs() {
  if timeout -k 5s 20s "$adb" -s "$device" logcat -b main -b crash -d -t 1000 \
    -v threadtime '*:E' >"$RUNNER_TEMP/installed-android.log" 2>&1; then
    return 0
  fi

  echo 'Could not retain bounded Android error and crash logs' >&2

  return 1
}

stop_driver() {
  if [ -z "$server" ]; then
    return 0
  fi

  # The child may already have exited; a signalled child also makes wait nonzero.
  kill "$server" 2>/dev/null || true
  wait "$server" 2>/dev/null || true
}

# A log failure replaces success; receipt and uninstall failures take precedence.
cleanup() {
  local status=$?

  if ! retain_receipt; then
    status=1
  fi

  if ! retain_logs && [ "$status" -eq 0 ]; then
    status=1
  fi

  stop_driver

  if ! "$adb" -s "$device" uninstall "$package"; then
    status=1
  fi

  exit "$status"
}
trap cleanup EXIT

driver_port=14731
command -v lsof >/dev/null

if lsof -nP -iTCP:"$driver_port" -sTCP:LISTEN; then
  echo 'Dedicated Android UI driver port is occupied' >&2
  exit 1
fi
(
  cd framework/setup/native-ui
  exec env -u APPIUM_HOME node node_modules/appium/build/lib/main.js \
    --address 127.0.0.1 --port "$driver_port" --use-drivers uiautomator2 --log-no-colors
) >"$RUNNER_TEMP/installed-appium.log" 2>&1 &
server=$!
deadline=$((SECONDS + 60))

until curl --silent --show-error --fail --user-agent solana-kit-verification \
  "http://127.0.0.1:$driver_port/status" >"$RUNNER_TEMP/installed-driver-status.json"; do
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
