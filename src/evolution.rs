use crate::{
    config::Config,
    extract,
    ollama::{self, Message},
    repo, score, Result,
};
use rand::Rng;
use std::{process::Command, sync::Arc};

pub async fn evolve(config: Arc<Config>, source: &str) -> Result<()> {
    let mut accepted = false;
    println!("Step 1 - getting a score for the original program.");
    let score_original = score::evaluate(config.clone(), source, false).await?;
    println!("score_original = {}", score_original);
    while !accepted {
        let mutation = mutation(config.clone(), source).await?;
        println!("Step 3 - getting a score for the mutated program.");
        let score_mutated = score::evaluate(config.clone(), &mutation, true).await?;
        println!("score_mutated = {}", score_mutated);

        let number: i32 = rand::rng().random_range(1..=100);
        println!("number = {}", number);

        if score_mutated > score_original {
            if number >= 20 {
                accepted = true;
            }
        } else {
            if number <= 40 {
                accepted = true;
            }
        }
    }

    if accepted {
        println!("\n\nEverything ok. Cycle finished.\n\n");
    }

    Ok(())
}

pub async fn mutation(config: Arc<Config>, source: &str) -> Result<String> {
    println!("Step 2 - creating mutation.");
    let mut messages = vec![];

    let system_prompt = "You will get a library code written in Rust. You need to do only one thing. You can choose from the following options: 1 Create a new feature. 2 Improve test coverage. 3 Optimize code. You can add only one small thing - feature, test, or optimize the code. There is a preference to create a new feature 50%, add test 30%, optimize code 20%. Make sure you maintain a proper balance between new features and tests. You can use only Rust's standard library. Forget about using rand crate. You can not use crates. Make sure you create pub functions, otherwise, there will be warnings about not-used functions. Make sure to put code inside the ```rust``` block.";
    let message = Message {
        content: system_prompt.to_string(),
        role: "system".to_string(),
    };
    messages.push(message);

    if let Some(theme) = &config.theme {
        let prompt = format!("The program should follow this theme: {}", theme);
        let message = Message {
            content: prompt,
            role: "user".to_string(),
        };
        messages.push(message);
    }

    let prompt = format!("Here is the code:\n\n{}", source);
    let message = Message {
        content: prompt,
        role: "user".to_string(),
    };
    messages.push(message);

    let mut response = ollama::request(config.clone(), messages.clone()).await?;
    println!("response {:?}", response);
    let mut code = extract::extract_rust(&response).await?;
    println!("code {:?}", code);
    repo::write_source(&config.dir, &code).await?;

    let (mut test_passed, mut test_result) = test_mutation(config.clone()).await?;

    println!("test_passed = {}", test_passed);
    println!("test_result = {}", test_result);

    if !test_passed {
        let message = Message {
            content: response.clone(),
            role: "agent".to_string(),
        };
        messages.push(message);

        while !test_passed {
            let prompt = format!(
                "Build or test failed. Fix the problem. Remember that this is a library code. You can use only Rust's standard library. Forget about using rand crate. You can not use crates. Check this error:\n\n{}",
                test_result
            );
            let message = Message {
                content: prompt,
                role: "user".to_string(),
            };
            messages.push(message);

            response = ollama::request(config.clone(), messages.clone()).await?;
            println!("response {:?}", response);
            code = extract::extract_rust(&response).await?;
            println!("code {:?}", code);
            repo::write_source(&config.dir, &code).await?;

            (test_passed, test_result) = test_mutation(config.clone()).await?;

            println!("test_passed = {}", test_passed);
            println!("test_result = {}", test_result);
        }
    }

    Ok(code)
}

pub async fn test_mutation(config: Arc<Config>) -> Result<(bool, String)> {
    let build_command = format!("cd {} && cargo build", config.dir);
    let build_command_result = Command::new("sh")
        .arg("-c")
        .arg(build_command)
        .output()?
        .stderr;
    let build_command_result = String::from_utf8(build_command_result)?;

    if build_command_result.contains("error: could not compile") {
        return Ok((false, build_command_result));
    }

    let test_command = format!("cd {} && cargo test", config.dir);
    let test_command_result = Command::new("sh")
        .arg("-c")
        .arg(test_command)
        .output()?
        .stderr;
    let test_command_result = String::from_utf8(test_command_result)?;

    if test_command_result.contains("test result: FAILED")
        || test_command_result.contains("error: test failed")
        || test_command_result.contains("error: doctest failed")
        || test_command_result.contains("error: could not compile")
    {
        return Ok((false, test_command_result));
    }

    let mut result = String::new();
    result.push_str(&build_command_result);
    result.push_str("\n\n");
    result.push_str(&test_command_result);

    Ok((true, result))
}
