use anyhow::Result;

pub fn validate_email(email: &str) -> Result<()> {
    if email.is_empty() {
        return Err(anyhow::anyhow!("Email cannot be empty"));
    }

    if !email.contains('@') {
        return Err(anyhow::anyhow!("Email must contain @"));
    }

    if email.len() > 254 {
        return Err(anyhow::anyhow!("Email is too long"));
    }

    Ok(())
}

pub fn validate_password(password: &str) -> Result<()> {
    if password.len() < 8 {
        return Err(anyhow::anyhow!(
            "Password must be at least 8 characters long"
        ));
    }

    if password.len() > 128 {
        return Err(anyhow::anyhow!(
            "Password must be at most 128 characters long"
        ));
    }

    Ok(())
}

pub fn validate_display_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(anyhow::anyhow!("Display name cannot be empty"));
    }

    if name.len() > 100 {
        return Err(anyhow::anyhow!("Display name is too long"));
    }

    Ok(())
}
