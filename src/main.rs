use pkarr::dns::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let keypair = pkarr::Keypair::random();
    let client = pkarr::Client::builder().build()?;

    publish_server_pkarr(&client, &keypair).await?;
    fetch_data(&client, &keypair).await?;

    Ok(())
}

async fn publish_server_pkarr(
    client: &pkarr::Client,
    keypair: &pkarr::Keypair,
) -> anyhow::Result<()> {
    let name = ".".try_into()?;
    let svcb = rdata::SVCB::new(0, "github.com".try_into()?);

    let signed_packet = pkarr::SignedPacket::builder()
        .https(name, svcb, 300)
        .sign(&keypair)?;

    client.publish(&signed_packet).await?;

    Ok(())
}

async fn fetch_data(client: &pkarr::Client, keypair: &pkarr::Keypair) -> anyhow::Result<()> {
    let client = reqwest::ClientBuilder::from(client.clone()).build()?;
    let url = format!("https://{}", keypair.public_key().to_z32());
    let result = client.get(url).send().await?;

    dbg!(result);

    Ok(())
}
