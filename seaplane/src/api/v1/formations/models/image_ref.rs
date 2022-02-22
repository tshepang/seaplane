use std::{fmt::Display, str::FromStr};

use nom::{
    bytes::complete::{tag, take_till, take_till1, take_while1},
    character::complete::{alphanumeric1, anychar},
    combinator::{opt, rest},
    error::{context, Error, VerboseError},
    sequence::{preceded, separated_pair},
    AsChar, IResult,
};
use serde::{
    de::{Deserialize, Deserializer, Error as DeserializeError},
    ser::{Serialize, Serializer},
};
use thiserror::Error;

// This is taken from:
// https://github.com/distribution/distribution/blob/a4d9db5a884b70be0c96dd6a7a9dbef4f2798c51/reference/reference.go#L4
//
// TODO:
// For now, not all rules are checked. We do our best efforts to validate here. This will
// slowly be improved in the future.
//
// TODO:
// In the future, we also want to support image reference using:
// https://github.com/containers/image/blob/main/docs/containers-transports.5.md
//
// Grammar
//
// 	reference                       := name [ ":" tag ] [ "@" digest ]
//	name                            := [domain '/'] path-component ['/' path-component]*
//	domain                          := domain-component ['.' domain-component]* [':' port-number]
//	domain-component                := /([a-zA-Z0-9]|[a-zA-Z0-9][a-zA-Z0-9-]*[a-zA-Z0-9])/
//	port-number                     := /[0-9]+/
//	path-component                  := alpha-numeric [separator alpha-numeric]*
// 	alpha-numeric                   := /[a-z0-9]+/
//	separator                       := /[_.]|__|[-]*/
//
//	tag                             := /[\w][\w.-]{0,127}/
//
//	digest                          := digest-algorithm ":" digest-hex
//	digest-algorithm                := digest-algorithm-component [ digest-algorithm-separator digest-algorithm-component ]*
//	digest-algorithm-separator      := /[+.-_]/
//	digest-algorithm-component      := /[A-Za-z][A-Za-z0-9]*/
//	digest-hex                      := /[0-9a-fA-F]{32,}/ ; At least 128 bit digest value
//
//	identifier                      := /[a-f0-9]{64}/
//	short-identifier                := /[a-f0-9]{6,64}/

const NAME_TOTAL_LENGTH_MAX: usize = 255;
const TAG_TOTAL_LENGTH_MAX: usize = 127;

type Res<T, U> = IResult<T, U, VerboseError<T>>;

/// A container image reference
#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct ImageReference {
    pub domain: String,
    pub path: String,
    pub tag: Option<String>,
    pub digest: Option<String>,
}

impl ImageReference {
    pub fn domain(&self) -> &str {
        &self.domain
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    /// Add the given digest to this image reference, replacing any digest that currently exists
    pub fn with_digest<S: Into<String>>(self, digest: S) -> Self {
        Self {
            digest: Some(digest.into()),
            ..self
        }
    }
}

impl Display for ImageReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.domain, self.path)?;
        if let Some(tag) = self.tag.as_ref() {
            write!(f, ":{}", tag)?;
        }

        if let Some(digest) = self.digest.as_ref() {
            write!(f, "@{}", digest)?;
        }

        Ok(())
    }
}

impl FromStr for ImageReference {
    type Err = ImageReferenceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse(s)
    }
}

impl Serialize for ImageReference {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ImageReference {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = <&str>::deserialize(deserializer)?;
        s.parse().map_err(D::Error::custom)
    }
}

pub fn parse(input: &str) -> Result<ImageReference, ImageReferenceError> {
    let reference = match parse_reference(input) {
        Ok((residual, reference)) => {
            if !residual.is_empty() {
                return Err(ImageReferenceError::ErrReferenceInvalidFormat);
            }

            reference
        }
        Err(_) => {
            return Err(ImageReferenceError::ErrReferenceInvalidFormat);
        }
    };

    if reference.domain.len() + reference.path.len() > NAME_TOTAL_LENGTH_MAX {
        return Err(ImageReferenceError::ErrNameTooLong);
    }

    if let Some(tag) = reference.tag.as_ref() {
        if let Err(err) = validate_tags(tag) {
            return Err(err);
        }
    }

    if let Some(digest) = reference.digest.as_ref() {
        if let Err(err) = validate_digest(digest) {
            return Err(err);
        }
    }

    Ok(reference)
}

