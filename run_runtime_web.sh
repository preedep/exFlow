RUST_LOG=exflow_runtime cargo run  --bin exflow_runtime --features=exflow_runtime_required  \
runtime --exflow-service-endpoint localhost:8081 \
--apm-connection-string "InstrumentationKey=c9fbbf32-d49e-4c43-99fe-802335dbcae4;IngestionEndpoint=https://southeastasia-1.in.applicationinsights.azure.com/;LiveEndpoint=https://southeastasia.livediagnostics.monitor.azure.com/"