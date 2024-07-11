use tokio::io::AsyncWriteExt;
use tokio::process::Command;

use std::process::Stdio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::new("sort");

    println!("called the main function");
    // Specifying that we want pipe both the output and the input.
    // Similarly to capturing the output, by configuring the pipe
    // to stdin it can now be used as an asynchronous writer.
    cmd.stdout(Stdio::piped());
    cmd.stdin(Stdio::piped());

    let mut child = cmd.spawn().expect("failed to spawn command");

    // These are the animals we want to sort
    let animals: &[&str] = &["dog", "bird", "frog", "cat", "fish"];

    let mut stdin = child
        .stdin
        .take()
        .expect("child did not have a handle to stdin");

    // Write our animals to the child process
    // Note that the behavior of `sort` is to buffer _all input_ before writing any output.
    // In the general sense, it is recommended to write to the child in a separate task as
    // awaiting its exit (or output) to avoid deadlocks (for example, the child tries to write
    // some output but gets stuck waiting on the parent to read from it, meanwhile the parent
    // is stuck waiting to write its input completely before reading the output).
    stdin
        .write(animals.join("\n").as_bytes())
        .await
        .expect("could not write to stdin");

    // We drop the handle here which signals EOF to the child process.
    // This tells the child process that it there is no more data on the pipe.
    drop(stdin);

    let op = child.wait_with_output().await?;

    // Results should come back in sorted order
    println!("{:?}", &op.stdout);
    // assert_eq!(op.stdout, "bird\ncat\ndog\nfish\nfrog\n".as_bytes());

    Ok(())
}