pub fn parse_reference(input: &str) -> Res<&str, ImageReference> {
    let (input, name) = parse_name(input)?;
    let (input, tag) = parse_tags(input)?;
    let (residual, digest) = parse_digest(input)?;
    let (_, (domain, path)) = split_domain(name)?;

    Ok((
        residual,
        ImageReference {
            domain: domain.to_string(),
            path: path.to_string(),
            tag: tag.map(|s| s.to_string()),
            digest: digest.map(|s| s.to_string()),
        },
    ))
}

fn parse_name(input: &str) -> Res<&str, &str> {
    context("parse_name", take_till1(|c| (c == ':' || c == '@')))(input)
}

// Split name into domain and path. Domain is the first component delimited by a
// '/'.  For example, name = domain/path1/path2/path3.
fn split_domain(input: &str) -> Res<&str, (&str, &str)> {
    context(
        "split_domain",
        separated_pair(take_till(|c| c == '/'), tag("/"), rest),
    )(input)
}

fn parse_tags(input: &str) -> Res<&str, Option<&str>> {
    context(
        "parse_tags",
        opt(preceded(tag(":"), take_till1(|c| c == '@'))),
    )(input)
}

fn validate_tags(input: &str) -> Result<(), ImageReferenceError> {
    let input: &str = match anychar::<_, Error<_>>(input) {
        Ok((rest, c)) => {
            if !c.is_alphanumeric() && c != '_' {
                return Err(ImageReferenceError::ErrTagInvalidFormat(input.to_string()));
            }

            rest
        }
        Err(_) => {
            return Err(ImageReferenceError::ErrTagInvalidFormat(input.to_string()));
        }
    };

    if input.len() > TAG_TOTAL_LENGTH_MAX {
        return Err(ImageReferenceError::ErrTagInvalidFormat(input.to_string()));
    }

    if !input
        .chars()
        .all(|c: char| c.is_alphanum() || c == '.' || c == '-' || c == '_')
    {
        return Err(ImageReferenceError::ErrTagInvalidFormat(input.to_string()));
    }

    Ok(())
}

fn parse_digest(input: &str) -> Res<&str, Option<&str>> {
    context("parse_tags", opt(preceded(tag("@"), rest)))(input)
}

fn validate_digest(input: &str) -> Result<(), ImageReferenceError> {
    let parse_hex = take_while1::<_, _, Error<_>>(|c: char| c.is_hex_digit());
    let (input, (_protocol, digest_hex)) =
        match separated_pair(alphanumeric1, tag(":"), parse_hex)(input) {
            Ok((rest, (protocol, digest_hex))) => (rest, (protocol, digest_hex)),
            Err(_) => {
                return Err(ImageReferenceError::ErrDigestInvalidFormat(
                    input.to_string(),
                ));
            }
        };
    if !input.is_empty() {
        return Err(ImageReferenceError::ErrDigestInvalidFormat(
            input.to_string(),
        ));
    }

    if !digest_hex.chars().all(|c| c.is_hex_digit()) {
        return Err(ImageReferenceError::ErrDigestInvalidFormat(
            input.to_string(),
        ));
    }

    Ok(())
}

#[derive(Debug, Error, PartialEq)]
pub enum ImageReferenceError {
    #[error("invalid reference format")]
    ErrReferenceInvalidFormat,

    #[error("invalid tag format: `{0}`")]
    ErrTagInvalidFormat(String),

    #[error("invalid digest format: `{0}`")]
    ErrDigestInvalidFormat(String),

