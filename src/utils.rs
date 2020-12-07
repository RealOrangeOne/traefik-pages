use std::env;

pub fn get_port() -> u16 {
    return env::var("PORT")
        .unwrap_or_else(|_| "5000".into())
        .parse::<u16>()
        .expect("Invalid port number");
}

pub fn get_workers() -> usize {
    return env::var("WORKERS")
        .unwrap_or_else(|_| "1".into())
        .parse::<usize>()
        .expect("Invalid worker count");
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
