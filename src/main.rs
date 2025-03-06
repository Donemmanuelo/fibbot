mod tests;
use octocrab::Octocrab;
use regex::Regex;
use std::env;
use std::error::Error;

use tests::lib::fibonacci;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // GitHub API token and repository details
    let github_token = env::var("GITHUB_TOKEN").unwrap_or_else(|_| "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".to_string());
    let repo_owner = "Donemmanuelo"; // Replace with your repository owner
    let repo_name = "fibbot";  // Replace with your repository name
    let pull_number = 1;               // Replace with your pull request number

    // Parameters
    let max_threshold: u128 = 100;      // Maximum number for Fibonacci computation
    let enable_fib: bool = true;       // Enable/disable Fibonacci computation and posting
    println!("GitHub Token: {}", &github_token);

    println!("Repository Owner: {}", repo_owner);
    println!("Repository Name: {}", repo_name);
    println!("Pull Request Number: {}", pull_number);

    let octocrab = Octocrab::builder()
    .personal_token(github_token.to_string())
    .build()?;

// Fetch your user information
//let user = octocrab.current().user().await?;
    // Fetch the pull request diff
    let pull_request = octocrab
        .pulls(repo_owner, repo_name)
        .get(pull_number)
        .await?;
    let diff_url = pull_request.diff_url.ok_or("No diff URL found")?;

    // Download the diff
    let diff_response = reqwest::get(diff_url).await?;
    let diff_text = diff_response.text().await?;

    // Extract numerical values from the diff
    let re = Regex::new(r"\b\d+\b")?;
    let mut numbers: Vec<u128> = re
        .find_iter(&diff_text)
        .filter_map(|m| m.as_str().parse().ok())
        .collect();

    // Fetch the pull request files
    let files = octocrab
        .pulls(repo_owner, repo_name)
        .list_files(pull_number)
        .await?;

    // Extract numerical values from all files
    for file in files {
        let file_url = file.raw_url.ok_or("No raw URL found for file")?;
        let file_response = reqwest::get(file_url).await?;
        let file_content = file_response.text().await?;

        // Extract numbers from file content
        let file_numbers: Vec<u128> = re
            .find_iter(&file_content)
            .filter_map(|m| m.as_str().parse().ok())
            .collect();
        numbers.extend(file_numbers);
    }

    // Calculate Fibonacci for each number (if enabled)
    if enable_fib {
        let mut comment_body = String::from("### Fibonacci Calculation Results\n");
        for number in numbers {
            if number <= max_threshold {
                let fib = fibonacci(number);
                comment_body.push_str(&format!("- Number `{}`: Fibonacci = `{}`\n", number, fib));
            } else {
                comment_body.push_str(&format!("- Number `{}`: Exceeds max threshold (`{}`)\n", number, max_threshold));
            }
        }

        // Post the comment to the pull request
        octocrab
            .issues(repo_owner, repo_name)
            .create_comment(pull_number, comment_body)
            .await?;

        println!("Comment posted successfully!");
    } else {
        println!("Fibonacci computation and posting are disabled.");
    }

    Ok(())
}
