use std::sync::Arc;
use std::time::Duration;

use clap::Subcommand;

use crate::api::HtbClient;
use crate::cache::Cache;
use crate::output::OutputFormat;

const CTF_BASE_URL: &str = "https://ctf.hackthebox.com";

#[derive(Subcommand)]
#[command(
    after_help = "Workflow:\n  1. htb ctf auth login                  Authenticate (separate token from labs)\n  2. htb ctf events                      Find an event\n  3. htb ctf use 1434                    Set active event (sticky)\n  4. htb ctf challenges                  Browse challenges\n  5. htb ctf start 31855                 Spin up the container\n  6. htb ctf download 31855              Grab challenge files\n     ... hack ...\n  7. htb ctf submit 31855 'HTB{flag}'    Submit your flag\n  8. htb ctf stop 31855                  Clean up the container\n\nOverride active event:\n  htb ctf start -e 1434 31855           Explicit event for one command\n  htb ctf use --clear                    Remove sticky event\n\nOther:\n  htb ctf info ctf-try-out-1434          Event details (by slug)\n  htb ctf scoreboard                     Team rankings\n  htb ctf solves                         Recent solves feed\n  htb ctf challenge-solves 31855         Who solved a challenge"
)]
pub enum CtfCommand {
    /// Manage CTF authentication
    Auth {
        #[command(subcommand)]
        command: CtfAuthCommand,
    },
    /// Set the active CTF event (persists to config)
    Use {
        /// Event ID to set as active
        event_id: Option<u64>,
        /// Clear the active event
        #[arg(long)]
        clear: bool,
    },
    /// List CTF events
    Events {
        #[arg(long, help = "Include past events")]
        all: bool,
    },
    /// Show CTF event details
    Info {
        /// Event slug (e.g. ctf-try-out-1434)
        slug: String,
    },
    /// List challenges in a CTF event
    Challenges {
        /// Event ID (uses active event if omitted)
        event_id: Option<u64>,
        #[arg(long, help = "Filter by difficulty")]
        difficulty: Option<String>,
    },
    /// Submit a flag
    Submit {
        /// Challenge ID
        challenge_id: u64,
        /// The flag
        flag: String,
    },
    /// Download challenge files
    Download {
        /// Challenge ID
        challenge_id: u64,
        /// Event ID (uses active event if omitted)
        #[arg(short = 'e', long = "event")]
        event_id: Option<u64>,
    },
    /// Start a challenge container
    Start {
        /// Challenge ID
        challenge_id: u64,
        /// Event ID (uses active event if omitted)
        #[arg(short = 'e', long = "event")]
        event_id: Option<u64>,
    },
    /// Stop a challenge container
    Stop {
        /// Challenge ID
        challenge_id: u64,
        /// Event ID (uses active event if omitted)
        #[arg(short = 'e', long = "event")]
        event_id: Option<u64>,
    },
    /// Show event scoreboard
    Scoreboard {
        /// Event ID (uses active event if omitted)
        event_id: Option<u64>,
    },
    /// Show recent solves for an event
    Solves {
        /// Event ID (uses active event if omitted)
        event_id: Option<u64>,
    },
    /// Show solves for a specific challenge
    ChallengeSolves {
        /// Challenge ID
        challenge_id: u64,
    },
}

#[derive(Subcommand)]
pub enum CtfAuthCommand {
    /// Save your CTF API token
    Login,
    /// Show CTF auth status
    Status,
    /// Remove stored CTF token
    Logout,
}

fn resolve_event_id(explicit: Option<u64>) -> anyhow::Result<u64> {
    match explicit.or_else(crate::config::read_ctf_event) {
        Some(id) => Ok(id),
        None => anyhow::bail!(
            "No event ID provided and no active event set. Run: htb ctf use <event_id>"
        ),
    }
}

