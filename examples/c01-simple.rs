use {{project-name}}::utils::greet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let msg = greet(Some("User"));
    println!("Message from greet: {}", msg);

    Ok(())
}
