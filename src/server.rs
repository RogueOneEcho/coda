use crate::get_options;
use crate::User;
use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;
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
    let options = get_options().expect("should be able to get options");
    let mut output = String::new();
    for options in options {
        let mut api = options.get_client();
        let user = api
            .get_user(options.user)
            .await
            .expect("should be able to get user");
        let user = User::from_gazelle(user);
        let yaml = serde_yaml::to_string(&user).expect("should be able to serialize");
        let metrics = yaml.replace(": ", &format!("{{name=\"{}\"}} ", options.name));
        output.push_str(&metrics);
    }
    Ok(output)
}
