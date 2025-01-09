use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};
use tonic::transport::{Channel, Error};
#[cfg(feature = "web")]
use tonic_web_wasm_client::Client;

use crate::stream::stream_service_client::StreamServiceClient;

pub struct GrpcConnectionManager {
    #[cfg(feature = "web")]
    client: Arc<Mutex<Option<StreamServiceClient<Client>>>>,
    #[cfg(feature = "desktop")]
    client: Arc<Mutex<Option<StreamServiceClient<Channel>>>>,
    endpoint: String,
}

impl GrpcConnectionManager {
    pub fn new(endpoint: String) -> Self {
        Self {
            client: Arc::new(Mutex::new(None)),
            endpoint,
        }
    }

    pub async fn connect(&self) -> Result<(), Error> {
        #[cfg(feature = "web")]
        let client_web = Client::new(self.endpoint.clone());
        #[cfg(feature = "web")]
        let client = StreamServiceClient::new(client_web);

        #[cfg(feature = "desktop")]
        let client = StreamServiceClient::connect(self.endpoint.clone()).await?;
        let mut client_lock = self.client.lock().unwrap();
        *client_lock = Some(client);
        Ok(())
    }

    #[cfg(feature = "web")]
    pub async fn get_client(&self) -> Result<StreamServiceClient<Client>, Error> {
        let client_option = {
            let client_lock = self.client.lock().unwrap();
            client_lock.clone()
        };

        if let Some(client) = client_option {
            Ok(client)
        } else {
            self.connect().await?;
            let client_lock = self.client.lock().unwrap();
            Ok(client_lock.clone().unwrap())
        }
    }

    #[cfg(feature = "desktop")]
    pub async fn get_client(&self) -> Result<StreamServiceClient<Channel>, Error> {
        let client_option = {
            let client_lock = self.client.lock().unwrap();
            client_lock.clone()
        };

        if let Some(client) = client_option {
            Ok(client)
        } else {
            self.connect().await?;
            let client_lock = self.client.lock().unwrap();
            Ok(client_lock.clone().unwrap())
        }
    }

    pub async fn reconnect_with_backoff(&self) {
        let mut delay = 1;
        loop {
            if self.connect().await.is_ok() {
                break;
            }
            sleep(Duration::from_secs(delay)).await;
            delay = (delay * 2).min(32);
        }
    }
}
