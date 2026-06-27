use std::sync::Arc;

use rumqttc::AsyncClient;
use sqlx::PgPool;
use tokio::sync::broadcast;

use crate::{db, models::WsMessage};

#[derive(Clone)]
pub struct AppState {
    inner: Arc<Inner>,
}

struct Inner {
    pub pool:        PgPool,
    pub ws_tx:       broadcast::Sender<WsMessage>,
    pub mqtt_client: tokio::sync::RwLock<Option<AsyncClient>>,
}

impl AppState {
    pub async fn new() -> anyhow::Result<Self> {
        let pool = db::create_pool().await?;
        let (ws_tx, _) = broadcast::channel(256);

        Ok(Self {
            inner: Arc::new(Inner {
                pool,
                ws_tx,
                mqtt_client: tokio::sync::RwLock::new(None),
            }),
        })
    }

    pub fn pool(&self) -> &PgPool {
        &self.inner.pool
    }

    pub fn ws_tx(&self) -> &broadcast::Sender<WsMessage> {
        &self.inner.ws_tx
    }

    pub fn ws_subscribe(&self) -> broadcast::Receiver<WsMessage> {
        self.inner.ws_tx.subscribe()
    }

    pub async fn set_mqtt_client(&self, client: AsyncClient) {
        *self.inner.mqtt_client.write().await = Some(client);
    }

    pub async fn mqtt_client(&self) -> Option<AsyncClient> {
        self.inner.mqtt_client.read().await.clone()
    }
}
