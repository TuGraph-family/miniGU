use std::{env, fmt};

use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Clone, Copy, Default)]
pub struct TrackingIssue(Option<u32>);

impl TrackingIssue {
    pub fn new(issue: Option<u32>) -> Self {
        Self(issue)
    }
}

impl From<u32> for TrackingIssue {
    fn from(issue: u32) -> Self {
        Self(Some(issue))
    }
}

impl From<Option<u32>> for TrackingIssue {
    fn from(issue: Option<u32>) -> Self {
        Self(issue)
    }
}

impl fmt::Display for TrackingIssue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repo = env!("CARGO_PKG_REPOSITORY");
        if let Some(issue) = self.0 {
            write!(
                f,
                "see tracking issue for more information: {repo}/issues/{issue}"
            )
        } else {
            write!(
                f,
                "no tracking issue is opened yet. feel free to open one at {repo}/issues"
            )
        }
    }
}

#[derive(Debug, Clone, Error, Diagnostic)]
#[error("feature is not yet implemented: {feature}")]
pub struct NotImplemented {
    feature: String,
    #[help]
    issue: TrackingIssue,
}

impl NotImplemented {
    #[inline]
    pub fn new(feature: String, issue: TrackingIssue) -> Self {
        Self { feature, issue }
    }

    #[inline]
    pub fn feature(&self) -> &str {
        &self.feature
    }

    #[inline]
    pub fn issue(&self) -> &TrackingIssue {
        &self.issue
    }
}

#[inline]
pub fn not_implemented<T, E>(feature: impl Into<String>, issue: Option<u32>) -> Result<T, E>
where
    E: From<NotImplemented>,
{
    Err(E::from(NotImplemented::new(feature.into(), issue.into())))
}
