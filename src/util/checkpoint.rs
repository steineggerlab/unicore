// Function that writes the file to the checkpoint directory
pub fn write_checkpoint(checkpoint_file: &str, content: &str) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::write(checkpoint_file, content)?;
    Ok(())
}

pub fn read_checkpoint(checkpoint_file: &str) -> Result<String, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(checkpoint_file)?;
    Ok(content)
}