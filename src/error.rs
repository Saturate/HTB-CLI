use std::fmt;

#[derive(Debug, thiserror::Error)]
pub enum HtbError {
    #[error("Not authenticated. Run `htb auth login` first.")]
    NotAuthenticated,

    #[error("API error ({status}): {message}")]
    Api { status: u16, message: String },

    #[error("Rate limited. Try again shortly.")]
    RateLimited,

    #[error("{0}")]
    Http(#[from] reqwest::Error),

    #[error("{0}")]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    Json(#[from] serde_json::Error),

    #[error("Config error: {0}")]
    Config(String),
}

#[derive(Debug, serde::Deserialize)]
pub struct ApiErrorBody {
    pub message: String,
}

impl fmt::Display for ApiErrorBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not_authenticated_display() {
        let err = HtbError::NotAuthenticated;
        assert_eq!(
            err.to_string(),
            "Not authenticated. Run `htb auth login` first."
        );
    }

    #[test]
    fn api_error_display() {
        let err = HtbError::Api {
            status: 403,
            message: "Incorrect flag".into(),
        };
        assert_eq!(err.to_string(), "API error (403): Incorrect flag");
    }
}
