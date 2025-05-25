use serde::Serialize;
use crate::types::Ident;

pub type Yield = Vec<YieldItem>;

#[derive(Debug, Serialize)]
pub struct YieldItem {
    pub name: Ident,
    pub alias: Option<Ident>,
}