SELF := justfile_directory()
DIST := SELF / 'dist'
BIN_NAME := if os() == 'windows' { 'seaplane.exe' } else { 'seaplane' }
CLI_DIR := 'seaplane-cli'
CLI_MANIFEST := CLI_DIR / 'Cargo.toml'
SDK_RUST_DIR := 'seaplane-sdk/rust'
SDK_RUST_MANIFEST := SDK_RUST_DIR / 'Cargo.toml'
SDK_PYTHON_DIR := 'seaplane-sdk/python'

GON_CONFIG := SELF / 'target/sign_' / TARGET / 'config.hcl'
export TARGET := arch()

SHORTSHA := `git rev-parse --short HEAD`
CURRENT_BRANCH := `git rev-parse --abbrev-ref HEAD`

TEST_RUNNER := if env_var_or_default("CI", '0') == "1" { 'cargo test' } else { 'cargo nextest run' }
ARG_SEP := if TEST_RUNNER == "cargo nextest run" { '' } else { '--' }

@_default:
    just --list

# Install all needed components and tools
@setup: (_cargo-install 'httpmock --features standalone') (_cargo-install 'cargo-lichking' 'cargo-audit' 'typos-cli' 'cargo-nextest')
    {{ if os() == 'macos' { 'just _install-gon' } else { '' } }}

# Run cargo-audit to scan for vulnerable crates
audit: (_cargo-install 'cargo-audit')
    cargo audit

# Run the CI suite for the SDK (only runs for your native os/arch!)
ci-sdk: lint-sdk-rust doc test-rust test-rust-api

# Run the CI suite for the CLI (only runs for your native os/arch!)
ci-cli: lint-cli (doc CLI_MANIFEST) (test-rust CLI_MANIFEST) (test-rust-api CLI_MANIFEST) test-ui

# Run the full CI suite (only runs for your native os/arch!)
ci: audit ci-cli ci-sdk

# Build documentation
doc MANIFEST=SDK_RUST_MANIFEST $RUSTDOCFLAGS="-D warnings":
    cargo doc --manifest-path {{ MANIFEST }} --no-deps --all-features --document-private-items

# Check if code formatter would make changes
fmt-check: fmt-check-cli fmt-check-sdk-rust

# Check if code formatter would make changes to the CLI
fmt-check-cli:
    cargo fmt --manifest-path {{ CLI_DIR / 'Cargo.toml' }} --check

# Check if code formatter would make changes to the Rust SDK
fmt-check-sdk-rust:
    cargo fmt --manifest-path {{ SDK_RUST_DIR / 'Cargo.toml' }} --check

# Format the CLI code
fmt-cli:
    cargo fmt --manifest-path {{ CLI_MANIFEST }} --check

# Format the Rust SDK code
fmt-sdk-rust:
    cargo fmt --manifest-path {{ SDK_RUST_MANIFEST }} --check

# Format the code
fmt: fmt-sdk-rust fmt-cli

# Run all lint hecks against the Rust SDK
lint-cli: spell-check fmt-check-cli (_lint-rust CLI_MANIFEST '--no-default-features')

# Run all lint hecks against the Rust SDK
lint-sdk-rust: spell-check fmt-check-sdk-rust _lint-rust (_lint-rust SDK_RUST_MANIFEST '--features unstable') (_lint-rust SDK_RUST_MANIFEST '--no-default-features')

# Run all checks and lints
lint: lint-sdk-rust lint-cli

_lint-rust MANIFEST=SDK_RUST_MANIFEST RUST_FEATURES='':
    cargo clippy {{ RUST_FEATURES }} --manifest-path {{ MANIFEST }} --all-targets -- -D warnings

# Run basic integration and unit tests for Rust
test-rust MANIFEST=SDK_RUST_MANIFEST FEATURES='' $RUSTFLAGS='-D warnings':
    {{ TEST_RUNNER }} {{ FEATURES }} --manifest-path {{ MANIFEST }}

# Run API tests using a mock HTTP server
test-rust-api MANIFEST=SDK_RUST_MANIFEST $RUSTFLAGS='-D warnings':
    {{ TEST_RUNNER }}  --features api_tests --manifest-path {{ MANIFEST }} {{ ARG_SEP }} --test-threads=1
    {{ TEST_RUNNER }}  --features unstable,api_tests --manifest-path {{ MANIFEST }} {{ ARG_SEP }} --test-threads=1

# Run documentation tests
test-doc MANIFEST=SDK_RUST_MANIFEST:
    cargo test --doc --manifest-path {{ MANIFEST }}

# Run UI tests
test-ui $RUSTFLAGS='-D warnings':
    {{ TEST_RUNNER }}  --features ui_tests --manifest-path {{ CLI_MANIFEST }}
    {{ TEST_RUNNER }}  --features unstable,semantic_ui_tests --manifest-path {{ CLI_MANIFEST }}

