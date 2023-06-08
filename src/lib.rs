use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;

pub mod stream;
pub use stream::*;

/// Streams
#[derive(Serialize, Debug, Clone)]
pub struct Streams {
    pub streams: Vec<Stream>,
}

/// Build information
#[derive(Debug, Deserialize)]
pub struct BuildInfo {
    pub version: String,
    pub revision: String,
    pub branch: String,
    #[serde(rename = "buildUser")]
    pub build_user: String,
    #[serde(rename = "buildDate")]
    pub build_date: String,
    #[serde(rename = "goVersion")]
    pub go_version: String,
}

/// Loki client
pub struct Loki {
    url: String,
    client: Client,
}

impl Loki {
    /// Creates a new client
    pub fn new<S>(url: S) -> Loki
    where
        S: Into<String>,
    {
        Loki {
            url: url.into(),
            client: Client::new(),
        }
    }

    /// Gets the ready status
    pub async fn ready(&self) -> Result<bool> {
        let uri = format!("{}/ready", self.url);

        Ok(self.client.get(uri).send().await?.status().is_success())
    }

    /// Gets the status of each service
    pub async fn services(&self) -> Result<HashMap<String, ServiceStatus>> {
        let uri = format!("{}/services", self.url);

        let res = self.client.get(uri).send().await?;
        let text = res.text().await?;

        let mut services = HashMap::new();

        for line in text.lines() {
            let part: Vec<&str> = line.split(" => ").collect();

            if part.len() == 2 {
                let (name, status) = (part[0], part[1]);

                if let Ok(status) = ServiceStatus::from_str(status) {
                    services.insert(name.to_owned(), status);
                }
            }
        }

        Ok(services)
    }

    /// Flush in-memory chunks to backing store
    pub async fn flush(&self) -> Result<()> {
        let uri = format!("{}/flush", self.url);

        self.client.post(uri).send().await?;

        Ok(())
    }

    /// Flush in-memory chunks and shut down
    pub async fn ingester_shutdown(&self) -> Result<()> {
        let uri = format!("{}/ingester/shutdown", self.url);

        self.client.post(uri).send().await?;

        Ok(())
    }

    /// Get build information
    pub async fn build_info(&self) -> Result<BuildInfo> {
        let uri = format!("{}/loki/api/v1/status/buildinfo", self.url);

        let res = self.client.get(uri).send().await?;
        let text = res.text().await?;

        let body = serde_json::from_str::<BuildInfo>(&text)?;

        Ok(body)
    }

    /// Push log entries to Loki
    pub async fn push(&self, streams: Vec<Stream>, tenant: Option<&String>) -> Result<()> {
        let uri = format!("{}/loki/api/v1/push", self.url);

        // encapsulate for the api
        let streams = Streams { streams };

        let body = serde_json::to_string(&streams)?;

        let mut req = self
            .client
            .post(uri)
            .body(body)
            .header("Content-Type", "application/json");

        if let Some(tenant) = tenant {
            req = req.header("X-ScopeOrgID", tenant);
        }

        let _ = req.send().await?;

        Ok(())
    }
}

/// Service status
#[derive(Debug)]
pub enum ServiceStatus {
    New,
    Starting,
    Running,
    Stopping,
    Terminated,
    Failed,
}

impl FromStr for ServiceStatus {
    type Err = &'static str;

    fn from_str(status: &str) -> Result<Self, Self::Err> {
        Ok(match status {
            "New" => ServiceStatus::New,
            "Starting" => ServiceStatus::Starting,
            "Running" => ServiceStatus::Running,
            "Stopping" => ServiceStatus::Stopping,
            "Terminated" => ServiceStatus::Terminated,
            "Failed" => ServiceStatus::Failed,
            _ => return Err("could not parse status"),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static URL: &str = "http://localhost:3100";

    #[tokio::test]
    async fn services() {
        let services = Loki::new(URL).services().await.unwrap();

        assert!(services.len() > 0);
    }

    #[tokio::test]
    async fn build_info() {
        let build_info = Loki::new(URL).build_info().await.unwrap();

        println!("{:?}", build_info);

        assert!(build_info.version != "");
    }

    #[tokio::test]
    async fn push() {
        let stream = stream::Builder::new()
            .label("test", "other")
            .log(None, "output")
            .build();

        Loki::new(URL).push(vec![stream], None).await.unwrap();
    }
}
