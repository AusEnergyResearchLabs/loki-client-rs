use serde::Serialize;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Stream
#[derive(Serialize, Debug, Clone)]
pub struct Stream {
    #[serde(rename = "stream")]
    labels: HashMap<String, String>,
    values: Vec<Vec<String>>,
}

/// Streams
#[derive(Serialize, Debug, Clone)]
pub struct Streams {
    pub streams: Vec<Stream>,
}

/// Incrementally constructs a `Stream`
#[derive(Debug)]
pub struct Builder {
    stream: Stream,
}

impl Builder {
    /// Creates a new builder
    pub fn new() -> Self {
        Self {
            stream: Stream {
                labels: HashMap::new(),
                values: Vec::new(),
            },
        }
    }

    /// Add a label to the stream
    pub fn label<S>(mut self, key: S, value: S) -> Self
    where
        S: Into<String>,
    {
        self.stream.labels.insert(key.into(), value.into());

        self
    }

    /// Add log message to the stream
    ///
    /// An optional timestamp can be provided
    pub fn log<S>(mut self, timestamp: Option<u128>, log: S) -> Self
    where
        S: Into<String>,
    {
        let log = log.into();

        let timestamp = match timestamp {
            Some(t) => t,
            None => SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_nanos(),
        };

        self.stream.values.push(vec![timestamp.to_string(), log]);

        self
    }

    pub fn build(self) -> Stream {
        self.stream
    }
}
