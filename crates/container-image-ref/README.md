# Container Image Reference

![Rust Version][rustc-image]
[![crates.io][crate-image]][crate-link]
[![Dependency Status][deps-image]][deps-link]

A library for validating and using container image references

<!-- vim-markdown-toc GFM -->

* [Grammar](#grammar)
* [License](#license)

<!-- vim-markdown-toc -->

## Grammar

```
reference                       := name [ ":" tag ] [ "@" digest ]
name                            := [domain '/'] path-component ['/' path-component]*
domain                          := domain-component ['.' domain-component]* [':' port-number]
domain-component                := /([a-zA-Z0-9]|[a-zA-Z0-9][a-zA-Z0-9-]*[a-zA-Z0-9])/
port-number                     := /[0-9]+/
path-component                  := alpha-numeric [separator alpha-numeric]*
alpha-numeric                   := /[a-z0-9]+/
separator                       := /[_.]|__|[-]*/

tag                             := /[\w][\w.-]{0,127}/

digest                          := digest-algorithm ":" digest-hex
digest-algorithm                := digest-algorithm-component [ digest-algorithm-separator
digest-algorithm-component ]* 	digest-algorithm-separator      := /[+.-_]/
digest-algorithm-component      := /[A-Za-z][A-Za-z0-9]*/
digest-hex                      := /[0-9a-fA-F]{32,}/ ; At least 128 bit digest value

identifier                      := /[a-f0-9]{64}/
short-identifier                := /[a-f0-9]{6,64}/
```

## License

Licensed under the Apache License, Version 2.0, [LICENSE]. Copyright 2023 Seaplane IO, Inc.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/container-image-ref.svg
[crate-link]: https://crates.io/crates/container-image-ref
[deps-image]: https://deps.rs/repo/github/seaplane-io/seaplane/status.svg
[deps-link]: https://deps.rs/crate/container-image-ref
[rustc-image]: https://img.shields.io/badge/rustc-1.60+-blue.svg

[//]: # (Links)

[LICENSE]: https://github.com/seaplane-io/seaplane/blob/main/LICENSE
