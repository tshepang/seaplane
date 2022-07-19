SHORTSHA := `git rev-parse --short HEAD`
export TARGET := arch()
GON_CONFIG := justfile_directory() + '/target/sign_' + TARGET + '/config.hcl'
CURRENT_BRANCH := `git rev-parse --abbrev-ref HEAD`
TEST_RUNNER := 'cargo nextest run'
ARG_SEP := if "cargo nextest run" == TEST_RUNNER { '' } else { '--' }

@_default:
    just --list

# Install all needed components and tools
setup: (_cargo-install 'httpmock --features standalone') (_cargo-install 'cargo-lichking' 'cargo-audit' 'typos-cli' 'cargo-nextest')
    {{ if os() == 'macos' { 'just _install-gon' } else { '' } }}

# Run cargo-audit to scan for vulnerable crates
audit: (_cargo-install 'cargo-audit')
    cargo audit

# Run the CI suite for the SDK (only runs for your native os/arch!)
ci-sdk: test test-api (doc 'seaplane-sdk/rust') fmt-check (lint 'seaplane-sdk/rust') (lint 'seaplane-sdk/rust' '--features unstable') (lint 'seaplane-sdk/rust' '--no-default-features')

# Run the CI suite for the CLI (only runs for your native os/arch!)
ci-cli: (test 'seaplane-cli') (doc 'seaplane-cli') test-ui (test-api 'seaplane-cli') fmt-check (lint 'seaplane-cli') (lint 'seaplane-cli' '--features unstable') (lint 'seaplane-cli' '--no-default-features')

# Run the full CI suite (only runs for your native os/arch!)
ci: audit ci-cli ci-sdk spell-check test-doc

# Build documentation
doc RUST_CRATE='' $RUSTDOCFLAGS="-D warnings":
    cargo doc --manifest-path {{justfile_directory()}}/{{RUST_CRATE}}/Cargo.toml --no-deps --all-features --document-private-items

# Check if code formatter would make changes
fmt-check:
    cargo fmt --all -- --check

# Format code
fmt:
    cargo fmt --all

# Run code linting with warnings denied
lint RUST_CRATE='' RUST_FEATURES='':
    cargo clippy {{ RUST_FEATURES }} --manifest-path {{justfile_directory()}}/{{RUST_CRATE}}/Cargo.toml --all-targets -- -D warnings

# Run basic integration and unit tests
test RUST_CRATE='seaplane-sdk/rust' FEATURES='' $RUSTFLAGS='-D warnings':
    cargo test {{ FEATURES }} --manifest-path {{justfile_directory()}}/{{RUST_CRATE}}/Cargo.toml --no-run
    {{ TEST_RUNNER }} {{ FEATURES }} --manifest-path {{justfile_directory()}}/{{RUST_CRATE}}/Cargo.toml

# Run API tests using a mock HTTP server
test-api RUST_CRATE='seaplane-sdk/rust' $RUSTFLAGS='-D warnings':
    cargo test --no-run  --features api_tests --manifest-path {{justfile_directory()}}/{{RUST_CRATE}}/Cargo.toml
    {{ TEST_RUNNER }}  --features api_tests --manifest-path {{justfile_directory()}}/{{RUST_CRATE}}/Cargo.toml {{ ARG_SEP }} --test-threads=1
    cargo test --no-run  --features unstable,api_tests --manifest-path {{justfile_directory()}}/{{RUST_CRATE}}/Cargo.toml
    {{ TEST_RUNNER }}  --features unstable,api_tests --manifest-path {{justfile_directory()}}/{{RUST_CRATE}}/Cargo.toml {{ ARG_SEP }} --test-threads=1

# Run documentation tests
test-doc RUST_CRATE='seaplane-sdk/rust':
    cargo test --doc --manifest-path {{justfile_directory()}}/{{RUST_CRATE}}/Cargo.toml

# Run UI tests
test-ui $RUSTFLAGS='-D warnings':
    cargo test  --features ui_tests --manifest-path {{justfile_directory()}}/seaplane-cli/Cargo.toml --no-run
    {{ TEST_RUNNER }}  --features ui_tests --manifest-path {{justfile_directory()}}/seaplane-cli/Cargo.toml
    cargo test  --features unstable,semantic_ui_tests --manifest-path {{justfile_directory()}}/seaplane-cli/Cargo.toml --no-run
    {{ TEST_RUNNER }}  --features unstable,semantic_ui_tests --manifest-path {{justfile_directory()}}/seaplane-cli/Cargo.toml

# Update all third party licenses
update-licenses: (_cargo-install 'cargo-lichking')
    cargo lichking bundle --variant name-only > {{justfile_directory()}}/share/third_party_licenses.md

spell-check: (_cargo-install 'typos-cli')
    typos {{justfile_directory()}}

#
# Small Helpers
#

# List TODO items in current branch only
todos-in-branch:
    git diff --name-only {{ CURRENT_BRANCH }}  $(git merge-base {{ CURRENT_BRANCH }} main) | xargs rg -o 'TODO:.*$'

# List 'TODO:' items
todos:
    rg -o 'TODO:.*$' -g '!justfile'

# Get the short SHA commit for HEAD
git-shortsha:
    @echo {{ SHORTSHA }}

#
# Private/Internal Items
#

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
    CARGOTGTDIR={{ justfile_directory() + '/target/' }}
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

