use tonic_axum_sqlx::generated::object_api::object_api_client::ObjectApiClient;
use tonic_axum_sqlx::generated::object_api::ObjectId;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ObjectApiClient::connect("http://localhost:3000").await?;

    let request = tonic::Request::new(ObjectId {
        id: 1
    });

    let response = client.get_object(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}