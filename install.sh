#!/bin/sh

set -e

devo_uri="https://raw.githubusercontent.com/tokiedokie/devo/master/devo"

deno_install="${DENO_INSTALL:-$HOME/.deno}"
bin_dir="$deno_install/bin"
devo_path="$bin_dir/devo"

if [ ! -d "$bin_dir" ]; then
  mkdir -p "$bin_dir"
fi

curl --fail --location --no-progress-meter --output "$devo_path" "$devo_uri"
chmod +x "$devo_path"

echo "Deno was installed successfully to $devo_path"

if command -v devo >/dev/null; then
  echo "Run 'devo --help' to get started"
else
  case $SHELL in
  /bin/zsh) shell_profile=".zshrc" ;;
  *) shell_profile=".bash_profile" ;;
  esac
  echo "Manually add the directory to your \$HOME/$shell_profile (or similar)"
  echo "  export DENO_INSTALL=\"$deno_install\""
  echo "  export PATH=\"\$DENO_INSTALL/bin:\$PATH\""
  echo "Run '$devo_path --help' to get started"
fi
