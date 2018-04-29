# This script takes care of testing your crate

set -euxo pipefail

main() {
    case $TARGET in
        x86_64-unknown-linux-gnu)
            cargo check --target $TARGET
            ;;
        *)
            xargo check --target $TARGET
            ;;
    esac
}

main
