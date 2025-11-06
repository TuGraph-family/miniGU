use logos::{Lexer as LogosLexer, Logos, Skip};
use smol_str::SmolStr;

use crate::error::TokenErrorKind;
use crate::unescape::unescape;

#[derive(Debug, Clone, PartialEq, Eq, Logos)]
#[logos(error = TokenErrorKind)]
// Whitespaces should be skipped.
#[logos(skip r"[\p{White_Space}]+")]
// Simple comments introduced by double solidus.
#[logos(skip r"//[^\r\n]*")]
// Simple comments introduced by double minus.
#[logos(skip r"--[^\r\n]*")]
pub enum TokenKind<'a> {
    // The followings are *reserved words*.
    #[token("abs", ignore(case))]
    Abs,
    #[token("acos", ignore(case))]
    Acos,
    #[token("all", ignore(case))]
    All,
    #[token("all_different", ignore(case))]
    AllDifferent,
    #[token("and", ignore(case))]
    And,
    #[token("any", ignore(case))]
    Any,
    #[token("approximate", ignore(case))]
    Approximate,
    #[token("array", ignore(case))]
    Array,
    #[token("as", ignore(case))]
    As,
    #[token("asc", ignore(case))]
    Asc,
    #[token("ascending", ignore(case))]
    Ascending,
    #[token("asin", ignore(case))]
    Asin,
    #[token("at", ignore(case))]
    At,
    #[token("atan", ignore(case))]
    Atan,
    #[token("avg", ignore(case))]
    Avg,
    #[token("big", ignore(case))]
    Big,
    #[token("bigint", ignore(case))]
    Bigint,
    #[token("binary", ignore(case))]
    Binary,
    #[token("bool", ignore(case))]
    Bool,
    #[token("boolean", ignore(case))]
    Boolean,
    #[token("both", ignore(case))]
    Both,
    #[token("btrim", ignore(case))]
    Btrim,
    #[token("by", ignore(case))]
    By,
    #[token("byte_length", ignore(case))]
    ByteLength,
    #[token("bytes", ignore(case))]
    Bytes,
    #[token("call", ignore(case))]
    Call,
    #[token("cardinality", ignore(case))]
    Cardinality,
    #[token("case", ignore(case))]
    Case,
    #[token("cast", ignore(case))]
    Cast,
    #[token("ceil", ignore(case))]
    Ceil,
    #[token("ceiling", ignore(case))]
    Ceiling,
    #[token("char", ignore(case))]
    Char,
    #[token("char_length", ignore(case))]
    CharLength,
    #[token("character_length", ignore(case))]
    CharacterLength,
    #[token("characteristics", ignore(case))]
    Characteristics,
    #[token("close", ignore(case))]
    Close,
    #[token("coalesce", ignore(case))]
    Coalesce,
    #[token("collect_list", ignore(case))]
    CollectList,
    #[token("commit", ignore(case))]
    Commit,
    #[token("copy", ignore(case))]
    Copy,
    #[token("cos", ignore(case))]
    Cos,
    #[token("cosh", ignore(case))]
    Cosh,
    #[token("cot", ignore(case))]
    Cot,
    #[token("count", ignore(case))]
    Count,
    #[token("create", ignore(case))]
    Create,
    #[token("current_date", ignore(case))]
    CurrentDate,
    #[token("current_graph", ignore(case))]
    CurrentGraph,
    #[token("current_property_graph", ignore(case))]
    CurrentPropertyGraph,
    #[token("current_schema", ignore(case))]
    CurrentSchema,
    #[token("current_time", ignore(case))]
    CurrentTime,
    #[token("current_timestamp", ignore(case))]
    CurrentTimestamp,
    #[token("date", ignore(case))]
    Date,
    #[token("datetime", ignore(case))]
    Datetime,
    #[token("day", ignore(case))]
    Day,
    #[token("dec", ignore(case))]
    Dec,
    #[token("decimal", ignore(case))]
    Decimal,
    #[token("degrees", ignore(case))]
    Degrees,
    #[token("delete", ignore(case))]
    Delete,
    #[token("desc", ignore(case))]
    Desc,
    #[token("descending", ignore(case))]
    Descending,
    #[token("detach", ignore(case))]
    Detach,
    #[token("distinct", ignore(case))]
    Distinct,
    #[token("double", ignore(case))]
    Double,
    #[token("drop", ignore(case))]
    Drop,
    #[token("duration", ignore(case))]
    Duration,
    #[token("duration_between", ignore(case))]
    DurationBetween,
    #[token("element_id", ignore(case))]
    ElementId,
    #[token("else", ignore(case))]
    Else,
    #[token("end", ignore(case))]
    End,
    #[token("except", ignore(case))]
    Except,
    #[token("exists", ignore(case))]
    Exists,
    #[token("exp", ignore(case))]
    Exp,
    #[token("false", ignore(case))]
    False,
    #[token("filter", ignore(case))]
    Filter,
    #[token("finish", ignore(case))]
    Finish,
    #[token("float", ignore(case))]
    Float,
    #[token("float16", ignore(case))]
    Float16,
    #[token("float32", ignore(case))]
    Float32,
    #[token("float64", ignore(case))]
    Float64,
    #[token("float128", ignore(case))]
    Float128,
    #[token("float256", ignore(case))]
    Float256,
    #[token("floor", ignore(case))]
    Floor,
    #[token("for", ignore(case))]
    For,
    #[token("from", ignore(case))]
    From,
    #[token("group", ignore(case))]
    Group,
    #[token("having", ignore(case))]
    Having,
    #[token("home_graph", ignore(case))]
    HomeGraph,
    #[token("home_property_graph", ignore(case))]
    HomePropertyGraph,
    #[token("home_schema", ignore(case))]
    HomeSchema,
    #[token("hour", ignore(case))]
    Hour,
    #[token("if", ignore(case))]
    If,
    #[token("implies", ignore(case))]
    Implies,
    #[token("in", ignore(case))]
    In,
    #[token("insert", ignore(case))]
    Insert,
    #[token("int", ignore(case))]
    Int,
    #[token("integer", ignore(case))]
    Integer,
    #[token("int8", ignore(case))]
    Int8,
    #[token("integer8", ignore(case))]
    Integer8,
    #[token("int16", ignore(case))]
    Int16,
    #[token("integer16", ignore(case))]
    Integer16,
    #[token("int32", ignore(case))]
    Int32,
    #[token("interval", ignore(case))]
    Interval,
    #[token("is", ignore(case))]
    Is,
    #[token("integer32", ignore(case))]
    Integer32,
    #[token("int64", ignore(case))]
    Int64,
    #[token("integer64", ignore(case))]
    Integer64,
    #[token("int128", ignore(case))]
    Int128,
    #[token("integer128", ignore(case))]
    Integer128,
    #[token("int256", ignore(case))]
    Int256,
    #[token("integer256", ignore(case))]
    Integer256,
    #[token("intersect", ignore(case))]
    Intersect,
    #[token("leading", ignore(case))]
    Leading,
    #[token("left", ignore(case))]
    Left,
    #[token("let", ignore(case))]
    Let,
    #[token("like", ignore(case))]
    Like,
    #[token("limit", ignore(case))]
    Limit,
    #[token("list", ignore(case))]
    List,
    #[token("ln", ignore(case))]
    Ln,
    #[token("local", ignore(case))]
    Local,
    #[token("local_datetime", ignore(case))]
    LocalDatetime,
    #[token("local_time", ignore(case))]
    LocalTime,
    #[token("local_timestamp", ignore(case))]
    LocalTimestamp,
    #[token("log", ignore(case))]
    Log,
    #[token("log10", ignore(case))]
    Log10,
    #[token("lower", ignore(case))]
    Lower,
    #[token("ltrim", ignore(case))]
    Ltrim,
    #[token("match", ignore(case))]
    Match,
    #[token("max", ignore(case))]
    Max,
    #[token("min", ignore(case))]
    Min,
    #[token("minute", ignore(case))]
    Minute,
    #[token("mod", ignore(case))]
    Mod,
    #[token("month", ignore(case))]
    Month,
    #[token("next", ignore(case))]
    Next,
    #[token("nodetach", ignore(case))]
    Nodetach,
    #[token("normalize", ignore(case))]
    Normalize,
    #[token("not", ignore(case))]
    Not,
    #[token("nothing", ignore(case))]
    Nothing,
    #[token("null", ignore(case))]
    Null,
    #[token("nulls", ignore(case))]
    Nulls,
    #[token("nullif", ignore(case))]
    Nullif,
    #[token("octet_length", ignore(case))]
    OctetLength,
    #[token("of", ignore(case))]
    Of,
    #[token("offset", ignore(case))]
    Offset,
    #[token("optional", ignore(case))]
    Optional,
    #[token("or", ignore(case))]
    Or,
    #[token("order", ignore(case))]
    Order,
    #[token("otherwise", ignore(case))]
    Otherwise,
    #[token("parameter", ignore(case))]
    Parameter,
    #[token("parameters", ignore(case))]
    Parameters,
    #[token("path", ignore(case))]
    Path,
    #[token("path_length", ignore(case))]
    PathLength,
    #[token("paths", ignore(case))]
    Paths,
    #[token("percentile_cont", ignore(case))]
    PercentileCont,
    #[token("percentile_disc", ignore(case))]
    PercentileDisc,
    #[token("power", ignore(case))]
    Power,
    #[token("precision", ignore(case))]
    Precision,
    #[token("property_exists", ignore(case))]
    PropertyExists,
    #[token("radians", ignore(case))]
    Radians,
    #[token("real", ignore(case))]
    Real,
    #[token("record", ignore(case))]
    Record,
    #[token("remove", ignore(case))]
    Remove,
    #[token("replace", ignore(case))]
    Replace,
    #[token("reset", ignore(case))]
    Reset,
    #[token("return", ignore(case))]
    Return,
    #[token("right", ignore(case))]
    Right,
    #[token("rollback", ignore(case))]
    Rollback,
    #[token("rtrim", ignore(case))]
    Rtrim,
    #[token("same", ignore(case))]
    Same,
    #[token("schema", ignore(case))]
    Schema,
    #[token("second", ignore(case))]
    Second,
    #[token("select", ignore(case))]
    Select,
    #[token("session", ignore(case))]
    Session,
    #[token("session_user", ignore(case))]
    SessionUser,
    #[token("set", ignore(case))]
    Set,
    #[token("signed", ignore(case))]
    Signed,
    #[token("sin", ignore(case))]
    Sin,
    #[token("sinh", ignore(case))]
    Sinh,
    #[token("size", ignore(case))]
    Size,
    #[token("skip", ignore(case))]
    Skip,
    #[token("small", ignore(case))]
    Small,
    #[token("smallint", ignore(case))]
    Smallint,
    #[token("sqrt", ignore(case))]
    Sqrt,
    #[token("start", ignore(case))]
    Start,
    #[token("stddev_pop", ignore(case))]
    StddevPop,
    #[token("stddev_samp", ignore(case))]
    StddevSamp,
    #[token("string", ignore(case))]
    String,
    #[token("sum", ignore(case))]
    Sum,
    #[token("tan", ignore(case))]
    Tan,
    #[token("tanh", ignore(case))]
    Tanh,
    #[token("then", ignore(case))]
    Then,
    #[token("time", ignore(case))]
    Time,
    #[token("timestamp", ignore(case))]
    Timestamp,
    #[token("trailing", ignore(case))]
    Trailing,
    #[token("trim", ignore(case))]
    Trim,
    #[token("true", ignore(case))]
    True,
    #[token("typed", ignore(case))]
    Typed,
    #[token("ubigint", ignore(case))]
    Ubigint,
    #[token("uint", ignore(case))]
    Uint,
    #[token("uint8", ignore(case))]
    Uint8,
    #[token("uint16", ignore(case))]
    Uint16,
    #[token("uint32", ignore(case))]
    Uint32,
    #[token("uint64", ignore(case))]
    Uint64,
    #[token("uint128", ignore(case))]
    Uint128,
    #[token("uint256", ignore(case))]
    Uint256,
    #[token("union", ignore(case))]
    Union,
    #[token("unknown", ignore(case))]
    Unknown,
    #[token("unsigned", ignore(case))]
    Unsigned,
    #[token("upper", ignore(case))]
    Upper,
    #[token("use", ignore(case))]
    Use,
    #[token("usmallint", ignore(case))]
    Usmallint,
    #[token("value", ignore(case))]
    Value,
    #[token("varbinary", ignore(case))]
    Varbinary,
    #[token("varchar", ignore(case))]
    Varchar,
    #[token("variable", ignore(case))]
    Variable,
    #[token("vector", ignore(case))]
    Vector,
    #[token("vector_distance", ignore(case))]
    VectorDistance,
    #[token("when", ignore(case))]
    When,
    #[token("where", ignore(case))]
    Where,
    #[token("with", ignore(case))]
    With,
    #[token("xor", ignore(case))]
    Xor,
    #[token("year", ignore(case))]
    Year,
    #[token("yield", ignore(case))]
    Yield,
    #[token("zoned", ignore(case))]
    Zoned,
    #[token("zoned_datetime", ignore(case))]
    ZonedDatetime,
    #[token("zoned_time", ignore(case))]
    ZonedTime,

    // The followings are *pre-reserved words*.
    #[token("abstract", ignore(case))]
    Abstract,
    #[token("aggregate", ignore(case))]
    Aggregate,
    #[token("aggregates", ignore(case))]
    Aggregates,
    #[token("alter", ignore(case))]
    Alter,
    #[token("catalog", ignore(case))]
    Catalog,
    #[token("clear", ignore(case))]
    Clear,
    #[token("clone", ignore(case))]
    Clone,
    #[token("constraint", ignore(case))]
    Constraint,
    #[token("current_role", ignore(case))]
    CurrentRole,
    #[token("current_user", ignore(case))]
    CurrentUser,
    #[token("data", ignore(case))]
    Data,
    #[token("directory", ignore(case))]
    Directory,
    #[token("dryrun", ignore(case))]
    Dryrun,
    #[token("exact", ignore(case))]
    Exact,
    #[token("existing", ignore(case))]
    Existing,
    #[token("function", ignore(case))]
    Function,
    #[token("gqlstatus", ignore(case))]
    Gqlstatus,
    #[token("grant", ignore(case))]
    Grant,
    #[token("instant", ignore(case))]
    Instant,
    #[token("infinity", ignore(case))]
    Infinity,
    #[token("number", ignore(case))]
    Number,
    #[token("numeric", ignore(case))]
    Numeric,
    #[token("on", ignore(case))]
    On,
    #[token("open", ignore(case))]
    Open,
    #[token("partition", ignore(case))]
    Partition,
    #[token("procedure", ignore(case))]
    Procedure,
    #[token("product", ignore(case))]
    Product,
    #[token("project", ignore(case))]
    Project,
    #[token("query", ignore(case))]
    Query,
    #[token("records", ignore(case))]
    Records,
    #[token("reference", ignore(case))]
    Reference,
    #[token("rename", ignore(case))]
    Rename,
    #[token("revoke", ignore(case))]
    Revoke,
    #[token("substring", ignore(case))]
    Substring,
    #[token("system_user", ignore(case))]
    SystemUser,
    #[token("temporal", ignore(case))]
    Temporal,
    #[token("unique", ignore(case))]
    Unique,
    #[token("unit", ignore(case))]
    Unit,
    #[token("values", ignore(case))]
    Values,
    #[token("whitespace", ignore(case))]
    Whitespace,

    // The followings are *non-reserved words*.
    #[token("acyclic", ignore(case))]
    Acyclic,
    #[token("binding", ignore(case))]
    Binding,
    #[token("bindings", ignore(case))]
    Bindings,
    #[token("connecting", ignore(case))]
    Connecting,
    #[token("destination", ignore(case))]
    Destination,
    #[token("different", ignore(case))]
    Different,
    #[token("directed", ignore(case))]
    Directed,
    #[token("edge", ignore(case))]
    Edge,
    #[token("edges", ignore(case))]
    Edges,
    #[token("element", ignore(case))]
    Element,
    #[token("elements", ignore(case))]
    Elements,
    #[token("first", ignore(case))]
    First,
    #[token("graph", ignore(case))]
    Graph,
    #[token("groups", ignore(case))]
    Groups,
    #[token("keep", ignore(case))]
    Keep,
    #[token("label", ignore(case))]
    Label,
    #[token("labeled", ignore(case))]
    Labeled,
    #[token("labels", ignore(case))]
    Labels,
    #[token("last", ignore(case))]
    Last,
    #[token("nfc", ignore(case))]
    Nfc,
    #[token("nfd", ignore(case))]
    Nfd,
    #[token("nfkc", ignore(case))]
    Nfkc,
    #[token("nfkd", ignore(case))]
    Nfkd,
    #[token("no", ignore(case))]
    No,
    #[token("node", ignore(case))]
    Node,
    #[token("normalized", ignore(case))]
    Normalized,
    #[token("only", ignore(case))]
    Only,
    #[token("ordinality", ignore(case))]
    Ordinality,
    #[token("property", ignore(case))]
    Property,
    #[token("read", ignore(case))]
    Read,
    #[token("relationship", ignore(case))]
    Relationship,
    #[token("relationships", ignore(case))]
    Relationships,
    #[token("repeatable", ignore(case))]
    Repeatable,
    #[token("shortest", ignore(case))]
    Shortest,
    #[token("simple", ignore(case))]
    Simple,
    #[token("source", ignore(case))]
    Source,
    #[token("table", ignore(case))]
    Table,
    #[token("to", ignore(case))]
    To,
    #[token("trail", ignore(case))]
    Trail,
    #[token("transaction", ignore(case))]
    Transaction,
    #[token("type", ignore(case))]
    Type,
    #[token("undirected", ignore(case))]
    Undirected,
    #[token("vertex", ignore(case))]
    Vertex,
    #[token("walk", ignore(case))]
    Walk,
    #[token("without", ignore(case))]
    Without,
    #[token("write", ignore(case))]
    Write,
    #[token("zone", ignore(case))]
    Zone,

    // The followings are *delimiter tokens*.
    #[token("]->")]
    BracketRightArrow,
    #[token("]~>")]
    BracketTildeRightArrow,
    #[token("||")]
    Concatenation,
    #[token("::")]
    DoubleColon,
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

    // The followings are *GQL special characters*.
    #[token("|+|")]
    Alternation,
    #[token("&")]
    Ampersand,
    #[token("*")]
    Asterisk,
    #[token(":")]
    Colon,
    #[token(",")]
    Comma,
    #[token("=")]
    Equals,
    #[token("!")]
    Exclamation,
    #[token(">")]
    RightAngleBracket,
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
    RegularIdentifier(&'a str),
    #[token("$", handle_parameter)]
    GeneralParameterReference(ParameterName<'a>),
    #[token("$$", handle_parameter)]
    SubstitutedParameterReference(ParameterName<'a>),
    #[regex(r"[0-9](_?[0-9])*")]
    UnsignedDecimalInteger(&'a str),
    #[regex(r"0o(_?[0-7])+")]
    UnsignedOctalInteger(&'a str),
    #[regex(r"0x(_?[0-9a-fA-F])+")]
    UnsignedHexInteger(&'a str),
    #[regex(r"0b(_?[01])+")]
    UnsignedBinaryInteger(&'a str),
    #[regex(r"((?:[0-9]+\.[0-9]*|\.[0-9]+)([eE][+-]?[0-9]+)?|[0-9]+[eE][+-]?[0-9]+)")]
    UnsignedFloatLiteral(&'a str),

    // The followings are quoted character sequences.
    #[regex(r#"'|@'"#, handle_quoted)]
    SingleQuoted(Quoted<'a>),
    #[regex(r#""|@"#, handle_quoted)]
    DoubleQuoted(Quoted<'a>),
    #[regex(r#"`|@`"#, handle_quoted)]
    AccentQuoted(Quoted<'a>),

    // Bracketed comments. This token should never be produced.
    #[regex(r"/\*", handle_comment)]
    _BracketedComment,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Logos)]
#[logos(error = TokenErrorKind)]
pub enum ParameterName<'a> {
    #[regex(r#""|`|@"|@`"#, handle_quoted)]
    Delimited(Quoted<'a>),
    #[regex(r"[\p{XID_Continue}]+")]
    Extended(&'a str),
}

impl ParameterName<'_> {
    pub fn unescape(&self) -> Option<SmolStr> {
        match self {
            Self::Delimited(quoted) => quoted.unescape(),
            Self::Extended(s) => Some(SmolStr::new(s)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Logos)]
#[logos(error = TokenErrorKind)]
pub enum Quoted<'a> {
    #[regex(r#"'([^'\\]|(\\[\\'"`tbnrf])|(\\u[0-9a-fA-F]{4})|(\\U[0-9a-fA-F]{6})|'')*'"#, |lex| strip::<false>(lex.slice()))]
    Single(&'a str),
    #[regex(r#""([^"\\]|(\\[\\'"`tbnrf])|(\\u[0-9a-fA-F]{4})|(\\U[0-9a-fA-F]{6})|"")*""#, |lex| strip::<false>(lex.slice()))]
    Double(&'a str),
    #[regex(r#"`([^`\\]|(\\[\\'"`tbnrf])|(\\u[0-9a-fA-F]{4})|(\\U[0-9a-fA-F]{6})|``)*`"#, |lex| strip::<false>(lex.slice()))]
    Accent(&'a str),
    #[regex(r#"@'([^']|'')*'"#, |lex| strip::<true>(lex.slice()))]
    UnescapedSingle(&'a str),
    #[regex(r#"@"([^"]|"")*""#, |lex| strip::<true>(lex.slice()))]
    UnescapedDouble(&'a str),
    #[regex(r#"@`([^`]|``)*`"#, |lex| strip::<true>(lex.slice()))]
    UnescapedAccent(&'a str),
}

impl Quoted<'_> {
    pub fn unescape(&self) -> Option<SmolStr> {
        match self {
            Self::Single(s) => unescape::<'\'', false>(s),
            Self::Double(s) => unescape::<'"', false>(s),
            Self::Accent(s) => unescape::<'`', false>(s),
            Self::UnescapedSingle(s) => unescape::<'\'', true>(s),
            Self::UnescapedDouble(s) => unescape::<'"', true>(s),
            Self::UnescapedAccent(s) => unescape::<'`', true>(s),
        }
    }
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

fn handle_comment<'a>(lex: &mut LogosLexer<'a, TokenKind<'a>>) -> Result<Skip, TokenErrorKind> {
    let remainder = lex.remainder();
    if let Some(len) = remainder.find("*/") {
        lex.bump(len + 2);
        Ok(Skip)
    } else {
        lex.bump(remainder.len());
        Err(TokenErrorKind::IncompleteComment)
    }
}

fn handle_quoted<'a, T>(lex: &mut LogosLexer<'a, T>) -> Result<Quoted<'a>, TokenErrorKind>
where
    T: Logos<'a, Source = str>,
{
    let span = lex.span();
    // SAFETY: `input` should be valid.
    let input = unsafe { lex.source().get_unchecked(span.start..) };
    let mut quoted_lex = Quoted::lexer(input);
    // SAFETY: `input` should have at least `span.len()` character.
    let token = unsafe { quoted_lex.next().unwrap_unchecked() };
    lex.bump(quoted_lex.span().len() - span.len());
    token
}

fn handle_parameter<'a>(
    lex: &mut LogosLexer<'a, TokenKind<'a>>,
) -> Result<ParameterName<'a>, TokenErrorKind> {
    let mut param_lex = ParameterName::lexer(lex.remainder());
    let token = param_lex.next();
    lex.bump(param_lex.span().len());
    token.ok_or(TokenErrorKind::InvalidToken)?
}

impl TokenKind<'_> {
    #[inline]
    pub fn is_prefix_of_unsigned_integer(&self) -> bool {
        matches!(
            self,
            Self::UnsignedDecimalInteger(_)
                | Self::UnsignedOctalInteger(_)
                | Self::UnsignedHexInteger(_)
                | Self::UnsignedBinaryInteger(_)
        )
    }

    #[inline]
    pub fn is_prefix_of_unsigned_float(&self) -> bool {
        matches!(self, Self::UnsignedFloatLiteral(_))
    }

    #[inline]
    pub fn is_prefix_of_numeric_literal(&self) -> bool {
        self.is_prefix_of_unsigned_integer() || self.is_prefix_of_unsigned_float()
    }

    #[inline]
    pub fn is_reserved_word(&self) -> bool {
        matches!(
            self,
            Self::Abs
                | Self::Acos
                | Self::All
                | Self::AllDifferent
                | Self::And
                | Self::Any
                | Self::Array
                | Self::As
                | Self::Asc
                | Self::Ascending
                | Self::Asin
                | Self::At
                | Self::Atan
                | Self::Avg
                | Self::Big
                | Self::Bigint
                | Self::Binary
                | Self::Bool
                | Self::Boolean
                | Self::Both
                | Self::Btrim
                | Self::By
                | Self::ByteLength
                | Self::Bytes
                | Self::Call
                | Self::Cardinality
                | Self::Case
                | Self::Cast
                | Self::Ceil
                | Self::Ceiling
                | Self::Char
                | Self::CharLength
                | Self::CharacterLength
                | Self::Characteristics
                | Self::Close
                | Self::Coalesce
                | Self::CollectList
                | Self::Commit
                | Self::Copy
                | Self::Cos
                | Self::Cosh
                | Self::Cot
                | Self::Count
                | Self::Create
                | Self::CurrentDate
                | Self::CurrentGraph
                | Self::CurrentPropertyGraph
                | Self::CurrentSchema
                | Self::CurrentTime
                | Self::CurrentTimestamp
                | Self::Date
                | Self::Datetime
                | Self::Day
                | Self::Dec
                | Self::Decimal
                | Self::Degrees
                | Self::Delete
                | Self::Desc
                | Self::Descending
                | Self::Detach
                | Self::Distinct
                | Self::Double
                | Self::Drop
                | Self::Duration
                | Self::DurationBetween
                | Self::ElementId
                | Self::Else
                | Self::End
                | Self::Except
                | Self::Exists
                | Self::Exp
                | Self::False
                | Self::Filter
                | Self::Finish
                | Self::Float
                | Self::Float16
                | Self::Float32
                | Self::Float64
                | Self::Float128
                | Self::Float256
                | Self::Floor
                | Self::For
                | Self::From
                | Self::Group
                | Self::Having
                | Self::HomeGraph
                | Self::HomePropertyGraph
                | Self::HomeSchema
                | Self::Hour
                | Self::If
                | Self::Implies
                | Self::In
                | Self::Insert
                | Self::Int
                | Self::Integer
                | Self::Int8
                | Self::Integer8
                | Self::Int16
                | Self::Integer16
                | Self::Int32
                | Self::Interval
                | Self::Is
                | Self::Integer32
                | Self::Int64
                | Self::Integer64
                | Self::Int128
                | Self::Integer128
                | Self::Int256
                | Self::Integer256
                | Self::Intersect
                | Self::Leading
                | Self::Left
                | Self::Let
                | Self::Like
                | Self::Limit
                | Self::List
                | Self::Ln
                | Self::Local
                | Self::LocalDatetime
                | Self::LocalTime
                | Self::LocalTimestamp
                | Self::Log
                | Self::Log10
                | Self::Lower
                | Self::Ltrim
                | Self::Match
                | Self::Max
                | Self::Min
                | Self::Minute
                | Self::Mod
                | Self::Month
                | Self::Next
                | Self::Nodetach
                | Self::Normalize
                | Self::Not
                | Self::Nothing
                | Self::Null
                | Self::Nulls
                | Self::Nullif
                | Self::OctetLength
                | Self::Of
                | Self::Offset
                | Self::Optional
                | Self::Or
                | Self::Order
                | Self::Otherwise
                | Self::Parameter
                | Self::Parameters
                | Self::Path
                | Self::PathLength
                | Self::Paths
                | Self::PercentileCont
                | Self::PercentileDisc
                | Self::Power
                | Self::Precision
                | Self::PropertyExists
                | Self::Radians
                | Self::Real
                | Self::Record
                | Self::Remove
                | Self::Replace
                | Self::Reset
                | Self::Return
                | Self::Right
                | Self::Rollback
                | Self::Rtrim
                | Self::Same
                | Self::Schema
                | Self::Second
                | Self::Select
                | Self::Session
                | Self::SessionUser
                | Self::Set
                | Self::Signed
                | Self::Sin
                | Self::Sinh
                | Self::Size
                | Self::Skip
                | Self::Small
                | Self::Smallint
                | Self::Sqrt
                | Self::Start
                | Self::StddevPop
                | Self::StddevSamp
                | Self::String
                | Self::Sum
                | Self::Tan
                | Self::Tanh
                | Self::Then
                | Self::Time
                | Self::Timestamp
                | Self::Trailing
                | Self::Trim
                | Self::True
                | Self::Typed
                | Self::Ubigint
                | Self::Uint
                | Self::Uint8
                | Self::Uint16
                | Self::Uint32
                | Self::Uint64
                | Self::Uint128
                | Self::Uint256
                | Self::Union
                | Self::Unknown
                | Self::Unsigned
                | Self::Upper
                | Self::Use
                | Self::Usmallint
                | Self::Value
                | Self::Varbinary
                | Self::Varchar
                | Self::Variable
                | Self::When
                | Self::Where
                | Self::With
                | Self::Xor
                | Self::Year
                | Self::Yield
                | Self::Zoned
                | Self::ZonedDatetime
                | Self::ZonedTime
                | Self::Abstract
                | Self::Aggregate
                | Self::Aggregates
                | Self::Alter
                | Self::Catalog
                | Self::Clear
                | Self::Clone
                | Self::Constraint
                | Self::CurrentRole
                | Self::CurrentUser
                | Self::Data
                | Self::Directory
                | Self::Dryrun
                | Self::Exact
                | Self::Existing
                | Self::Function
                | Self::Gqlstatus
                | Self::Grant
                | Self::Instant
                | Self::Infinity
                | Self::Number
                | Self::Numeric
                | Self::On
                | Self::Open
                | Self::Partition
                | Self::Procedure
                | Self::Product
                | Self::Project
                | Self::Query
                | Self::Records
                | Self::Reference
                | Self::Rename
                | Self::Revoke
                | Self::Substring
                | Self::SystemUser
                | Self::Temporal
                | Self::Unique
                | Self::Unit
                | Self::Values
                | Self::Whitespace
        )
    }

    #[inline]
    pub fn is_non_reserved_word(&self) -> bool {
        matches!(
            self,
            Self::Acyclic
                | Self::Binding
                | Self::Bindings
                | Self::Connecting
                | Self::Destination
                | Self::Different
                | Self::Directed
                | Self::Edge
                | Self::Edges
                | Self::Element
                | Self::Elements
                | Self::First
                | Self::Graph
                | Self::Groups
                | Self::Keep
                | Self::Label
                | Self::Labeled
                | Self::Labels
                | Self::Last
                | Self::Nfc
                | Self::Nfd
                | Self::Nfkc
                | Self::Nfkd
                | Self::No
                | Self::Node
                | Self::Normalized
                | Self::Only
                | Self::Ordinality
                | Self::Property
                | Self::Read
                | Self::Relationship
                | Self::Relationships
                | Self::Repeatable
                | Self::Shortest
                | Self::Simple
                | Self::Source
                | Self::Table
                | Self::To
                | Self::Trail
                | Self::Transaction
                | Self::Type
                | Self::Undirected
                | Self::Vertex
                | Self::Walk
                | Self::Without
                | Self::Write
                | Self::Zone
        )
    }

    #[inline]
    pub fn is_prefix_of_regular_identifier(&self) -> bool {
        matches!(self, Self::RegularIdentifier(_)) || self.is_non_reserved_word()
    }

    #[inline]
    pub fn is_prefix_of_general_set_function(&self) -> bool {
        matches!(
            self,
            Self::Avg
                | Self::Count
                | Self::Max
                | Self::Min
                | Self::Sum
                | Self::CollectList
                | Self::StddevSamp
                | Self::StddevPop
        )
    }

    #[inline]
    pub fn is_prefix_of_binary_set_function(&self) -> bool {
        matches!(self, Self::PercentileCont | Self::PercentileDisc)
    }

    #[inline]
    pub fn is_prefix_of_aggregate_function(&self) -> bool {
        self.is_prefix_of_general_set_function() || self.is_prefix_of_binary_set_function()
    }

    #[inline]
    pub fn is_prefix_of_predefined_type(&self) -> bool {
        matches!(
            self,
            Self::Bool
                | Self::Boolean
                | Self::String
                | Self::Char
                | Self::Varchar
                | Self::Bytes
                | Self::Binary
                | Self::Varbinary
                | Self::Decimal
                | Self::Dec
                | Self::Null
                | Self::Nothing
                | Self::Vector
        ) || self.is_prefix_of_signed_exact_numeric_type()
            || self.is_prefix_of_unsigned_exact_numeric_type()
            || self.is_prefix_of_temporal_type()
    }

    #[inline]
    pub fn is_prefix_of_signed_exact_numeric_type(&self) -> bool {
        matches!(
            self,
            Self::Int8
                | Self::Int16
                | Self::Int32
                | Self::Int64
                | Self::Int128
                | Self::Int256
                | Self::Smallint
                | Self::Int
                | Self::Bigint
                | Self::Signed
        ) || self.is_prefix_of_verbose_exact_numeric_type()
    }

    #[inline]
    pub fn is_prefix_of_unsigned_exact_numeric_type(&self) -> bool {
        matches!(
            self,
            Self::Uint8
                | Self::Uint16
                | Self::Uint32
                | Self::Uint64
                | Self::Uint128
                | Self::Uint256
                | Self::Usmallint
                | Self::Uint
                | Self::Ubigint
                | Self::Unsigned
        )
    }

    #[inline]
    pub fn is_prefix_of_verbose_exact_numeric_type(&self) -> bool {
        matches!(
            self,
            Self::Integer8
                | Self::Integer16
                | Self::Integer32
                | Self::Integer64
                | Self::Integer128
                | Self::Integer256
                | Self::Small
                | Self::Integer
                | Self::Big
        )
    }

    #[inline]
    pub fn is_prefix_of_temporal_type(&self) -> bool {
        matches!(
            self,
            Self::Duration | Self::Zoned | Self::Timestamp | Self::Local | Self::Date | Self::Time
        )
    }

    #[inline]
    pub fn is_prefix_of_ambient_linear_query_statement(&self) -> bool {
        matches!(
            self,
            Self::Return
                | Self::Finish
                | Self::LeftBrace
                | Self::Optional
                | Self::Call
                | Self::Match
                | Self::Let
                | Self::For
                | Self::Filter
                | Self::Order
                | Self::Limit
                | Self::Offset
                | Self::Skip
        )
    }

    #[inline]
    pub fn is_prefix_of_abbreviated_edge_pattern(&self) -> bool {
        matches!(
            self,
            Self::LeftArrow
                | Self::Tilde
                | Self::RightArrow
                | Self::LeftArrowTilde
                | Self::TildeRightArrow
                | Self::LeftMinusRight
                | Self::Minus
        )
    }

    #[inline]
    pub fn is_prefix_of_full_edge_pattern(&self) -> bool {
        matches!(
            self,
            Self::LeftArrowBracket
                | Self::TildeLeftBracket
                | Self::MinusLeftBracket
                | Self::LeftArrowTildeBracket
        )
    }

    #[inline]
    pub fn is_prefix_of_edge_pattern(&self) -> bool {
        self.is_prefix_of_abbreviated_edge_pattern() || self.is_prefix_of_full_edge_pattern()
    }

    #[inline]
    pub fn is_prefix_of_simple_query_statement(&self) -> bool {
        matches!(
            self,
            Self::Match
                | Self::Optional
                | Self::Let
                | Self::For
                | Self::Filter
                | Self::Order
                | Self::Limit
                | Self::Offset
                | Self::Skip
                | Self::Call
        )
    }

    #[inline]
    pub fn is_prefix_of_result_statement(&self) -> bool {
        matches!(self, Self::Return | Self::Finish)
    }

    #[inline]
    pub fn is_prefix_of_numeric_value_function(&self) -> bool {
        matches!(
            self,
            Self::CharLength
                | Self::CharacterLength
                | Self::ByteLength
                | Self::OctetLength
                | Self::PathLength
                | Self::Cardinality
                | Self::Size
                | Self::Abs
                | Self::Mod
                | Self::Log
                | Self::Log10
                | Self::Ln
                | Self::Exp
                | Self::Power
                | Self::Sqrt
                | Self::Floor
                | Self::Ceil
                | Self::Ceiling
        ) || self.is_prefix_of_trigonometric_function()
    }

    #[inline]
    pub fn is_prefix_of_trigonometric_function(&self) -> bool {
        matches!(
            self,
            Self::Sin
                | Self::Cos
                | Self::Tan
                | Self::Cot
                | Self::Sinh
                | Self::Cosh
                | Self::Tanh
                | Self::Asin
                | Self::Acos
                | Self::Atan
                | Self::Degrees
                | Self::Radians
        )
    }

    #[inline]
    pub fn is_prefix_of_value_function(&self) -> bool {
        self.is_prefix_of_regular_identifier()
            || self.is_prefix_of_numeric_value_function()
            || matches!(self, Self::VectorDistance)
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use logos::Logos;

    use super::TokenKind;
    use crate::lexer::{ParameterName, Quoted, TokenErrorKind};

    #[test]
    fn test_simple_comment() {
        let mut lexer = TokenKind::lexer("// This is a comment");
        assert_eq!(lexer.next(), None);
        let mut lexer = TokenKind::lexer("-- This is a comment");
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_bracketed_comment() {
        let mut lexer = TokenKind::lexer("/* This is a comment */");
        assert_eq!(lexer.next(), None);
        let mut lexer = TokenKind::lexer("/***/");
        assert_eq!(lexer.next(), None);
        let mut lexer = TokenKind::lexer("/***");
        assert_eq!(lexer.next(), Some(Err(TokenErrorKind::IncompleteComment)));
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_quoted() {
        let lexer = TokenKind::lexer(r#"'ab\ncd'"#);
        let tokens: Vec<_> = lexer.collect();
        assert_eq!(tokens, vec![Ok(TokenKind::SingleQuoted(Quoted::Single(
            r"ab\ncd"
        )))]);

        let lexer = TokenKind::lexer(r#""ab\ncd""#);
        let tokens: Vec<_> = lexer.collect();
        assert_eq!(tokens, vec![Ok(TokenKind::DoubleQuoted(Quoted::Double(
            r"ab\ncd"
        )))]);

        let lexer = TokenKind::lexer(r#"`ab\ncd`"#);
        let tokens: Vec<_> = lexer.collect();
        assert_eq!(tokens, vec![Ok(TokenKind::AccentQuoted(Quoted::Accent(
            r"ab\ncd"
        )))]);
    }

    #[test]
    fn test_parameter_name() {
        let lexer = TokenKind::lexer(r#"$_abc"#);
        let tokens: Vec<_> = lexer.collect();
        assert_eq!(tokens, vec![Ok(TokenKind::GeneralParameterReference(
            ParameterName::Extended("_abc")
        ))]);

        let lexer = TokenKind::lexer(r#"$$_abc"#);
        let tokens: Vec<_> = lexer.collect();
        assert_eq!(tokens, vec![Ok(TokenKind::SubstitutedParameterReference(
            ParameterName::Extended("_abc")
        ))]);

        let lexer = TokenKind::lexer(r#"$@"a""bc""#);
        let tokens: Vec<_> = lexer.collect();
        assert_eq!(tokens, vec![Ok(TokenKind::GeneralParameterReference(
            ParameterName::Delimited(Quoted::UnescapedDouble("a\"\"bc"))
        ))]);

        let lexer = TokenKind::lexer(r#"$'abc'"#);
        let tokens: Vec<_> = lexer.collect();
        // Single quoted sequence is not allowed in parameter reference.
        assert_eq!(tokens, vec![
            Err(TokenErrorKind::InvalidToken),
            Ok(TokenKind::RegularIdentifier("abc")),
            Err(TokenErrorKind::InvalidToken)
        ]);
    }

    #[test]
    fn test_float_literal_scientific() {
        let lexer = TokenKind::lexer("1.23e-4 1e10 1.23");
        let tokens: Vec<_> = lexer.collect();
        assert_eq!(tokens, vec![
            Ok(TokenKind::UnsignedFloatLiteral("1.23e-4")),
            Ok(TokenKind::UnsignedFloatLiteral("1e10")),
            Ok(TokenKind::UnsignedFloatLiteral("1.23"))
        ]);
    }
}
