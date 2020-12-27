use log::error;
use std::env;
use std::process::exit;

pub fn log_error_and_quit(msg: &str) -> ! {
    error!("{}", msg);
    exit(1);
}

#[inline]
pub fn get_env_or_default(var_name: &str, default: Option<&str>) -> String {
    match env::var(var_name)
        .ok()
        .or_else(|| default.map(String::from))
    {
        Some(v) => v,
        None => log_error_and_quit(&format!("Missing required env var {}.", var_name)),
    }
}

pub fn get_port() -> u16 {
    get_env_or_default("PORT", Some("5000"))
        .parse::<u16>()
        .expect("Invalid port number")
}

pub fn get_workers() -> usize {
    get_env_or_default("WORKERS", Some("1"))
        .parse::<usize>()
        .expect("Invalid worker count")
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::env::set_var;

    #[test]
    fn test_get_workers() {
        assert_eq!(get_workers(), 1);
        set_var("WORKERS", "3");
        assert_eq!(get_workers(), 3);
    }

    #[test]
    fn test_get_port() {
        assert_eq!(get_port(), 5000);
        set_var("PORT", "8000");
        assert_eq!(get_port(), 8000);
    }
}