# Update all third party licenses
update-licenses: (_cargo-install 'cargo-lichking')
    cargo lichking bundle --variant name-only > {{ CLI_DIR / 'share/third_party_licenses.md' }}

spell-check:
    #!/usr/bin/env bash
    set -euo pipefail
    if ! command -v just 2>&1 >/dev/null ; then
      cargo install typos-cli --force
    fi
    typos

#
# Small Helpers
#

# List TODO items in current branch only
todos-in-branch:
    git diff --name-only {{ CURRENT_BRANCH }}  $(git merge-base {{ CURRENT_BRANCH }} main) | xargs rg -o 'TODO:.*$'

# List 'TODO:' items
todos:
    rg -o 'TODO:.*$' -g '!justfile'

# Create a nightly CLI release package (latest commit)
package-nightly:
    just _package-build "cli-$(just _git-shortsha-cli)"

# Create a CLI release package (latest 'cli-v*' tag)
package-release: _package-build
    just _package-build "$(git tag --list | grep cli-v | head -n1)"

#
# Private/Internal Items
#

# Get the short SHA commit for HEAD
_git-shortsha:
    @echo {{ SHORTSHA }}

# Get the latest short SHA commit for the just CLI directory
_git-shortsha-cli:
    @git --no-pager log -n1 --pretty=format:%h $(dirname {{ CLI_DIR }})

_package-build $TAG='':
    #!/usr/bin/env bash
    set -euo pipefail
    BUILDDIR={{ justfile_directory() / 'target/release/' }}
    DISTDIR=dist/
    cargo build --release --manifest-path {{ CLI_MANIFEST }}
    mkdir -p ${DISTDIR}/{bin,share/doc/seaplane/}
    cp ${BUILDDIR}/{{BIN_NAME}} ${DISTDIR}/bin/
    cp seaplane-cli/share/third_party_licenses.md ${DISTDIR}/share/doc/seaplane/
    cp LICENSE ${DISTDIR}/share/doc/seaplane/
    cd ${DISTDIR}
    if [[ "{{os()}}" == "windows" ]]; then
      zip ../seaplane-${TAG}-$(uname -m).zip ./*
    else
      tar czf ../seaplane-${TAG}-$(uname -m).tar.gz ./*
    fi

_cargo-install +TOOLS:
    cargo install {{ TOOLS }}

_install-gon:
    #!/usr/bin/env bash
    echo {{ if os() != 'macos' { error('only macOS') } else { '' } }}
    if ! which gon; then brew install mitchellh/gon/gon; fi

# Sign and notarize a release for macOS
_sign $AC_PASSWORD SIGNER='${USER}': _install-gon
    #!/usr/bin/env bash
    echo {{ if os() != 'macos' { error('only macOS needs to sign') } else { '' } }}
    CARGOTGTDIR={{ justfile_directory() / 'target' }}
    SIGNDIR=${CARGOTGTDIR}/sign_${TARGET}/
    ARTIFACTSDIR=${CARGOTGTDIR}/${TARGET}-apple-darwin/release/
    echo Cleaning previous runs
    rm -rf $SIGNDIR
    cargo clean
    mkdir -p $SIGNDIR
    echo Generating Config...
    echo 'source = ["./seaplane"]' >> {{GON_CONFIG}}
    echo 'bundle_id = "io.Seaplane.seaplane"' >> {{GON_CONFIG}}
    echo 'apple_id {' >> {{GON_CONFIG}}
    echo "  username = \"${USER}@seaplane.io\"" >> {{GON_CONFIG}}
    echo "  password = \"$AC_PASSWORD\"" >> {{GON_CONFIG}}
    echo '}' >> {{GON_CONFIG}}
    echo 'sign {' >> {{GON_CONFIG}}
    echo '  application_identity = "663170B344CE42EF1F583807B756239878A92FC8"' >> {{GON_CONFIG}}
    echo '}' >> {{GON_CONFIG}}
    echo 'zip {' >> {{GON_CONFIG}}
    echo "  output_path = \"seaplane-{{SHORTSHA}}-${TARGET}-apple-darwin.zip\"" >> {{GON_CONFIG}}
    echo '}' >> {{GON_CONFIG}}
    echo Compiling ${TARGET}...
    cargo --quiet build --release --manifest-path {{justfile_directory() + '/seaplane-cli/Cargo.toml' }} --target ${TARGET}-apple-darwin
    echo Copying binaries...
    cp ${ARTIFACTSDIR}/seaplane ${SIGNDIR}
    echo Signing...
    cd ${SIGNDIR}; gon config.hcl
    echo Done!
    echo Saving Artifacts to ${CARGOTGTDIR}
    cp ${SIGNDIR}/seaplane-{{SHORTSHA}}-${TARGET}-apple-darwin.zip ${CARGOTGTDIR}

