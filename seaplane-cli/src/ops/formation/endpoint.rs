use std::{result::Result as StdResult, str::FromStr};

use seaplane::api::v1::formations::{
    EndpointKey as EndpointKeyModel, EndpointValue as EndpointValueModel,
};

use crate::cli::validator::validate_flight_name;

#[derive(Debug, PartialEq, Clone)]
pub struct Endpoint {
    src: EndpointSrc,
    dst: EndpointDst,
}

impl Endpoint {
    pub fn key(&self) -> EndpointKeyModel {
        match &self.src {
            EndpointSrc::Http(p) => EndpointKeyModel::Http { path: p.to_owned() },
            EndpointSrc::Tcp(p) => EndpointKeyModel::Tcp { port: *p },
            EndpointSrc::Udp(p) => EndpointKeyModel::Udp { port: *p },
        }
    }
    pub fn value(&self) -> EndpointValueModel {
        EndpointValueModel {
            flight_name: self.dst.flight.clone(),
            port: self.dst.port,
        }
    }
}

impl FromStr for Endpoint {
    type Err = String;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        let mut parts = s.split('=');
        Ok(Self {
            src: parts
                .next()
                .ok_or_else(|| String::from("invalid endpoint source"))?
                .parse()?,
            dst: parts
                .next()
                .ok_or_else(|| String::from("invalid endpoint destination"))?
                .parse()?,
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum EndpointSrc {
    Http(String),
    Tcp(u16),
    Udp(u16),
}

impl FromStr for EndpointSrc {
    type Err = String;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        let mut parts = s.split(':');
        let proto = parts.next().ok_or_else(|| String::from("http"))?;
        let ep = match &*proto.to_ascii_lowercase() {
            "http" | "https" => EndpointSrc::Http(if let Some(route) = parts.next() {
                if route.is_empty() {
                    return Err(String::from("missing http route"));
                } else if !route.starts_with('/') {
                    return Err(String::from("route must start with a leady slash ('/')"));
                }
                route.to_string()
            } else {
                return Err(String::from("missing http route"));
            }),
            "tcp" => EndpointSrc::Tcp(
                parts
                    .next()
                    .ok_or_else(|| String::from("missing network port number"))?
                    .parse::<u16>()
                    .map_err(|_| String::from("invalid network port number"))?,
            ),
            "udp" => EndpointSrc::Udp(
                parts
                    .next()
                    .ok_or_else(|| String::from("missing network port number"))?
                    .parse::<u16>()
                    .map_err(|_| String::from("invalid network port number"))?,
            ),
            proto if proto.starts_with('/') => EndpointSrc::Http(proto.to_string()),
            _ => {
                return Err(format!(
                    "invalid protocol '{}' (valid options: http, https, tcp, udp)",
                    proto
                ))
            }
        };
        Ok(ep)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EndpointDst {
    flight: String,
    port: u16,
}

impl FromStr for EndpointDst {
    type Err = String;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        let mut parts = s.split(':');
        let flight = parts
            .next()
            .ok_or_else(|| ("missing destinaion flight").to_string())?;
        validate_flight_name(flight)?;
        let port = parts
            .next()
            .ok_or_else(|| ("missing destination port number").to_string())?
            .parse::<u16>()
            .map_err(|_| ("invalid port number").to_string())?;

        Ok(Self {
            flight: flight.to_string(),
            port,
        })
    }
}

#[cfg(test)]
mod endpoint_test {
    use super::*;

    #[test]
    fn endpoint_valid_http() {
        let ep: Endpoint = "http:/foo/bar=baz:1234".parse().unwrap();
        assert_eq!(
            ep,
            Endpoint {
                src: EndpointSrc::Http("/foo/bar".into()),
                dst: EndpointDst {
                    flight: "baz".into(),
                    port: 1234
                }
            }
        )
    }

    #[test]
    fn endpoint_valid_https() {
        let ep: Endpoint = "https:/foo/bar=baz:1234".parse().unwrap();
        assert_eq!(
            ep,
            Endpoint {
                src: EndpointSrc::Http("/foo/bar".into()),
                dst: EndpointDst {
                    flight: "baz".into(),
                    port: 1234
                }
            }
        )
    }

    #[test]
    fn endpoint_missing_dst_or_src() {
        assert!("baz:1234".parse::<Endpoint>().is_err());
    }

    #[test]
    fn endpoint_infer_http() {
        assert!("/foo/bar=baz:1234".parse::<Endpoint>().is_ok());
    }

    #[test]
    fn endpoint_http_missing_leading_slash() {
        assert!("foo/bar=baz:1234".parse::<Endpoint>().is_err());
        assert!(":foo/bar=baz:1234".parse::<Endpoint>().is_err());
        assert!("http:foo/bar=baz:1234".parse::<Endpoint>().is_err());
        assert!("https:foo/bar/=baz:1234".parse::<Endpoint>().is_err());
        assert!("http:=baz:1234".parse::<Endpoint>().is_err(),);
    }

    // TODO: might allow eliding destination port
    #[test]
    fn endpoint_missing_dst() {
        assert!("tcp:1234=baz".parse::<Endpoint>().is_err());
        assert!("udp:1234=:1234".parse::<Endpoint>().is_err());
        assert!("http:/foo/bar=baz:".parse::<Endpoint>().is_err());
        assert!("http:/foo/bar=".parse::<Endpoint>().is_err());
    }

    #[test]
    fn endpoint_valid_tcp() {
        let ep: Endpoint = "tcp:1234=baz:4321".parse().unwrap();
        assert_eq!(
            ep,
            Endpoint {
                src: EndpointSrc::Tcp(1234),
                dst: EndpointDst {
                    flight: "baz".into(),
                    port: 4321
                }
            }
        )
    }

    #[test]
    fn endpoint_invalid_tcp_udp() {
        assert!("udp:/foo/bar=baz:1234".parse::<Endpoint>().is_err());
        assert!("udp:1234=baz:9999999".parse::<Endpoint>().is_err());
        assert!("udp:1234=baz:/foo".parse::<Endpoint>().is_err());
    }
}
