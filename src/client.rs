use std::{collections::HashMap, fmt::Debug, time::Duration, io::{ErrorKind, Error}};

use reqwest::Body;
use serde::{de::DeserializeOwned, Serialize};
use tokio::io::AsyncRead;
use tokio_util::{codec::{BytesCodec, FramedRead}, io::StreamReader};

use crate::error::{Result, self};

pub(crate) const API_PREFIX: &str = "api/v1";
pub(crate) const PATHS_PREFIX: &str = "paths";
pub(crate) const STREAMS_PREFIX: &str = "streams";

// Path endpoints
pub(crate) const CREATE_DIRECTORY: &str = "create-directory";
pub(crate) const CREATE_FILE: &str = "create-file";
pub(crate) const DELETE: &str = "delete";
pub(crate) const EXISTS: &str = "exists";
pub(crate) const FREE: &str = "free";
pub(crate) const GET_STATUS: &str = "get-status";
pub(crate) const LIST_STATUS: &str = "list-status";
pub(crate) const MOUNT: &str = "mount";
pub(crate) const OPEN_FILE: &str = "open-file";
pub(crate) const RENAME: &str = "rename";
pub(crate) const SET_ATTRIBUTE: &str = "set-attribute";
pub(crate) const UNMOUNT: &str = "unmount";

// Stream endpoints
pub(crate) const CLOSE: &str = "close";
pub(crate) const READ: &str = "read";
pub(crate) const WRITE: &str = "write";

#[derive(Debug)]
pub struct Client {
    pub(crate) host: String,
    pub(crate) port: u16,
    pub(crate) prefix: String,
    pub(crate) client: reqwest::Client,
}

impl Client {
    pub fn new(host: String, port: u16, timeout: Duration) -> Result<Client> {
        let client = reqwest::Client::builder()
            .timeout(timeout)
            .no_proxy()
            .build()?;
        Ok(Client {
            host,
            port,
            prefix: API_PREFIX.to_string(),
            client,
        })
    }

    fn endpoint_url(&self, resource: &str, mut params: HashMap<&str, &str>) -> String {
        let param_pairs: Vec<String> = params
            .iter_mut()
            .map(|(k, v)| format!("{}={}&", k, url_escape::encode_query(v)))
            .collect();

        format!(
            "http://{}:{}/{}/{}?{}",
            self.host,
            self.port,
            self.prefix,
            resource,
            param_pairs.join("&")
        )
    }

    pub(crate) async fn post<I: Serialize, O: DeserializeOwned + Debug + Default>(
        &self,
        resource: String,
        params: HashMap<&str, &str>,
        input: I,
    ) -> Result<O> {
        let resp = self
            .client
            .post(self.endpoint_url(resource.as_str(), params.clone()))
            .json(&input)
            .send()
            .await?;
        let status = resp.status();
        if status.is_success() {
            let body = resp.bytes().await?;
            if body.is_empty() {
                return Ok(O::default());
            }
            return Ok(serde_json::from_slice(body.as_ref())?);
        }
        let body = resp.bytes().await?;
        let body = std::str::from_utf8(body.as_ref())?;
        Err(error::Error::Server(body.to_string()))
    }

    pub async fn write<'a, I>(&'a self, id: i64, input: I) -> Result<i64>
    where
        I: AsyncRead + Send + Sync + 'a + 'static,
    {
        let stream = FramedRead::new(input, BytesCodec::new());
        let suffix = [STREAMS_PREFIX, &id.to_string(), WRITE].join("/");
        let output = self
            .client
            .post(self.endpoint_url(&suffix, HashMap::new()))
            .body(Body::wrap_stream(stream))
            .send()
            .await?
            .json()
            .await?;
        Ok(output)
    }

    pub async fn close(&self, id: i64) -> Result<()> {
        self.post([STREAMS_PREFIX, &id.to_string(), CLOSE].join("/"), HashMap::new(), serde_json::Value::Null).await?;
        Ok(())
    }

    pub async fn read(&self, id: i64) -> Result<impl AsyncRead> {
        let suffix = [STREAMS_PREFIX, &id.to_string(), READ].join("/");

        let resp = self
            .client
            .post(self.endpoint_url(&suffix, HashMap::new()))
            .send()
            .await?;
        use futures::stream::TryStreamExt;
        let stream = resp.bytes_stream().map_err(convert_error);
        let read = StreamReader::new(stream);
        Ok(read)
    }


}

fn convert_error(err: reqwest::Error) -> std::io::Error {
    Error::new(ErrorKind::Other, "reqwest error")
}
