use console::{style, Term};
use dialoguer::{theme::ColorfulTheme, Input, Select};
use spinners::{Spinner, Spinners};

use shuru_ai::{
    context::Context,
    client::{AIClient, client_factory::AIClientFactory},
    engine::ActionEngine,
    plan::{AIPlan, Action},
};

use shuru_core::{error::ReplError, global_config::ShuruGlobalConfig};

pub type Result<T> = std::result::Result<T, ReplError>;

pub struct AIRepl {
    client: Box<dyn AIClient>,
    engine: ActionEngine,
    term: Term,
    theme: ColorfulTheme,
}

impl AIRepl {
    pub fn new(engine: ActionEngine, client: Box<dyn AIClient>) -> Self {
        Self {
            client,
            engine,
            term: Term::stdout(),
            theme: ColorfulTheme::default(),
        }
    }

    pub async fn start(&mut self) -> Result<std::process::ExitStatus> {
        self.term
            .clear_screen()
            .map_err(|e| ReplError::Terminal(e.to_string()))?;
        println!("{}", style("👋 Hey there! I am Shuru AI!").bold().cyan());
        println!("I'm here to help you set up and manage your project. Let's get started! What would you like to do?\n");

        loop {
            let prompt: String = Input::with_theme(&self.theme)
                .with_prompt("→")
                .interact_text()?;

            if prompt == "/exit" || prompt == "/quit" {
                break;
            }

            if let Some(command_with_args) = prompt.strip_prefix("/run ") {
                let parts: Vec<String> = command_with_args
                    .split_whitespace()
                    .map(String::from)
                    .collect();

                let Some(command) = parts.first().cloned() else {
                    println!("Error: No command provided.");
                    continue;
                };

                let args = &parts[1..];

                match self.engine.run_command(&command, args).await {
                    Ok(_) => {
                        println!("Command executed successfully.");
                    }
                    Err(e) => {
                        println!("Failed to run the command. Error: {}", e);
                    }
                }
                continue;
            }

            let plan = {
                let mut sp = Spinner::new(Spinners::Dots8, "Thinking...".to_string());
                match self
                    .client
                    .generate_plan(&self.engine.context, &prompt)
                    .await
                {
                    Ok(plan) => {
                        sp.stop_and_persist("✓", "Got it! Here's what I suggest:".into());
                        plan
                    }
                    Err(e) => {
                        sp.stop_and_persist("✗", "Oops! Something went wrong.".into());
                        println!("Error: {}", e);
                        continue;
                    }
                }
            };

            self.display_plan(&plan)?;

            let choices = &["Execute plan", "Modify plan", "Start over", "Exit"];
            let selection = Select::with_theme(&self.theme)
                .with_prompt("What would you like to do?")
                .items(choices)
                .default(0)
                .interact()?;

            match selection {
                0 => {
                    self.engine.execute_plan_with_progress(plan).await?;
                }
                1 => {
                    self.modify_plan(plan).await?;
                }
                2 => continue,
                _ => break,
            }
        }

        println!("\nGoodbye! 👋");

        std::process::exit(0)
    }

    fn display_plan(&self, plan: &AIPlan) -> Result<()> {
        println!("\n{}", style("📋 Planned Actions:").bold());
        println!("{}\n", style("─".repeat(50)).dim());

        println!(
            "{}: {}",
            style("Type").bold().yellow(),
            style(format!("{:?}", plan.plan_type)).cyan()
        );

        println!(
            "{}: {}\n",
            style("Description").bold().yellow(),
            plan.description
        );

        for (i, action) in plan.actions.iter().enumerate() {
            let action_str = match action {
                Action::ChangeWorkDir { path, .. } => {
                    format!("📁 Change Work Directory: {}", style(path).green())
                }
                Action::CreateFile { path, .. } => {
                    format!("📝 Create file: {}", style(path).green())
                }
                Action::CreateDirectory { path } => {
                    format!("📁 Create directory: {}", style(path).green())
                }
                Action::InstallPackage { name, version, dev } => format!(
                    "📦 Install {}: {}{}",
                    if dev.unwrap_or(false) {
                        "dev package"
                    } else {
                        "package"
                    },
                    style(name).green(),
                    version
                        .as_ref()
                        .map_or("".to_string(), |v| format!("@{}", v))
                ),
                Action::AddShuruCommand { name, command, .. } => format!(
                    "🔧 Add command: {} ({})",
                    style(name).green(),
                    style(command).dim()
                ),
                Action::ModifyShuruConfig { .. } => "⚙️  Modify configuration".to_string(),
                Action::RunCommand { command, args } => {
                    format!(
                        "▶️  Run: {} {}",
                        style(command).green(),
                        style(args.join(" ")).dim()
                    )
                }
                Action::RunTask { task } => {
                    format!("▶️  Run Task: {}", style(task).green())
                }
            };
            println!("{}. {}", i + 1, action_str);
        }

        println!("\n{}", style("─".repeat(50)).dim());
        Ok(())
    }

    async fn modify_plan(&mut self, plan: AIPlan) -> Result<()> {
        let choices = &["Regenerate plan with feedback", "Cancel modifications"];

        let selection = Select::with_theme(&self.theme)
            .with_prompt("How would you like to modify the plan?")
            .items(choices)
            .default(0)
            .interact()?;

        match selection {
            0 => self.regenerate_plan_with_feedback(plan).await,
            _ => Ok(()),
        }
    }

    async fn regenerate_plan_with_feedback(&mut self, plan: AIPlan) -> Result<()> {
        let feedback: String = Input::with_theme(&self.theme)
            .with_prompt("Please provide feedback to refine the plan")
            .interact_text()?;

        let original_prompt = format!("{} (Original plan: {})", feedback, plan.description);

        let new_plan = match self
            .client
            .generate_plan(&self.engine.context, &original_prompt)
            .await
        {
            Ok(regenerated_plan) => regenerated_plan,
            Err(e) => {
                println!("Failed to regenerate plan: {}", e);
                return Ok(());
            }
        };

        self.display_plan(&new_plan)?;

        let choices = &["Execute new plan", "Modify new plan", "Start over"];
        let selection = Select::with_theme(&self.theme)
            .with_prompt("What would you like to do with the new plan?")
            .items(choices)
            .default(0)
            .interact()?;

        match selection {
            0 => {
                self.engine.execute_plan_with_progress(new_plan).await?;
            }
            1 => {
                Box::pin(self.modify_plan(new_plan)).await?;
            }
            _ => {}
        }

        Ok(())
    }
}

pub async fn start_ai_repl(
    config: Option<shuru_core::config::Config>,
) -> Result<std::process::ExitStatus> {
    let global_config = ShuruGlobalConfig::load()?;
    let client_factory = AIClientFactory::new(global_config);
    let client = client_factory
        .create_client(None)
        .map_err(|e| ReplError::AIClient(e.to_string()))?;

    let work_dir = std::env::current_dir()?;
    let npm_client = ActionEngine::detect_package_manager(&work_dir);
    let context = Context::new(work_dir, config, npm_client);
    let engine = ActionEngine::new(context);

    let mut repl = AIRepl::new(engine, client);
    repl.start().await
}