pub async fn handle(
    cmd: CtfCommand,
    format: OutputFormat,
    cache: &Arc<Cache>,
) -> anyhow::Result<()> {
    match cmd {
        CtfCommand::Auth { command } => handle_auth(command, format, cache).await,
        CtfCommand::Use { event_id, clear } => {
            if clear {
                crate::config::save_ctf_event(None)?;
                crate::output::print_message("Active event cleared.");
                Ok(())
            } else if let Some(id) = event_id {
                use_event(id, cache).await
            } else {
                match crate::config::read_ctf_event() {
                    Some(id) => {
                        crate::output::print_message(&format!("Active event: {id}"));
                        Ok(())
                    }
                    None => anyhow::bail!(
                        "No active event set. Usage: htb ctf use <event_id> or --clear"
                    ),
                }
            }
        }
        CtfCommand::Events { all } => events(all, format, cache).await,
        CtfCommand::Info { slug } => info(&slug, format, cache).await,
        CtfCommand::Challenges {
            event_id,
            difficulty,
        } => {
            let eid = resolve_event_id(event_id)?;
            challenges(eid, difficulty.as_deref(), format, cache).await
        }
        CtfCommand::Submit { challenge_id, flag } => submit(challenge_id, &flag, cache).await,
        CtfCommand::Download {
            event_id,
            challenge_id,
        } => {
            let eid = resolve_event_id(event_id)?;
            download(eid, challenge_id, cache).await
        }
        CtfCommand::Start {
            event_id,
            challenge_id,
        } => {
            let eid = resolve_event_id(event_id)?;
            start(eid, challenge_id, cache).await
        }
        CtfCommand::Stop {
            event_id,
            challenge_id,
        } => {
            let eid = resolve_event_id(event_id)?;
            stop(eid, challenge_id, cache).await
        }
        CtfCommand::Scoreboard { event_id } => {
            let eid = resolve_event_id(event_id)?;
            scoreboard(eid, format, cache).await
        }
        CtfCommand::Solves { event_id } => {
            let eid = resolve_event_id(event_id)?;
            solves(eid, format, cache).await
        }
        CtfCommand::ChallengeSolves { challenge_id } => {
            challenge_solves(challenge_id, format, cache).await
        }
    }
}

fn ctf_client(cache: &Arc<Cache>) -> anyhow::Result<HtbClient> {
    let token = crate::config::read_ctf_token()?;
    Ok(HtbClient::with_base_url_and_cache(
        token,
        CTF_BASE_URL.to_string(),
        Arc::clone(cache),
    ))
}

async fn handle_auth(
    cmd: CtfAuthCommand,
    format: OutputFormat,
    cache: &Arc<Cache>,
) -> anyhow::Result<()> {
    match cmd {
        CtfAuthCommand::Login => login(cache).await,
        CtfAuthCommand::Status => status(format).await,
        CtfAuthCommand::Logout => logout(cache),
    }
}

async fn use_event(event_id: u64, cache: &Arc<Cache>) -> anyhow::Result<()> {
    let client = ctf_client(cache)?;
    let events = client.ctf().events().await?;
    let event = events
        .iter()
        .find(|e| e.id == event_id)
        .ok_or_else(|| anyhow::anyhow!("Event {event_id} not found"))?;

    crate::config::save_ctf_event(Some(event_id))?;
    crate::output::print_message(&format!(
        "Active event: {} ({})",
        event.name,
        event.status.as_deref().unwrap_or("unknown")
    ));
    Ok(())
}

async fn login(cache: &Cache) -> anyhow::Result<()> {
    println!("Enter your CTF API token (from https://ctf.hackthebox.com/settings):");
    let token = rpassword::read_password()?;
    let token = token.trim().to_string();

    if token.is_empty() {
        anyhow::bail!("Token cannot be empty");
    }

    let client = HtbClient::with_base_url(token.clone(), CTF_BASE_URL.to_string());
    let user = client.ctf().profile().await?;

    crate::config::save_ctf_token(&token)?;
    cache.clear();
    println!("Authenticated as {}", user.name);
    Ok(())
}

async fn status(format: OutputFormat) -> anyhow::Result<()> {
    let token = crate::config::read_ctf_token()?;
    let client = HtbClient::with_base_url(token, CTF_BASE_URL.to_string());
    let user = client.ctf().profile().await?;

    let fields = vec![
        ("Username", user.name.clone()),
        ("ID", user.id.to_string()),
        ("Email", user.email.clone().unwrap_or_default()),
        ("Timezone", user.timezone.clone().unwrap_or_default()),
        (
            "Has Team",
            if user.has_any_team { "Yes" } else { "No" }.into(),
        ),
    ];

    crate::output::print_detail(&user, format, &fields);
    Ok(())
}

fn logout(cache: &Cache) -> anyhow::Result<()> {
    crate::config::remove_ctf_token()?;
    cache.clear();
    println!("CTF token removed.");
    Ok(())
}

async fn events(all: bool, format: OutputFormat, cache: &Arc<Cache>) -> anyhow::Result<()> {
    let client = ctf_client(cache)?;
    let mut events = client.ctf().events().await?;

    if !all {
        events.retain(|e| {
            e.status
                .as_deref()
                .is_some_and(|s| s == "Ongoing" || s == "Upcoming")
        });
    }

    crate::output::print_list(&events, format);
    Ok(())
}

