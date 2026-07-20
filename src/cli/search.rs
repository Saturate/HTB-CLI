use crate::api::HtbClient;
use crate::output;

pub async fn handle(client: &HtbClient, query: &str) -> anyhow::Result<()> {
    let results = client.search().fetch(query).await?;
    output::json::print_json(&results);
    Ok(())
}