    #[error(
        "repository name must not be more than {} characters",
        NAME_TOTAL_LENGTH_MAX
    )]
    ErrNameTooLong,

    #[error("repository name must be lower case")]
    ErrNameContainsUppercase,

    #[error("repository name must not be empty")]
    ErrNameEmpty,

    #[error("repository name must be canonical")]
    ErrNameNotCanonical,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            parse("docker.io/library/busybox@sha256:7cc4b5aefd1d0cadf8d97d4350462ba51c694ebca145b08d7d41b41acc8db5aa"), Ok(ImageReference{
            domain: "docker.io".to_string(),
            path:"library/busybox".to_string(),
            tag: None,
            digest: Some("sha256:7cc4b5aefd1d0cadf8d97d4350462ba51c694ebca145b08d7d41b41acc8db5aa".to_string())
        }));
    }

    #[test]
    fn test_display() {
        let input = "docker.io/library/busybox:latest@sha256:7cc4b5aefd1d0cadf8d97d4350462ba51c694ebca145b08d7d41b41acc8db5aa";
        let reference = parse(input).expect("failed to parse input");
        let output = reference.to_string();
        assert_eq!(input, output);
    }

    #[test]
    fn test_parse_name() {
        assert_eq!(
            parse_name("registry.seaplanet.io/seaplane/busybox:latest"),
            Ok((":latest", "registry.seaplanet.io/seaplane/busybox"))
        );
        assert_eq!(
            parse_name("registry.seaplanet.io/seaplane/busybox@sha256:XXX"),
            Ok(("@sha256:XXX", "registry.seaplanet.io/seaplane/busybox"))
        );
    }

    #[test]
    fn test_parse_tag() {
        // Parse tags only
        assert_eq!(parse_tags(":latest"), Ok(("", Some("latest"))));
        // Parse both tags and digest
        assert_eq!(
            parse_tags(":latest@sha256:XXX"),
            Ok(("@sha256:XXX", Some("latest")))
        );
        // Parse only digest
        assert_eq!(parse_tags("@sha256:XXX"), Ok(("@sha256:XXX", None)));
        // Parse no tags
        assert_eq!(parse_tags("registry.in"), Ok(("registry.in", None)));
    }

    #[test]
    fn test_parse_digest() {
        assert_eq!(parse_digest("@sha256:XXX"), Ok(("", Some("sha256:XXX"))));
        assert_eq!(parse_digest("registry.in"), Ok(("registry.in", None)));
        assert_eq!(
            parse_digest(":latest@sha256:XXX"),
            Ok((":latest@sha256:XXX", None))
        );
    }

    #[test]
    fn test_validate_tags() {
        assert!(validate_tags("v1.0").is_ok());
        assert!(validate_tags("v1-0").is_ok());
        assert!(validate_tags("1-0").is_ok());
        assert!(validate_tags("1.0").is_ok());

        assert!(validate_tags(".--..)()00").is_err());
        assert!(validate_tags(".V100)()00").is_err());
        assert!(validate_tags("]-g90)()00").is_err());
        assert!(validate_tags(&"x".repeat(TAG_TOTAL_LENGTH_MAX + 10)).is_err());
    }

    #[test]
    fn test_validate_digest() {
        assert!(validate_digest(
            "sha256:7cc4b5aefd1d0cadf8d97d4350462ba51c694ebca145b08d7d41b41acc8db5aa"
        )
        .is_ok());
        assert!(validate_digest(
            "sha256:7cc4b5aefd1d0cadf8d97d435046wwwwwww2ba51c694ebca145b08d7d41b41acc8db5aa"
        )
        .is_err());
        assert!(validate_digest(
            "sha256*7cc4b5aefd1d0cadf8d97d435046wwwwwww2ba51c694ebca145b08d7d41b41acc8db5aa"
        )
        .is_err());
        assert!(validate_digest("sha256:").is_err());
    }

    #[test]
    fn test_split_domain() {
        assert_eq!(
            split_domain("domain/path1/path2"),
            Ok(("", ("domain", "path1/path2")))
        );
    }
}
