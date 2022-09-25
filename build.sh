cargo install cargo-vcpkg
cargo vcpkg build
export VCPKG_ROOT="${CARGO_TARGET_DIR:-$(pwd)/target}/vcpkg"
export TECTONIC_DEP_BACKEND=vcpkg
export RUSTFLAGS='-Ctarget-feature=+crt-static'
pipenv run maturin build --release