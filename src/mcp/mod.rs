mod server;

use rmcp::ServiceExt;

use crate::api::HtbClient;
use server::HtbMcp;

pub async fn run_stdio() -> anyhow::Result<()> {
    let token = crate::config::read_token()
        .map_err(|_| anyhow::anyhow!("Not authenticated. Run `htb auth login` first."))?;

    let client = HtbClient::new(token);

    // Validate token
    let user = client.user().current().await?;
    eprintln!("htb-mcp: authenticated as {}", user.name);

    let service = HtbMcp::new(client).serve(rmcp::transport::stdio()).await?;

    service.waiting().await?;
    Ok(())
}
