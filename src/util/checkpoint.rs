// Function that writes the file to the checkpoint directory
pub fn write_checkpoint(checkpoint_dir: &str, content: &str) -> Result<(), Box<dyn std::error::Error>> {
    let checkpoint_file = format!("{}/complete.txt", checkpoint_dir);
    std::fs::write(&checkpoint_file, content)?;
    Ok(())
}