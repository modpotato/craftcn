use anyhow::Result;
use colored::Colorize;
use dialoguer::Input;
use std::process::Command;

pub async fn handle_contribute() -> Result<()> {
    let contribution_url = "https://github.com/craftcn/cli/blob/main/CONTRIBUTING.md";

    println!();
    println!("{}", "CraftCN Contribution Guide".bold().cyan());
    println!("{}", "═".repeat(40).cyan());
    println!();
    println!(
        "{}",
        "Thank you for your interest in contributing to CraftCN!".cyan()
    );
    println!();
    println!(
        "{}",
        "Contribution Guide:".cyan(),
        contribution_url.yellow()
    );
    println!();
    println!("{}", "Quick Start:".bold());
    println!("  1. Fork the CraftCN repository");
    println!("  2. Create a new branch: git checkout -b feature/my-component");
    println!("  3. Add your component to: registry/components/<component-name>");
    println!("  4. Update registry/index.json with your component metadata");
    println!("  5. Test your changes locally");
    println!("  6. Commit and push: git push origin feature/my-component");
    println!("  7. Submit a Pull Request to: main CraftCN repository");
    println!();
    println!("{}", "─".repeat(40).cyan());
    println!();
    println!("{}", "Need help?".bold());
    println!("  Read the full contribution guide for detailed instructions");
    println!("  Ask questions in GitHub Discussions");

    #[cfg(target_os = "windows")]
    {
        let input = Input::with_theme(&dialoguer::theme::ColorfulTheme::default())
            .with_prompt("Press Enter to continue, or type 'open' to open guide in browser")
            .default("n".to_string())
            .interact()?;

        if input.to_lowercase() == "open"
            || input.to_lowercase() == "y"
            || input.to_lowercase() == "yes"
        {
            if let Err(e) = Command::new("cmd")
                .args(&["/C", "start", "", contribution_url])
                .status()
                .spawn()
            {
                eprintln!("{} Failed to open browser: {}", "✗".red(), e);
            }
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        let input = Input::with_theme(&dialoguer::theme::ColorfulTheme::default())
            .with_prompt("Press Enter to continue, or type 'open' to open guide in browser")
            .default("n".to_string())
            .interact()?;

        if input.to_lowercase() == "open"
            || input.to_lowercase() == "y"
            || input.to_lowercase() == "yes"
        {
            if let Err(e) = Command::new("xdg-open")
                .args(&[contribution_url])
                .status()
                .spawn()
            {
                eprintln!("{} Failed to open browser: {}", "✗".red(), e);
            }
        }
    }

    Ok(())
}
