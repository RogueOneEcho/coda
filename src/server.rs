use crate::options::UserOptions;
use crate::User;
use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;
use gazelle_api::{GazelleClient, GazelleClientOptions};
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

async fn metrics() -> Result<String, StatusCode> {
    let options = get_options().map_err(|e| {
        e.log();
        StatusCode::from_u16(500).expect("status code should be valid")
    })?;
    let mut output = String::new();
    for (client_options, user_options) in options {
        let mut api = GazelleClient::from_options(client_options.clone());
        let user = api
            .get_user(user_options.user)
            .await
            .expect("should be able to get user");
        let user = User::from_gazelle(user);
        let yaml = serde_yaml::to_string(&user).expect("should be able to serialize");
        let metrics = yaml.replace(": ", &format!("{{name=\"{}\"}} ", client_options.name));
        output.push_str(&metrics);
    }
    Ok(output)
}

fn get_options() -> Result<Vec<(GazelleClientOptions, UserOptions)>, Error> {
    let clients: Vec<GazelleClientOptions> = YamlOptionsProvider::get()?;
    let users: Vec<UserOptions> = YamlOptionsProvider::get()?;
    let vec = clients.into_iter().zip(users).collect();
    Ok(vec)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[tokio::test]
    pub async fn metrics_test() {
        // Arrange
        let clients: Vec<GazelleClientOptions> = YamlOptionsProvider::get().unwrap();
        let expected = clients.len() * 9;

        // Act
        let result = metrics().await;

        // Assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines().count(), expected);
    }
}
