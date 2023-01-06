use once_cell::sync::Lazy;
use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

/// The target of a formation endpoint
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EndpointValue {
    /// What flight does this endpoint point to?
    pub flight_name: String,
    /// And what port on that flight?
    pub port: u16,
}

impl std::fmt::Display for EndpointValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.flight_name, self.port)
    }
}

impl std::str::FromStr for EndpointValue {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: Flight name pattern
        static REGEX: Lazy<regex::Regex> =
            Lazy::new(|| regex::Regex::new(r"^(?P<flight_name>.+):(?P<port>\d+)$").unwrap());

        let captures = REGEX
            .captures(s)
            .ok_or_else(|| "Invalid endpoint format".to_string())?;
        let flight_name = captures.name("flight_name").unwrap().as_str().to_string();
        let port = captures
            .name("port")
            .unwrap()
            .as_str()
            .parse()
            .map_err(|e| format!("{e}"))?;
        Ok(Self { flight_name, port })
    }
}

impl Serialize for EndpointValue {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for EndpointValue {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = <&str>::deserialize(deserializer)?;
        s.parse().map_err(D::Error::custom)
    }
}

#[cfg(test)]
mod test_value {
    use super::*;

    #[test]
    fn test_valid() {
        let v = EndpointValue { flight_name: "test:test".to_string(), port: 1234 };

        let s = serde_json::to_string(&v).unwrap();
        assert_eq!(s, "\"test:test:1234\"");

        let v2 = serde_json::from_str(&s).unwrap();
        assert_eq!(v, v2);
    }

    #[test]
    fn test_invalid() {
        let strings = ["foo", "1234", "foo:bar", "foo:100000"];
        for s in strings {
            assert!(serde_json::from_str::<EndpointValue>(s).is_err());
        }
    }
}

/// A load-balanced exposed formation endpoint
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum EndpointKey {
    Http { path: String },
    Tcp { port: u16 },
    Udp { port: u16 },
}

impl std::fmt::Display for EndpointKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EndpointKey::Http { path } => {
                write!(f, "http:{path}")
            }
            EndpointKey::Tcp { port } => {
                write!(f, "tcp:{port}")
            }
            EndpointKey::Udp { port } => {
                write!(f, "udp:{port}")
            }
        }
    }
}

impl std::str::FromStr for EndpointKey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static REGEX: Lazy<regex::Regex> =
            Lazy::new(|| regex::Regex::new(r"^(?P<protocol>http|tcp|udp):(?P<route>.+)$").unwrap());

        let captures = REGEX
            .captures(s)
            .ok_or_else(|| "Invalid endpoint format".to_string())?;
        match captures.name("protocol").unwrap().as_str() {
            "http" => {
                // TODO: Any sort of validation we want to do on this?
                Ok(EndpointKey::Http { path: captures.name("route").unwrap().as_str().to_string() })
            }
            "tcp" => Ok(EndpointKey::Tcp {
                port: captures
                    .name("route")
                    .unwrap()
                    .as_str()
                    .parse()
                    .map_err(|e| format!("{e}"))?,
            }),
            "udp" => Ok(EndpointKey::Udp {
                port: captures
                    .name("route")
                    .unwrap()
                    .as_str()
                    .parse()
                    .map_err(|e| format!("{e}"))?,
            }),
            _ => {
                unreachable!();
            }
        }
    }
}

impl Serialize for EndpointKey {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for EndpointKey {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = <&str>::deserialize(deserializer)?;
        s.parse().map_err(D::Error::custom)
    }
}

#[cfg(test)]
mod test_key {
    use super::*;

    #[test]
    fn test_valid() {
        let values = [
            (EndpointKey::Http { path: "/test".to_string() }, "\"http:/test\""),
            (EndpointKey::Http { path: "test:test".to_string() }, "\"http:test:test\""),
            (EndpointKey::Tcp { port: 1234 }, "\"tcp:1234\""),
            (EndpointKey::Udp { port: 1234 }, "\"udp:1234\""),
        ];

        for (v, target_string) in values {
            let s = serde_json::to_string(&v).unwrap();
            assert_eq!(s, target_string);
            let v2 = serde_json::from_str(&s).unwrap();
            assert_eq!(v, v2);
        }
    }

    #[test]
    fn test_invalid() {
        let strings = ["foo", "test:http:test", "tcp:test", "udp:test", "tcp:100000", "udp:100000"];
        for s in strings {
            assert!(serde_json::from_str::<EndpointKey>(s).is_err());
        }
    }
}
