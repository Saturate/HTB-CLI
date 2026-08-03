use clap::Subcommand;
use serde_json::Value;

use crate::api::HtbClient;
use crate::output::{self, OutputFormat};

#[derive(Subcommand)]
pub enum UserCommand {
    /// Show your profile
    Me,
    /// Show another user's profile
    Info {
        /// Username or user ID
        user: String,
    },
    /// Show recent activity (owns, solves)
    Activity {
        /// Username or user ID (defaults to you)
        user: Option<String>,
    },
}

pub async fn handle(
    client: &HtbClient,
    cmd: UserCommand,
    format: OutputFormat,
) -> anyhow::Result<()> {
    match cmd {
        UserCommand::Me => {
            let current = client.user().current().await?;
            let profile = client.user().profile(current.id).await?;
            let fields = vec![
                ("Username", profile.name.clone()),
                ("ID", profile.id.to_string()),
                ("Rank", profile.rank.clone().unwrap_or_default()),
                ("Points", profile.points.to_string()),
                (
                    "Ranking",
                    profile
                        .ranking
                        .map(|r| format!("#{r}"))
                        .unwrap_or_else(|| "-".into()),
                ),
                ("User Owns", profile.user_owns.to_string()),
                ("System Owns", profile.system_owns.to_string()),
                ("User Bloods", profile.user_bloods.to_string()),
                ("System Bloods", profile.system_bloods.to_string()),
                ("Country", profile.country_name.clone().unwrap_or_default()),
                ("Server", profile.server.clone().unwrap_or_default()),
            ];
            output::print_detail(&profile, format, &fields);
        }

        UserCommand::Info { user } => {
            let user_id = match user.parse::<u64>() {
                Ok(id) => id,
                Err(_) => {
                    let results = client.search().fetch(&user).await?;
                    resolve_user_id(&results, &user)
                        .ok_or_else(|| anyhow::anyhow!("User '{user}' not found."))?
                }
            };
            let profile = client.user().profile(user_id).await?;
            let fields = vec![
                ("Username", profile.name.clone()),
                ("ID", profile.id.to_string()),
                ("Rank", profile.rank.clone().unwrap_or_default()),
                ("Points", profile.points.to_string()),
                (
                    "Ranking",
                    profile
                        .ranking
                        .map(|r| format!("#{r}"))
                        .unwrap_or_else(|| "-".into()),
                ),
                ("User Owns", profile.user_owns.to_string()),
                ("System Owns", profile.system_owns.to_string()),
                ("Country", profile.country_name.clone().unwrap_or_default()),
            ];
            output::print_detail(&profile, format, &fields);
        }
        UserCommand::Activity { user } => {
            let user_id = match user {
                Some(u) => match u.parse::<u64>() {
                    Ok(id) => id,
                    Err(_) => {
                        let results = client.search().fetch(&u).await?;
                        resolve_user_id(&results, &u)
                            .ok_or_else(|| anyhow::anyhow!("User '{u}' not found."))?
                    }
                },
                None => client.user().current().await?.id,
            };
            let activity = client.user().activity(user_id).await?;
            if activity.is_empty() {
                output::print_message("No recent activity.");
            } else {
                output::print_list(&activity, format);
            }
        }
    }
    Ok(())
}

fn resolve_user_id(results: &Value, username: &str) -> Option<u64> {
    let users = results.get("users")?.as_array()?;
    let lower = username.to_lowercase();
    for user in users {
        if user
            .get("value")
            .and_then(|n| n.as_str())
            .is_some_and(|n| n.to_lowercase() == lower)
        {
            return user.get("id")?.as_u64();
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn resolve_user_id_found() {
        let data = json!({"users": [{"id": 2751322, "value": "johndoe"}]});
        assert_eq!(resolve_user_id(&data, "johndoe"), Some(2751322));
    }

    #[test]
    fn resolve_user_id_case_insensitive() {
        let data = json!({"users": [{"id": 2751322, "value": "JohnDoe"}]});
        assert_eq!(resolve_user_id(&data, "johndoe"), Some(2751322));
    }

    #[test]
    fn resolve_user_id_not_found() {
        let data = json!({"users": [{"id": 1, "value": "alice"}]});
        assert_eq!(resolve_user_id(&data, "bob"), None);
    }

    #[test]
    fn resolve_user_id_empty() {
        let data = json!({"users": []});
        assert_eq!(resolve_user_id(&data, "alice"), None);
    }

    #[test]
    fn resolve_user_id_missing_key() {
        let data = json!({"machines": []});
        assert_eq!(resolve_user_id(&data, "alice"), None);
    }
}
