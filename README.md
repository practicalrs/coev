# Codex Evolutis

Many AI researchers want to use LLMs to generate working and practical applications. This project is different. I want to figure out what LLM can do on its own with only a little supervision pushed in a certain direction.

## What do I want to achieve?

The assumption is simple. We start with a basic Rust hello world library and tell LLM that we want to push it in a direction like:
- RPG game
- productivity tool
- new language compiler

## How do I want to achieve it?

The whole process will work in cycles. Each cycle will end with a commit to a repo with the program.

At the beginning of the cycle, code read from the repo is sent to LLM for evaluation in the following criteria:
- documentation
- efficiency
- features
- maintainability
- readability
- robustness
- security
- test coverage

Next, LLM is asked to do one of three things:
- create a new feature
- improve test coverage
- optimize code

**This part is called mutation.**

The code returned by LLM is checked to see if it builds and if all tests pass.

```sh
cargo build
```

```sh
cargo test
```

If the code doesn't build fine or one of the tests is not passing, LLM is asked to fix the code.

This process is repeated until LLM gives us a modified and working code.

When we have a working code, it is sent for evaluation. The score from this evaluation is compared with the score from the version from the beginning of the cycle.

If the score of the new code is higher than the score of the code from the beginning of the cycle, there are 80% chance that the code will be accepted. If the score of the new code is lower than the score of the code from the beginning of the cycle, there are 40% chance that the code will be accepted.

**This part is called natural selection.**

If the code is not accepted, the cycle starts from the beginning (after initial evaluation). If the code gets accepted, the cycle ends. At the end of the cycle, the whole generated code, together with a full log of messages from LLM in the project journal.

## The great unknown

I wonder what the code will look like after dozens, hundreds, and thousands of such cycles. I wonder what kind of abominations will different LLM models generate.

## Usage

Actually, there are no plans to publish this program on crates. (Unless there will be a higher demand, I don't see a point in polluting crates repo.) You need to download code from this repository and run it manually.

```sh
cargo run -- --dir=/home/michal/projects/test-lib2 --theme="AI lib" --model="qwen2.5-coder:3b"
```
