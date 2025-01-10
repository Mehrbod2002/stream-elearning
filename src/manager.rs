use crate::stream::stream_service_client::StreamServiceClient;
use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};
use tonic::transport::{Channel, Error};

pub struct GrpcConnectionManager {
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
        let client = StreamServiceClient::connect(self.endpoint.clone()).await?;
        let mut client_lock = self.client.lock().unwrap();
        *client_lock = Some(client);

        Ok(())
    }

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
