declare -a targets=("aarch64-apple-darwin" "x86_64-unknown-linux-gnu" "aarch64-unknown-linux-gnu" "x86_64-apple-darwin" "x86_64-pc-windows-gnu")

export CC_x86_64_unknown_linux_gnu=x86_64-unknown-linux-gnu-gcc
export CXX_x86_64_unknown_linux_gnu=x86_64-unknown-linux-gnu-g++
export AR_x86_64_unknown_linux_gnu=x86_64-unknown-linux-gnu-ar
export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=x86_64-unknown-linux-gnu-gcc
export CC_aarch64_unknown_linux_gnu=aarch64-unknown-linux-gnu-gcc
export CXX_aarch64_unknown_linux_gnu=aarch64-unknown-linux-gnu-g++
export AR_aarch64_unknown_linux_gnu=aarch64-unknown-linux-gnu-ar
export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-unknown-linux-gnu-gcc
# shellcheck disable=SC1073
for target in "${targets[@]}"
do
  echo "Building for platform ${target}..."
  cargo build --target=${target} --release
  echo ""
done
