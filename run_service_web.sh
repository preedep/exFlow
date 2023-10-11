RUST_LOG=exflow_service cargo run  --bin exflow_service --features=exflow_service_required  \
-- \
--apm-connection-string "InstrumentationKey=c9fbbf32-d49e-4c43-99fe-802335dbcae4;IngestionEndpoint=https://southeastasia-1.in.applicationinsights.azure.com/;LiveEndpoint=https://southeastasia.livediagnostics.monitor.azure.com/"