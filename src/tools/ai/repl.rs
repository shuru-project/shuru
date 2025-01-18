use console::{style, Term};
use dialoguer::{theme::ColorfulTheme, Input, Select};
use spinners::{Spinner, Spinners};
use thiserror::Error;

use shuru::tools::ai::{
    client::{openai_client::OpenAIClient, AIClient},
    engine::{AIPlan, Action, ActionEngine, EngineError},
};

use super::context::Context;

#[derive(Error, Debug)]
pub enum ReplError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to initialize terminal: {0}")]
    Terminal(String),

    #[error("User input error: {0}")]
    Input(#[from] dialoguer::Error),

    #[error("AI client error: {0}")]
    AIClient(String),

    #[error("Engine error: {0}")]
    Engine(#[from] EngineError),

    #[error("Environment variable not found: {0}")]
    EnvVar(#[from] std::env::VarError),
}

pub type Result<T> = std::result::Result<T, ReplError>;

pub struct AIRepl {
    client: Box<dyn AIClient>,
    engine: ActionEngine,
    term: Term,
    theme: ColorfulTheme,
}

impl AIRepl {
    pub fn new(context: Context, client: Box<dyn AIClient>) -> Self {
        Self {
            client,
            engine: ActionEngine::new(context),
            term: Term::stdout(),
            theme: ColorfulTheme::default(),
        }
    }

    pub async fn start(&mut self) -> Result<std::process::ExitStatus> {
        self.term
            .clear_screen()
            .map_err(|e| ReplError::Terminal(e.to_string()))?;
        println!("{}", style("🤖 Shuru AI Assistant").bold().cyan());
        println!("I'll help you set up and manage your project. What would you like to do?\n");

        loop {
            let prompt: String = Input::with_theme(&self.theme)
                .with_prompt("→")
                .interact_text()?;

            if prompt == "exit" || prompt == "quit" {
                break;
            }

            let plan = {
                let mut sp = Spinner::new(Spinners::Dots12, "Thinking...".into());
                match self.client.generate_plan(&prompt).await {
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
            style(format!("{:?}", plan.action_type)).cyan()
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
            };
            println!("{}. {}", i + 1, action_str);
        }

        println!("\n{}", style("─".repeat(50)).dim());
        Ok(())
    }

    async fn modify_plan(&mut self, mut _plan: AIPlan) -> Result<()> {
        todo!()
    }
}

pub async fn start_ai_repl(
    config: Option<shuru::config::Config>,
) -> Result<std::process::ExitStatus> {
    let api_key = std::env::var("OPENAI_API_KEY")?;
    let client = OpenAIClient::new(api_key);
    let work_dir = std::env::current_dir()?;
    let context = Context::new(work_dir, config);

    let mut repl = AIRepl::new(context, Box::new(client));
    repl.start().await
}
