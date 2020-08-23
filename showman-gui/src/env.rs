pub const APPLICATION_TITLE: &'static str = "ShowMan";

#[derive(Serialize)]
pub struct Environment {
    pub application_title: &'static str,
    pub api_hostname: String,
    pub auth_hostname: String,
    pub hostname: String
}

impl Environment {
    pub fn new() -> Result<Environment, std::env::VarError> {
        Ok(Environment {
            application_title: APPLICATION_TITLE,
            api_hostname: std::env::var("API_HOSTNAME")?,
            auth_hostname: std::env::var("AUTH_HOSTNAME")?,
            hostname: std::env::var("HOSTNAME")?
        })
    }
}