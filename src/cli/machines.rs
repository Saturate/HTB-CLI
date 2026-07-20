use clap::Subcommand;

use crate::api::HtbClient;
use crate::output::{self, OutputFormat};

#[derive(Subcommand)]
#[command(
    after_help = "Examples:\n  htb machines list --os linux          Linux machines only\n  htb machines list --difficulty easy    Easy machines only\n  htb machines info Bedside             Machine details\n  htb machines start Bedside            Spawn a machine\n  htb machines submit Bedside 'HTB{f}'  Submit a flag\n  htb machines active                   Current machine\n  htb machines todo                     Your todo list"
)]
pub enum MachineCommand {
    /// List machines
    List {
        #[arg(long, help = "Include retired machines")]
        retired: bool,
        #[arg(long, help = "Filter by OS (linux, windows)")]
        os: Option<String>,
        #[arg(long, help = "Filter by difficulty (easy, medium, hard, insane)")]
        difficulty: Option<String>,
        #[arg(long, help = "Page number")]
        page: Option<u32>,
        #[arg(long, help = "Fetch all pages")]
        all: bool,
    },
    /// Show machine details
    Info {
        /// Machine name or ID
        name_or_id: String,
    },
    /// Spawn a machine
    Start {
        /// Machine name or ID
        name_or_id: String,
    },
    /// Stop the active machine
    Stop,
    /// Reset a machine
    Reset {
        /// Machine name or ID
        name_or_id: String,
    },
    /// Submit a flag
    Submit {
        /// Machine name or ID
        name_or_id: String,
        /// The flag
        flag: String,
    },
    /// Show currently active machine
    Active,
    /// Manage todo list
    Todo {
        #[command(subcommand)]
        command: Option<TodoCommand>,
    },
}

#[derive(Subcommand)]
pub enum TodoCommand {
    /// Add a machine to todo list
    Add {
        /// Machine name or ID
        name_or_id: String,
    },
    /// Remove a machine from todo list
    Remove {
        /// Machine name or ID
        name_or_id: String,
    },
}

pub async fn handle(
    client: &HtbClient,
    cmd: MachineCommand,
    format: OutputFormat,
) -> anyhow::Result<()> {
    match cmd {
        MachineCommand::List {
            retired,
            os,
            difficulty,
            page,
            all,
        } => {
            if retired {
                tracing::warn!(
                    "--retired flag is not yet supported by the v5 API; showing all machines"
                );
            }
            let per_page = 100;
            let start_page = page.unwrap_or(1);

            let result = client.machines().list(start_page, per_page).await?;
            let mut machines = result.data;

            if all {
                let mut next = result.links.next;
                let mut current = start_page;
                while next.is_some() {
                    current += 1;
                    let page_result = client.machines().list(current, per_page).await?;
                    next = page_result.links.next;
                    machines.extend(page_result.data);
                }
            }

            if let Some(ref os_filter) = os {
                machines.retain(|m| m.os.eq_ignore_ascii_case(os_filter));
            }
            if let Some(ref diff_filter) = difficulty {
                machines.retain(|m| {
                    m.difficulty_text
                        .as_ref()
                        .is_some_and(|d| d.eq_ignore_ascii_case(diff_filter))
                });
            }

            output::print_list(&machines, format);
            if !all {
                output::print_pagination(
                    result.meta.current_page,
                    result.meta.last_page,
                    result.meta.total,
                );
            }
        }

        MachineCommand::Info { name_or_id } => {
            let machine = client.machines().profile(&name_or_id).await?;
            let fields = vec![
                ("ID", machine.id.to_string()),
                ("Name", machine.name.clone()),
                ("OS", machine.os.clone()),
                (
                    "Difficulty",
                    machine.difficulty_text.clone().unwrap_or_default(),
                ),
                (
                    "Rating",
                    machine
                        .rating
                        .map(|r| format!("{r:.1}"))
                        .unwrap_or_default(),
                ),
                ("Points", machine.points.to_string()),
                ("State", machine.state.clone().unwrap_or_default()),
                ("User Owns", machine.user_owns_count.to_string()),
                ("Root Owns", machine.root_owns_count.to_string()),
                ("IP", machine.ip.clone().unwrap_or_else(|| "-".into())),
                (
                    "Creator",
                    machine
                        .first_creator
                        .as_ref()
                        .map(|c| c.name.clone())
                        .unwrap_or_default(),
                ),
            ];
            output::print_detail(&machine, format, &fields);
        }

        MachineCommand::Start { name_or_id } => {
            let machine = client.machines().profile(&name_or_id).await?;
            let resp = client.machines().start(machine.id).await?;
            output::print_message(&resp.message);
        }

        MachineCommand::Stop => {
            let resp = client.machines().stop().await?;
            output::print_message(&resp.message);
        }

        MachineCommand::Reset { name_or_id } => {
            let machine = client.machines().profile(&name_or_id).await?;
            eprint!("Reset {}? [y/N] ", machine.name);
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            if input.trim().eq_ignore_ascii_case("y") {
                let resp = client.machines().reset(machine.id).await?;
                output::print_message(&resp.message);
            } else {
                output::print_message("Cancelled.");
            }
        }

        MachineCommand::Submit { name_or_id, flag } => {
            let machine = client.machines().profile(&name_or_id).await?;
            // difficulty field from the machine profile (0-100 scale)
            let difficulty = machine.difficulty.unwrap_or(50);
            let resp = client
                .machines()
                .submit_flag(machine.id, &flag, difficulty)
                .await?;
            output::print_message(&resp.message);
        }

        MachineCommand::Active => {
            let active = client.machines().active().await?;
            match active {
                Some(vm) => {
                    let fields = vec![
                        ("ID", vm.id.to_string()),
                        ("Name", vm.name.clone()),
                        ("Type", vm.vm_type.clone().unwrap_or_default()),
                        (
                            "Expires",
                            vm.expires_at.clone().unwrap_or_else(|| "-".into()),
                        ),
                    ];
                    output::print_detail(&vm, format, &fields);
                }
                None => output::print_message("No active machine."),
            }
        }

        MachineCommand::Todo { command } => match command {
            None => {
                let todos = client.machines().todo_list().await?;
                if todos.is_empty() {
                    output::print_message("No machines in todo list.");
                } else {
                    output::print_list(&todos, format);
                }
            }
            Some(TodoCommand::Add { name_or_id }) => {
                let machine = client.machines().profile(&name_or_id).await?;
                let resp = client.machines().todo_toggle(machine.id).await?;
                output::print_message(&resp.message);
            }
            Some(TodoCommand::Remove { name_or_id }) => {
                let machine = client.machines().profile(&name_or_id).await?;
                let resp = client.machines().todo_toggle(machine.id).await?;
                output::print_message(&resp.message);
            }
        },
    }
    Ok(())
}
