use crate::options::get_options;
use rogue_logging::{Error, Logger};

#[tokio::test]
async fn get_user() -> Result<(), Error> {
    // Arrange
    Logger::force_init("gazelle_api".to_owned());
    for options in get_options()? {
        println!("Indexer: {}", options.name);
        let mut client = options.get_client();

        // Act
        let user = client.get_user(options.user).await?;

        // Assert
        assert!(!user.username.is_empty());
    }
    Ok(())
}
