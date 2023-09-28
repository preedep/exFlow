use azure_identity::DefaultAzureCredential;

pub async fn test() {
    let credential = DefaultAzureCredential::default();
}