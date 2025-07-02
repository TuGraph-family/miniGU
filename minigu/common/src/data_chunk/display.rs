use arrow::util::display::{ArrayFormatter, FormatOptions};
use itertools::Itertools;
use tabled::builder::Builder;
use tabled::settings::{Style, Theme};

use super::DataChunk;
use crate::data_type::{DataSchema, DataSchemaRef};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum TableStyle {
    #[default]
    Sharp,
    Modern,
    Psql,
    Markdown,
    /// Csv with custom delimiter.
    Csv(u8),
    Json,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableOptions {
    style: TableStyle,
    type_info: bool,
    null_str: String,
}

impl Default for TableOptions {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl TableOptions {
    #[inline]
    pub fn new() -> Self {
        Self {
            style: TableStyle::default(),
            type_info: true,
            null_str: "".into(),
        }
    }

    #[inline]
    pub fn with_style(mut self, style: TableStyle) -> Self {
        self.style = style;
        self
    }

    #[inline]
    pub fn with_type_info(mut self, type_info: bool) -> Self {
        self.type_info = type_info;
        self
    }

    #[inline]
    pub fn with_null_str(mut self, null_str: String) -> Self {
        self.null_str = null_str;
        self
    }
}

#[derive(Debug)]
pub struct TableBuilder {
    options: TableOptions,
    has_header: bool,
    inner: TableBuilderInner,
}

#[derive(Debug)]
enum TableBuilderInner {
    Tabled(Builder),
    Csv {
        rows: Vec<Vec<String>>,
        delimiter: u8,
    },
    Json {
        rows: Vec<serde_json::Value>,
        col_schema: Vec<Vec<String>>,
    },
}

impl TableBuilderInner {
    fn append_header(&mut self, schema: &DataSchema, type_info: bool) {
        match self {
            TableBuilderInner::Tabled(builder) => {
                let header = schema.fields().iter().map(|f| {
                    if !type_info {
                        f.name().to_string()
                    } else {
                        format!("{}\n{}", f.name(), f.ty())
                    }
                });
                builder.push_record(header);
            }
            TableBuilderInner::Csv { rows, .. } => {
                let header: Vec<String> = schema
                    .fields()
                    .iter()
                    .map(|f| f.name().to_string())
                    .collect();
                rows.push(header);
            }
            TableBuilderInner::Json { col_schema, .. } => {
                let header = schema
                    .fields()
                    .iter()
                    .map(|f| f.name().to_string())
                    .collect();
                col_schema.push(header);
            }
        }
    }

    fn append_chunk(&mut self, chunk: &DataChunk, null_str: &str) {
        match self {
            TableBuilderInner::Tabled(builder) => {
                let options = FormatOptions::new().with_null(null_str);
                let formatters = chunk
                    .columns()
                    .iter()
                    .map(|c| {
                        ArrayFormatter::try_new(c, &options)
                            .expect("Column should be able to be formatted")
                    })
                    .collect_vec();
                for row in chunk.rows() {
                    let index = row.row_index();
                    builder.push_record(formatters.iter().map(|f| f.value(index).to_string()));
                }
            }
            TableBuilderInner::Csv { rows, .. } => {
                let options = FormatOptions::new().with_null(null_str);
                let formatters: Vec<ArrayFormatter> = chunk
                    .columns()
                    .iter()
                    .map(|c| {
                        ArrayFormatter::try_new(c, &options)
                            .expect("Column should be able to be formatted")
                    })
                    .collect();

                for row in chunk.rows() {
                    let index = row.row_index();
                    let record = formatters
                        .iter()
                        .map(|f| f.value(index).to_string())
                        .collect();
                    rows.push(record);
                }
            }
            TableBuilderInner::Json { rows, col_schema } => {
                let options = FormatOptions::new().with_null(null_str);
                let formatters = chunk
                    .columns()
                    .iter()
                    .map(|c| {
                        ArrayFormatter::try_new(c, &options)
                            .expect("Column should be able to be formatted")
                    })
                    .collect_vec();

                let field_names = col_schema[0].clone();

                for row in chunk.rows() {
                    let index = row.row_index();
                    let mut map = serde_json::Map::new();

                    for (i, f) in formatters.iter().enumerate() {
                        let field_name = &field_names[i];
                        map.insert(
                            field_name.clone(),
                            serde_json::Value::String(f.value(index).to_string()),
                        );
                    }

                    rows.push(serde_json::Value::Object(map));
                }
            }
        }
    }
}

impl TableBuilder {
    #[inline]
    pub fn new(schema: Option<DataSchemaRef>, options: TableOptions) -> Self {
        let mut inner = match options.style {
            TableStyle::Sharp | TableStyle::Modern | TableStyle::Psql | TableStyle::Markdown => {
                TableBuilderInner::Tabled(Builder::new())
            }
            TableStyle::Csv(delimiter) => TableBuilderInner::Csv {
                rows: vec![],
                delimiter,
            },
            TableStyle::Json => TableBuilderInner::Json {
                rows: vec![],
                col_schema: vec![],
            },
        };
        let has_header = schema.is_some();
        if let Some(schema) = schema {
            inner.append_header(&schema, options.type_info);
        }
        Self {
            options,
            has_header,
            inner,
        }
    }

    #[inline]
    pub fn append_chunk(mut self, chunk: &DataChunk) -> Self {
        self.inner.append_chunk(chunk, &self.options.null_str);
        self
    }

    #[inline]
    pub fn build(self) -> Table {
        match self.inner {
            TableBuilderInner::Tabled(builder) => {
                let mut table = builder.build();
                let mut theme = match self.options.style {
                    TableStyle::Sharp => Theme::from_style(Style::sharp()),
                    TableStyle::Modern => Theme::from_style(Style::modern()),
                    TableStyle::Psql => Theme::from_style(Style::psql()),
                    TableStyle::Markdown => Theme::from_style(Style::markdown()),
                    _ => unreachable!(),
                };
                match self.options.style {
                    TableStyle::Sharp => {
                        if !self.has_header || table.count_rows() == 1 {
                            theme.remove_horizontal_lines();
                        }
                    }
                    TableStyle::Psql | TableStyle::Markdown => {
                        if !self.has_header {
                            theme.remove_horizontal_lines();
                        }
                    }
                    _ => (),
                }
                table.with(theme);
                Table::Tabled(Box::new(table))
            }
            TableBuilderInner::Csv { rows, delimiter } => {
                let mut wrt = Vec::new();
                {
                    let mut writer = csv::WriterBuilder::new()
                        .delimiter(delimiter)
                        .from_writer(&mut wrt);
                    for row in rows {
                        writer.write_record(row).unwrap();
                    }
                    writer.flush().unwrap();
                }
                Table::Csv(String::from_utf8(wrt).unwrap())
            }
            TableBuilderInner::Json { rows, .. } => {
                Table::Json(serde_json::to_string_pretty(&rows).unwrap())
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Table {
    Tabled(Box<tabled::Table>),
    Csv(String),
    Json(String),
}

impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Table::Tabled(t) => write!(f, "{}", t),
            Table::Csv(s) | Table::Json(s) => write!(f, "{}", s),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use insta::assert_snapshot;

    use super::*;
    use crate::data_chunk;
    use crate::data_type::{DataField, LogicalType};

    fn build_test_schema() -> DataSchemaRef {
        Arc::new(DataSchema::new(vec![
            DataField::new("a".into(), LogicalType::Int32, false),
            DataField::new("b".into(), LogicalType::String, false),
        ]))
    }

    fn build_test_data_chunk() -> DataChunk {
        data_chunk!(
            { false, true, true },
            (Int32, [1, 2, 3]),
            (Utf8, ["abc", "def", "ghi"])
        )
    }

    #[test]
    fn test_table_without_header_sharp() {
        let options = TableOptions::new().with_style(TableStyle::Sharp);
        let table = TableBuilder::new(None, options)
            .append_chunk(&build_test_data_chunk())
            .build();
        assert_snapshot!(table, @r"
        ┌───┬─────┐
        │ 2 │ def │
        │ 3 │ ghi │
        └───┴─────┘
        ");
    }

    #[test]
    fn test_table_without_header_modern() {
        let options = TableOptions::new().with_style(TableStyle::Modern);
        let table = TableBuilder::new(None, options)
            .append_chunk(&build_test_data_chunk())
            .build();
        assert_snapshot!(table, @r"
        ┌───┬─────┐
        │ 2 │ def │
        ├───┼─────┤
        │ 3 │ ghi │
        └───┴─────┘
        ");
    }

    #[test]
    fn test_table_without_header_psql() {
        let options = TableOptions::new().with_style(TableStyle::Psql);
        let table = TableBuilder::new(None, options)
            .append_chunk(&build_test_data_chunk())
            .build();
        assert_snapshot!(table, @r"
        2 | def 
        3 | ghi
        ");
    }

    #[test]
    fn test_table_without_header_markdown() {
        let options = TableOptions::new().with_style(TableStyle::Markdown);
        let table = TableBuilder::new(None, options)
            .append_chunk(&build_test_data_chunk())
            .build();
        assert_snapshot!(table, @r"
        | 2 | def |
        | 3 | ghi |
        ");
    }

    #[test]
    fn test_table_without_data_sharp() {
        let schema = build_test_schema();
        let options = TableOptions::new().with_style(TableStyle::Sharp);
        let table = TableBuilder::new(Some(schema), options).build();
        assert_snapshot!(table, @r"
        ┌───────┬────────┐
        │ a     │ b      │
        │ int32 │ string │
        └───────┴────────┘
        ");
    }

    #[test]
    fn test_table_without_data_modern() {
        let schema = build_test_schema();
        let options = TableOptions::new().with_style(TableStyle::Modern);
        let table = TableBuilder::new(Some(schema), options).build();
        assert_snapshot!(table, @r"
        ┌───────┬────────┐
        │ a     │ b      │
        │ int32 │ string │
        └───────┴────────┘
        ");
    }

    #[test]
    fn test_table_without_data_psql() {
        let schema = build_test_schema();
        let options = TableOptions::new().with_style(TableStyle::Psql);
        let table = TableBuilder::new(Some(schema), options).build();
        assert_snapshot!(table, @r"
         a     | b      
         int32 | string 
        -------+--------
        ");
    }

    #[test]
    fn test_table_without_data_markdown() {
        let schema = build_test_schema();
        let options = TableOptions::new().with_style(TableStyle::Markdown);
        let table = TableBuilder::new(Some(schema), options).build();
        assert_snapshot!(table, @r"
        | a     | b      |
        | int32 | string |
        |-------|--------|
        ");
    }

    #[test]
    fn test_table_with_data_sharp() {
        let schema = build_test_schema();
        let options = TableOptions::new().with_style(TableStyle::Sharp);
        let table = TableBuilder::new(Some(schema), options)
            .append_chunk(&build_test_data_chunk())
            .build();
        assert_snapshot!(table, @r"
        ┌───────┬────────┐
        │ a     │ b      │
        │ int32 │ string │
        ├───────┼────────┤
        │ 2     │ def    │
        │ 3     │ ghi    │
        └───────┴────────┘
        ");
    }

    #[test]
    fn test_table_with_data_modern() {
        let schema = build_test_schema();
        let options = TableOptions::new().with_style(TableStyle::Modern);
        let table = TableBuilder::new(Some(schema), options)
            .append_chunk(&build_test_data_chunk())
            .build();
        assert_snapshot!(table, @r"
        ┌───────┬────────┐
        │ a     │ b      │
        │ int32 │ string │
        ├───────┼────────┤
        │ 2     │ def    │
        ├───────┼────────┤
        │ 3     │ ghi    │
        └───────┴────────┘
        ");
    }

    #[test]
    fn test_table_with_data_psql() {
        let schema = build_test_schema();
        let options = TableOptions::new().with_style(TableStyle::Psql);
        let table = TableBuilder::new(Some(schema), options)
            .append_chunk(&build_test_data_chunk())
            .build();
        assert_snapshot!(table, @r"
         a     | b      
         int32 | string 
        -------+--------
         2     | def    
         3     | ghi
        ");
    }

    #[test]
    fn test_table_with_data_markdown() {
        let schema = build_test_schema();
        let options = TableOptions::new().with_style(TableStyle::Markdown);
        let table = TableBuilder::new(Some(schema), options)
            .append_chunk(&build_test_data_chunk())
            .build();
        assert_snapshot!(table, @r"
        | a     | b      |
        | int32 | string |
        |-------|--------|
        | 2     | def    |
        | 3     | ghi    |
        ");
    }
    #[test]
    fn test_table_csv() {
        let schema = build_test_schema();
        let options = TableOptions::new()
            .with_style(TableStyle::Csv(b','))
            .with_type_info(false);

        let table = TableBuilder::new(Some(schema), options)
            .append_chunk(&build_test_data_chunk())
            .build();
        assert_snapshot!(table, @r"
        a,b
        2,def
        3,ghi
        ");
    }

    #[test]
    fn test_table_json() {
        let schema = build_test_schema();
        let options = TableOptions::new()
            .with_style(TableStyle::Json)
            .with_type_info(false);

        let table = TableBuilder::new(Some(schema), options)
            .append_chunk(&build_test_data_chunk())
            .build();

        println!("{}", table);
        assert_snapshot!(table, @r#"
[
  {
    "a": "2",
    "b": "def"
  },
  {
    "a": "3",
    "b": "ghi"
  }
]
        "#);
    }
}
