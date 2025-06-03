use reqwest::{Method, StatusCode};
use wtransport::{endpoint::endpoint_side::{Client, Server}, Connection, Endpoint};

use crate::network::HttpMethod;

pub type NetworkClient = Endpoint<Client>;
pub type NetworkServer = Endpoint<Server>;
pub type NetworkHttpClient = reqwest::Client;
pub type NetworkConnection = Connection;

impl Into<Method> for HttpMethod {
    fn into(self) -> Method {
        match self {
            HttpMethod::Delete => Method::DELETE,
            HttpMethod::Head => Method::HEAD,
            HttpMethod::Get => Method::GET,
            HttpMethod::Post => Method::POST,
            HttpMethod::Put => Method::PUT
        }
    }
}
