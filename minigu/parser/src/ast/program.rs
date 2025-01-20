use super::session::SessionActivity;
use super::transaction::TransactionActivity;
use crate::macros::base;

#[apply(base)]
pub struct Program<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub activity: Option<ProgramActivity<'a>>,
    pub session_close: bool,
}

#[apply(base)]
pub enum ProgramActivity<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    Session(SessionActivity<'a>),

    #[cfg_attr(feature = "serde", serde(borrow))]
    Transaction(TransactionActivity<'a>),
}
