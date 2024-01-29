pub fn get(var_name: &str) -> anyhow::Result<String> {
    let var = std::env::var(var_name)?;

    Ok(var)
}

pub fn init() {
    dotenv::dotenv().ok();
}
