use local_ip_address::local_ip;
use log::debug;
use serde::{Deserialize, Serialize};
use sysinfo::{NetworkExt, System, SystemExt};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;

use crate::mod_utils::errors::ExFlowError;

type ExFlowResult<T> = Result<T, ExFlowError>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInformation {
    pub host_name: String,
    pub host_ip: String,
}

impl SystemInformation {
    pub fn new(host_name: &str, host_ip: &str) -> Self {
        SystemInformation {
            host_name: host_name.to_string(),
            host_ip: host_ip.to_string(),
        }
    }
}

pub fn string_to_static_str(s: &String) -> &'static str {
    Box::leak(s.clone().into_boxed_str())
}

pub fn set_global_apm_tracing(apm_connection_string: &str, service_name: &str) {
    if apm_connection_string.len() > 0 {
        debug!("APPLICATIONINSIGHTS_CON_STRING = {}", apm_connection_string);
        let exporter = opentelemetry_application_insights::new_pipeline_from_connection_string(
            apm_connection_string,
        )
        .unwrap()
        .with_client(reqwest::Client::new())
        .with_service_name(service_name.to_string())
        .install_batch(opentelemetry::runtime::Tokio);

        let telemetry = tracing_opentelemetry::layer().with_tracer(exporter);
        let subscriber = Registry::default().with(telemetry);
        tracing::subscriber::set_global_default(subscriber).expect("setting global default failed");
    }
}

pub fn get_system_info() -> ExFlowResult<SystemInformation> {
    debug!("get_system_info");

    // Please note that we use "new_all" to ensure that all list of
    // components, network interfaces, disks and users are already
    // filled!
    let mut sys = System::new_all();

    // First we update all information of our `System` struct.
    sys.refresh_all();

    // We display all disks' information:
    debug!("=> disks:");
    for disk in sys.disks() {
        println!("{:?}", disk);
    }
    // Network interfaces name, data received and data transmitted:
    debug!("=> networks:");
    for (interface_name, data) in sys.networks() {
        debug!(
            "{}: {}/{} B",
            interface_name,
            data.received(),
            data.transmitted()
        );
    }
    // Components temperature:
    debug!("=> components:");
    for component in sys.components() {
        debug!("{:?}", component);
    }

    debug!("=> system:");
    // RAM and swap information:
    debug!("total memory: {} bytes", sys.total_memory());
    debug!("used memory : {} bytes", sys.used_memory());
    debug!("total swap  : {} bytes", sys.total_swap());
    debug!("used swap   : {} bytes", sys.used_swap());

    // Display system information:
    debug!("System name:             {:?}", sys.name());
    debug!("System kernel version:   {:?}", sys.kernel_version());
    debug!("System OS version:       {:?}", sys.os_version());
    debug!("System host name:        {:?}", sys.host_name());

    // Number of CPUs:
    debug!("NB CPUs: {}", sys.cpus().len());

    let my_local_ip = local_ip();
    my_local_ip.map_err(|e| ExFlowError::new("Error")).map(|r| {
        SystemInformation::new(
            &sys.host_name().unwrap_or("".to_string()).as_str(),
            r.to_string().as_str(),
        )
    })
}