async fn info(slug: &str, format: OutputFormat, cache: &Arc<Cache>) -> anyhow::Result<()> {
    let client = ctf_client(cache)?;
    let detail = client.ctf().event_details(slug).await?;

    let fields = vec![
        ("ID", detail.id.to_string()),
        ("Name", detail.name.clone()),
        ("Status", detail.status.clone().unwrap_or_default()),
        ("Format", detail.format.clone().unwrap_or_default()),
        ("Type", detail.event_type.clone().unwrap_or_default()),
        ("Location", detail.location.clone().unwrap_or_default()),
        ("Start", detail.start_date.clone().unwrap_or_default()),
        ("End", detail.end_date.clone().unwrap_or_default()),
        (
            "Players",
            detail
                .players_joined
                .map(|p| p.to_string())
                .unwrap_or_default(),
        ),
        (
            "Teams",
            detail
                .teams_joined
                .map(|t| t.to_string())
                .unwrap_or_default(),
        ),
        (
            "Challenges",
            detail.challenges.map(|c| c.to_string()).unwrap_or_default(),
        ),
        (
            "Max Team Size",
            detail
                .max_team_size
                .map(|m| m.to_string())
                .unwrap_or_default(),
        ),
        (
            "Prize Pool",
            detail.prize_pool.clone().unwrap_or_else(|| "-".into()),
        ),
    ];

    crate::output::print_detail(&detail, format, &fields);
    Ok(())
}

async fn challenges(
    event_id: u64,
    difficulty: Option<&str>,
    format: OutputFormat,
    cache: &Arc<Cache>,
) -> anyhow::Result<()> {
    let client = ctf_client(cache)?;
    let data = client.ctf().event_data(event_id).await?;

    let mut challenges = data.challenges;

    if let Some(diff_filter) = difficulty {
        challenges.retain(|c| {
            c.difficulty
                .as_deref()
                .is_some_and(|d| d.eq_ignore_ascii_case(diff_filter))
        });
    }

    if format != OutputFormat::Json {
        if let Some(team) = &data.participating_team {
            crate::output::print_message(&format!(
                "Team: {} | Rank: {} | Solved: {}/{} | Points: {}",
                team.name,
                team.rank
                    .map(|r| r.to_string())
                    .unwrap_or_else(|| "-".into()),
                team.solved_challenges.unwrap_or(0),
                team.total_challenges.unwrap_or(0),
                team.points.unwrap_or(0),
            ));
        }
    }

    crate::output::print_list(&challenges, format);
    Ok(())
}

async fn submit(challenge_id: u64, flag: &str, cache: &Arc<Cache>) -> anyhow::Result<()> {
    let client = ctf_client(cache)?;
    let result = client.ctf().submit_flag(challenge_id, flag).await?;
    if let Some(points) = result.points {
        crate::output::print_message(&format!("{} (+{} points)", result.message, points));
    } else {
        crate::output::print_message(&result.message);
    }
    Ok(())
}

async fn download(event_id: u64, challenge_id: u64, cache: &Arc<Cache>) -> anyhow::Result<()> {
    let client = ctf_client(cache)?;

    let data = client.ctf().event_data(event_id).await?;
    let challenge = data
        .challenges
        .iter()
        .find(|c| c.id == challenge_id)
        .ok_or_else(|| anyhow::anyhow!("Challenge {challenge_id} not found in event {event_id}"))?;

    let filename = match &challenge.filename {
        Some(f) => f.clone(),
        None => anyhow::bail!("No files available for this challenge."),
    };

    let bytes = client.ctf().download_file(challenge_id).await?;
    let safe_name = crate::sanitize_filename(&filename, &format!("{challenge_id}.zip"));
    std::fs::write(&safe_name, &bytes)?;
    crate::output::print_message(&format!("Downloaded {} ({} bytes)", safe_name, bytes.len()));
    Ok(())
}

