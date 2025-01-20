use logos::{Logos, SpannedIter};

use crate::error::UserError;

type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

pub(crate) struct Lexer<'a> {
    tokens: SpannedIter<'a, TokenKind<'a>>,
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(input: &'a str) -> Self {
        Self {
            tokens: TokenKind::lexer(input).spanned(),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Spanned<Token<'a>, usize, UserError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.tokens.next().map(|(kind, span)| {
            kind.map(|kind| {
                (
                    span.start,
                    Token {
                        kind,
                        slice: self.tokens.slice(),
                    },
                    span.end,
                )
            })
            .map_err(|()| UserError::InvalidToken(span.into()))
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Token<'a> {
    pub(crate) kind: TokenKind<'a>,
    pub(crate) slice: &'a str,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Logos)]
#[logos(skip r"[\p{White_Space}]+")]
pub(crate) enum TokenKind<'a> {
    #[token("abs", |_| ReservedWord::Abs, ignore(case))]
    #[token("acos", |_| ReservedWord::Acos, ignore(case))]
    #[token("all", |_| ReservedWord::All, ignore(case))]
    #[token("alldifferent", |_| ReservedWord::AllDifferent, ignore(case))]
    #[token("and", |_| ReservedWord::And, ignore(case))]
    #[token("any", |_| ReservedWord::Any, ignore(case))]
    #[token("array", |_| ReservedWord::Array, ignore(case))]
    #[token("as", |_| ReservedWord::As, ignore(case))]
    #[token("asc", |_| ReservedWord::Asc, ignore(case))]
    #[token("ascending", |_| ReservedWord::Ascending, ignore(case))]
    #[token("asin", |_| ReservedWord::Asin, ignore(case))]
    #[token("at", |_| ReservedWord::At, ignore(case))]
    #[token("atan", |_| ReservedWord::Atan, ignore(case))]
    #[token("avg", |_| ReservedWord::Avg, ignore(case))]
    #[token("big", |_| ReservedWord::Big, ignore(case))]
    #[token("bigint", |_| ReservedWord::Bigint, ignore(case))]
    #[token("binary", |_| ReservedWord::Binary, ignore(case))]
    #[token("bool", |_| ReservedWord::Bool, ignore(case))]
    #[token("boolean", |_| ReservedWord::Boolean, ignore(case))]
    #[token("both", |_| ReservedWord::Both, ignore(case))]
    #[token("btrim", |_| ReservedWord::Btrim, ignore(case))]
    #[token("by", |_| ReservedWord::By, ignore(case))]
    #[token("bytelength", |_| ReservedWord::ByteLength, ignore(case))]
    #[token("bytes", |_| ReservedWord::Bytes, ignore(case))]
    #[token("call", |_| ReservedWord::Call, ignore(case))]
    #[token("cardinality", |_| ReservedWord::Cardinality, ignore(case))]
    #[token("case", |_| ReservedWord::Case, ignore(case))]
    #[token("cast", |_| ReservedWord::Cast, ignore(case))]
    #[token("ceil", |_| ReservedWord::Ceil, ignore(case))]
    #[token("ceiling", |_| ReservedWord::Ceiling, ignore(case))]
    #[token("char", |_| ReservedWord::Char, ignore(case))]
    #[token("charlength", |_| ReservedWord::CharLength, ignore(case))]
    #[token("characterlength", |_| ReservedWord::CharacterLength, ignore(case))]
    #[token("characteristics", |_| ReservedWord::Characteristics, ignore(case))]
    #[token("close", |_| ReservedWord::Close, ignore(case))]
    #[token("coalesce", |_| ReservedWord::Coalesce, ignore(case))]
    #[token("collectlist", |_| ReservedWord::CollectList, ignore(case))]
    #[token("commit", |_| ReservedWord::Commit, ignore(case))]
    #[token("copy", |_| ReservedWord::Copy, ignore(case))]
    #[token("cos", |_| ReservedWord::Cos, ignore(case))]
    #[token("cosh", |_| ReservedWord::Cosh, ignore(case))]
    #[token("cot", |_| ReservedWord::Cot, ignore(case))]
    #[token("count", |_| ReservedWord::Count, ignore(case))]
    #[token("create", |_| ReservedWord::Create, ignore(case))]
    #[token("currentdate", |_| ReservedWord::CurrentDate, ignore(case))]
    #[token("currentgraph", |_| ReservedWord::CurrentGraph, ignore(case))]
    #[token("currentpropertygraph", |_| ReservedWord::CurrentPropertyGraph, ignore(case))]
    #[token("currentschema", |_| ReservedWord::CurrentSchema, ignore(case))]
    #[token("currenttime", |_| ReservedWord::CurrentTime, ignore(case))]
    #[token("currenttimestamp", |_| ReservedWord::CurrentTimestamp, ignore(case))]
    #[token("date", |_| ReservedWord::Date, ignore(case))]
    #[token("datetime", |_| ReservedWord::Datetime, ignore(case))]
    #[token("day", |_| ReservedWord::Day, ignore(case))]
    #[token("dec", |_| ReservedWord::Dec, ignore(case))]
    #[token("decimal", |_| ReservedWord::Decimal, ignore(case))]
    #[token("degrees", |_| ReservedWord::Degrees, ignore(case))]
    #[token("delete", |_| ReservedWord::Delete, ignore(case))]
    #[token("desc", |_| ReservedWord::Desc, ignore(case))]
    #[token("descending", |_| ReservedWord::Descending, ignore(case))]
    #[token("detach", |_| ReservedWord::Detach, ignore(case))]
    #[token("distinct", |_| ReservedWord::Distinct, ignore(case))]
    #[token("double", |_| ReservedWord::Double, ignore(case))]
    #[token("drop", |_| ReservedWord::Drop, ignore(case))]
    #[token("duration", |_| ReservedWord::Duration, ignore(case))]
    #[token("durationbetween", |_| ReservedWord::DurationBetween, ignore(case))]
    #[token("elementid", |_| ReservedWord::ElementId, ignore(case))]
    #[token("else", |_| ReservedWord::Else, ignore(case))]
    #[token("end", |_| ReservedWord::End, ignore(case))]
    #[token("except", |_| ReservedWord::Except, ignore(case))]
    #[token("exists", |_| ReservedWord::Exists, ignore(case))]
    #[token("exp", |_| ReservedWord::Exp, ignore(case))]
    #[token("false", |_| ReservedWord::False, ignore(case))]
    #[token("filter", |_| ReservedWord::Filter, ignore(case))]
    #[token("finish", |_| ReservedWord::Finish, ignore(case))]
    #[token("float", |_| ReservedWord::Float, ignore(case))]
    #[token("float16", |_| ReservedWord::Float16, ignore(case))]
    #[token("float32", |_| ReservedWord::Float32, ignore(case))]
    #[token("float64", |_| ReservedWord::Float64, ignore(case))]
    #[token("float128", |_| ReservedWord::Float128, ignore(case))]
    #[token("float256", |_| ReservedWord::Float256, ignore(case))]
    #[token("floor", |_| ReservedWord::Floor, ignore(case))]
    #[token("for", |_| ReservedWord::For, ignore(case))]
    #[token("from", |_| ReservedWord::From, ignore(case))]
    #[token("group", |_| ReservedWord::Group, ignore(case))]
    #[token("having", |_| ReservedWord::Having, ignore(case))]
    #[token("homegraph", |_| ReservedWord::HomeGraph, ignore(case))]
    #[token("homepropertygraph", |_| ReservedWord::HomePropertyGraph, ignore(case))]
    #[token("homeschema", |_| ReservedWord::HomeSchema, ignore(case))]
    #[token("hour", |_| ReservedWord::Hour, ignore(case))]
    #[token("if", |_| ReservedWord::If, ignore(case))]
    #[token("implies", |_| ReservedWord::Implies, ignore(case))]
    #[token("in", |_| ReservedWord::In, ignore(case))]
    #[token("insert", |_| ReservedWord::Insert, ignore(case))]
    #[token("int", |_| ReservedWord::Int, ignore(case))]
    #[token("integer", |_| ReservedWord::Integer, ignore(case))]
    #[token("int8", |_| ReservedWord::Int8, ignore(case))]
    #[token("integer8", |_| ReservedWord::Integer8, ignore(case))]
    #[token("int16", |_| ReservedWord::Int16, ignore(case))]
    #[token("integer16", |_| ReservedWord::Integer16, ignore(case))]
    #[token("int32", |_| ReservedWord::Int32, ignore(case))]
    #[token("interval", |_| ReservedWord::Interval, ignore(case))]
    #[token("is", |_| ReservedWord::Is, ignore(case))]
    #[token("integer32", |_| ReservedWord::Integer32, ignore(case))]
    #[token("int64", |_| ReservedWord::Int64, ignore(case))]
    #[token("integer64", |_| ReservedWord::Integer64, ignore(case))]
    #[token("int128", |_| ReservedWord::Int128, ignore(case))]
    #[token("integer128", |_| ReservedWord::Integer128, ignore(case))]
    #[token("int256", |_| ReservedWord::Int256, ignore(case))]
    #[token("integer256", |_| ReservedWord::Integer256, ignore(case))]
    #[token("intersect", |_| ReservedWord::Intersect, ignore(case))]
    #[token("leading", |_| ReservedWord::Leading, ignore(case))]
    #[token("left", |_| ReservedWord::Left, ignore(case))]
    #[token("let", |_| ReservedWord::Let, ignore(case))]
    #[token("like", |_| ReservedWord::Like, ignore(case))]
    #[token("limit", |_| ReservedWord::Limit, ignore(case))]
    #[token("list", |_| ReservedWord::List, ignore(case))]
    #[token("ln", |_| ReservedWord::Ln, ignore(case))]
    #[token("local", |_| ReservedWord::Local, ignore(case))]
    #[token("localdatetime", |_| ReservedWord::LocalDatetime, ignore(case))]
    #[token("localtime", |_| ReservedWord::LocalTime, ignore(case))]
    #[token("localtimestamp", |_| ReservedWord::LocalTimestamp, ignore(case))]
    #[token("log", |_| ReservedWord::Log, ignore(case))]
    #[token("log10", |_| ReservedWord::Log10, ignore(case))]
    #[token("lower", |_| ReservedWord::Lower, ignore(case))]
    #[token("ltrim", |_| ReservedWord::Ltrim, ignore(case))]
    #[token("match", |_| ReservedWord::Match, ignore(case))]
    #[token("max", |_| ReservedWord::Max, ignore(case))]
    #[token("min", |_| ReservedWord::Min, ignore(case))]
    #[token("minute", |_| ReservedWord::Minute, ignore(case))]
    #[token("mod", |_| ReservedWord::Mod, ignore(case))]
    #[token("month", |_| ReservedWord::Month, ignore(case))]
    #[token("next", |_| ReservedWord::Next, ignore(case))]
    #[token("nodetach", |_| ReservedWord::Nodetach, ignore(case))]
    #[token("normalize", |_| ReservedWord::Normalize, ignore(case))]
    #[token("not", |_| ReservedWord::Not, ignore(case))]
    #[token("nothing", |_| ReservedWord::Nothing, ignore(case))]
    #[token("null", |_| ReservedWord::Null, ignore(case))]
    #[token("nulls", |_| ReservedWord::Nulls, ignore(case))]
    #[token("nullif", |_| ReservedWord::Nullif, ignore(case))]
    #[token("octetlength", |_| ReservedWord::OctetLength, ignore(case))]
    #[token("of", |_| ReservedWord::Of, ignore(case))]
    #[token("offset", |_| ReservedWord::Offset, ignore(case))]
    #[token("optional", |_| ReservedWord::Optional, ignore(case))]
    #[token("or", |_| ReservedWord::Or, ignore(case))]
    #[token("order", |_| ReservedWord::Order, ignore(case))]
    #[token("otherwise", |_| ReservedWord::Otherwise, ignore(case))]
    #[token("parameter", |_| ReservedWord::Parameter, ignore(case))]
    #[token("parameters", |_| ReservedWord::Parameters, ignore(case))]
    #[token("path", |_| ReservedWord::Path, ignore(case))]
    #[token("pathlength", |_| ReservedWord::PathLength, ignore(case))]
    #[token("paths", |_| ReservedWord::Paths, ignore(case))]
    #[token("percentilecont", |_| ReservedWord::PercentileCont, ignore(case))]
    #[token("percentiledisc", |_| ReservedWord::PercentileDisc, ignore(case))]
    #[token("power", |_| ReservedWord::Power, ignore(case))]
    #[token("precision", |_| ReservedWord::Precision, ignore(case))]
    #[token("propertyexists", |_| ReservedWord::PropertyExists, ignore(case))]
    #[token("radians", |_| ReservedWord::Radians, ignore(case))]
    #[token("real", |_| ReservedWord::Real, ignore(case))]
    #[token("record", |_| ReservedWord::Record, ignore(case))]
    #[token("remove", |_| ReservedWord::Remove, ignore(case))]
    #[token("replace", |_| ReservedWord::Replace, ignore(case))]
    #[token("reset", |_| ReservedWord::Reset, ignore(case))]
    #[token("return", |_| ReservedWord::Return, ignore(case))]
    #[token("right", |_| ReservedWord::Right, ignore(case))]
    #[token("rollback", |_| ReservedWord::Rollback, ignore(case))]
    #[token("rtrim", |_| ReservedWord::Rtrim, ignore(case))]
    #[token("same", |_| ReservedWord::Same, ignore(case))]
    #[token("schema", |_| ReservedWord::Schema, ignore(case))]
    #[token("second", |_| ReservedWord::Second, ignore(case))]
    #[token("select", |_| ReservedWord::Select, ignore(case))]
    #[token("session", |_| ReservedWord::Session, ignore(case))]
    #[token("sessionuser", |_| ReservedWord::SessionUser, ignore(case))]
    #[token("set", |_| ReservedWord::Set, ignore(case))]
    #[token("signed", |_| ReservedWord::Signed, ignore(case))]
    #[token("sin", |_| ReservedWord::Sin, ignore(case))]
    #[token("sinh", |_| ReservedWord::Sinh, ignore(case))]
    #[token("size", |_| ReservedWord::Size, ignore(case))]
    #[token("skip", |_| ReservedWord::Skip, ignore(case))]
    #[token("small", |_| ReservedWord::Small, ignore(case))]
    #[token("smallint", |_| ReservedWord::Smallint, ignore(case))]
    #[token("sqrt", |_| ReservedWord::Sqrt, ignore(case))]
    #[token("start", |_| ReservedWord::Start, ignore(case))]
    #[token("stddevpop", |_| ReservedWord::StddevPop, ignore(case))]
    #[token("stddevsamp", |_| ReservedWord::StddevSamp, ignore(case))]
    #[token("string", |_| ReservedWord::String, ignore(case))]
    #[token("sum", |_| ReservedWord::Sum, ignore(case))]
    #[token("tan", |_| ReservedWord::Tan, ignore(case))]
    #[token("tanh", |_| ReservedWord::Tanh, ignore(case))]
    #[token("then", |_| ReservedWord::Then, ignore(case))]
    #[token("time", |_| ReservedWord::Time, ignore(case))]
    #[token("timestamp", |_| ReservedWord::Timestamp, ignore(case))]
    #[token("trailing", |_| ReservedWord::Trailing, ignore(case))]
    #[token("trim", |_| ReservedWord::Trim, ignore(case))]
    #[token("true", |_| ReservedWord::True, ignore(case))]
    #[token("typed", |_| ReservedWord::Typed, ignore(case))]
    #[token("ubigint", |_| ReservedWord::Ubigint, ignore(case))]
    #[token("uint", |_| ReservedWord::Uint, ignore(case))]
    #[token("uint8", |_| ReservedWord::Uint8, ignore(case))]
    #[token("uint16", |_| ReservedWord::Uint16, ignore(case))]
    #[token("uint32", |_| ReservedWord::Uint32, ignore(case))]
    #[token("uint64", |_| ReservedWord::Uint64, ignore(case))]
    #[token("uint128", |_| ReservedWord::Uint128, ignore(case))]
    #[token("uint256", |_| ReservedWord::Uint256, ignore(case))]
    #[token("union", |_| ReservedWord::Union, ignore(case))]
    #[token("unknown", |_| ReservedWord::Unknown, ignore(case))]
    #[token("unsigned", |_| ReservedWord::Unsigned, ignore(case))]
    #[token("upper", |_| ReservedWord::Upper, ignore(case))]
    #[token("use", |_| ReservedWord::Use, ignore(case))]
    #[token("usmallint", |_| ReservedWord::Usmallint, ignore(case))]
    #[token("value", |_| ReservedWord::Value, ignore(case))]
    #[token("varbinary", |_| ReservedWord::Varbinary, ignore(case))]
    #[token("varchar", |_| ReservedWord::Varchar, ignore(case))]
    #[token("variable", |_| ReservedWord::Variable, ignore(case))]
    #[token("when", |_| ReservedWord::When, ignore(case))]
    #[token("where", |_| ReservedWord::Where, ignore(case))]
    #[token("with", |_| ReservedWord::With, ignore(case))]
    #[token("xor", |_| ReservedWord::Xor, ignore(case))]
    #[token("year", |_| ReservedWord::Year, ignore(case))]
    #[token("yield", |_| ReservedWord::Yield, ignore(case))]
    #[token("zoned", |_| ReservedWord::Zoned, ignore(case))]
    #[token("zoneddatetime", |_| ReservedWord::ZonedDatetime, ignore(case))]
    #[token("zonedtime", |_| ReservedWord::ZonedTime, ignore(case))]
    #[token("abstract", |_| ReservedWord::Abstract, ignore(case))]
    #[token("aggregate", |_| ReservedWord::Aggregate, ignore(case))]
    #[token("aggregates", |_| ReservedWord::Aggregates, ignore(case))]
    #[token("alter", |_| ReservedWord::Alter, ignore(case))]
    #[token("catalog", |_| ReservedWord::Catalog, ignore(case))]
    #[token("clear", |_| ReservedWord::Clear, ignore(case))]
    #[token("clone", |_| ReservedWord::Clone, ignore(case))]
    #[token("constraint", |_| ReservedWord::Constraint, ignore(case))]
    #[token("currentrole", |_| ReservedWord::CurrentRole, ignore(case))]
    #[token("currentuser", |_| ReservedWord::CurrentUser, ignore(case))]
    #[token("data", |_| ReservedWord::Data, ignore(case))]
    #[token("directory", |_| ReservedWord::Directory, ignore(case))]
    #[token("dryrun", |_| ReservedWord::Dryrun, ignore(case))]
    #[token("exact", |_| ReservedWord::Exact, ignore(case))]
    #[token("existing", |_| ReservedWord::Existing, ignore(case))]
    #[token("function", |_| ReservedWord::Function, ignore(case))]
    #[token("gqlstatus", |_| ReservedWord::Gqlstatus, ignore(case))]
    #[token("grant", |_| ReservedWord::Grant, ignore(case))]
    #[token("instant", |_| ReservedWord::Instant, ignore(case))]
    #[token("infinity", |_| ReservedWord::Infinity, ignore(case))]
    #[token("number", |_| ReservedWord::Number, ignore(case))]
    #[token("numeric", |_| ReservedWord::Numeric, ignore(case))]
    #[token("on", |_| ReservedWord::On, ignore(case))]
    #[token("open", |_| ReservedWord::Open, ignore(case))]
    #[token("partition", |_| ReservedWord::Partition, ignore(case))]
    #[token("procedure", |_| ReservedWord::Procedure, ignore(case))]
    #[token("product", |_| ReservedWord::Product, ignore(case))]
    #[token("project", |_| ReservedWord::Project, ignore(case))]
    #[token("query", |_| ReservedWord::Query, ignore(case))]
    #[token("records", |_| ReservedWord::Records, ignore(case))]
    #[token("reference", |_| ReservedWord::Reference, ignore(case))]
    #[token("rename", |_| ReservedWord::Rename, ignore(case))]
    #[token("revoke", |_| ReservedWord::Revoke, ignore(case))]
    #[token("substring", |_| ReservedWord::Substring, ignore(case))]
    #[token("systemuser", |_| ReservedWord::SystemUser, ignore(case))]
    #[token("temporal", |_| ReservedWord::Temporal, ignore(case))]
    #[token("unique", |_| ReservedWord::Unique, ignore(case))]
    #[token("unit", |_| ReservedWord::Unit, ignore(case))]
    #[token("values", |_| ReservedWord::Values, ignore(case))]
    #[token("whitespace", |_| ReservedWord::Whitespace, ignore(case))]
    ReservedWord(ReservedWord),

    #[token("acyclic", |_| NonReservedWord::Acyclic, ignore(case))]
    #[token("binding", |_| NonReservedWord::Binding, ignore(case))]
    #[token("bindings", |_| NonReservedWord::Bindings, ignore(case))]
    #[token("connecting", |_| NonReservedWord::Connecting, ignore(case))]
    #[token("destination", |_| NonReservedWord::Destination, ignore(case))]
    #[token("different", |_| NonReservedWord::Different, ignore(case))]
    #[token("directed", |_| NonReservedWord::Directed, ignore(case))]
    #[token("edge", |_| NonReservedWord::Edge, ignore(case))]
    #[token("edges", |_| NonReservedWord::Edges, ignore(case))]
    #[token("element", |_| NonReservedWord::Element, ignore(case))]
    #[token("elements", |_| NonReservedWord::Elements, ignore(case))]
    #[token("first", |_| NonReservedWord::First, ignore(case))]
    #[token("graph", |_| NonReservedWord::Graph, ignore(case))]
    #[token("groups", |_| NonReservedWord::Groups, ignore(case))]
    #[token("keep", |_| NonReservedWord::Keep, ignore(case))]
    #[token("label", |_| NonReservedWord::Label, ignore(case))]
    #[token("labeled", |_| NonReservedWord::Labeled, ignore(case))]
    #[token("labels", |_| NonReservedWord::Labels, ignore(case))]
    #[token("last", |_| NonReservedWord::Last, ignore(case))]
    #[token("nfc", |_| NonReservedWord::Nfc, ignore(case))]
    #[token("nfd", |_| NonReservedWord::Nfd, ignore(case))]
    #[token("nfkc", |_| NonReservedWord::Nfkc, ignore(case))]
    #[token("nfkd", |_| NonReservedWord::Nfkd, ignore(case))]
    #[token("no", |_| NonReservedWord::No, ignore(case))]
    #[token("node", |_| NonReservedWord::Node, ignore(case))]
    #[token("normalized", |_| NonReservedWord::Normalized, ignore(case))]
    #[token("only", |_| NonReservedWord::Only, ignore(case))]
    #[token("ordinality", |_| NonReservedWord::Ordinality, ignore(case))]
    #[token("property", |_| NonReservedWord::Property, ignore(case))]
    #[token("read", |_| NonReservedWord::Read, ignore(case))]
    #[token("relationship", |_| NonReservedWord::Relationship, ignore(case))]
    #[token("relationships", |_| NonReservedWord::Relationships, ignore(case))]
    #[token("repeatable", |_| NonReservedWord::Repeatable, ignore(case))]
    #[token("shortest", |_| NonReservedWord::Shortest, ignore(case))]
    #[token("simple", |_| NonReservedWord::Simple, ignore(case))]
    #[token("source", |_| NonReservedWord::Source, ignore(case))]
    #[token("table", |_| NonReservedWord::Table, ignore(case))]
    #[token("temp", |_| NonReservedWord::Temp, ignore(case))]
    #[token("to", |_| NonReservedWord::To, ignore(case))]
    #[token("trail", |_| NonReservedWord::Trail, ignore(case))]
    #[token("transaction", |_| NonReservedWord::Transaction, ignore(case))]
    #[token("type", |_| NonReservedWord::Type, ignore(case))]
    #[token("undirected", |_| NonReservedWord::Undirected, ignore(case))]
    #[token("vertex", |_| NonReservedWord::Vertex, ignore(case))]
    #[token("walk", |_| NonReservedWord::Walk, ignore(case))]
    #[token("without", |_| NonReservedWord::Without, ignore(case))]
    #[token("write", |_| NonReservedWord::Write, ignore(case))]
    #[token("zone", |_| NonReservedWord::Zone, ignore(case))]
    NonReservedWord(NonReservedWord),

    // The followings are *delimiter tokens*.
    #[token("]->")]
    BracketRightArrow,
    #[token("]~>")]
    BracketTildeRightArrow,
    #[token("||")]
    Concatenation,
    #[token("::")]
    DoubleColon,
    #[token("$$")]
    DoubleDollar,
    #[token("--")]
    DoubleMinus,
    #[token("..")]
    DoublePeriod,
    #[token(">=")]
    GreaterThanOrEquals,
    #[token("<-")]
    LeftArrow,
    #[token("<~")]
    LeftArrowTilde,
    #[token("<-[")]
    LeftArrowBracket,
    #[token("<~[")]
    LeftArrowTildeBracket,
    #[token("<->")]
    LeftMinusRight,
    #[token("<-/")]
    LeftMinusSlash,
    #[token("<~/")]
    LeftTildeSlash,
    #[token("<=")]
    LessThanOrEquals,
    #[token("-[")]
    MinusLeftBracket,
    #[token("-/")]
    MinusSlash,
    #[token("<>")]
    NotEquals,
    #[token("->")]
    RightArrow,
    #[token("]-")]
    RightBracketMinus,
    #[token("]~")]
    RightBracketTilde,
    #[token("=>")]
    RightDoubleArrow,
    #[token("/-")]
    SlashMinus,
    #[token("/->")]
    SlashMinusRight,
    #[token("/~")]
    SlashTilde,
    #[token("/~>")]
    SlashTildeRight,
    #[token("~[")]
    TildeLeftBracket,
    #[token("~>")]
    TildeRightArrow,
    #[token("~/")]
    TildeSlash,
    #[token("//")]
    DoubleSolidus,

    // The followings are *GQL special characters*.
    #[token("&")]
    Ampersand,
    #[token("*")]
    Asterisk,
    #[token(":")]
    Colon,
    #[token(",")]
    Comma,
    #[token("@")]
    CommercialAt,
    #[token("$")]
    Dollar,
    #[token("\"")]
    DoubleQuote,
    #[token("=")]
    Equals,
    #[token("!")]
    Exclamation,
    #[token(">")]
    RightAngleBracket,
    #[token("`")]
    GraveAccent,
    #[token("{")]
    LeftBrace,
    #[token("[")]
    LeftBracket,
    #[token("(")]
    LeftParen,
    #[token("<")]
    LeftAngleBracket,
    #[token("-")]
    Minus,
    #[token("%")]
    Percent,
    #[token(".")]
    Period,
    #[token("+")]
    Plus,
    #[token("?")]
    QuestionMark,
    #[token("'")]
    Quote,
    #[token("\\")]
    ReverseSolidus,
    #[token("}")]
    RightBrace,
    #[token("]")]
    RightBracket,
    #[token(")")]
    RightParen,
    #[token("/")]
    Solidus,
    #[token("~")]
    Tilde,
    #[token("|")]
    VerticalBar,

    // The followings are identifiers and literals.
    #[regex(r"[\p{XID_Start}\p{Pc}][\p{XID_Continue}]*")]
    RegularIdent(&'a str),

    // The followings are quoted character sequences.
    #[regex(r#"'([^'\\]|(\\[\\'"`tbnrf])|(\\u[0-9a-fA-F]{4})|(\\U[0-9a-fA-F]{6})|'')*'"#, |lex| strip::<false>(lex.slice()))]
    SingleQuoted(&'a str),
    #[regex(r#"@'([^']|'')*'"#, |lex| strip::<true>(lex.slice()))]
    UnescapedSingleQuoted(&'a str),
    #[regex(r#""([^"\\]|(\\[\\'"`tbnrf])|(\\u[0-9a-fA-F]{4})|(\\U[0-9a-fA-F]{6})|"")*""#, |lex| strip::<false>(lex.slice()))]
    DoubleQuoted(&'a str),
    #[regex(r#"@"([^"]|"")*""#, |lex| strip::<true>(lex.slice()))]
    UnescapedDoubleQuoted(&'a str),
    #[regex(r#"`([^`\\]|(\\[\\'"`tbnrf])|(\\u[0-9a-fA-F]{4})|(\\U[0-9a-fA-F]{6})|``)*`"#, |lex| strip::<false>(lex.slice()))]
    AccentQuoted(&'a str),
    #[regex(r#"@`([^`]|``)*`"#, |lex| strip::<true>(lex.slice()))]
    UnescapedAccentQuoted(&'a str),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum ReservedWord {
    Abs,
    Acos,
    All,
    AllDifferent,
    And,
    Any,
    Array,
    As,
    Asc,
    Ascending,
    Asin,
    At,
    Atan,
    Avg,
    Big,
    Bigint,
    Binary,
    Bool,
    Boolean,
    Both,
    Btrim,
    By,
    ByteLength,
    Bytes,
    Call,
    Cardinality,
    Case,
    Cast,
    Ceil,
    Ceiling,
    Char,
    CharLength,
    CharacterLength,
    Characteristics,
    Close,
    Coalesce,
    CollectList,
    Commit,
    Copy,
    Cos,
    Cosh,
    Cot,
    Count,
    Create,
    CurrentDate,
    CurrentGraph,
    CurrentPropertyGraph,
    CurrentSchema,
    CurrentTime,
    CurrentTimestamp,
    Date,
    Datetime,
    Day,
    Dec,
    Decimal,
    Degrees,
    Delete,
    Desc,
    Descending,
    Detach,
    Distinct,
    Double,
    Drop,
    Duration,
    DurationBetween,
    ElementId,
    Else,
    End,
    Except,
    Exists,
    Exp,
    False,
    Filter,
    Finish,
    Float,
    Float16,
    Float32,
    Float64,
    Float128,
    Float256,
    Floor,
    For,
    From,
    Group,
    Having,
    HomeGraph,
    HomePropertyGraph,
    HomeSchema,
    Hour,
    If,
    Implies,
    In,
    Insert,
    Int,
    Integer,
    Int8,
    Integer8,
    Int16,
    Integer16,
    Int32,
    Interval,
    Is,
    Integer32,
    Int64,
    Integer64,
    Int128,
    Integer128,
    Int256,
    Integer256,
    Intersect,
    Leading,
    Left,
    Let,
    Like,
    Limit,
    List,
    Ln,
    Local,
    LocalDatetime,
    LocalTime,
    LocalTimestamp,
    Log,
    Log10,
    Lower,
    Ltrim,
    Match,
    Max,
    Min,
    Minute,
    Mod,
    Month,
    Next,
    Nodetach,
    Normalize,
    Not,
    Nothing,
    Null,
    Nulls,
    Nullif,
    OctetLength,
    Of,
    Offset,
    Optional,
    Or,
    Order,
    Otherwise,
    Parameter,
    Parameters,
    Path,
    PathLength,
    Paths,
    PercentileCont,
    PercentileDisc,
    Power,
    Precision,
    PropertyExists,
    Radians,
    Real,
    Record,
    Remove,
    Replace,
    Reset,
    Return,
    Right,
    Rollback,
    Rtrim,
    Same,
    Schema,
    Second,
    Select,
    Session,
    SessionUser,
    Set,
    Signed,
    Sin,
    Sinh,
    Size,
    Skip,
    Small,
    Smallint,
    Sqrt,
    Start,
    StddevPop,
    StddevSamp,
    String,
    Sum,
    Tan,
    Tanh,
    Then,
    Time,
    Timestamp,
    Trailing,
    Trim,
    True,
    Typed,
    Ubigint,
    Uint,
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Uint128,
    Uint256,
    Union,
    Unknown,
    Unsigned,
    Upper,
    Use,
    Usmallint,
    Value,
    Varbinary,
    Varchar,
    Variable,
    When,
    Where,
    With,
    Xor,
    Year,
    Yield,
    Zoned,
    ZonedDatetime,
    ZonedTime,

    // The followings are *pre-reserved words*.
    Abstract,
    Aggregate,
    Aggregates,
    Alter,
    Catalog,
    Clear,
    Clone,
    Constraint,
    CurrentRole,
    CurrentUser,
    Data,
    Directory,
    Dryrun,
    Exact,
    Existing,
    Function,
    Gqlstatus,
    Grant,
    Instant,
    Infinity,
    Number,
    Numeric,
    On,
    Open,
    Partition,
    Procedure,
    Product,
    Project,
    Query,
    Records,
    Reference,
    Rename,
    Revoke,
    Substring,
    SystemUser,
    Temporal,
    Unique,
    Unit,
    Values,
    Whitespace,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum NonReservedWord {
    Acyclic,
    Binding,
    Bindings,
    Connecting,
    Destination,
    Different,
    Directed,
    Edge,
    Edges,
    Element,
    Elements,
    First,
    Graph,
    Groups,
    Keep,
    Label,
    Labeled,
    Labels,
    Last,
    Nfc,
    Nfd,
    Nfkc,
    Nfkd,
    No,
    Node,
    Normalized,
    Only,
    Ordinality,
    Property,
    Read,
    Relationship,
    Relationships,
    Repeatable,
    Shortest,
    Simple,
    Source,
    Table,
    Temp,
    To,
    Trail,
    Transaction,
    Type,
    Undirected,
    Vertex,
    Walk,
    Without,
    Write,
    Zone,
}

/// Return the input with the first (or with '@' if `NO_ESCAPE`) and last characters removed.
fn strip<const NO_ESCAPE: bool>(input: &str) -> &str {
    // SAFETY: The length of `input` is guaranteed to be >= 3 (NO_ESCAPE) or >= 2 (otherwise).
    unsafe {
        if NO_ESCAPE {
            input.get_unchecked(2..(input.len() - 1))
        } else {
            input.get_unchecked(1..(input.len() - 1))
        }
    }
}
