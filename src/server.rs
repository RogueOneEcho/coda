use crate::User;
use crate::UserOptions;
use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;
use gazelle_api::{GazelleClient, GazelleClientOptions};
use log::warn;
use rogue_config::{OptionsProvider, YamlOptionsProvider};
use rogue_logging::Error;
use tokio::net::TcpListener;

pub struct Server {
    pub router: Router,
    pub listener: TcpListener,
}

impl Server {
    pub async fn create(host: String) -> Result<Self, Error> {
        let router = Router::new().route("/metrics", get(metrics));
        let listener = TcpListener::bind(host).await.map_err(|e| Error {
            action: "start server".to_owned(),
            message: e.to_string(),
            ..Error::default()
        })?;
        Ok(Self { router, listener })
    }

    pub async fn start(self) -> Result<(), Error> {
        axum::serve(self.listener, self.router)
            .await
            .map_err(|e| Error {
                action: "start server".to_owned(),
                message: e.to_string(),
                ..Error::default()
            })?;
        Ok(())
    }
}

async fn metrics_internal() -> Result<String, Error> {
    let options = get_options()?;
    let mut output = String::new();
    for (client_options, user_options) in options {
        let mut api = GazelleClient::from_options(client_options.clone());
        let user = match api.get_user(user_options.user).await {
            Ok(user) => user,
            Err(e) => {
                warn!("Failed to get user for client: {}", client_options.name);
                e.log();
                continue;
            }
        };
        let user = User::from_gazelle(user);
        let yaml = match serde_yaml::to_string(&user) {
            Ok(yaml) => yaml,
            Err(e) => {
                warn!(
                    "Failed to deserialize user for client: {}",
                    client_options.name
                );
                warn!("{e}");
                continue;
            }
        };
        let metrics = yaml.replace(": ", &format!("{{name=\"{}\"}} ", client_options.name));
        output.push_str(&metrics);
    }
    Ok(output)
}

async fn metrics() -> Result<String, StatusCode> {
    metrics_internal().await.map_err(|e| {
        e.log();
        StatusCode::from_u16(500).expect("status code should be valid")
    })
}

fn get_options() -> Result<Vec<(GazelleClientOptions, UserOptions)>, Error> {
    let clients: Vec<GazelleClientOptions> = YamlOptionsProvider::get()?;
    let users: Vec<UserOptions> = YamlOptionsProvider::get()?;
    let vec = clients.into_iter().zip(users).collect();
    Ok(vec)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::init_logging;

    #[tokio::test]
    pub async fn metrics_test() -> Result<(), Error> {
        // Arrange
        init_logging();
        let clients: Vec<GazelleClientOptions> = YamlOptionsProvider::get()?;
        let expected = clients.len() * 9;

        // Act
        let result = metrics_internal().await?;

        // Assert
        assert_eq!(result.lines().count(), expected);
        Ok(())
    }
}
