SHORTSHA := `git rev-parse --short HEAD`
export TARGET := arch()
GON_CONFIG := justfile_directory() + '/target/sign_' + TARGET + '/config.hcl'
CURRENT_BRANCH := `git rev-parse --abbrev-ref HEAD`
TEST_RUNNER := 'cargo nextest run'

@_default:
    just --list

_cargo-install +TOOLS:
    cargo install {{ TOOLS }}

_install-gon:
    #!/usr/bin/env bash
    echo {{ if os() != 'macos' { error('only macOS') } else { '' } }}
    if ! which gon; then brew install mitchellh/gon/gon; fi

# Install all needed components and tools
setup: (_cargo-install 'httpmock --features standalone') (_cargo-install 'cargo-lichking' 'cargo-audit' 'typos-cli' 'cargo-nextest')
    {{ if os() == 'macos' { 'just _install-gon' } else { '' } }}

todos-in-branch:
    git diff --name-only {{ CURRENT_BRANCH }}  $(git merge-base {{ CURRENT_BRANCH }} main) | xargs rg -o 'TODO:.*$'

# Get the short SHA commit for HEAD
git-shortsha:
    @echo {{ SHORTSHA }}

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

_test CRATE='seaplane' FEATURES='' $RUSTFLAGS='-D warnings':
	cargo test {{ FEATURES }} --manifest-path {{justfile_directory()}}/{{CRATE}}/Cargo.toml  --no-run
	{{ TEST_RUNNER }} {{ FEATURES }} --manifest-path {{justfile_directory()}}/{{CRATE}}/Cargo.toml

# Run cargo-audit to scan for vulnerable crates
audit: (_cargo-install 'cargo-audit')
	cargo audit

# Ensure documentation builds
test-doc-builds CRATE='seaplane' $RUSTDOCFLAGS="-D warnings":
    cargo doc --manifest-path {{justfile_directory()}}/{{CRATE}}/Cargo.toml --no-deps --all-features --document-private-items

test-doc CRATE='seaplane':
	cargo test  --doc --manifest-path {{justfile_directory()}}/{{CRATE}}/Cargo.toml

# Run UI tests
test-ui $RUSTFLAGS='-D warnings':
	cargo test  --features ui_tests --manifest-path {{justfile_directory()}}/seaplane-cli/Cargo.toml --no-run
	{{ TEST_RUNNER }}  --features ui_tests --manifest-path {{justfile_directory()}}/seaplane-cli/Cargo.toml
	cargo test  --features unstable,semantic_ui_tests --manifest-path {{justfile_directory()}}/seaplane-cli/Cargo.toml --no-run
	{{ TEST_RUNNER }}  --features unstable,semantic_ui_tests --manifest-path {{justfile_directory()}}/seaplane-cli/Cargo.toml

# Check if rustfmt would make changes
rustfmt-check CRATE='seaplane':
	cargo fmt  --all --manifest-path {{justfile_directory()}}/{{CRATE}}/Cargo.toml -- --check

# Format code using rustfmt
rustfmt CRATE='seaplane':
	cargo fmt  --all --manifest-path {{justfile_directory()}}/{{CRATE}}/Cargo.toml

# Run clippy and with warnings denied
clippy CRATE='' FEATURES='':
	cargo clippy  {{ FEATURES }} --manifest-path {{justfile_directory()}}/{{CRATE}}/Cargo.toml --all-targets -- -D warnings

# Run API tests using a mock HTTP server
test-api CRATE='seaplane' $RUSTFLAGS='-D warnings':
	cargo test --no-run  --features api_tests --manifest-path {{justfile_directory()}}/{{CRATE}}/Cargo.toml
	{{ TEST_RUNNER }}  --features api_tests --manifest-path {{justfile_directory()}}/{{CRATE}}/Cargo.toml --test-threads=1
	cargo test --no-run  --features unstable,api_tests --manifest-path {{justfile_directory()}}/{{CRATE}}/Cargo.toml
	{{ TEST_RUNNER }}  --features unstable,api_tests --manifest-path {{justfile_directory()}}/{{CRATE}}/Cargo.toml --test-threads=1

# Run cli-API tests using a mock HTTP server
test-cli-api:
	{{ TEST_RUNNER }} --features api_tests --test-threads=1

test-cli-locks:
	{{ TEST_RUNNER }} locks_list --features api_tests --no-capture -- --exact --test-threads=1

# Update all third party licenses
update-licenses: (_cargo-install 'cargo-lichking')
    cargo lichking bundle --variant name-only > {{justfile_directory()}}/share/third_party_licenses.md

spell-check: (_cargo-install 'typos-cli')
    typos {{justfile_directory()}}

# Run the CI suite for the SDK (only runs for your native os/arch!)
sdk-ci: _test test-api test-doc-builds rustfmt-check (clippy 'seaplane') (clippy 'seaplane' '--features unstable')

# Run the CI suite for the CLI (only runs for your native os/arch!)
cli-ci: (_test 'seaplane-cli') (test-doc-builds 'seaplane-cli') test-ui (test-api 'seaplane-cli') (rustfmt-check 'seaplane-cli') (clippy 'seaplane-cli') (clippy 'seaplane-cli' '--features unstable')

# Run the full CI suite (only runs for your native os/arch!)
ci: audit cli-ci sdk-ci spell-check test-doc