async fn start(event_id: u64, challenge_id: u64, cache: &Arc<Cache>) -> anyhow::Result<()> {
    let client = ctf_client(cache)?;

    let data = client.ctf().event_data(event_id).await?;
    let challenge = data
        .challenges
        .iter()
        .find(|c| c.id == challenge_id)
        .ok_or_else(|| anyhow::anyhow!("Challenge {challenge_id} not found in event {event_id}"))?;

    if challenge.has_docker.unwrap_or(0) == 0 {
        anyhow::bail!("This challenge doesn't use a container.");
    }

    let resp = client.ctf().container_start(challenge_id).await?;
    crate::output::print_message(&resp.message);

    // Poll for container ready state
    for _ in 0..15 {
        tokio::time::sleep(Duration::from_secs(2)).await;
        let poll = client.ctf().event_data(event_id).await?;
        if let Some(c) = poll.challenges.iter().find(|c| c.id == challenge_id) {
            if c.docker_online.unwrap_or(0) > 0 {
                let host = c.hostname.as_deref().unwrap_or("unknown");
                let ports = c.docker_ports.as_deref().unwrap_or(&[]);
                if ports.is_empty() {
                    crate::output::print_message(&format!("Ready: {host}"));
                } else {
                    let port_str: Vec<_> = ports.iter().map(|p| p.to_string()).collect();
                    crate::output::print_message(&format!("Ready: {host}:{}", port_str.join(",")));
                }
                return Ok(());
            }
        }
    }

    crate::output::print_message("Container started but not ready yet. Check back shortly.");
    Ok(())
}

async fn stop(event_id: u64, challenge_id: u64, cache: &Arc<Cache>) -> anyhow::Result<()> {
    let client = ctf_client(cache)?;

    let data = client.ctf().event_data(event_id).await?;
    let challenge = data
        .challenges
        .iter()
        .find(|c| c.id == challenge_id)
        .ok_or_else(|| anyhow::anyhow!("Challenge {challenge_id} not found in event {event_id}"))?;

    if challenge.has_docker.unwrap_or(0) == 0 {
        anyhow::bail!("This challenge doesn't use a container.");
    }

    let resp = client.ctf().container_stop(challenge_id).await?;
    crate::output::print_message(&resp.message);
    Ok(())
}

async fn scoreboard(event_id: u64, format: OutputFormat, cache: &Arc<Cache>) -> anyhow::Result<()> {
    let client = ctf_client(cache)?;

    let menu = client.ctf().menu(event_id).await?;
    if menu.user_can_view_scoreboard == Some(0) {
        anyhow::bail!("Scoreboard is hidden for this event.");
    }

    let sb = client.ctf().scoreboard(event_id).await?;

    if format != OutputFormat::Json {
        if let Some(team) = &sb.participating_team {
            crate::output::print_message(&format!(
                "Your team: {} | Rank: {} | Points: {} | Flags: {} | Bloods: {}",
                team.name,
                team.position
                    .map(|p| p.to_string())
                    .unwrap_or_else(|| "-".into()),
                team.points.unwrap_or(0),
                team.owned_flags.unwrap_or(0),
                team.first_bloods.unwrap_or(0),
            ));
        }
    }

    // Add rank numbers to the score table
    let scores: Vec<_> = sb
        .scores
        .iter()
        .enumerate()
        .map(|(i, s)| RankedScore {
            rank: i as u32 + 1,
            score: s,
        })
        .collect();

    crate::output::print_list(&scores, format);
    Ok(())
}

struct RankedScore<'a> {
    rank: u32,
    score: &'a crate::models::ctf::CtfTeamScore,
}

impl serde::Serialize for RankedScore<'_> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.score.serialize(serializer)
    }
}

impl crate::output::Tabular for RankedScore<'_> {
    fn headers() -> Vec<&'static str> {
        vec!["#", "Team", "Country", "Points", "Flags", "Bloods"]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.rank.to_string(),
            self.score.name.clone(),
            self.score.country_code.clone().unwrap_or_default(),
            self.score.points.map(|p| p.to_string()).unwrap_or_default(),
            self.score
                .owned_flags
                .map(|f| f.to_string())
                .unwrap_or_default(),
            self.score
                .first_bloods
                .map(|b| b.to_string())
                .unwrap_or_default(),
        ]
    }
}

async fn solves(event_id: u64, format: OutputFormat, cache: &Arc<Cache>) -> anyhow::Result<()> {
    let client = ctf_client(cache)?;
    let solves = client.ctf().solves(event_id).await?;
    crate::output::print_list(&solves, format);
    Ok(())
}

async fn challenge_solves(
    challenge_id: u64,
    format: OutputFormat,
    cache: &Arc<Cache>,
) -> anyhow::Result<()> {
    let client = ctf_client(cache)?;
    let solves = client.ctf().challenge_solves(challenge_id).await?;
    crate::output::print_list(&solves, format);
    Ok(())
}
