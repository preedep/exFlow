use log::debug;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;

pub fn set_global_tracing(apm_connection_string: &String) {
    if apm_connection_string.len() > 0 {
        debug!("APPLICATIONINSIGHTS_CON_STRING = {}", apm_connection_string);
        let exporter = opentelemetry_application_insights::new_pipeline_from_connection_string(
            apm_connection_string,
        )
            .unwrap()
            .with_client(reqwest::Client::new())
            .with_service_name("ExFlow-Runtime")
            .install_batch(opentelemetry::runtime::Tokio);

        let telemetry = tracing_opentelemetry::layer().with_tracer(exporter);
        let subscriber = Registry::default().with(telemetry);
        tracing::subscriber::set_global_default(subscriber).expect("setting global default failed");
    }
}
