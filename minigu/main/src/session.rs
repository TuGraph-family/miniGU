use gql_parser::parse_gql;

use crate::error::Result;

#[derive(Debug)]
pub struct Session {}

impl Session {
    pub fn query(&self, query: &str) -> Result<()> {
        let program = parse_gql(query)?;
        Ok(())
    }
}
