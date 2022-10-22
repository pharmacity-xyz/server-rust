use pharmacity::configuration::get_configuration;
use pharmacity::startup::Application;
use pharmacity::telemetry::{get_subscriber, init_subscriber};


#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("pharmacity".into(), "info".into());
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
}
