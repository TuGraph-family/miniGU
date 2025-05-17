use clap::{Parser, ValueEnum};
use minigu::common::data_chunk::display::TableStyle;
use strum::Display;

#[derive(Debug, Parser, ValueEnum, Clone, Copy, Display)]
#[strum(serialize_all = "kebab-case")]
pub enum OutputMode {
    Sharp,
    Modern,
    Psql,
    Markdown,
    Csv,
    Json,
}

impl From<OutputMode> for TableStyle {
    fn from(mode: OutputMode) -> Self {
        match mode {
            OutputMode::Sharp => TableStyle::Sharp,
            OutputMode::Modern => TableStyle::Modern,
            OutputMode::Psql => TableStyle::Psql,
            OutputMode::Markdown => TableStyle::Markdown,
            OutputMode::Csv => TableStyle::Csv(b','),
            OutputMode::Json => TableStyle::Json,
        }
    }
}
