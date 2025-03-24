
// Generated from GQL.g4 by ANTLR 4.13.2

#pragma once


#include "antlr4-runtime.h"




class  GQLParser : public antlr4::Parser {
public:
  enum {
    IMPLIES = 1, BOOLEAN_LITERAL = 2, SINGLE_QUOTED_CHARACTER_SEQUENCE = 3, 
    DOUBLE_QUOTED_CHARACTER_SEQUENCE = 4, ACCENT_QUOTED_CHARACTER_SEQUENCE = 5, 
    NO_ESCAPE = 6, BYTE_STRING_LITERAL = 7, UNSIGNED_DECIMAL_IN_SCIENTIFIC_NOTATION_WITH_EXACT_NUMBER_SUFFIX = 8, 
    UNSIGNED_DECIMAL_IN_SCIENTIFIC_NOTATION_WITHOUT_SUFFIX = 9, UNSIGNED_DECIMAL_IN_SCIENTIFIC_NOTATION_WITH_APPROXIMATE_NUMBER_SUFFIX = 10, 
    UNSIGNED_DECIMAL_IN_COMMON_NOTATION_WITH_EXACT_NUMBER_SUFFIX = 11, UNSIGNED_DECIMAL_IN_COMMON_NOTATION_WITHOUT_SUFFIX = 12, 
    UNSIGNED_DECIMAL_IN_COMMON_NOTATION_WITH_APPROXIMATE_NUMBER_SUFFIX = 13, 
    UNSIGNED_DECIMAL_INTEGER_WITH_EXACT_NUMBER_SUFFIX = 14, UNSIGNED_DECIMAL_INTEGER_WITH_APPROXIMATE_NUMBER_SUFFIX = 15, 
    UNSIGNED_DECIMAL_INTEGER = 16, UNSIGNED_HEXADECIMAL_INTEGER = 17, UNSIGNED_OCTAL_INTEGER = 18, 
    UNSIGNED_BINARY_INTEGER = 19, ABS = 20, ACOS = 21, ALL = 22, ALL_DIFFERENT = 23, 
    AND = 24, ANY = 25, ARRAY = 26, AS = 27, ASC = 28, ASCENDING = 29, ASIN = 30, 
    AT = 31, ATAN = 32, AVG = 33, BIG = 34, BIGINT = 35, BINARY = 36, BOOL = 37, 
    BOOLEAN = 38, BOTH = 39, BTRIM = 40, BY = 41, BYTE_LENGTH = 42, BYTES = 43, 
    CALL = 44, CARDINALITY = 45, CASE = 46, CAST = 47, CEIL = 48, CEILING = 49, 
    CHAR = 50, CHAR_LENGTH = 51, CHARACTER_LENGTH = 52, CHARACTERISTICS = 53, 
    CLOSE = 54, COALESCE = 55, COLLECT_LIST = 56, COMMIT = 57, COPY = 58, 
    COS = 59, COSH = 60, COT = 61, COUNT = 62, CREATE = 63, CURRENT_DATE = 64, 
    CURRENT_GRAPH = 65, CURRENT_PROPERTY_GRAPH = 66, CURRENT_SCHEMA = 67, 
    CURRENT_TIME = 68, CURRENT_TIMESTAMP = 69, DATE = 70, DATETIME = 71, 
    DAY = 72, DEC = 73, DECIMAL = 74, DEGREES = 75, DELETE = 76, DESC = 77, 
    DESCENDING = 78, DETACH = 79, DISTINCT = 80, DOUBLE = 81, DROP = 82, 
    DURATION = 83, DURATION_BETWEEN = 84, ELEMENT_ID = 85, ELSE = 86, END = 87, 
    EXCEPT = 88, EXISTS = 89, EXP = 90, FILTER = 91, FINISH = 92, FLOAT = 93, 
    FLOAT16 = 94, FLOAT32 = 95, FLOAT64 = 96, FLOAT128 = 97, FLOAT256 = 98, 
    FLOOR = 99, FOR = 100, FROM = 101, GROUP = 102, HAVING = 103, HOME_GRAPH = 104, 
    HOME_PROPERTY_GRAPH = 105, HOME_SCHEMA = 106, HOUR = 107, IF = 108, 
    IN = 109, INSERT = 110, INT = 111, INTEGER = 112, INT8 = 113, INTEGER8 = 114, 
    INT16 = 115, INTEGER16 = 116, INT32 = 117, INTEGER32 = 118, INT64 = 119, 
    INTEGER64 = 120, INT128 = 121, INTEGER128 = 122, INT256 = 123, INTEGER256 = 124, 
    INTERSECT = 125, INTERVAL = 126, IS = 127, LEADING = 128, LEFT = 129, 
    LET = 130, LIKE = 131, LIMIT = 132, LIST = 133, LN = 134, LOCAL = 135, 
    LOCAL_DATETIME = 136, LOCAL_TIME = 137, LOCAL_TIMESTAMP = 138, LOG_KW = 139, 
    LOG10 = 140, LOWER = 141, LTRIM = 142, MATCH = 143, MAX = 144, MIN = 145, 
    MINUTE = 146, MOD = 147, MONTH = 148, NEXT = 149, NODETACH = 150, NORMALIZE = 151, 
    NOT = 152, NOTHING = 153, NULL_KW = 154, NULLS = 155, NULLIF = 156, 
    OCTET_LENGTH = 157, OF = 158, OFFSET = 159, OPTIONAL = 160, OR = 161, 
    ORDER = 162, OTHERWISE = 163, PARAMETER = 164, PARAMETERS = 165, PATH = 166, 
    PATH_LENGTH = 167, PATHS = 168, PERCENTILE_CONT = 169, PERCENTILE_DISC = 170, 
    POWER = 171, PRECISION = 172, PROPERTY_EXISTS = 173, RADIANS = 174, 
    REAL = 175, RECORD = 176, REMOVE = 177, REPLACE = 178, RESET = 179, 
    RETURN = 180, RIGHT = 181, ROLLBACK = 182, RTRIM = 183, SAME = 184, 
    SCHEMA = 185, SECOND = 186, SELECT = 187, SESSION = 188, SESSION_USER = 189, 
    SET = 190, SIGNED = 191, SIN = 192, SINH = 193, SIZE = 194, SKIP_RESERVED_WORD = 195, 
    SMALL = 196, SMALLINT = 197, SQRT = 198, START = 199, STDDEV_POP = 200, 
    STDDEV_SAMP = 201, STRING = 202, SUM = 203, TAN = 204, TANH = 205, THEN = 206, 
    TIME = 207, TIMESTAMP = 208, TRAILING = 209, TRIM = 210, TYPED = 211, 
    UBIGINT = 212, UINT = 213, UINT8 = 214, UINT16 = 215, UINT32 = 216, 
    UINT64 = 217, UINT128 = 218, UINT256 = 219, UNION = 220, UNSIGNED = 221, 
    UPPER = 222, USE = 223, USMALLINT = 224, VALUE = 225, VARBINARY = 226, 
    VARCHAR = 227, VARIABLE = 228, WHEN = 229, WHERE = 230, WITH = 231, 
    XOR = 232, YEAR = 233, YIELD = 234, ZONED = 235, ZONED_DATETIME = 236, 
    ZONED_TIME = 237, ABSTRACT = 238, AGGREGATE = 239, AGGREGATES = 240, 
    ALTER = 241, CATALOG = 242, CLEAR = 243, CLONE = 244, CONSTRAINT = 245, 
    CURRENT_ROLE = 246, CURRENT_USER = 247, DATA = 248, DIRECTORY = 249, 
    DRYRUN = 250, EXACT = 251, EXISTING = 252, FUNCTION = 253, GQLSTATUS = 254, 
    GRANT = 255, INSTANT = 256, INFINITY_KW = 257, NUMBER = 258, NUMERIC = 259, 
    ON = 260, OPEN = 261, PARTITION = 262, PROCEDURE = 263, PRODUCT = 264, 
    PROJECT = 265, QUERY = 266, RECORDS = 267, REFERENCE = 268, RENAME = 269, 
    REVOKE = 270, SUBSTRING = 271, SYSTEM_USER = 272, TEMPORAL = 273, UNIQUE = 274, 
    UNIT = 275, VALUES = 276, ACYCLIC = 277, BINDING = 278, BINDINGS = 279, 
    CONNECTING = 280, DESTINATION = 281, DIFFERENT = 282, DIRECTED = 283, 
    EDGE = 284, EDGES = 285, ELEMENT = 286, ELEMENTS = 287, FIRST = 288, 
    GRAPH = 289, GROUPS = 290, KEEP = 291, LABEL = 292, LABELED = 293, LABELS = 294, 
    LAST = 295, NFC = 296, NFD = 297, NFKC = 298, NFKD = 299, NO = 300, 
    NODE = 301, NORMALIZED = 302, ONLY = 303, ORDINALITY = 304, PROPERTY = 305, 
    READ = 306, RELATIONSHIP = 307, RELATIONSHIPS = 308, REPEATABLE = 309, 
    SHORTEST = 310, SIMPLE = 311, SOURCE = 312, TABLE = 313, TO = 314, TRAIL = 315, 
    TRANSACTION = 316, TYPE = 317, UNDIRECTED = 318, VERTEX = 319, WALK = 320, 
    WITHOUT = 321, WRITE = 322, ZONE = 323, REGULAR_IDENTIFIER = 324, SUBSTITUTED_PARAMETER_REFERENCE = 325, 
    GENERAL_PARAMETER_REFERENCE = 326, MULTISET_ALTERNATION_OPERATOR = 327, 
    BRACKET_RIGHT_ARROW = 328, BRACKET_TILDE_RIGHT_ARROW = 329, CONCATENATION_OPERATOR = 330, 
    DOUBLE_COLON = 331, DOUBLE_DOLLAR_SIGN = 332, DOUBLE_PERIOD = 333, GREATER_THAN_OR_EQUALS_OPERATOR = 334, 
    LEFT_ARROW = 335, LEFT_ARROW_TILDE = 336, LEFT_ARROW_BRACKET = 337, 
    LEFT_ARROW_TILDE_BRACKET = 338, LEFT_MINUS_RIGHT = 339, LEFT_MINUS_SLASH = 340, 
    LEFT_TILDE_SLASH = 341, LESS_THAN_OR_EQUALS_OPERATOR = 342, MINUS_LEFT_BRACKET = 343, 
    MINUS_SLASH = 344, NOT_EQUALS_OPERATOR = 345, RIGHT_ARROW = 346, RIGHT_BRACKET_MINUS = 347, 
    RIGHT_BRACKET_TILDE = 348, RIGHT_DOUBLE_ARROW = 349, SLASH_MINUS = 350, 
    SLASH_MINUS_RIGHT = 351, SLASH_TILDE = 352, SLASH_TILDE_RIGHT = 353, 
    TILDE_LEFT_BRACKET = 354, TILDE_RIGHT_ARROW = 355, TILDE_SLASH = 356, 
    AMPERSAND = 357, ASTERISK = 358, COLON = 359, COMMA = 360, COMMERCIAL_AT = 361, 
    DOLLAR_SIGN = 362, DOUBLE_QUOTE = 363, EQUALS_OPERATOR = 364, EXCLAMATION_MARK = 365, 
    RIGHT_ANGLE_BRACKET = 366, GRAVE_ACCENT = 367, LEFT_BRACE = 368, LEFT_BRACKET = 369, 
    LEFT_PAREN = 370, LEFT_ANGLE_BRACKET = 371, MINUS_SIGN = 372, PERCENT = 373, 
    PERIOD = 374, PLUS_SIGN = 375, QUESTION_MARK = 376, QUOTE = 377, REVERSE_SOLIDUS = 378, 
    RIGHT_BRACE = 379, RIGHT_BRACKET = 380, RIGHT_PAREN = 381, SOLIDUS = 382, 
    TILDE = 383, UNDERSCORE = 384, VERTICAL_BAR = 385, SP = 386, WHITESPACE = 387, 
    BRACKETED_COMMENT = 388, SIMPLE_COMMENT_SOLIDUS = 389, SIMPLE_COMMENT_MINUS = 390
  };

  enum {
    RuleGqlProgram = 0, RuleProgramActivity = 1, RuleSessionActivity = 2, 
    RuleTransactionActivity = 3, RuleEndTransactionCommand = 4, RuleSessionSetCommand = 5, 
    RuleSessionSetSchemaClause = 6, RuleSessionSetGraphClause = 7, RuleSessionSetTimeZoneClause = 8, 
    RuleSetTimeZoneValue = 9, RuleSessionSetParameterClause = 10, RuleSessionSetGraphParameterClause = 11, 
    RuleSessionSetBindingTableParameterClause = 12, RuleSessionSetValueParameterClause = 13, 
    RuleSessionSetParameterName = 14, RuleSessionResetCommand = 15, RuleSessionResetArguments = 16, 
    RuleSessionCloseCommand = 17, RuleSessionParameterSpecification = 18, 
    RuleStartTransactionCommand = 19, RuleTransactionCharacteristics = 20, 
    RuleTransactionMode = 21, RuleTransactionAccessMode = 22, RuleRollbackCommand = 23, 
    RuleCommitCommand = 24, RuleNestedProcedureSpecification = 25, RuleProcedureSpecification = 26, 
    RuleNestedDataModifyingProcedureSpecification = 27, RuleNestedQuerySpecification = 28, 
    RuleProcedureBody = 29, RuleBindingVariableDefinitionBlock = 30, RuleBindingVariableDefinition = 31, 
    RuleStatementBlock = 32, RuleStatement = 33, RuleNextStatement = 34, 
    RuleGraphVariableDefinition = 35, RuleOptTypedGraphInitializer = 36, 
    RuleGraphInitializer = 37, RuleBindingTableVariableDefinition = 38, 
    RuleOptTypedBindingTableInitializer = 39, RuleBindingTableInitializer = 40, 
    RuleValueVariableDefinition = 41, RuleOptTypedValueInitializer = 42, 
    RuleValueInitializer = 43, RuleGraphExpression = 44, RuleCurrentGraph = 45, 
    RuleBindingTableExpression = 46, RuleNestedBindingTableQuerySpecification = 47, 
    RuleObjectExpressionPrimary = 48, RuleLinearCatalogModifyingStatement = 49, 
    RuleSimpleCatalogModifyingStatement = 50, RulePrimitiveCatalogModifyingStatement = 51, 
    RuleCreateSchemaStatement = 52, RuleDropSchemaStatement = 53, RuleCreateGraphStatement = 54, 
    RuleOpenGraphType = 55, RuleOfGraphType = 56, RuleGraphTypeLikeGraph = 57, 
    RuleGraphSource = 58, RuleDropGraphStatement = 59, RuleCreateGraphTypeStatement = 60, 
    RuleGraphTypeSource = 61, RuleCopyOfGraphType = 62, RuleDropGraphTypeStatement = 63, 
    RuleCallCatalogModifyingProcedureStatement = 64, RuleLinearDataModifyingStatement = 65, 
    RuleFocusedLinearDataModifyingStatement = 66, RuleFocusedLinearDataModifyingStatementBody = 67, 
    RuleFocusedNestedDataModifyingProcedureSpecification = 68, RuleAmbientLinearDataModifyingStatement = 69, 
    RuleAmbientLinearDataModifyingStatementBody = 70, RuleSimpleLinearDataAccessingStatement = 71, 
    RuleSimpleDataAccessingStatement = 72, RuleSimpleDataModifyingStatement = 73, 
    RulePrimitiveDataModifyingStatement = 74, RuleInsertStatement = 75, 
    RuleSetStatement = 76, RuleSetItemList = 77, RuleSetItem = 78, RuleSetPropertyItem = 79, 
    RuleSetAllPropertiesItem = 80, RuleSetLabelItem = 81, RuleRemoveStatement = 82, 
    RuleRemoveItemList = 83, RuleRemoveItem = 84, RuleRemovePropertyItem = 85, 
    RuleRemoveLabelItem = 86, RuleDeleteStatement = 87, RuleDeleteItemList = 88, 
    RuleDeleteItem = 89, RuleCallDataModifyingProcedureStatement = 90, RuleCompositeQueryStatement = 91, 
    RuleCompositeQueryExpression = 92, RuleQueryConjunction = 93, RuleSetOperator = 94, 
    RuleCompositeQueryPrimary = 95, RuleLinearQueryStatement = 96, RuleFocusedLinearQueryStatement = 97, 
    RuleFocusedLinearQueryStatementPart = 98, RuleFocusedLinearQueryAndPrimitiveResultStatementPart = 99, 
    RuleFocusedPrimitiveResultStatement = 100, RuleFocusedNestedQuerySpecification = 101, 
    RuleAmbientLinearQueryStatement = 102, RuleSimpleLinearQueryStatement = 103, 
    RuleSimpleQueryStatement = 104, RulePrimitiveQueryStatement = 105, RuleMatchStatement = 106, 
    RuleSimpleMatchStatement = 107, RuleOptionalMatchStatement = 108, RuleOptionalOperand = 109, 
    RuleMatchStatementBlock = 110, RuleCallQueryStatement = 111, RuleFilterStatement = 112, 
    RuleLetStatement = 113, RuleLetVariableDefinitionList = 114, RuleLetVariableDefinition = 115, 
    RuleForStatement = 116, RuleForItem = 117, RuleForItemAlias = 118, RuleForItemSource = 119, 
    RuleForOrdinalityOrOffset = 120, RuleOrderByAndPageStatement = 121, 
    RulePrimitiveResultStatement = 122, RuleReturnStatement = 123, RuleReturnStatementBody = 124, 
    RuleReturnItemList = 125, RuleReturnItem = 126, RuleReturnItemAlias = 127, 
    RuleSelectStatement = 128, RuleSelectItemList = 129, RuleSelectItem = 130, 
    RuleSelectItemAlias = 131, RuleHavingClause = 132, RuleSelectStatementBody = 133, 
    RuleSelectGraphMatchList = 134, RuleSelectGraphMatch = 135, RuleSelectQuerySpecification = 136, 
    RuleCallProcedureStatement = 137, RuleProcedureCall = 138, RuleInlineProcedureCall = 139, 
    RuleVariableScopeClause = 140, RuleBindingVariableReferenceList = 141, 
    RuleNamedProcedureCall = 142, RuleProcedureArgumentList = 143, RuleProcedureArgument = 144, 
    RuleAtSchemaClause = 145, RuleUseGraphClause = 146, RuleGraphPatternBindingTable = 147, 
    RuleGraphPatternYieldClause = 148, RuleGraphPatternYieldItemList = 149, 
    RuleGraphPatternYieldItem = 150, RuleGraphPattern = 151, RuleMatchMode = 152, 
    RuleRepeatableElementsMatchMode = 153, RuleDifferentEdgesMatchMode = 154, 
    RuleElementBindingsOrElements = 155, RuleEdgeBindingsOrEdges = 156, 
    RulePathPatternList = 157, RulePathPattern = 158, RulePathVariableDeclaration = 159, 
    RuleKeepClause = 160, RuleGraphPatternWhereClause = 161, RuleInsertGraphPattern = 162, 
    RuleInsertPathPatternList = 163, RuleInsertPathPattern = 164, RuleInsertNodePattern = 165, 
    RuleInsertEdgePattern = 166, RuleInsertEdgePointingLeft = 167, RuleInsertEdgePointingRight = 168, 
    RuleInsertEdgeUndirected = 169, RuleInsertElementPatternFiller = 170, 
    RuleLabelAndPropertySetSpecification = 171, RulePathPatternPrefix = 172, 
    RulePathModePrefix = 173, RulePathMode = 174, RulePathSearchPrefix = 175, 
    RuleAllPathSearch = 176, RulePathOrPaths = 177, RuleAnyPathSearch = 178, 
    RuleNumberOfPaths = 179, RuleShortestPathSearch = 180, RuleAllShortestPathSearch = 181, 
    RuleAnyShortestPathSearch = 182, RuleCountedShortestPathSearch = 183, 
    RuleCountedShortestGroupSearch = 184, RuleNumberOfGroups = 185, RulePathPatternExpression = 186, 
    RulePathTerm = 187, RulePathFactor = 188, RulePathPrimary = 189, RuleElementPattern = 190, 
    RuleNodePattern = 191, RuleElementPatternFiller = 192, RuleElementVariableDeclaration = 193, 
    RuleIsLabelExpression = 194, RuleIsOrColon = 195, RuleElementPatternPredicate = 196, 
    RuleElementPatternWhereClause = 197, RuleElementPropertySpecification = 198, 
    RulePropertyKeyValuePairList = 199, RulePropertyKeyValuePair = 200, 
    RuleEdgePattern = 201, RuleFullEdgePattern = 202, RuleFullEdgePointingLeft = 203, 
    RuleFullEdgeUndirected = 204, RuleFullEdgePointingRight = 205, RuleFullEdgeLeftOrUndirected = 206, 
    RuleFullEdgeUndirectedOrRight = 207, RuleFullEdgeLeftOrRight = 208, 
    RuleFullEdgeAnyDirection = 209, RuleAbbreviatedEdgePattern = 210, RuleParenthesizedPathPatternExpression = 211, 
    RuleSubpathVariableDeclaration = 212, RuleParenthesizedPathPatternWhereClause = 213, 
    RuleLabelExpression = 214, RulePathVariableReference = 215, RuleElementVariableReference = 216, 
    RuleGraphPatternQuantifier = 217, RuleFixedQuantifier = 218, RuleGeneralQuantifier = 219, 
    RuleLowerBound = 220, RuleUpperBound = 221, RuleSimplifiedPathPatternExpression = 222, 
    RuleSimplifiedDefaultingLeft = 223, RuleSimplifiedDefaultingUndirected = 224, 
    RuleSimplifiedDefaultingRight = 225, RuleSimplifiedDefaultingLeftOrUndirected = 226, 
    RuleSimplifiedDefaultingUndirectedOrRight = 227, RuleSimplifiedDefaultingLeftOrRight = 228, 
    RuleSimplifiedDefaultingAnyDirection = 229, RuleSimplifiedContents = 230, 
    RuleSimplifiedPathUnion = 231, RuleSimplifiedMultisetAlternation = 232, 
    RuleSimplifiedTerm = 233, RuleSimplifiedFactorLow = 234, RuleSimplifiedFactorHigh = 235, 
    RuleSimplifiedQuantified = 236, RuleSimplifiedQuestioned = 237, RuleSimplifiedTertiary = 238, 
    RuleSimplifiedDirectionOverride = 239, RuleSimplifiedOverrideLeft = 240, 
    RuleSimplifiedOverrideUndirected = 241, RuleSimplifiedOverrideRight = 242, 
    RuleSimplifiedOverrideLeftOrUndirected = 243, RuleSimplifiedOverrideUndirectedOrRight = 244, 
    RuleSimplifiedOverrideLeftOrRight = 245, RuleSimplifiedOverrideAnyDirection = 246, 
    RuleSimplifiedSecondary = 247, RuleSimplifiedNegation = 248, RuleSimplifiedPrimary = 249, 
    RuleWhereClause = 250, RuleYieldClause = 251, RuleYieldItemList = 252, 
    RuleYieldItem = 253, RuleYieldItemName = 254, RuleYieldItemAlias = 255, 
    RuleGroupByClause = 256, RuleGroupingElementList = 257, RuleGroupingElement = 258, 
    RuleEmptyGroupingSet = 259, RuleOrderByClause = 260, RuleSortSpecificationList = 261, 
    RuleSortSpecification = 262, RuleSortKey = 263, RuleOrderingSpecification = 264, 
    RuleNullOrdering = 265, RuleLimitClause = 266, RuleOffsetClause = 267, 
    RuleOffsetSynonym = 268, RuleSchemaReference = 269, RuleAbsoluteCatalogSchemaReference = 270, 
    RuleCatalogSchemaParentAndName = 271, RuleRelativeCatalogSchemaReference = 272, 
    RulePredefinedSchemaReference = 273, RuleAbsoluteDirectoryPath = 274, 
    RuleRelativeDirectoryPath = 275, RuleSimpleDirectoryPath = 276, RuleGraphReference = 277, 
    RuleCatalogGraphParentAndName = 278, RuleHomeGraph = 279, RuleGraphTypeReference = 280, 
    RuleCatalogGraphTypeParentAndName = 281, RuleBindingTableReference = 282, 
    RuleProcedureReference = 283, RuleCatalogProcedureParentAndName = 284, 
    RuleCatalogObjectParentReference = 285, RuleReferenceParameterSpecification = 286, 
    RuleNestedGraphTypeSpecification = 287, RuleGraphTypeSpecificationBody = 288, 
    RuleElementTypeList = 289, RuleElementTypeSpecification = 290, RuleNodeTypeSpecification = 291, 
    RuleNodeTypePattern = 292, RuleNodeTypePhrase = 293, RuleNodeTypePhraseFiller = 294, 
    RuleNodeTypeFiller = 295, RuleLocalNodeTypeAlias = 296, RuleNodeTypeImpliedContent = 297, 
    RuleNodeTypeKeyLabelSet = 298, RuleNodeTypeLabelSet = 299, RuleNodeTypePropertyTypes = 300, 
    RuleEdgeTypeSpecification = 301, RuleEdgeTypePattern = 302, RuleEdgeTypePhrase = 303, 
    RuleEdgeTypePhraseFiller = 304, RuleEdgeTypeFiller = 305, RuleEdgeTypeImpliedContent = 306, 
    RuleEdgeTypeKeyLabelSet = 307, RuleEdgeTypeLabelSet = 308, RuleEdgeTypePropertyTypes = 309, 
    RuleEdgeTypePatternDirected = 310, RuleEdgeTypePatternPointingRight = 311, 
    RuleEdgeTypePatternPointingLeft = 312, RuleEdgeTypePatternUndirected = 313, 
    RuleArcTypePointingRight = 314, RuleArcTypePointingLeft = 315, RuleArcTypeUndirected = 316, 
    RuleSourceNodeTypeReference = 317, RuleDestinationNodeTypeReference = 318, 
    RuleEdgeKind = 319, RuleEndpointPairPhrase = 320, RuleEndpointPair = 321, 
    RuleEndpointPairDirected = 322, RuleEndpointPairPointingRight = 323, 
    RuleEndpointPairPointingLeft = 324, RuleEndpointPairUndirected = 325, 
    RuleConnectorPointingRight = 326, RuleConnectorUndirected = 327, RuleSourceNodeTypeAlias = 328, 
    RuleDestinationNodeTypeAlias = 329, RuleLabelSetPhrase = 330, RuleLabelSetSpecification = 331, 
    RulePropertyTypesSpecification = 332, RulePropertyTypeList = 333, RulePropertyType = 334, 
    RulePropertyValueType = 335, RuleBindingTableType = 336, RuleValueType = 337, 
    RuleTyped = 338, RulePredefinedType = 339, RuleBooleanType = 340, RuleCharacterStringType = 341, 
    RuleByteStringType = 342, RuleMinLength = 343, RuleMaxLength = 344, 
    RuleFixedLength = 345, RuleNumericType = 346, RuleExactNumericType = 347, 
    RuleBinaryExactNumericType = 348, RuleSignedBinaryExactNumericType = 349, 
    RuleUnsignedBinaryExactNumericType = 350, RuleVerboseBinaryExactNumericType = 351, 
    RuleDecimalExactNumericType = 352, RulePrecision = 353, RuleScale = 354, 
    RuleApproximateNumericType = 355, RuleTemporalType = 356, RuleTemporalInstantType = 357, 
    RuleDatetimeType = 358, RuleLocaldatetimeType = 359, RuleDateType = 360, 
    RuleTimeType = 361, RuleLocaltimeType = 362, RuleTemporalDurationType = 363, 
    RuleTemporalDurationQualifier = 364, RuleReferenceValueType = 365, RuleImmaterialValueType = 366, 
    RuleNullType = 367, RuleEmptyType = 368, RuleGraphReferenceValueType = 369, 
    RuleClosedGraphReferenceValueType = 370, RuleOpenGraphReferenceValueType = 371, 
    RuleBindingTableReferenceValueType = 372, RuleNodeReferenceValueType = 373, 
    RuleClosedNodeReferenceValueType = 374, RuleOpenNodeReferenceValueType = 375, 
    RuleEdgeReferenceValueType = 376, RuleClosedEdgeReferenceValueType = 377, 
    RuleOpenEdgeReferenceValueType = 378, RulePathValueType = 379, RuleListValueTypeName = 380, 
    RuleListValueTypeNameSynonym = 381, RuleRecordType = 382, RuleFieldTypesSpecification = 383, 
    RuleFieldTypeList = 384, RuleNotNull = 385, RuleFieldType = 386, RuleSearchCondition = 387, 
    RulePredicate = 388, RuleCompOp = 389, RuleExistsPredicate = 390, RuleNullPredicate = 391, 
    RuleNullPredicatePart2 = 392, RuleValueTypePredicate = 393, RuleValueTypePredicatePart2 = 394, 
    RuleNormalizedPredicatePart2 = 395, RuleDirectedPredicate = 396, RuleDirectedPredicatePart2 = 397, 
    RuleLabeledPredicate = 398, RuleLabeledPredicatePart2 = 399, RuleIsLabeledOrColon = 400, 
    RuleSourceDestinationPredicate = 401, RuleNodeReference = 402, RuleSourcePredicatePart2 = 403, 
    RuleDestinationPredicatePart2 = 404, RuleEdgeReference = 405, RuleAll_differentPredicate = 406, 
    RuleSamePredicate = 407, RuleProperty_existsPredicate = 408, RuleValueExpression = 409, 
    RuleValueFunction = 410, RuleBooleanValueExpression = 411, RuleCharacterOrByteStringFunction = 412, 
    RuleSubCharacterOrByteString = 413, RuleTrimSingleCharacterOrByteString = 414, 
    RuleFoldCharacterString = 415, RuleTrimMultiCharacterCharacterString = 416, 
    RuleNormalizeCharacterString = 417, RuleNodeReferenceValueExpression = 418, 
    RuleEdgeReferenceValueExpression = 419, RuleAggregatingValueExpression = 420, 
    RuleValueExpressionPrimary = 421, RuleParenthesizedValueExpression = 422, 
    RuleNonParenthesizedValueExpressionPrimary = 423, RuleNonParenthesizedValueExpressionPrimarySpecialCase = 424, 
    RuleUnsignedValueSpecification = 425, RuleNonNegativeIntegerSpecification = 426, 
    RuleGeneralValueSpecification = 427, RuleDynamicParameterSpecification = 428, 
    RuleLetValueExpression = 429, RuleValueQueryExpression = 430, RuleCaseExpression = 431, 
    RuleCaseAbbreviation = 432, RuleCaseSpecification = 433, RuleSimpleCase = 434, 
    RuleSearchedCase = 435, RuleSimpleWhenClause = 436, RuleSearchedWhenClause = 437, 
    RuleElseClause = 438, RuleCaseOperand = 439, RuleWhenOperandList = 440, 
    RuleWhenOperand = 441, RuleResult = 442, RuleResultExpression = 443, 
    RuleCastSpecification = 444, RuleCastOperand = 445, RuleCastTarget = 446, 
    RuleAggregateFunction = 447, RuleGeneralSetFunction = 448, RuleBinarySetFunction = 449, 
    RuleGeneralSetFunctionType = 450, RuleSetQuantifier = 451, RuleBinarySetFunctionType = 452, 
    RuleDependentValueExpression = 453, RuleIndependentValueExpression = 454, 
    RuleElement_idFunction = 455, RuleBindingVariableReference = 456, RulePathValueExpression = 457, 
    RulePathValueConstructor = 458, RulePathValueConstructorByEnumeration = 459, 
    RulePathElementList = 460, RulePathElementListStart = 461, RulePathElementListStep = 462, 
    RuleListValueExpression = 463, RuleListValueFunction = 464, RuleTrimListFunction = 465, 
    RuleElementsFunction = 466, RuleListValueConstructor = 467, RuleListValueConstructorByEnumeration = 468, 
    RuleListElementList = 469, RuleListElement = 470, RuleRecordConstructor = 471, 
    RuleFieldsSpecification = 472, RuleFieldList = 473, RuleField = 474, 
    RuleTruthValue = 475, RuleNumericValueExpression = 476, RuleNumericValueFunction = 477, 
    RuleLengthExpression = 478, RuleCardinalityExpression = 479, RuleCardinalityExpressionArgument = 480, 
    RuleCharLengthExpression = 481, RuleByteLengthExpression = 482, RulePathLengthExpression = 483, 
    RuleAbsoluteValueExpression = 484, RuleModulusExpression = 485, RuleNumericValueExpressionDividend = 486, 
    RuleNumericValueExpressionDivisor = 487, RuleTrigonometricFunction = 488, 
    RuleTrigonometricFunctionName = 489, RuleGeneralLogarithmFunction = 490, 
    RuleGeneralLogarithmBase = 491, RuleGeneralLogarithmArgument = 492, 
    RuleCommonLogarithm = 493, RuleNaturalLogarithm = 494, RuleExponentialFunction = 495, 
    RulePowerFunction = 496, RuleNumericValueExpressionBase = 497, RuleNumericValueExpressionExponent = 498, 
    RuleSquareRoot = 499, RuleFloorFunction = 500, RuleCeilingFunction = 501, 
    RuleCharacterStringValueExpression = 502, RuleByteStringValueExpression = 503, 
    RuleTrimOperands = 504, RuleTrimCharacterOrByteStringSource = 505, RuleTrimSpecification = 506, 
    RuleTrimCharacterOrByteString = 507, RuleNormalForm = 508, RuleStringLength = 509, 
    RuleDatetimeValueExpression = 510, RuleDatetimeValueFunction = 511, 
    RuleDateFunction = 512, RuleTimeFunction = 513, RuleLocaltimeFunction = 514, 
    RuleDatetimeFunction = 515, RuleLocaldatetimeFunction = 516, RuleDateFunctionParameters = 517, 
    RuleTimeFunctionParameters = 518, RuleDatetimeFunctionParameters = 519, 
    RuleDurationValueExpression = 520, RuleDatetimeSubtraction = 521, RuleDatetimeSubtractionParameters = 522, 
    RuleDatetimeValueExpression1 = 523, RuleDatetimeValueExpression2 = 524, 
    RuleDurationValueFunction = 525, RuleDurationFunction = 526, RuleDurationFunctionParameters = 527, 
    RuleObjectName = 528, RuleObjectNameOrBindingVariable = 529, RuleDirectoryName = 530, 
    RuleSchemaName = 531, RuleGraphName = 532, RuleDelimitedGraphName = 533, 
    RuleGraphTypeName = 534, RuleNodeTypeName = 535, RuleEdgeTypeName = 536, 
    RuleBindingTableName = 537, RuleDelimitedBindingTableName = 538, RuleProcedureName = 539, 
    RuleLabelName = 540, RulePropertyName = 541, RuleFieldName = 542, RuleElementVariable = 543, 
    RulePathVariable = 544, RuleSubpathVariable = 545, RuleBindingVariable = 546, 
    RuleUnsignedLiteral = 547, RuleGeneralLiteral = 548, RuleTemporalLiteral = 549, 
    RuleDateLiteral = 550, RuleTimeLiteral = 551, RuleDatetimeLiteral = 552, 
    RuleListLiteral = 553, RuleRecordLiteral = 554, RuleIdentifier = 555, 
    RuleRegularIdentifier = 556, RuleTimeZoneString = 557, RuleCharacterStringLiteral = 558, 
    RuleUnsignedNumericLiteral = 559, RuleExactNumericLiteral = 560, RuleApproximateNumericLiteral = 561, 
    RuleUnsignedInteger = 562, RuleUnsignedDecimalInteger = 563, RuleNullLiteral = 564, 
    RuleDateString = 565, RuleTimeString = 566, RuleDatetimeString = 567, 
    RuleDurationLiteral = 568, RuleDurationString = 569, RuleNodeSynonym = 570, 
    RuleEdgesSynonym = 571, RuleEdgeSynonym = 572, RuleNonReservedWords = 573
  };

  explicit GQLParser(antlr4::TokenStream *input);

  GQLParser(antlr4::TokenStream *input, const antlr4::atn::ParserATNSimulatorOptions &options);

  ~GQLParser() override;

  std::string getGrammarFileName() const override;

  const antlr4::atn::ATN& getATN() const override;

  const std::vector<std::string>& getRuleNames() const override;

  const antlr4::dfa::Vocabulary& getVocabulary() const override;

  antlr4::atn::SerializedATNView getSerializedATN() const override;


  class GqlProgramContext;
  class ProgramActivityContext;
  class SessionActivityContext;
  class TransactionActivityContext;
  class EndTransactionCommandContext;
  class SessionSetCommandContext;
  class SessionSetSchemaClauseContext;
  class SessionSetGraphClauseContext;
  class SessionSetTimeZoneClauseContext;
  class SetTimeZoneValueContext;
  class SessionSetParameterClauseContext;
  class SessionSetGraphParameterClauseContext;
  class SessionSetBindingTableParameterClauseContext;
  class SessionSetValueParameterClauseContext;
  class SessionSetParameterNameContext;
  class SessionResetCommandContext;
  class SessionResetArgumentsContext;
  class SessionCloseCommandContext;
  class SessionParameterSpecificationContext;
  class StartTransactionCommandContext;
  class TransactionCharacteristicsContext;
  class TransactionModeContext;
  class TransactionAccessModeContext;
  class RollbackCommandContext;
  class CommitCommandContext;
  class NestedProcedureSpecificationContext;
  class ProcedureSpecificationContext;
  class NestedDataModifyingProcedureSpecificationContext;
  class NestedQuerySpecificationContext;
  class ProcedureBodyContext;
  class BindingVariableDefinitionBlockContext;
  class BindingVariableDefinitionContext;
  class StatementBlockContext;
  class StatementContext;
  class NextStatementContext;
  class GraphVariableDefinitionContext;
  class OptTypedGraphInitializerContext;
  class GraphInitializerContext;
  class BindingTableVariableDefinitionContext;
  class OptTypedBindingTableInitializerContext;
  class BindingTableInitializerContext;
  class ValueVariableDefinitionContext;
  class OptTypedValueInitializerContext;
  class ValueInitializerContext;
  class GraphExpressionContext;
  class CurrentGraphContext;
  class BindingTableExpressionContext;
  class NestedBindingTableQuerySpecificationContext;
  class ObjectExpressionPrimaryContext;
  class LinearCatalogModifyingStatementContext;
  class SimpleCatalogModifyingStatementContext;
  class PrimitiveCatalogModifyingStatementContext;
  class CreateSchemaStatementContext;
  class DropSchemaStatementContext;
  class CreateGraphStatementContext;
  class OpenGraphTypeContext;
  class OfGraphTypeContext;
  class GraphTypeLikeGraphContext;
  class GraphSourceContext;
  class DropGraphStatementContext;
  class CreateGraphTypeStatementContext;
  class GraphTypeSourceContext;
  class CopyOfGraphTypeContext;
  class DropGraphTypeStatementContext;
  class CallCatalogModifyingProcedureStatementContext;
  class LinearDataModifyingStatementContext;
  class FocusedLinearDataModifyingStatementContext;
  class FocusedLinearDataModifyingStatementBodyContext;
  class FocusedNestedDataModifyingProcedureSpecificationContext;
  class AmbientLinearDataModifyingStatementContext;
  class AmbientLinearDataModifyingStatementBodyContext;
  class SimpleLinearDataAccessingStatementContext;
  class SimpleDataAccessingStatementContext;
  class SimpleDataModifyingStatementContext;
  class PrimitiveDataModifyingStatementContext;
  class InsertStatementContext;
  class SetStatementContext;
  class SetItemListContext;
  class SetItemContext;
  class SetPropertyItemContext;
  class SetAllPropertiesItemContext;
  class SetLabelItemContext;
  class RemoveStatementContext;
  class RemoveItemListContext;
  class RemoveItemContext;
  class RemovePropertyItemContext;
  class RemoveLabelItemContext;
  class DeleteStatementContext;
  class DeleteItemListContext;
  class DeleteItemContext;
  class CallDataModifyingProcedureStatementContext;
  class CompositeQueryStatementContext;
  class CompositeQueryExpressionContext;
  class QueryConjunctionContext;
  class SetOperatorContext;
  class CompositeQueryPrimaryContext;
  class LinearQueryStatementContext;
  class FocusedLinearQueryStatementContext;
  class FocusedLinearQueryStatementPartContext;
  class FocusedLinearQueryAndPrimitiveResultStatementPartContext;
  class FocusedPrimitiveResultStatementContext;
  class FocusedNestedQuerySpecificationContext;
  class AmbientLinearQueryStatementContext;
  class SimpleLinearQueryStatementContext;
  class SimpleQueryStatementContext;
  class PrimitiveQueryStatementContext;
  class MatchStatementContext;
  class SimpleMatchStatementContext;
  class OptionalMatchStatementContext;
  class OptionalOperandContext;
  class MatchStatementBlockContext;
  class CallQueryStatementContext;
  class FilterStatementContext;
  class LetStatementContext;
  class LetVariableDefinitionListContext;
  class LetVariableDefinitionContext;
  class ForStatementContext;
  class ForItemContext;
  class ForItemAliasContext;
  class ForItemSourceContext;
  class ForOrdinalityOrOffsetContext;
  class OrderByAndPageStatementContext;
  class PrimitiveResultStatementContext;
  class ReturnStatementContext;
  class ReturnStatementBodyContext;
  class ReturnItemListContext;
  class ReturnItemContext;
  class ReturnItemAliasContext;
  class SelectStatementContext;
  class SelectItemListContext;
  class SelectItemContext;
  class SelectItemAliasContext;
  class HavingClauseContext;
  class SelectStatementBodyContext;
  class SelectGraphMatchListContext;
  class SelectGraphMatchContext;
  class SelectQuerySpecificationContext;
  class CallProcedureStatementContext;
  class ProcedureCallContext;
  class InlineProcedureCallContext;
  class VariableScopeClauseContext;
  class BindingVariableReferenceListContext;
  class NamedProcedureCallContext;
  class ProcedureArgumentListContext;
  class ProcedureArgumentContext;
  class AtSchemaClauseContext;
  class UseGraphClauseContext;
  class GraphPatternBindingTableContext;
  class GraphPatternYieldClauseContext;
  class GraphPatternYieldItemListContext;
  class GraphPatternYieldItemContext;
  class GraphPatternContext;
  class MatchModeContext;
  class RepeatableElementsMatchModeContext;
  class DifferentEdgesMatchModeContext;
  class ElementBindingsOrElementsContext;
  class EdgeBindingsOrEdgesContext;
  class PathPatternListContext;
  class PathPatternContext;
  class PathVariableDeclarationContext;
  class KeepClauseContext;
  class GraphPatternWhereClauseContext;
  class InsertGraphPatternContext;
  class InsertPathPatternListContext;
  class InsertPathPatternContext;
  class InsertNodePatternContext;
  class InsertEdgePatternContext;
  class InsertEdgePointingLeftContext;
  class InsertEdgePointingRightContext;
  class InsertEdgeUndirectedContext;
  class InsertElementPatternFillerContext;
  class LabelAndPropertySetSpecificationContext;
  class PathPatternPrefixContext;
  class PathModePrefixContext;
  class PathModeContext;
  class PathSearchPrefixContext;
  class AllPathSearchContext;
  class PathOrPathsContext;
  class AnyPathSearchContext;
  class NumberOfPathsContext;
  class ShortestPathSearchContext;
  class AllShortestPathSearchContext;
  class AnyShortestPathSearchContext;
  class CountedShortestPathSearchContext;
  class CountedShortestGroupSearchContext;
  class NumberOfGroupsContext;
  class PathPatternExpressionContext;
  class PathTermContext;
  class PathFactorContext;
  class PathPrimaryContext;
  class ElementPatternContext;
  class NodePatternContext;
  class ElementPatternFillerContext;
  class ElementVariableDeclarationContext;
  class IsLabelExpressionContext;
  class IsOrColonContext;
  class ElementPatternPredicateContext;
  class ElementPatternWhereClauseContext;
  class ElementPropertySpecificationContext;
  class PropertyKeyValuePairListContext;
  class PropertyKeyValuePairContext;
  class EdgePatternContext;
  class FullEdgePatternContext;
  class FullEdgePointingLeftContext;
  class FullEdgeUndirectedContext;
  class FullEdgePointingRightContext;
  class FullEdgeLeftOrUndirectedContext;
  class FullEdgeUndirectedOrRightContext;
  class FullEdgeLeftOrRightContext;
  class FullEdgeAnyDirectionContext;
  class AbbreviatedEdgePatternContext;
  class ParenthesizedPathPatternExpressionContext;
  class SubpathVariableDeclarationContext;
  class ParenthesizedPathPatternWhereClauseContext;
  class LabelExpressionContext;
  class PathVariableReferenceContext;
  class ElementVariableReferenceContext;
  class GraphPatternQuantifierContext;
  class FixedQuantifierContext;
  class GeneralQuantifierContext;
  class LowerBoundContext;
  class UpperBoundContext;
  class SimplifiedPathPatternExpressionContext;
  class SimplifiedDefaultingLeftContext;
  class SimplifiedDefaultingUndirectedContext;
  class SimplifiedDefaultingRightContext;
  class SimplifiedDefaultingLeftOrUndirectedContext;
  class SimplifiedDefaultingUndirectedOrRightContext;
  class SimplifiedDefaultingLeftOrRightContext;
  class SimplifiedDefaultingAnyDirectionContext;
  class SimplifiedContentsContext;
  class SimplifiedPathUnionContext;
  class SimplifiedMultisetAlternationContext;
  class SimplifiedTermContext;
  class SimplifiedFactorLowContext;
  class SimplifiedFactorHighContext;
  class SimplifiedQuantifiedContext;
  class SimplifiedQuestionedContext;
  class SimplifiedTertiaryContext;
  class SimplifiedDirectionOverrideContext;
  class SimplifiedOverrideLeftContext;
  class SimplifiedOverrideUndirectedContext;
  class SimplifiedOverrideRightContext;
  class SimplifiedOverrideLeftOrUndirectedContext;
  class SimplifiedOverrideUndirectedOrRightContext;
  class SimplifiedOverrideLeftOrRightContext;
  class SimplifiedOverrideAnyDirectionContext;
  class SimplifiedSecondaryContext;
  class SimplifiedNegationContext;
  class SimplifiedPrimaryContext;
  class WhereClauseContext;
  class YieldClauseContext;
  class YieldItemListContext;
  class YieldItemContext;
  class YieldItemNameContext;
  class YieldItemAliasContext;
  class GroupByClauseContext;
  class GroupingElementListContext;
  class GroupingElementContext;
  class EmptyGroupingSetContext;
  class OrderByClauseContext;
  class SortSpecificationListContext;
  class SortSpecificationContext;
  class SortKeyContext;
  class OrderingSpecificationContext;
  class NullOrderingContext;
  class LimitClauseContext;
  class OffsetClauseContext;
  class OffsetSynonymContext;
  class SchemaReferenceContext;
  class AbsoluteCatalogSchemaReferenceContext;
  class CatalogSchemaParentAndNameContext;
  class RelativeCatalogSchemaReferenceContext;
  class PredefinedSchemaReferenceContext;
  class AbsoluteDirectoryPathContext;
  class RelativeDirectoryPathContext;
  class SimpleDirectoryPathContext;
  class GraphReferenceContext;
  class CatalogGraphParentAndNameContext;
  class HomeGraphContext;
  class GraphTypeReferenceContext;
  class CatalogGraphTypeParentAndNameContext;
  class BindingTableReferenceContext;
  class ProcedureReferenceContext;
  class CatalogProcedureParentAndNameContext;
  class CatalogObjectParentReferenceContext;
  class ReferenceParameterSpecificationContext;
  class NestedGraphTypeSpecificationContext;
  class GraphTypeSpecificationBodyContext;
  class ElementTypeListContext;
  class ElementTypeSpecificationContext;
  class NodeTypeSpecificationContext;
  class NodeTypePatternContext;
  class NodeTypePhraseContext;
  class NodeTypePhraseFillerContext;
  class NodeTypeFillerContext;
  class LocalNodeTypeAliasContext;
  class NodeTypeImpliedContentContext;
  class NodeTypeKeyLabelSetContext;
  class NodeTypeLabelSetContext;
  class NodeTypePropertyTypesContext;
  class EdgeTypeSpecificationContext;
  class EdgeTypePatternContext;
  class EdgeTypePhraseContext;
  class EdgeTypePhraseFillerContext;
  class EdgeTypeFillerContext;
  class EdgeTypeImpliedContentContext;
  class EdgeTypeKeyLabelSetContext;
  class EdgeTypeLabelSetContext;
  class EdgeTypePropertyTypesContext;
  class EdgeTypePatternDirectedContext;
  class EdgeTypePatternPointingRightContext;
  class EdgeTypePatternPointingLeftContext;
  class EdgeTypePatternUndirectedContext;
  class ArcTypePointingRightContext;
  class ArcTypePointingLeftContext;
  class ArcTypeUndirectedContext;
  class SourceNodeTypeReferenceContext;
  class DestinationNodeTypeReferenceContext;
  class EdgeKindContext;
  class EndpointPairPhraseContext;
  class EndpointPairContext;
  class EndpointPairDirectedContext;
  class EndpointPairPointingRightContext;
  class EndpointPairPointingLeftContext;
  class EndpointPairUndirectedContext;
  class ConnectorPointingRightContext;
  class ConnectorUndirectedContext;
  class SourceNodeTypeAliasContext;
  class DestinationNodeTypeAliasContext;
  class LabelSetPhraseContext;
  class LabelSetSpecificationContext;
  class PropertyTypesSpecificationContext;
  class PropertyTypeListContext;
  class PropertyTypeContext;
  class PropertyValueTypeContext;
  class BindingTableTypeContext;
  class ValueTypeContext;
  class TypedContext;
  class PredefinedTypeContext;
  class BooleanTypeContext;
  class CharacterStringTypeContext;
  class ByteStringTypeContext;
  class MinLengthContext;
  class MaxLengthContext;
  class FixedLengthContext;
  class NumericTypeContext;
  class ExactNumericTypeContext;
  class BinaryExactNumericTypeContext;
  class SignedBinaryExactNumericTypeContext;
  class UnsignedBinaryExactNumericTypeContext;
  class VerboseBinaryExactNumericTypeContext;
  class DecimalExactNumericTypeContext;
  class PrecisionContext;
  class ScaleContext;
  class ApproximateNumericTypeContext;
  class TemporalTypeContext;
  class TemporalInstantTypeContext;
  class DatetimeTypeContext;
  class LocaldatetimeTypeContext;
  class DateTypeContext;
  class TimeTypeContext;
  class LocaltimeTypeContext;
  class TemporalDurationTypeContext;
  class TemporalDurationQualifierContext;
  class ReferenceValueTypeContext;
  class ImmaterialValueTypeContext;
  class NullTypeContext;
  class EmptyTypeContext;
  class GraphReferenceValueTypeContext;
  class ClosedGraphReferenceValueTypeContext;
  class OpenGraphReferenceValueTypeContext;
  class BindingTableReferenceValueTypeContext;
  class NodeReferenceValueTypeContext;
  class ClosedNodeReferenceValueTypeContext;
  class OpenNodeReferenceValueTypeContext;
  class EdgeReferenceValueTypeContext;
  class ClosedEdgeReferenceValueTypeContext;
  class OpenEdgeReferenceValueTypeContext;
  class PathValueTypeContext;
  class ListValueTypeNameContext;
  class ListValueTypeNameSynonymContext;
  class RecordTypeContext;
  class FieldTypesSpecificationContext;
  class FieldTypeListContext;
  class NotNullContext;
  class FieldTypeContext;
  class SearchConditionContext;
  class PredicateContext;
  class CompOpContext;
  class ExistsPredicateContext;
  class NullPredicateContext;
  class NullPredicatePart2Context;
  class ValueTypePredicateContext;
  class ValueTypePredicatePart2Context;
  class NormalizedPredicatePart2Context;
  class DirectedPredicateContext;
  class DirectedPredicatePart2Context;
  class LabeledPredicateContext;
  class LabeledPredicatePart2Context;
  class IsLabeledOrColonContext;
  class SourceDestinationPredicateContext;
  class NodeReferenceContext;
  class SourcePredicatePart2Context;
  class DestinationPredicatePart2Context;
  class EdgeReferenceContext;
  class All_differentPredicateContext;
  class SamePredicateContext;
  class Property_existsPredicateContext;
  class ValueExpressionContext;
  class ValueFunctionContext;
  class BooleanValueExpressionContext;
  class CharacterOrByteStringFunctionContext;
  class SubCharacterOrByteStringContext;
  class TrimSingleCharacterOrByteStringContext;
  class FoldCharacterStringContext;
  class TrimMultiCharacterCharacterStringContext;
  class NormalizeCharacterStringContext;
  class NodeReferenceValueExpressionContext;
  class EdgeReferenceValueExpressionContext;
  class AggregatingValueExpressionContext;
  class ValueExpressionPrimaryContext;
  class ParenthesizedValueExpressionContext;
  class NonParenthesizedValueExpressionPrimaryContext;
  class NonParenthesizedValueExpressionPrimarySpecialCaseContext;
  class UnsignedValueSpecificationContext;
  class NonNegativeIntegerSpecificationContext;
  class GeneralValueSpecificationContext;
  class DynamicParameterSpecificationContext;
  class LetValueExpressionContext;
  class ValueQueryExpressionContext;
  class CaseExpressionContext;
  class CaseAbbreviationContext;
  class CaseSpecificationContext;
  class SimpleCaseContext;
  class SearchedCaseContext;
  class SimpleWhenClauseContext;
  class SearchedWhenClauseContext;
  class ElseClauseContext;
  class CaseOperandContext;
  class WhenOperandListContext;
  class WhenOperandContext;
  class ResultContext;
  class ResultExpressionContext;
  class CastSpecificationContext;
  class CastOperandContext;
  class CastTargetContext;
  class AggregateFunctionContext;
  class GeneralSetFunctionContext;
  class BinarySetFunctionContext;
  class GeneralSetFunctionTypeContext;
  class SetQuantifierContext;
  class BinarySetFunctionTypeContext;
  class DependentValueExpressionContext;
  class IndependentValueExpressionContext;
  class Element_idFunctionContext;
  class BindingVariableReferenceContext;
  class PathValueExpressionContext;
  class PathValueConstructorContext;
  class PathValueConstructorByEnumerationContext;
  class PathElementListContext;
  class PathElementListStartContext;
  class PathElementListStepContext;
  class ListValueExpressionContext;
  class ListValueFunctionContext;
  class TrimListFunctionContext;
  class ElementsFunctionContext;
  class ListValueConstructorContext;
  class ListValueConstructorByEnumerationContext;
  class ListElementListContext;
  class ListElementContext;
  class RecordConstructorContext;
  class FieldsSpecificationContext;
  class FieldListContext;
  class FieldContext;
  class TruthValueContext;
  class NumericValueExpressionContext;
  class NumericValueFunctionContext;
  class LengthExpressionContext;
  class CardinalityExpressionContext;
  class CardinalityExpressionArgumentContext;
  class CharLengthExpressionContext;
  class ByteLengthExpressionContext;
  class PathLengthExpressionContext;
  class AbsoluteValueExpressionContext;
  class ModulusExpressionContext;
  class NumericValueExpressionDividendContext;
  class NumericValueExpressionDivisorContext;
  class TrigonometricFunctionContext;
  class TrigonometricFunctionNameContext;
  class GeneralLogarithmFunctionContext;
  class GeneralLogarithmBaseContext;
  class GeneralLogarithmArgumentContext;
  class CommonLogarithmContext;
  class NaturalLogarithmContext;
  class ExponentialFunctionContext;
  class PowerFunctionContext;
  class NumericValueExpressionBaseContext;
  class NumericValueExpressionExponentContext;
  class SquareRootContext;
  class FloorFunctionContext;
  class CeilingFunctionContext;
  class CharacterStringValueExpressionContext;
  class ByteStringValueExpressionContext;
  class TrimOperandsContext;
  class TrimCharacterOrByteStringSourceContext;
  class TrimSpecificationContext;
  class TrimCharacterOrByteStringContext;
  class NormalFormContext;
  class StringLengthContext;
  class DatetimeValueExpressionContext;
  class DatetimeValueFunctionContext;
  class DateFunctionContext;
  class TimeFunctionContext;
  class LocaltimeFunctionContext;
  class DatetimeFunctionContext;
  class LocaldatetimeFunctionContext;
  class DateFunctionParametersContext;
  class TimeFunctionParametersContext;
  class DatetimeFunctionParametersContext;
  class DurationValueExpressionContext;
  class DatetimeSubtractionContext;
  class DatetimeSubtractionParametersContext;
  class DatetimeValueExpression1Context;
  class DatetimeValueExpression2Context;
  class DurationValueFunctionContext;
  class DurationFunctionContext;
  class DurationFunctionParametersContext;
  class ObjectNameContext;
  class ObjectNameOrBindingVariableContext;
  class DirectoryNameContext;
  class SchemaNameContext;
  class GraphNameContext;
  class DelimitedGraphNameContext;
  class GraphTypeNameContext;
  class NodeTypeNameContext;
  class EdgeTypeNameContext;
  class BindingTableNameContext;
  class DelimitedBindingTableNameContext;
  class ProcedureNameContext;
  class LabelNameContext;
  class PropertyNameContext;
  class FieldNameContext;
  class ElementVariableContext;
  class PathVariableContext;
  class SubpathVariableContext;
  class BindingVariableContext;
  class UnsignedLiteralContext;
  class GeneralLiteralContext;
  class TemporalLiteralContext;
  class DateLiteralContext;
  class TimeLiteralContext;
  class DatetimeLiteralContext;
  class ListLiteralContext;
  class RecordLiteralContext;
  class IdentifierContext;
  class RegularIdentifierContext;
  class TimeZoneStringContext;
  class CharacterStringLiteralContext;
  class UnsignedNumericLiteralContext;
  class ExactNumericLiteralContext;
  class ApproximateNumericLiteralContext;
  class UnsignedIntegerContext;
  class UnsignedDecimalIntegerContext;
  class NullLiteralContext;
  class DateStringContext;
  class TimeStringContext;
  class DatetimeStringContext;
  class DurationLiteralContext;
  class DurationStringContext;
  class NodeSynonymContext;
  class EdgesSynonymContext;
  class EdgeSynonymContext;
  class NonReservedWordsContext; 

  class  GqlProgramContext : public antlr4::ParserRuleContext {
  public:
    GqlProgramContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    ProgramActivityContext *programActivity();
    antlr4::tree::TerminalNode *EOF();
    SessionCloseCommandContext *sessionCloseCommand();

   
  };

  GqlProgramContext* gqlProgram();

  class  ProgramActivityContext : public antlr4::ParserRuleContext {
  public:
    ProgramActivityContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    SessionActivityContext *sessionActivity();
    TransactionActivityContext *transactionActivity();

   
  };

  ProgramActivityContext* programActivity();

  class  SessionActivityContext : public antlr4::ParserRuleContext {
  public:
    SessionActivityContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    std::vector<SessionResetCommandContext *> sessionResetCommand();
    SessionResetCommandContext* sessionResetCommand(size_t i);
    std::vector<SessionSetCommandContext *> sessionSetCommand();
    SessionSetCommandContext* sessionSetCommand(size_t i);

   
  };

  SessionActivityContext* sessionActivity();

  class  TransactionActivityContext : public antlr4::ParserRuleContext {
  public:
    TransactionActivityContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    StartTransactionCommandContext *startTransactionCommand();
    ProcedureSpecificationContext *procedureSpecification();
    EndTransactionCommandContext *endTransactionCommand();

   
  };

  TransactionActivityContext* transactionActivity();

  class  EndTransactionCommandContext : public antlr4::ParserRuleContext {
  public:
    EndTransactionCommandContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    RollbackCommandContext *rollbackCommand();
    CommitCommandContext *commitCommand();

   
  };

  EndTransactionCommandContext* endTransactionCommand();

  class  SessionSetCommandContext : public antlr4::ParserRuleContext {
  public:
    SessionSetCommandContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *SESSION();
    antlr4::tree::TerminalNode *SET();
    SessionSetSchemaClauseContext *sessionSetSchemaClause();
    SessionSetGraphClauseContext *sessionSetGraphClause();
    SessionSetTimeZoneClauseContext *sessionSetTimeZoneClause();
    SessionSetParameterClauseContext *sessionSetParameterClause();

   
  };

  SessionSetCommandContext* sessionSetCommand();

  class  SessionSetSchemaClauseContext : public antlr4::ParserRuleContext {
  public:
    SessionSetSchemaClauseContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *SCHEMA();
    SchemaReferenceContext *schemaReference();

   
  };

  SessionSetSchemaClauseContext* sessionSetSchemaClause();

  class  SessionSetGraphClauseContext : public antlr4::ParserRuleContext {
  public:
    SessionSetGraphClauseContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *GRAPH();
    GraphExpressionContext *graphExpression();
    antlr4::tree::TerminalNode *PROPERTY();

   
  };

  SessionSetGraphClauseContext* sessionSetGraphClause();

  class  SessionSetTimeZoneClauseContext : public antlr4::ParserRuleContext {
  public:
    SessionSetTimeZoneClauseContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *TIME();
    antlr4::tree::TerminalNode *ZONE();
    SetTimeZoneValueContext *setTimeZoneValue();

   
  };

  SessionSetTimeZoneClauseContext* sessionSetTimeZoneClause();

  class  SetTimeZoneValueContext : public antlr4::ParserRuleContext {
  public:
    SetTimeZoneValueContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    TimeZoneStringContext *timeZoneString();

   
  };

  SetTimeZoneValueContext* setTimeZoneValue();

  class  SessionSetParameterClauseContext : public antlr4::ParserRuleContext {
  public:
    SessionSetParameterClauseContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    SessionSetGraphParameterClauseContext *sessionSetGraphParameterClause();
    SessionSetBindingTableParameterClauseContext *sessionSetBindingTableParameterClause();
    SessionSetValueParameterClauseContext *sessionSetValueParameterClause();

   
  };

  SessionSetParameterClauseContext* sessionSetParameterClause();

  class  SessionSetGraphParameterClauseContext : public antlr4::ParserRuleContext {
  public:
    SessionSetGraphParameterClauseContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *GRAPH();
    SessionSetParameterNameContext *sessionSetParameterName();
    OptTypedGraphInitializerContext *optTypedGraphInitializer();
    antlr4::tree::TerminalNode *PROPERTY();

   
  };

  SessionSetGraphParameterClauseContext* sessionSetGraphParameterClause();

  class  SessionSetBindingTableParameterClauseContext : public antlr4::ParserRuleContext {
  public:
    SessionSetBindingTableParameterClauseContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *TABLE();
    SessionSetParameterNameContext *sessionSetParameterName();
    OptTypedBindingTableInitializerContext *optTypedBindingTableInitializer();
    antlr4::tree::TerminalNode *BINDING();

   
  };

  SessionSetBindingTableParameterClauseContext* sessionSetBindingTableParameterClause();

  class  SessionSetValueParameterClauseContext : public antlr4::ParserRuleContext {
  public:
    SessionSetValueParameterClauseContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *VALUE();
    SessionSetParameterNameContext *sessionSetParameterName();
    OptTypedValueInitializerContext *optTypedValueInitializer();

   
  };

  SessionSetValueParameterClauseContext* sessionSetValueParameterClause();

  class  SessionSetParameterNameContext : public antlr4::ParserRuleContext {
  public:
    SessionSetParameterNameContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    SessionParameterSpecificationContext *sessionParameterSpecification();
    antlr4::tree::TerminalNode *IF();
    antlr4::tree::TerminalNode *NOT();
    antlr4::tree::TerminalNode *EXISTS();

   
  };

  SessionSetParameterNameContext* sessionSetParameterName();

  class  SessionResetCommandContext : public antlr4::ParserRuleContext {
  public:
    SessionResetCommandContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *SESSION();
    antlr4::tree::TerminalNode *RESET();
    SessionResetArgumentsContext *sessionResetArguments();

   
  };

  SessionResetCommandContext* sessionResetCommand();

  class  SessionResetArgumentsContext : public antlr4::ParserRuleContext {
  public:
    SessionResetArgumentsContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *PARAMETERS();
    antlr4::tree::TerminalNode *CHARACTERISTICS();
    antlr4::tree::TerminalNode *ALL();
    antlr4::tree::TerminalNode *SCHEMA();
    antlr4::tree::TerminalNode *GRAPH();
    antlr4::tree::TerminalNode *PROPERTY();
    antlr4::tree::TerminalNode *TIME();
    antlr4::tree::TerminalNode *ZONE();
    SessionParameterSpecificationContext *sessionParameterSpecification();
    antlr4::tree::TerminalNode *PARAMETER();

   
  };

  SessionResetArgumentsContext* sessionResetArguments();

  class  SessionCloseCommandContext : public antlr4::ParserRuleContext {
  public:
    SessionCloseCommandContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *SESSION();
    antlr4::tree::TerminalNode *CLOSE();

   
  };

  SessionCloseCommandContext* sessionCloseCommand();

  class  SessionParameterSpecificationContext : public antlr4::ParserRuleContext {
  public:
    SessionParameterSpecificationContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *GENERAL_PARAMETER_REFERENCE();

   
  };

  SessionParameterSpecificationContext* sessionParameterSpecification();

  class  StartTransactionCommandContext : public antlr4::ParserRuleContext {
  public:
    StartTransactionCommandContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *START();
    antlr4::tree::TerminalNode *TRANSACTION();
    TransactionCharacteristicsContext *transactionCharacteristics();

   
  };

  StartTransactionCommandContext* startTransactionCommand();

  class  TransactionCharacteristicsContext : public antlr4::ParserRuleContext {
  public:
    TransactionCharacteristicsContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    std::vector<TransactionModeContext *> transactionMode();
    TransactionModeContext* transactionMode(size_t i);
    std::vector<antlr4::tree::TerminalNode *> COMMA();
    antlr4::tree::TerminalNode* COMMA(size_t i);

   
  };

  TransactionCharacteristicsContext* transactionCharacteristics();

  class  TransactionModeContext : public antlr4::ParserRuleContext {
  public:
    TransactionModeContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    TransactionAccessModeContext *transactionAccessMode();

   
  };

  TransactionModeContext* transactionMode();

  class  TransactionAccessModeContext : public antlr4::ParserRuleContext {
  public:
    TransactionAccessModeContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *READ();
    antlr4::tree::TerminalNode *ONLY();
    antlr4::tree::TerminalNode *WRITE();

   
  };

  TransactionAccessModeContext* transactionAccessMode();

  class  RollbackCommandContext : public antlr4::ParserRuleContext {
  public:
    RollbackCommandContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *ROLLBACK();

   
  };

  RollbackCommandContext* rollbackCommand();

  class  CommitCommandContext : public antlr4::ParserRuleContext {
  public:
    CommitCommandContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *COMMIT();

   
  };

  CommitCommandContext* commitCommand();

  class  NestedProcedureSpecificationContext : public antlr4::ParserRuleContext {
  public:
    NestedProcedureSpecificationContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LEFT_BRACE();
    ProcedureSpecificationContext *procedureSpecification();
    antlr4::tree::TerminalNode *RIGHT_BRACE();

   
  };

  NestedProcedureSpecificationContext* nestedProcedureSpecification();

  class  ProcedureSpecificationContext : public antlr4::ParserRuleContext {
  public:
    ProcedureSpecificationContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    ProcedureBodyContext *procedureBody();

   
  };

  ProcedureSpecificationContext* procedureSpecification();

  class  NestedDataModifyingProcedureSpecificationContext : public antlr4::ParserRuleContext {
  public:
    NestedDataModifyingProcedureSpecificationContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LEFT_BRACE();
    ProcedureBodyContext *procedureBody();
    antlr4::tree::TerminalNode *RIGHT_BRACE();

   
  };

  NestedDataModifyingProcedureSpecificationContext* nestedDataModifyingProcedureSpecification();

  class  NestedQuerySpecificationContext : public antlr4::ParserRuleContext {
  public:
    NestedQuerySpecificationContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LEFT_BRACE();
    ProcedureBodyContext *procedureBody();
    antlr4::tree::TerminalNode *RIGHT_BRACE();

   
  };

  NestedQuerySpecificationContext* nestedQuerySpecification();

  class  ProcedureBodyContext : public antlr4::ParserRuleContext {
  public:
    ProcedureBodyContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    StatementBlockContext *statementBlock();
    AtSchemaClauseContext *atSchemaClause();
    BindingVariableDefinitionBlockContext *bindingVariableDefinitionBlock();

   
  };

  ProcedureBodyContext* procedureBody();

  class  BindingVariableDefinitionBlockContext : public antlr4::ParserRuleContext {
  public:
    BindingVariableDefinitionBlockContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    std::vector<BindingVariableDefinitionContext *> bindingVariableDefinition();
    BindingVariableDefinitionContext* bindingVariableDefinition(size_t i);

   
  };

  BindingVariableDefinitionBlockContext* bindingVariableDefinitionBlock();

  class  BindingVariableDefinitionContext : public antlr4::ParserRuleContext {
  public:
    BindingVariableDefinitionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    GraphVariableDefinitionContext *graphVariableDefinition();
    BindingTableVariableDefinitionContext *bindingTableVariableDefinition();
    ValueVariableDefinitionContext *valueVariableDefinition();

   
  };

  BindingVariableDefinitionContext* bindingVariableDefinition();

  class  StatementBlockContext : public antlr4::ParserRuleContext {
  public:
    StatementBlockContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    StatementContext *statement();
    std::vector<NextStatementContext *> nextStatement();
    NextStatementContext* nextStatement(size_t i);

   
  };

  StatementBlockContext* statementBlock();

  class  StatementContext : public antlr4::ParserRuleContext {
  public:
    StatementContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    LinearCatalogModifyingStatementContext *linearCatalogModifyingStatement();
    LinearDataModifyingStatementContext *linearDataModifyingStatement();
    CompositeQueryStatementContext *compositeQueryStatement();

   
  };

  StatementContext* statement();

  class  NextStatementContext : public antlr4::ParserRuleContext {
  public:
    NextStatementContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *NEXT();
    StatementContext *statement();
    YieldClauseContext *yieldClause();

   
  };

  NextStatementContext* nextStatement();

  class  GraphVariableDefinitionContext : public antlr4::ParserRuleContext {
  public:
    GraphVariableDefinitionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *GRAPH();
    BindingVariableContext *bindingVariable();
    OptTypedGraphInitializerContext *optTypedGraphInitializer();
    antlr4::tree::TerminalNode *PROPERTY();

   
  };

  GraphVariableDefinitionContext* graphVariableDefinition();

  class  OptTypedGraphInitializerContext : public antlr4::ParserRuleContext {
  public:
    OptTypedGraphInitializerContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    GraphInitializerContext *graphInitializer();
    GraphReferenceValueTypeContext *graphReferenceValueType();
    TypedContext *typed();

   
  };

  OptTypedGraphInitializerContext* optTypedGraphInitializer();

  class  GraphInitializerContext : public antlr4::ParserRuleContext {
  public:
    GraphInitializerContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *EQUALS_OPERATOR();
    GraphExpressionContext *graphExpression();

   
  };

  GraphInitializerContext* graphInitializer();

  class  BindingTableVariableDefinitionContext : public antlr4::ParserRuleContext {
  public:
    BindingTableVariableDefinitionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *TABLE();
    BindingVariableContext *bindingVariable();
    OptTypedBindingTableInitializerContext *optTypedBindingTableInitializer();
    antlr4::tree::TerminalNode *BINDING();

   
  };

  BindingTableVariableDefinitionContext* bindingTableVariableDefinition();

  class  OptTypedBindingTableInitializerContext : public antlr4::ParserRuleContext {
  public:
    OptTypedBindingTableInitializerContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    BindingTableInitializerContext *bindingTableInitializer();
    BindingTableReferenceValueTypeContext *bindingTableReferenceValueType();
    TypedContext *typed();

   
  };

  OptTypedBindingTableInitializerContext* optTypedBindingTableInitializer();

  class  BindingTableInitializerContext : public antlr4::ParserRuleContext {
  public:
    BindingTableInitializerContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *EQUALS_OPERATOR();
    BindingTableExpressionContext *bindingTableExpression();

   
  };

  BindingTableInitializerContext* bindingTableInitializer();

  class  ValueVariableDefinitionContext : public antlr4::ParserRuleContext {
  public:
    ValueVariableDefinitionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *VALUE();
    BindingVariableContext *bindingVariable();
    OptTypedValueInitializerContext *optTypedValueInitializer();

   
  };

  ValueVariableDefinitionContext* valueVariableDefinition();

  class  OptTypedValueInitializerContext : public antlr4::ParserRuleContext {
  public:
    OptTypedValueInitializerContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    ValueInitializerContext *valueInitializer();
    ValueTypeContext *valueType();
    TypedContext *typed();

   
  };

  OptTypedValueInitializerContext* optTypedValueInitializer();

  class  ValueInitializerContext : public antlr4::ParserRuleContext {
  public:
    ValueInitializerContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *EQUALS_OPERATOR();
    ValueExpressionContext *valueExpression();

   
  };

  ValueInitializerContext* valueInitializer();

  class  GraphExpressionContext : public antlr4::ParserRuleContext {
  public:
    GraphExpressionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    GraphReferenceContext *graphReference();
    ObjectExpressionPrimaryContext *objectExpressionPrimary();
    ObjectNameOrBindingVariableContext *objectNameOrBindingVariable();
    CurrentGraphContext *currentGraph();

   
  };

  GraphExpressionContext* graphExpression();

  class  CurrentGraphContext : public antlr4::ParserRuleContext {
  public:
    CurrentGraphContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *CURRENT_PROPERTY_GRAPH();
    antlr4::tree::TerminalNode *CURRENT_GRAPH();

   
  };

  CurrentGraphContext* currentGraph();

  class  BindingTableExpressionContext : public antlr4::ParserRuleContext {
  public:
    BindingTableExpressionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    NestedBindingTableQuerySpecificationContext *nestedBindingTableQuerySpecification();
    BindingTableReferenceContext *bindingTableReference();
    ObjectExpressionPrimaryContext *objectExpressionPrimary();
    ObjectNameOrBindingVariableContext *objectNameOrBindingVariable();

   
  };

  BindingTableExpressionContext* bindingTableExpression();

  class  NestedBindingTableQuerySpecificationContext : public antlr4::ParserRuleContext {
  public:
    NestedBindingTableQuerySpecificationContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    NestedQuerySpecificationContext *nestedQuerySpecification();

   
  };

  NestedBindingTableQuerySpecificationContext* nestedBindingTableQuerySpecification();

  class  ObjectExpressionPrimaryContext : public antlr4::ParserRuleContext {
  public:
    ObjectExpressionPrimaryContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *VARIABLE();
    ValueExpressionPrimaryContext *valueExpressionPrimary();
    ParenthesizedValueExpressionContext *parenthesizedValueExpression();
    NonParenthesizedValueExpressionPrimarySpecialCaseContext *nonParenthesizedValueExpressionPrimarySpecialCase();

   
  };

  ObjectExpressionPrimaryContext* objectExpressionPrimary();

  class  LinearCatalogModifyingStatementContext : public antlr4::ParserRuleContext {
  public:
    LinearCatalogModifyingStatementContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    std::vector<SimpleCatalogModifyingStatementContext *> simpleCatalogModifyingStatement();
    SimpleCatalogModifyingStatementContext* simpleCatalogModifyingStatement(size_t i);

   
  };

  LinearCatalogModifyingStatementContext* linearCatalogModifyingStatement();

  class  SimpleCatalogModifyingStatementContext : public antlr4::ParserRuleContext {
  public:
    SimpleCatalogModifyingStatementContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    PrimitiveCatalogModifyingStatementContext *primitiveCatalogModifyingStatement();
    CallCatalogModifyingProcedureStatementContext *callCatalogModifyingProcedureStatement();

   
  };

  SimpleCatalogModifyingStatementContext* simpleCatalogModifyingStatement();

  class  PrimitiveCatalogModifyingStatementContext : public antlr4::ParserRuleContext {
  public:
    PrimitiveCatalogModifyingStatementContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    CreateSchemaStatementContext *createSchemaStatement();
    DropSchemaStatementContext *dropSchemaStatement();
    CreateGraphStatementContext *createGraphStatement();
    DropGraphStatementContext *dropGraphStatement();
    CreateGraphTypeStatementContext *createGraphTypeStatement();
    DropGraphTypeStatementContext *dropGraphTypeStatement();

   
  };

  PrimitiveCatalogModifyingStatementContext* primitiveCatalogModifyingStatement();

  class  CreateSchemaStatementContext : public antlr4::ParserRuleContext {
  public:
    CreateSchemaStatementContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *CREATE();
    antlr4::tree::TerminalNode *SCHEMA();
    CatalogSchemaParentAndNameContext *catalogSchemaParentAndName();
    antlr4::tree::TerminalNode *IF();
    antlr4::tree::TerminalNode *NOT();
    antlr4::tree::TerminalNode *EXISTS();

   
  };

  CreateSchemaStatementContext* createSchemaStatement();

  class  DropSchemaStatementContext : public antlr4::ParserRuleContext {
  public:
    DropSchemaStatementContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *DROP();
    antlr4::tree::TerminalNode *SCHEMA();
    CatalogSchemaParentAndNameContext *catalogSchemaParentAndName();
    antlr4::tree::TerminalNode *IF();
    antlr4::tree::TerminalNode *EXISTS();

   
  };

  DropSchemaStatementContext* dropSchemaStatement();

  class  CreateGraphStatementContext : public antlr4::ParserRuleContext {
  public:
    CreateGraphStatementContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *CREATE();
    CatalogGraphParentAndNameContext *catalogGraphParentAndName();
    antlr4::tree::TerminalNode *GRAPH();
    antlr4::tree::TerminalNode *OR();
    antlr4::tree::TerminalNode *REPLACE();
    OpenGraphTypeContext *openGraphType();
    OfGraphTypeContext *ofGraphType();
    GraphSourceContext *graphSource();
    antlr4::tree::TerminalNode *PROPERTY();
    antlr4::tree::TerminalNode *IF();
    antlr4::tree::TerminalNode *NOT();
    antlr4::tree::TerminalNode *EXISTS();

   
  };

  CreateGraphStatementContext* createGraphStatement();

  class  OpenGraphTypeContext : public antlr4::ParserRuleContext {
  public:
    OpenGraphTypeContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *ANY();
    TypedContext *typed();
    antlr4::tree::TerminalNode *GRAPH();
    antlr4::tree::TerminalNode *PROPERTY();

   
  };

  OpenGraphTypeContext* openGraphType();

  class  OfGraphTypeContext : public antlr4::ParserRuleContext {
  public:
    OfGraphTypeContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    GraphTypeLikeGraphContext *graphTypeLikeGraph();
    GraphTypeReferenceContext *graphTypeReference();
    TypedContext *typed();
    NestedGraphTypeSpecificationContext *nestedGraphTypeSpecification();
    antlr4::tree::TerminalNode *GRAPH();
    antlr4::tree::TerminalNode *PROPERTY();

   
  };

  OfGraphTypeContext* ofGraphType();

  class  GraphTypeLikeGraphContext : public antlr4::ParserRuleContext {
  public:
    GraphTypeLikeGraphContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LIKE();
    GraphExpressionContext *graphExpression();

   
  };

  GraphTypeLikeGraphContext* graphTypeLikeGraph();

  class  GraphSourceContext : public antlr4::ParserRuleContext {
  public:
    GraphSourceContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *AS();
    antlr4::tree::TerminalNode *COPY();
    antlr4::tree::TerminalNode *OF();
    GraphExpressionContext *graphExpression();

   
  };

  GraphSourceContext* graphSource();

  class  DropGraphStatementContext : public antlr4::ParserRuleContext {
  public:
    DropGraphStatementContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *DROP();
    antlr4::tree::TerminalNode *GRAPH();
    CatalogGraphParentAndNameContext *catalogGraphParentAndName();
    antlr4::tree::TerminalNode *PROPERTY();
    antlr4::tree::TerminalNode *IF();
    antlr4::tree::TerminalNode *EXISTS();

   
  };

  DropGraphStatementContext* dropGraphStatement();

  class  CreateGraphTypeStatementContext : public antlr4::ParserRuleContext {
  public:
    CreateGraphTypeStatementContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *CREATE();
    CatalogGraphTypeParentAndNameContext *catalogGraphTypeParentAndName();
    GraphTypeSourceContext *graphTypeSource();
    antlr4::tree::TerminalNode *GRAPH();
    antlr4::tree::TerminalNode *TYPE();
    antlr4::tree::TerminalNode *OR();
    antlr4::tree::TerminalNode *REPLACE();
    antlr4::tree::TerminalNode *PROPERTY();
    antlr4::tree::TerminalNode *IF();
    antlr4::tree::TerminalNode *NOT();
    antlr4::tree::TerminalNode *EXISTS();

   
  };

  CreateGraphTypeStatementContext* createGraphTypeStatement();

  class  GraphTypeSourceContext : public antlr4::ParserRuleContext {
  public:
    GraphTypeSourceContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    CopyOfGraphTypeContext *copyOfGraphType();
    antlr4::tree::TerminalNode *AS();
    GraphTypeLikeGraphContext *graphTypeLikeGraph();
    NestedGraphTypeSpecificationContext *nestedGraphTypeSpecification();

   
  };

  GraphTypeSourceContext* graphTypeSource();

  class  CopyOfGraphTypeContext : public antlr4::ParserRuleContext {
  public:
    CopyOfGraphTypeContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *COPY();
    antlr4::tree::TerminalNode *OF();
    GraphTypeReferenceContext *graphTypeReference();

   
  };

  CopyOfGraphTypeContext* copyOfGraphType();

  class  DropGraphTypeStatementContext : public antlr4::ParserRuleContext {
  public:
    DropGraphTypeStatementContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *DROP();
    antlr4::tree::TerminalNode *GRAPH();
    antlr4::tree::TerminalNode *TYPE();
    CatalogGraphTypeParentAndNameContext *catalogGraphTypeParentAndName();
    antlr4::tree::TerminalNode *PROPERTY();
    antlr4::tree::TerminalNode *IF();
    antlr4::tree::TerminalNode *EXISTS();

   
  };

  DropGraphTypeStatementContext* dropGraphTypeStatement();

  class  CallCatalogModifyingProcedureStatementContext : public antlr4::ParserRuleContext {
  public:
    CallCatalogModifyingProcedureStatementContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    CallProcedureStatementContext *callProcedureStatement();

   
  };

  CallCatalogModifyingProcedureStatementContext* callCatalogModifyingProcedureStatement();

  class  LinearDataModifyingStatementContext : public antlr4::ParserRuleContext {
  public:
    LinearDataModifyingStatementContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    FocusedLinearDataModifyingStatementContext *focusedLinearDataModifyingStatement();
    AmbientLinearDataModifyingStatementContext *ambientLinearDataModifyingStatement();

   
  };

  LinearDataModifyingStatementContext* linearDataModifyingStatement();

  class  FocusedLinearDataModifyingStatementContext : public antlr4::ParserRuleContext {
  public:
    FocusedLinearDataModifyingStatementContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    FocusedLinearDataModifyingStatementBodyContext *focusedLinearDataModifyingStatementBody();
    FocusedNestedDataModifyingProcedureSpecificationContext *focusedNestedDataModifyingProcedureSpecification();

   
  };

  FocusedLinearDataModifyingStatementContext* focusedLinearDataModifyingStatement();

  class  FocusedLinearDataModifyingStatementBodyContext : public antlr4::ParserRuleContext {
  public:
    FocusedLinearDataModifyingStatementBodyContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    UseGraphClauseContext *useGraphClause();
    SimpleLinearDataAccessingStatementContext *simpleLinearDataAccessingStatement();
    PrimitiveResultStatementContext *primitiveResultStatement();

   
  };

  FocusedLinearDataModifyingStatementBodyContext* focusedLinearDataModifyingStatementBody();

  class  FocusedNestedDataModifyingProcedureSpecificationContext : public antlr4::ParserRuleContext {
  public:
    FocusedNestedDataModifyingProcedureSpecificationContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    UseGraphClauseContext *useGraphClause();
    NestedDataModifyingProcedureSpecificationContext *nestedDataModifyingProcedureSpecification();

   
  };

  FocusedNestedDataModifyingProcedureSpecificationContext* focusedNestedDataModifyingProcedureSpecification();

  class  AmbientLinearDataModifyingStatementContext : public antlr4::ParserRuleContext {
  public:
    AmbientLinearDataModifyingStatementContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    AmbientLinearDataModifyingStatementBodyContext *ambientLinearDataModifyingStatementBody();
    NestedDataModifyingProcedureSpecificationContext *nestedDataModifyingProcedureSpecification();

   
  };

  AmbientLinearDataModifyingStatementContext* ambientLinearDataModifyingStatement();

  class  AmbientLinearDataModifyingStatementBodyContext : public antlr4::ParserRuleContext {
  public:
    AmbientLinearDataModifyingStatementBodyContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    SimpleLinearDataAccessingStatementContext *simpleLinearDataAccessingStatement();
    PrimitiveResultStatementContext *primitiveResultStatement();

   
  };

  AmbientLinearDataModifyingStatementBodyContext* ambientLinearDataModifyingStatementBody();

  class  SimpleLinearDataAccessingStatementContext : public antlr4::ParserRuleContext {
  public:
    SimpleLinearDataAccessingStatementContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    std::vector<SimpleDataAccessingStatementContext *> simpleDataAccessingStatement();
    SimpleDataAccessingStatementContext* simpleDataAccessingStatement(size_t i);

   
  };

  SimpleLinearDataAccessingStatementContext* simpleLinearDataAccessingStatement();

  class  SimpleDataAccessingStatementContext : public antlr4::ParserRuleContext {
  public:
    SimpleDataAccessingStatementContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    SimpleQueryStatementContext *simpleQueryStatement();
    SimpleDataModifyingStatementContext *simpleDataModifyingStatement();

   
  };

  SimpleDataAccessingStatementContext* simpleDataAccessingStatement();

  class  SimpleDataModifyingStatementContext : public antlr4::ParserRuleContext {
  public:
    SimpleDataModifyingStatementContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    PrimitiveDataModifyingStatementContext *primitiveDataModifyingStatement();
    CallDataModifyingProcedureStatementContext *callDataModifyingProcedureStatement();

   
  };

  SimpleDataModifyingStatementContext* simpleDataModifyingStatement();

  class  PrimitiveDataModifyingStatementContext : public antlr4::ParserRuleContext {
  public:
    PrimitiveDataModifyingStatementContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    InsertStatementContext *insertStatement();
    SetStatementContext *setStatement();
    RemoveStatementContext *removeStatement();
    DeleteStatementContext *deleteStatement();

   
  };

  PrimitiveDataModifyingStatementContext* primitiveDataModifyingStatement();

  class  InsertStatementContext : public antlr4::ParserRuleContext {
  public:
    InsertStatementContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *INSERT();
    InsertGraphPatternContext *insertGraphPattern();

   
  };

  InsertStatementContext* insertStatement();

  class  SetStatementContext : public antlr4::ParserRuleContext {
  public:
    SetStatementContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *SET();
    SetItemListContext *setItemList();

   
  };

  SetStatementContext* setStatement();

  class  SetItemListContext : public antlr4::ParserRuleContext {
  public:
    SetItemListContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    std::vector<SetItemContext *> setItem();
    SetItemContext* setItem(size_t i);
    std::vector<antlr4::tree::TerminalNode *> COMMA();
    antlr4::tree::TerminalNode* COMMA(size_t i);

   
  };

  SetItemListContext* setItemList();

  class  SetItemContext : public antlr4::ParserRuleContext {
  public:
    SetItemContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    SetPropertyItemContext *setPropertyItem();
    SetAllPropertiesItemContext *setAllPropertiesItem();
    SetLabelItemContext *setLabelItem();

   
  };

  SetItemContext* setItem();

  class  SetPropertyItemContext : public antlr4::ParserRuleContext {
  public:
    SetPropertyItemContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    BindingVariableReferenceContext *bindingVariableReference();
    antlr4::tree::TerminalNode *PERIOD();
    PropertyNameContext *propertyName();
    antlr4::tree::TerminalNode *EQUALS_OPERATOR();
    ValueExpressionContext *valueExpression();

   
  };

  SetPropertyItemContext* setPropertyItem();

  class  SetAllPropertiesItemContext : public antlr4::ParserRuleContext {
  public:
    SetAllPropertiesItemContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    BindingVariableReferenceContext *bindingVariableReference();
    antlr4::tree::TerminalNode *EQUALS_OPERATOR();
    antlr4::tree::TerminalNode *LEFT_BRACE();
    antlr4::tree::TerminalNode *RIGHT_BRACE();
    PropertyKeyValuePairListContext *propertyKeyValuePairList();

   
  };

  SetAllPropertiesItemContext* setAllPropertiesItem();

  class  SetLabelItemContext : public antlr4::ParserRuleContext {
  public:
    SetLabelItemContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    BindingVariableReferenceContext *bindingVariableReference();
    IsOrColonContext *isOrColon();
    LabelNameContext *labelName();

   
  };

  SetLabelItemContext* setLabelItem();

  class  RemoveStatementContext : public antlr4::ParserRuleContext {
  public:
    RemoveStatementContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *REMOVE();
    RemoveItemListContext *removeItemList();

   
  };

  RemoveStatementContext* removeStatement();

  class  RemoveItemListContext : public antlr4::ParserRuleContext {
  public:
    RemoveItemListContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    std::vector<RemoveItemContext *> removeItem();
    RemoveItemContext* removeItem(size_t i);
    std::vector<antlr4::tree::TerminalNode *> COMMA();
    antlr4::tree::TerminalNode* COMMA(size_t i);

   
  };

  RemoveItemListContext* removeItemList();

  class  RemoveItemContext : public antlr4::ParserRuleContext {
  public:
    RemoveItemContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    RemovePropertyItemContext *removePropertyItem();
    RemoveLabelItemContext *removeLabelItem();

   
  };

  RemoveItemContext* removeItem();

  class  RemovePropertyItemContext : public antlr4::ParserRuleContext {
  public:
    RemovePropertyItemContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    BindingVariableReferenceContext *bindingVariableReference();
    antlr4::tree::TerminalNode *PERIOD();
    PropertyNameContext *propertyName();

   
  };

  RemovePropertyItemContext* removePropertyItem();

  class  RemoveLabelItemContext : public antlr4::ParserRuleContext {
  public:
    RemoveLabelItemContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    BindingVariableReferenceContext *bindingVariableReference();
    IsOrColonContext *isOrColon();
    LabelNameContext *labelName();

   
  };

  RemoveLabelItemContext* removeLabelItem();

  class  DeleteStatementContext : public antlr4::ParserRuleContext {
  public:
    DeleteStatementContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *DELETE();
    DeleteItemListContext *deleteItemList();
    antlr4::tree::TerminalNode *DETACH();
    antlr4::tree::TerminalNode *NODETACH();

   
  };

  DeleteStatementContext* deleteStatement();

  class  DeleteItemListContext : public antlr4::ParserRuleContext {
  public:
    DeleteItemListContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    std::vector<DeleteItemContext *> deleteItem();
    DeleteItemContext* deleteItem(size_t i);
    std::vector<antlr4::tree::TerminalNode *> COMMA();
    antlr4::tree::TerminalNode* COMMA(size_t i);

   
  };

  DeleteItemListContext* deleteItemList();

  class  DeleteItemContext : public antlr4::ParserRuleContext {
  public:
    DeleteItemContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    ValueExpressionContext *valueExpression();

   
  };

  DeleteItemContext* deleteItem();

  class  CallDataModifyingProcedureStatementContext : public antlr4::ParserRuleContext {
  public:
    CallDataModifyingProcedureStatementContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    CallProcedureStatementContext *callProcedureStatement();

   
  };

  CallDataModifyingProcedureStatementContext* callDataModifyingProcedureStatement();

  class  CompositeQueryStatementContext : public antlr4::ParserRuleContext {
  public:
    CompositeQueryStatementContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    CompositeQueryExpressionContext *compositeQueryExpression();

   
  };

  CompositeQueryStatementContext* compositeQueryStatement();

  class  CompositeQueryExpressionContext : public antlr4::ParserRuleContext {
  public:
    CompositeQueryExpressionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    CompositeQueryPrimaryContext *compositeQueryPrimary();
    CompositeQueryExpressionContext *compositeQueryExpression();
    QueryConjunctionContext *queryConjunction();

   
  };

  CompositeQueryExpressionContext* compositeQueryExpression();
  CompositeQueryExpressionContext* compositeQueryExpression(int precedence);
  class  QueryConjunctionContext : public antlr4::ParserRuleContext {
  public:
    QueryConjunctionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    SetOperatorContext *setOperator();
    antlr4::tree::TerminalNode *OTHERWISE();

   
  };

  QueryConjunctionContext* queryConjunction();

  class  SetOperatorContext : public antlr4::ParserRuleContext {
  public:
    SetOperatorContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *UNION();
    SetQuantifierContext *setQuantifier();
    antlr4::tree::TerminalNode *EXCEPT();
    antlr4::tree::TerminalNode *INTERSECT();

   
  };

  SetOperatorContext* setOperator();

  class  CompositeQueryPrimaryContext : public antlr4::ParserRuleContext {
  public:
    CompositeQueryPrimaryContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    LinearQueryStatementContext *linearQueryStatement();

   
  };

  CompositeQueryPrimaryContext* compositeQueryPrimary();

  class  LinearQueryStatementContext : public antlr4::ParserRuleContext {
  public:
    LinearQueryStatementContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    FocusedLinearQueryStatementContext *focusedLinearQueryStatement();
    AmbientLinearQueryStatementContext *ambientLinearQueryStatement();

   
  };

  LinearQueryStatementContext* linearQueryStatement();

  class  FocusedLinearQueryStatementContext : public antlr4::ParserRuleContext {
  public:
    FocusedLinearQueryStatementContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    FocusedLinearQueryAndPrimitiveResultStatementPartContext *focusedLinearQueryAndPrimitiveResultStatementPart();
    std::vector<FocusedLinearQueryStatementPartContext *> focusedLinearQueryStatementPart();
    FocusedLinearQueryStatementPartContext* focusedLinearQueryStatementPart(size_t i);
    FocusedPrimitiveResultStatementContext *focusedPrimitiveResultStatement();
    FocusedNestedQuerySpecificationContext *focusedNestedQuerySpecification();
    SelectStatementContext *selectStatement();

   
  };

  FocusedLinearQueryStatementContext* focusedLinearQueryStatement();

  class  FocusedLinearQueryStatementPartContext : public antlr4::ParserRuleContext {
  public:
    FocusedLinearQueryStatementPartContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    UseGraphClauseContext *useGraphClause();
    SimpleLinearQueryStatementContext *simpleLinearQueryStatement();

   
  };

  FocusedLinearQueryStatementPartContext* focusedLinearQueryStatementPart();

  class  FocusedLinearQueryAndPrimitiveResultStatementPartContext : public antlr4::ParserRuleContext {
  public:
    FocusedLinearQueryAndPrimitiveResultStatementPartContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    UseGraphClauseContext *useGraphClause();
    SimpleLinearQueryStatementContext *simpleLinearQueryStatement();
    PrimitiveResultStatementContext *primitiveResultStatement();

   
  };

  FocusedLinearQueryAndPrimitiveResultStatementPartContext* focusedLinearQueryAndPrimitiveResultStatementPart();

  class  FocusedPrimitiveResultStatementContext : public antlr4::ParserRuleContext {
  public:
    FocusedPrimitiveResultStatementContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    UseGraphClauseContext *useGraphClause();
    PrimitiveResultStatementContext *primitiveResultStatement();

   
  };

  FocusedPrimitiveResultStatementContext* focusedPrimitiveResultStatement();

  class  FocusedNestedQuerySpecificationContext : public antlr4::ParserRuleContext {
  public:
    FocusedNestedQuerySpecificationContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    UseGraphClauseContext *useGraphClause();
    NestedQuerySpecificationContext *nestedQuerySpecification();

   
  };

  FocusedNestedQuerySpecificationContext* focusedNestedQuerySpecification();

  class  AmbientLinearQueryStatementContext : public antlr4::ParserRuleContext {
  public:
    AmbientLinearQueryStatementContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    PrimitiveResultStatementContext *primitiveResultStatement();
    SimpleLinearQueryStatementContext *simpleLinearQueryStatement();
    NestedQuerySpecificationContext *nestedQuerySpecification();

   
  };

  AmbientLinearQueryStatementContext* ambientLinearQueryStatement();

  class  SimpleLinearQueryStatementContext : public antlr4::ParserRuleContext {
  public:
    SimpleLinearQueryStatementContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    std::vector<SimpleQueryStatementContext *> simpleQueryStatement();
    SimpleQueryStatementContext* simpleQueryStatement(size_t i);

   
  };

  SimpleLinearQueryStatementContext* simpleLinearQueryStatement();

  class  SimpleQueryStatementContext : public antlr4::ParserRuleContext {
  public:
    SimpleQueryStatementContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    PrimitiveQueryStatementContext *primitiveQueryStatement();
    CallQueryStatementContext *callQueryStatement();

   
  };

  SimpleQueryStatementContext* simpleQueryStatement();

  class  PrimitiveQueryStatementContext : public antlr4::ParserRuleContext {
  public:
    PrimitiveQueryStatementContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    MatchStatementContext *matchStatement();
    LetStatementContext *letStatement();
    ForStatementContext *forStatement();
    FilterStatementContext *filterStatement();
    OrderByAndPageStatementContext *orderByAndPageStatement();

   
  };

  PrimitiveQueryStatementContext* primitiveQueryStatement();

  class  MatchStatementContext : public antlr4::ParserRuleContext {
  public:
    MatchStatementContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    SimpleMatchStatementContext *simpleMatchStatement();
    OptionalMatchStatementContext *optionalMatchStatement();

   
  };

  MatchStatementContext* matchStatement();

  class  SimpleMatchStatementContext : public antlr4::ParserRuleContext {
  public:
    SimpleMatchStatementContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *MATCH();
    GraphPatternBindingTableContext *graphPatternBindingTable();

   
  };

  SimpleMatchStatementContext* simpleMatchStatement();

  class  OptionalMatchStatementContext : public antlr4::ParserRuleContext {
  public:
    OptionalMatchStatementContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *OPTIONAL();
    OptionalOperandContext *optionalOperand();

   
  };

  OptionalMatchStatementContext* optionalMatchStatement();

  class  OptionalOperandContext : public antlr4::ParserRuleContext {
  public:
    OptionalOperandContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    SimpleMatchStatementContext *simpleMatchStatement();
    antlr4::tree::TerminalNode *LEFT_BRACE();
    MatchStatementBlockContext *matchStatementBlock();
    antlr4::tree::TerminalNode *RIGHT_BRACE();
    antlr4::tree::TerminalNode *LEFT_PAREN();
    antlr4::tree::TerminalNode *RIGHT_PAREN();

   
  };

  OptionalOperandContext* optionalOperand();

  class  MatchStatementBlockContext : public antlr4::ParserRuleContext {
  public:
    MatchStatementBlockContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    std::vector<MatchStatementContext *> matchStatement();
    MatchStatementContext* matchStatement(size_t i);

   
  };

  MatchStatementBlockContext* matchStatementBlock();

  class  CallQueryStatementContext : public antlr4::ParserRuleContext {
  public:
    CallQueryStatementContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    CallProcedureStatementContext *callProcedureStatement();

   
  };

  CallQueryStatementContext* callQueryStatement();

  class  FilterStatementContext : public antlr4::ParserRuleContext {
  public:
    FilterStatementContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *FILTER();
    WhereClauseContext *whereClause();
    SearchConditionContext *searchCondition();

   
  };

  FilterStatementContext* filterStatement();

  class  LetStatementContext : public antlr4::ParserRuleContext {
  public:
    LetStatementContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LET();
    LetVariableDefinitionListContext *letVariableDefinitionList();

   
  };

  LetStatementContext* letStatement();

  class  LetVariableDefinitionListContext : public antlr4::ParserRuleContext {
  public:
    LetVariableDefinitionListContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    std::vector<LetVariableDefinitionContext *> letVariableDefinition();
    LetVariableDefinitionContext* letVariableDefinition(size_t i);
    std::vector<antlr4::tree::TerminalNode *> COMMA();
    antlr4::tree::TerminalNode* COMMA(size_t i);

   
  };

  LetVariableDefinitionListContext* letVariableDefinitionList();

  class  LetVariableDefinitionContext : public antlr4::ParserRuleContext {
  public:
    LetVariableDefinitionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    ValueVariableDefinitionContext *valueVariableDefinition();
    BindingVariableContext *bindingVariable();
    antlr4::tree::TerminalNode *EQUALS_OPERATOR();
    ValueExpressionContext *valueExpression();

   
  };

  LetVariableDefinitionContext* letVariableDefinition();

  class  ForStatementContext : public antlr4::ParserRuleContext {
  public:
    ForStatementContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *FOR();
    ForItemContext *forItem();
    ForOrdinalityOrOffsetContext *forOrdinalityOrOffset();

   
  };

  ForStatementContext* forStatement();

  class  ForItemContext : public antlr4::ParserRuleContext {
  public:
    ForItemContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    ForItemAliasContext *forItemAlias();
    ForItemSourceContext *forItemSource();

   
  };

  ForItemContext* forItem();

  class  ForItemAliasContext : public antlr4::ParserRuleContext {
  public:
    ForItemAliasContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    BindingVariableContext *bindingVariable();
    antlr4::tree::TerminalNode *IN();

   
  };

  ForItemAliasContext* forItemAlias();

  class  ForItemSourceContext : public antlr4::ParserRuleContext {
  public:
    ForItemSourceContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    ValueExpressionContext *valueExpression();

   
  };

  ForItemSourceContext* forItemSource();

  class  ForOrdinalityOrOffsetContext : public antlr4::ParserRuleContext {
  public:
    ForOrdinalityOrOffsetContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *WITH();
    BindingVariableContext *bindingVariable();
    antlr4::tree::TerminalNode *ORDINALITY();
    antlr4::tree::TerminalNode *OFFSET();

   
  };

  ForOrdinalityOrOffsetContext* forOrdinalityOrOffset();

  class  OrderByAndPageStatementContext : public antlr4::ParserRuleContext {
  public:
    OrderByAndPageStatementContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    OrderByClauseContext *orderByClause();
    OffsetClauseContext *offsetClause();
    LimitClauseContext *limitClause();

   
  };

  OrderByAndPageStatementContext* orderByAndPageStatement();

  class  PrimitiveResultStatementContext : public antlr4::ParserRuleContext {
  public:
    PrimitiveResultStatementContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    ReturnStatementContext *returnStatement();
    OrderByAndPageStatementContext *orderByAndPageStatement();
    antlr4::tree::TerminalNode *FINISH();

   
  };

  PrimitiveResultStatementContext* primitiveResultStatement();

  class  ReturnStatementContext : public antlr4::ParserRuleContext {
  public:
    ReturnStatementContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *RETURN();
    ReturnStatementBodyContext *returnStatementBody();

   
  };

  ReturnStatementContext* returnStatement();

  class  ReturnStatementBodyContext : public antlr4::ParserRuleContext {
  public:
    ReturnStatementBodyContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *ASTERISK();
    ReturnItemListContext *returnItemList();
    SetQuantifierContext *setQuantifier();
    GroupByClauseContext *groupByClause();

   
  };

  ReturnStatementBodyContext* returnStatementBody();

  class  ReturnItemListContext : public antlr4::ParserRuleContext {
  public:
    ReturnItemListContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    std::vector<ReturnItemContext *> returnItem();
    ReturnItemContext* returnItem(size_t i);
    std::vector<antlr4::tree::TerminalNode *> COMMA();
    antlr4::tree::TerminalNode* COMMA(size_t i);

   
  };

  ReturnItemListContext* returnItemList();

  class  ReturnItemContext : public antlr4::ParserRuleContext {
  public:
    ReturnItemContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    AggregatingValueExpressionContext *aggregatingValueExpression();
    ReturnItemAliasContext *returnItemAlias();

   
  };

  ReturnItemContext* returnItem();

  class  ReturnItemAliasContext : public antlr4::ParserRuleContext {
  public:
    ReturnItemAliasContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *AS();
    IdentifierContext *identifier();

   
  };

  ReturnItemAliasContext* returnItemAlias();

  class  SelectStatementContext : public antlr4::ParserRuleContext {
  public:
    SelectStatementContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *SELECT();
    antlr4::tree::TerminalNode *ASTERISK();
    SelectItemListContext *selectItemList();
    SetQuantifierContext *setQuantifier();
    SelectStatementBodyContext *selectStatementBody();
    WhereClauseContext *whereClause();
    GroupByClauseContext *groupByClause();
    HavingClauseContext *havingClause();
    OrderByClauseContext *orderByClause();
    OffsetClauseContext *offsetClause();
    LimitClauseContext *limitClause();

   
  };

  SelectStatementContext* selectStatement();

  class  SelectItemListContext : public antlr4::ParserRuleContext {
  public:
    SelectItemListContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    std::vector<SelectItemContext *> selectItem();
    SelectItemContext* selectItem(size_t i);
    std::vector<antlr4::tree::TerminalNode *> COMMA();
    antlr4::tree::TerminalNode* COMMA(size_t i);

   
  };

  SelectItemListContext* selectItemList();

  class  SelectItemContext : public antlr4::ParserRuleContext {
  public:
    SelectItemContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    AggregatingValueExpressionContext *aggregatingValueExpression();
    SelectItemAliasContext *selectItemAlias();

   
  };

  SelectItemContext* selectItem();

  class  SelectItemAliasContext : public antlr4::ParserRuleContext {
  public:
    SelectItemAliasContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *AS();
    IdentifierContext *identifier();

   
  };

  SelectItemAliasContext* selectItemAlias();

  class  HavingClauseContext : public antlr4::ParserRuleContext {
  public:
    HavingClauseContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *HAVING();
    SearchConditionContext *searchCondition();

   
  };

  HavingClauseContext* havingClause();

  class  SelectStatementBodyContext : public antlr4::ParserRuleContext {
  public:
    SelectStatementBodyContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *FROM();
    SelectGraphMatchListContext *selectGraphMatchList();
    SelectQuerySpecificationContext *selectQuerySpecification();

   
  };

  SelectStatementBodyContext* selectStatementBody();

  class  SelectGraphMatchListContext : public antlr4::ParserRuleContext {
  public:
    SelectGraphMatchListContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    std::vector<SelectGraphMatchContext *> selectGraphMatch();
    SelectGraphMatchContext* selectGraphMatch(size_t i);
    std::vector<antlr4::tree::TerminalNode *> COMMA();
    antlr4::tree::TerminalNode* COMMA(size_t i);

   
  };

  SelectGraphMatchListContext* selectGraphMatchList();

  class  SelectGraphMatchContext : public antlr4::ParserRuleContext {
  public:
    SelectGraphMatchContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    GraphExpressionContext *graphExpression();
    MatchStatementContext *matchStatement();

   
  };

  SelectGraphMatchContext* selectGraphMatch();

  class  SelectQuerySpecificationContext : public antlr4::ParserRuleContext {
  public:
    SelectQuerySpecificationContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    NestedQuerySpecificationContext *nestedQuerySpecification();
    GraphExpressionContext *graphExpression();

   
  };

  SelectQuerySpecificationContext* selectQuerySpecification();

  class  CallProcedureStatementContext : public antlr4::ParserRuleContext {
  public:
    CallProcedureStatementContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *CALL();
    ProcedureCallContext *procedureCall();
    antlr4::tree::TerminalNode *OPTIONAL();

   
  };

  CallProcedureStatementContext* callProcedureStatement();

  class  ProcedureCallContext : public antlr4::ParserRuleContext {
  public:
    ProcedureCallContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    InlineProcedureCallContext *inlineProcedureCall();
    NamedProcedureCallContext *namedProcedureCall();

   
  };

  ProcedureCallContext* procedureCall();

  class  InlineProcedureCallContext : public antlr4::ParserRuleContext {
  public:
    InlineProcedureCallContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    NestedProcedureSpecificationContext *nestedProcedureSpecification();
    VariableScopeClauseContext *variableScopeClause();

   
  };

  InlineProcedureCallContext* inlineProcedureCall();

  class  VariableScopeClauseContext : public antlr4::ParserRuleContext {
  public:
    VariableScopeClauseContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LEFT_PAREN();
    antlr4::tree::TerminalNode *RIGHT_PAREN();
    BindingVariableReferenceListContext *bindingVariableReferenceList();

   
  };

  VariableScopeClauseContext* variableScopeClause();

  class  BindingVariableReferenceListContext : public antlr4::ParserRuleContext {
  public:
    BindingVariableReferenceListContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    std::vector<BindingVariableReferenceContext *> bindingVariableReference();
    BindingVariableReferenceContext* bindingVariableReference(size_t i);
    std::vector<antlr4::tree::TerminalNode *> COMMA();
    antlr4::tree::TerminalNode* COMMA(size_t i);

   
  };

  BindingVariableReferenceListContext* bindingVariableReferenceList();

  class  NamedProcedureCallContext : public antlr4::ParserRuleContext {
  public:
    NamedProcedureCallContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    ProcedureReferenceContext *procedureReference();
    antlr4::tree::TerminalNode *LEFT_PAREN();
    antlr4::tree::TerminalNode *RIGHT_PAREN();
    ProcedureArgumentListContext *procedureArgumentList();
    YieldClauseContext *yieldClause();

   
  };

  NamedProcedureCallContext* namedProcedureCall();

  class  ProcedureArgumentListContext : public antlr4::ParserRuleContext {
  public:
    ProcedureArgumentListContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    std::vector<ProcedureArgumentContext *> procedureArgument();
    ProcedureArgumentContext* procedureArgument(size_t i);
    std::vector<antlr4::tree::TerminalNode *> COMMA();
    antlr4::tree::TerminalNode* COMMA(size_t i);

   
  };

  ProcedureArgumentListContext* procedureArgumentList();

  class  ProcedureArgumentContext : public antlr4::ParserRuleContext {
  public:
    ProcedureArgumentContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    ValueExpressionContext *valueExpression();

   
  };

  ProcedureArgumentContext* procedureArgument();

  class  AtSchemaClauseContext : public antlr4::ParserRuleContext {
  public:
    AtSchemaClauseContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *AT();
    SchemaReferenceContext *schemaReference();

   
  };

  AtSchemaClauseContext* atSchemaClause();

  class  UseGraphClauseContext : public antlr4::ParserRuleContext {
  public:
    UseGraphClauseContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *USE();
    GraphExpressionContext *graphExpression();

   
  };

  UseGraphClauseContext* useGraphClause();

  class  GraphPatternBindingTableContext : public antlr4::ParserRuleContext {
  public:
    GraphPatternBindingTableContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    GraphPatternContext *graphPattern();
    GraphPatternYieldClauseContext *graphPatternYieldClause();

   
  };

  GraphPatternBindingTableContext* graphPatternBindingTable();

  class  GraphPatternYieldClauseContext : public antlr4::ParserRuleContext {
  public:
    GraphPatternYieldClauseContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *YIELD();
    GraphPatternYieldItemListContext *graphPatternYieldItemList();

   
  };

  GraphPatternYieldClauseContext* graphPatternYieldClause();

  class  GraphPatternYieldItemListContext : public antlr4::ParserRuleContext {
  public:
    GraphPatternYieldItemListContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    std::vector<GraphPatternYieldItemContext *> graphPatternYieldItem();
    GraphPatternYieldItemContext* graphPatternYieldItem(size_t i);
    std::vector<antlr4::tree::TerminalNode *> COMMA();
    antlr4::tree::TerminalNode* COMMA(size_t i);

   
  };

  GraphPatternYieldItemListContext* graphPatternYieldItemList();

  class  GraphPatternYieldItemContext : public antlr4::ParserRuleContext {
  public:
    GraphPatternYieldItemContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    BindingVariableReferenceContext *bindingVariableReference();

   
  };

  GraphPatternYieldItemContext* graphPatternYieldItem();

  class  GraphPatternContext : public antlr4::ParserRuleContext {
  public:
    GraphPatternContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    PathPatternListContext *pathPatternList();
    MatchModeContext *matchMode();
    KeepClauseContext *keepClause();
    GraphPatternWhereClauseContext *graphPatternWhereClause();

   
  };

  GraphPatternContext* graphPattern();

  class  MatchModeContext : public antlr4::ParserRuleContext {
  public:
    MatchModeContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    RepeatableElementsMatchModeContext *repeatableElementsMatchMode();
    DifferentEdgesMatchModeContext *differentEdgesMatchMode();

   
  };

  MatchModeContext* matchMode();

  class  RepeatableElementsMatchModeContext : public antlr4::ParserRuleContext {
  public:
    RepeatableElementsMatchModeContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *REPEATABLE();
    ElementBindingsOrElementsContext *elementBindingsOrElements();

   
  };

  RepeatableElementsMatchModeContext* repeatableElementsMatchMode();

  class  DifferentEdgesMatchModeContext : public antlr4::ParserRuleContext {
  public:
    DifferentEdgesMatchModeContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *DIFFERENT();
    EdgeBindingsOrEdgesContext *edgeBindingsOrEdges();

   
  };

  DifferentEdgesMatchModeContext* differentEdgesMatchMode();

  class  ElementBindingsOrElementsContext : public antlr4::ParserRuleContext {
  public:
    ElementBindingsOrElementsContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *ELEMENT();
    antlr4::tree::TerminalNode *BINDINGS();
    antlr4::tree::TerminalNode *ELEMENTS();

   
  };

  ElementBindingsOrElementsContext* elementBindingsOrElements();

  class  EdgeBindingsOrEdgesContext : public antlr4::ParserRuleContext {
  public:
    EdgeBindingsOrEdgesContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    EdgeSynonymContext *edgeSynonym();
    antlr4::tree::TerminalNode *BINDINGS();
    EdgesSynonymContext *edgesSynonym();

   
  };

  EdgeBindingsOrEdgesContext* edgeBindingsOrEdges();

  class  PathPatternListContext : public antlr4::ParserRuleContext {
  public:
    PathPatternListContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    std::vector<PathPatternContext *> pathPattern();
    PathPatternContext* pathPattern(size_t i);
    std::vector<antlr4::tree::TerminalNode *> COMMA();
    antlr4::tree::TerminalNode* COMMA(size_t i);

   
  };

  PathPatternListContext* pathPatternList();

  class  PathPatternContext : public antlr4::ParserRuleContext {
  public:
    PathPatternContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    PathPatternExpressionContext *pathPatternExpression();
    PathVariableDeclarationContext *pathVariableDeclaration();
    PathPatternPrefixContext *pathPatternPrefix();

   
  };

  PathPatternContext* pathPattern();

  class  PathVariableDeclarationContext : public antlr4::ParserRuleContext {
  public:
    PathVariableDeclarationContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    PathVariableContext *pathVariable();
    antlr4::tree::TerminalNode *EQUALS_OPERATOR();

   
  };

  PathVariableDeclarationContext* pathVariableDeclaration();

  class  KeepClauseContext : public antlr4::ParserRuleContext {
  public:
    KeepClauseContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *KEEP();
    PathPatternPrefixContext *pathPatternPrefix();

   
  };

  KeepClauseContext* keepClause();

  class  GraphPatternWhereClauseContext : public antlr4::ParserRuleContext {
  public:
    GraphPatternWhereClauseContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *WHERE();
    SearchConditionContext *searchCondition();

   
  };

  GraphPatternWhereClauseContext* graphPatternWhereClause();

  class  InsertGraphPatternContext : public antlr4::ParserRuleContext {
  public:
    InsertGraphPatternContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    InsertPathPatternListContext *insertPathPatternList();

   
  };

  InsertGraphPatternContext* insertGraphPattern();

  class  InsertPathPatternListContext : public antlr4::ParserRuleContext {
  public:
    InsertPathPatternListContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    std::vector<InsertPathPatternContext *> insertPathPattern();
    InsertPathPatternContext* insertPathPattern(size_t i);
    std::vector<antlr4::tree::TerminalNode *> COMMA();
    antlr4::tree::TerminalNode* COMMA(size_t i);

   
  };

  InsertPathPatternListContext* insertPathPatternList();

  class  InsertPathPatternContext : public antlr4::ParserRuleContext {
  public:
    InsertPathPatternContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    std::vector<InsertNodePatternContext *> insertNodePattern();
    InsertNodePatternContext* insertNodePattern(size_t i);
    std::vector<InsertEdgePatternContext *> insertEdgePattern();
    InsertEdgePatternContext* insertEdgePattern(size_t i);

   
  };

  InsertPathPatternContext* insertPathPattern();

  class  InsertNodePatternContext : public antlr4::ParserRuleContext {
  public:
    InsertNodePatternContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LEFT_PAREN();
    antlr4::tree::TerminalNode *RIGHT_PAREN();
    InsertElementPatternFillerContext *insertElementPatternFiller();

   
  };

  InsertNodePatternContext* insertNodePattern();

  class  InsertEdgePatternContext : public antlr4::ParserRuleContext {
  public:
    InsertEdgePatternContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    InsertEdgePointingLeftContext *insertEdgePointingLeft();
    InsertEdgePointingRightContext *insertEdgePointingRight();
    InsertEdgeUndirectedContext *insertEdgeUndirected();

   
  };

  InsertEdgePatternContext* insertEdgePattern();

  class  InsertEdgePointingLeftContext : public antlr4::ParserRuleContext {
  public:
    InsertEdgePointingLeftContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LEFT_ARROW_BRACKET();
    antlr4::tree::TerminalNode *RIGHT_BRACKET_MINUS();
    InsertElementPatternFillerContext *insertElementPatternFiller();

   
  };

  InsertEdgePointingLeftContext* insertEdgePointingLeft();

  class  InsertEdgePointingRightContext : public antlr4::ParserRuleContext {
  public:
    InsertEdgePointingRightContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *MINUS_LEFT_BRACKET();
    antlr4::tree::TerminalNode *BRACKET_RIGHT_ARROW();
    InsertElementPatternFillerContext *insertElementPatternFiller();

   
  };

  InsertEdgePointingRightContext* insertEdgePointingRight();

  class  InsertEdgeUndirectedContext : public antlr4::ParserRuleContext {
  public:
    InsertEdgeUndirectedContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *TILDE_LEFT_BRACKET();
    antlr4::tree::TerminalNode *RIGHT_BRACKET_TILDE();
    InsertElementPatternFillerContext *insertElementPatternFiller();

   
  };

  InsertEdgeUndirectedContext* insertEdgeUndirected();

  class  InsertElementPatternFillerContext : public antlr4::ParserRuleContext {
  public:
    InsertElementPatternFillerContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    ElementVariableDeclarationContext *elementVariableDeclaration();
    LabelAndPropertySetSpecificationContext *labelAndPropertySetSpecification();

   
  };

  InsertElementPatternFillerContext* insertElementPatternFiller();

  class  LabelAndPropertySetSpecificationContext : public antlr4::ParserRuleContext {
  public:
    LabelAndPropertySetSpecificationContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    IsOrColonContext *isOrColon();
    LabelSetSpecificationContext *labelSetSpecification();
    ElementPropertySpecificationContext *elementPropertySpecification();

   
  };

  LabelAndPropertySetSpecificationContext* labelAndPropertySetSpecification();

  class  PathPatternPrefixContext : public antlr4::ParserRuleContext {
  public:
    PathPatternPrefixContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    PathModePrefixContext *pathModePrefix();
    PathSearchPrefixContext *pathSearchPrefix();

   
  };

  PathPatternPrefixContext* pathPatternPrefix();

  class  PathModePrefixContext : public antlr4::ParserRuleContext {
  public:
    PathModePrefixContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    PathModeContext *pathMode();
    PathOrPathsContext *pathOrPaths();

   
  };

  PathModePrefixContext* pathModePrefix();

  class  PathModeContext : public antlr4::ParserRuleContext {
  public:
    PathModeContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *WALK();
    antlr4::tree::TerminalNode *TRAIL();
    antlr4::tree::TerminalNode *SIMPLE();
    antlr4::tree::TerminalNode *ACYCLIC();

   
  };

  PathModeContext* pathMode();

  class  PathSearchPrefixContext : public antlr4::ParserRuleContext {
  public:
    PathSearchPrefixContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    AllPathSearchContext *allPathSearch();
    AnyPathSearchContext *anyPathSearch();
    ShortestPathSearchContext *shortestPathSearch();

   
  };

  PathSearchPrefixContext* pathSearchPrefix();

  class  AllPathSearchContext : public antlr4::ParserRuleContext {
  public:
    AllPathSearchContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *ALL();
    PathModeContext *pathMode();
    PathOrPathsContext *pathOrPaths();

   
  };

  AllPathSearchContext* allPathSearch();

  class  PathOrPathsContext : public antlr4::ParserRuleContext {
  public:
    PathOrPathsContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *PATH();
    antlr4::tree::TerminalNode *PATHS();

   
  };

  PathOrPathsContext* pathOrPaths();

  class  AnyPathSearchContext : public antlr4::ParserRuleContext {
  public:
    AnyPathSearchContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *ANY();
    NumberOfPathsContext *numberOfPaths();
    PathModeContext *pathMode();
    PathOrPathsContext *pathOrPaths();

   
  };

  AnyPathSearchContext* anyPathSearch();

  class  NumberOfPathsContext : public antlr4::ParserRuleContext {
  public:
    NumberOfPathsContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    NonNegativeIntegerSpecificationContext *nonNegativeIntegerSpecification();

   
  };

  NumberOfPathsContext* numberOfPaths();

  class  ShortestPathSearchContext : public antlr4::ParserRuleContext {
  public:
    ShortestPathSearchContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    AllShortestPathSearchContext *allShortestPathSearch();
    AnyShortestPathSearchContext *anyShortestPathSearch();
    CountedShortestPathSearchContext *countedShortestPathSearch();
    CountedShortestGroupSearchContext *countedShortestGroupSearch();

   
  };

  ShortestPathSearchContext* shortestPathSearch();

  class  AllShortestPathSearchContext : public antlr4::ParserRuleContext {
  public:
    AllShortestPathSearchContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *ALL();
    antlr4::tree::TerminalNode *SHORTEST();
    PathModeContext *pathMode();
    PathOrPathsContext *pathOrPaths();

   
  };

  AllShortestPathSearchContext* allShortestPathSearch();

  class  AnyShortestPathSearchContext : public antlr4::ParserRuleContext {
  public:
    AnyShortestPathSearchContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *ANY();
    antlr4::tree::TerminalNode *SHORTEST();
    PathModeContext *pathMode();
    PathOrPathsContext *pathOrPaths();

   
  };

  AnyShortestPathSearchContext* anyShortestPathSearch();

  class  CountedShortestPathSearchContext : public antlr4::ParserRuleContext {
  public:
    CountedShortestPathSearchContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *SHORTEST();
    NumberOfPathsContext *numberOfPaths();
    PathModeContext *pathMode();
    PathOrPathsContext *pathOrPaths();

   
  };

  CountedShortestPathSearchContext* countedShortestPathSearch();

  class  CountedShortestGroupSearchContext : public antlr4::ParserRuleContext {
  public:
    CountedShortestGroupSearchContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *SHORTEST();
    antlr4::tree::TerminalNode *GROUP();
    antlr4::tree::TerminalNode *GROUPS();
    NumberOfGroupsContext *numberOfGroups();
    PathModeContext *pathMode();
    PathOrPathsContext *pathOrPaths();

   
  };

  CountedShortestGroupSearchContext* countedShortestGroupSearch();

  class  NumberOfGroupsContext : public antlr4::ParserRuleContext {
  public:
    NumberOfGroupsContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    NonNegativeIntegerSpecificationContext *nonNegativeIntegerSpecification();

   
  };

  NumberOfGroupsContext* numberOfGroups();

  class  PathPatternExpressionContext : public antlr4::ParserRuleContext {
  public:
    PathPatternExpressionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
   
    PathPatternExpressionContext() = default;
    void copyFrom(PathPatternExpressionContext *context);
    using antlr4::ParserRuleContext::copyFrom;

    virtual size_t getRuleIndex() const override;

   
  };

  class  PpePatternUnionContext : public PathPatternExpressionContext {
  public:
    PpePatternUnionContext(PathPatternExpressionContext *ctx);

    std::vector<PathTermContext *> pathTerm();
    PathTermContext* pathTerm(size_t i);
    std::vector<antlr4::tree::TerminalNode *> VERTICAL_BAR();
    antlr4::tree::TerminalNode* VERTICAL_BAR(size_t i);
  };

  class  PpePathTermContext : public PathPatternExpressionContext {
  public:
    PpePathTermContext(PathPatternExpressionContext *ctx);

    PathTermContext *pathTerm();
  };

  class  PpeMultisetAlternationContext : public PathPatternExpressionContext {
  public:
    PpeMultisetAlternationContext(PathPatternExpressionContext *ctx);

    std::vector<PathTermContext *> pathTerm();
    PathTermContext* pathTerm(size_t i);
    std::vector<antlr4::tree::TerminalNode *> MULTISET_ALTERNATION_OPERATOR();
    antlr4::tree::TerminalNode* MULTISET_ALTERNATION_OPERATOR(size_t i);
  };

  PathPatternExpressionContext* pathPatternExpression();

  class  PathTermContext : public antlr4::ParserRuleContext {
  public:
    PathTermContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    std::vector<PathFactorContext *> pathFactor();
    PathFactorContext* pathFactor(size_t i);

   
  };

  PathTermContext* pathTerm();

  class  PathFactorContext : public antlr4::ParserRuleContext {
  public:
    PathFactorContext(antlr4::ParserRuleContext *parent, size_t invokingState);
   
    PathFactorContext() = default;
    void copyFrom(PathFactorContext *context);
    using antlr4::ParserRuleContext::copyFrom;

    virtual size_t getRuleIndex() const override;

   
  };

  class  PfQuantifiedPathPrimaryContext : public PathFactorContext {
  public:
    PfQuantifiedPathPrimaryContext(PathFactorContext *ctx);

    PathPrimaryContext *pathPrimary();
    GraphPatternQuantifierContext *graphPatternQuantifier();
  };

  class  PfQuestionedPathPrimaryContext : public PathFactorContext {
  public:
    PfQuestionedPathPrimaryContext(PathFactorContext *ctx);

    PathPrimaryContext *pathPrimary();
    antlr4::tree::TerminalNode *QUESTION_MARK();
  };

  class  PfPathPrimaryContext : public PathFactorContext {
  public:
    PfPathPrimaryContext(PathFactorContext *ctx);

    PathPrimaryContext *pathPrimary();
  };

  PathFactorContext* pathFactor();

  class  PathPrimaryContext : public antlr4::ParserRuleContext {
  public:
    PathPrimaryContext(antlr4::ParserRuleContext *parent, size_t invokingState);
   
    PathPrimaryContext() = default;
    void copyFrom(PathPrimaryContext *context);
    using antlr4::ParserRuleContext::copyFrom;

    virtual size_t getRuleIndex() const override;

   
  };

  class  PpParenthesizedPathPatternExpressionContext : public PathPrimaryContext {
  public:
    PpParenthesizedPathPatternExpressionContext(PathPrimaryContext *ctx);

    ParenthesizedPathPatternExpressionContext *parenthesizedPathPatternExpression();
  };

  class  PpElementPatternContext : public PathPrimaryContext {
  public:
    PpElementPatternContext(PathPrimaryContext *ctx);

    ElementPatternContext *elementPattern();
  };

  class  PpSimplifiedPathPatternExpressionContext : public PathPrimaryContext {
  public:
    PpSimplifiedPathPatternExpressionContext(PathPrimaryContext *ctx);

    SimplifiedPathPatternExpressionContext *simplifiedPathPatternExpression();
  };

  PathPrimaryContext* pathPrimary();

  class  ElementPatternContext : public antlr4::ParserRuleContext {
  public:
    ElementPatternContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    NodePatternContext *nodePattern();
    EdgePatternContext *edgePattern();

   
  };

  ElementPatternContext* elementPattern();

  class  NodePatternContext : public antlr4::ParserRuleContext {
  public:
    NodePatternContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LEFT_PAREN();
    ElementPatternFillerContext *elementPatternFiller();
    antlr4::tree::TerminalNode *RIGHT_PAREN();

   
  };

  NodePatternContext* nodePattern();

  class  ElementPatternFillerContext : public antlr4::ParserRuleContext {
  public:
    ElementPatternFillerContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    ElementVariableDeclarationContext *elementVariableDeclaration();
    IsLabelExpressionContext *isLabelExpression();
    ElementPatternPredicateContext *elementPatternPredicate();

   
  };

  ElementPatternFillerContext* elementPatternFiller();

  class  ElementVariableDeclarationContext : public antlr4::ParserRuleContext {
  public:
    ElementVariableDeclarationContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    ElementVariableContext *elementVariable();

   
  };

  ElementVariableDeclarationContext* elementVariableDeclaration();

  class  IsLabelExpressionContext : public antlr4::ParserRuleContext {
  public:
    IsLabelExpressionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    IsOrColonContext *isOrColon();
    LabelExpressionContext *labelExpression();

   
  };

  IsLabelExpressionContext* isLabelExpression();

  class  IsOrColonContext : public antlr4::ParserRuleContext {
  public:
    IsOrColonContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *IS();
    antlr4::tree::TerminalNode *COLON();

   
  };

  IsOrColonContext* isOrColon();

  class  ElementPatternPredicateContext : public antlr4::ParserRuleContext {
  public:
    ElementPatternPredicateContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    ElementPatternWhereClauseContext *elementPatternWhereClause();
    ElementPropertySpecificationContext *elementPropertySpecification();

   
  };

  ElementPatternPredicateContext* elementPatternPredicate();

  class  ElementPatternWhereClauseContext : public antlr4::ParserRuleContext {
  public:
    ElementPatternWhereClauseContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *WHERE();
    SearchConditionContext *searchCondition();

   
  };

  ElementPatternWhereClauseContext* elementPatternWhereClause();

  class  ElementPropertySpecificationContext : public antlr4::ParserRuleContext {
  public:
    ElementPropertySpecificationContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LEFT_BRACE();
    PropertyKeyValuePairListContext *propertyKeyValuePairList();
    antlr4::tree::TerminalNode *RIGHT_BRACE();

   
  };

  ElementPropertySpecificationContext* elementPropertySpecification();

  class  PropertyKeyValuePairListContext : public antlr4::ParserRuleContext {
  public:
    PropertyKeyValuePairListContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    std::vector<PropertyKeyValuePairContext *> propertyKeyValuePair();
    PropertyKeyValuePairContext* propertyKeyValuePair(size_t i);
    std::vector<antlr4::tree::TerminalNode *> COMMA();
    antlr4::tree::TerminalNode* COMMA(size_t i);

   
  };

  PropertyKeyValuePairListContext* propertyKeyValuePairList();

  class  PropertyKeyValuePairContext : public antlr4::ParserRuleContext {
  public:
    PropertyKeyValuePairContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    PropertyNameContext *propertyName();
    antlr4::tree::TerminalNode *COLON();
    ValueExpressionContext *valueExpression();

   
  };

  PropertyKeyValuePairContext* propertyKeyValuePair();

  class  EdgePatternContext : public antlr4::ParserRuleContext {
  public:
    EdgePatternContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    FullEdgePatternContext *fullEdgePattern();
    AbbreviatedEdgePatternContext *abbreviatedEdgePattern();

   
  };

  EdgePatternContext* edgePattern();

  class  FullEdgePatternContext : public antlr4::ParserRuleContext {
  public:
    FullEdgePatternContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    FullEdgePointingLeftContext *fullEdgePointingLeft();
    FullEdgeUndirectedContext *fullEdgeUndirected();
    FullEdgePointingRightContext *fullEdgePointingRight();
    FullEdgeLeftOrUndirectedContext *fullEdgeLeftOrUndirected();
    FullEdgeUndirectedOrRightContext *fullEdgeUndirectedOrRight();
    FullEdgeLeftOrRightContext *fullEdgeLeftOrRight();
    FullEdgeAnyDirectionContext *fullEdgeAnyDirection();

   
  };

  FullEdgePatternContext* fullEdgePattern();

  class  FullEdgePointingLeftContext : public antlr4::ParserRuleContext {
  public:
    FullEdgePointingLeftContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LEFT_ARROW_BRACKET();
    ElementPatternFillerContext *elementPatternFiller();
    antlr4::tree::TerminalNode *RIGHT_BRACKET_MINUS();

   
  };

  FullEdgePointingLeftContext* fullEdgePointingLeft();

  class  FullEdgeUndirectedContext : public antlr4::ParserRuleContext {
  public:
    FullEdgeUndirectedContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *TILDE_LEFT_BRACKET();
    ElementPatternFillerContext *elementPatternFiller();
    antlr4::tree::TerminalNode *RIGHT_BRACKET_TILDE();

   
  };

  FullEdgeUndirectedContext* fullEdgeUndirected();

  class  FullEdgePointingRightContext : public antlr4::ParserRuleContext {
  public:
    FullEdgePointingRightContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *MINUS_LEFT_BRACKET();
    ElementPatternFillerContext *elementPatternFiller();
    antlr4::tree::TerminalNode *BRACKET_RIGHT_ARROW();

   
  };

  FullEdgePointingRightContext* fullEdgePointingRight();

  class  FullEdgeLeftOrUndirectedContext : public antlr4::ParserRuleContext {
  public:
    FullEdgeLeftOrUndirectedContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LEFT_ARROW_TILDE_BRACKET();
    ElementPatternFillerContext *elementPatternFiller();
    antlr4::tree::TerminalNode *RIGHT_BRACKET_TILDE();

   
  };

  FullEdgeLeftOrUndirectedContext* fullEdgeLeftOrUndirected();

  class  FullEdgeUndirectedOrRightContext : public antlr4::ParserRuleContext {
  public:
    FullEdgeUndirectedOrRightContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *TILDE_LEFT_BRACKET();
    ElementPatternFillerContext *elementPatternFiller();
    antlr4::tree::TerminalNode *BRACKET_TILDE_RIGHT_ARROW();

   
  };

  FullEdgeUndirectedOrRightContext* fullEdgeUndirectedOrRight();

  class  FullEdgeLeftOrRightContext : public antlr4::ParserRuleContext {
  public:
    FullEdgeLeftOrRightContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LEFT_ARROW_BRACKET();
    ElementPatternFillerContext *elementPatternFiller();
    antlr4::tree::TerminalNode *BRACKET_RIGHT_ARROW();

   
  };

  FullEdgeLeftOrRightContext* fullEdgeLeftOrRight();

  class  FullEdgeAnyDirectionContext : public antlr4::ParserRuleContext {
  public:
    FullEdgeAnyDirectionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *MINUS_LEFT_BRACKET();
    ElementPatternFillerContext *elementPatternFiller();
    antlr4::tree::TerminalNode *RIGHT_BRACKET_MINUS();

   
  };

  FullEdgeAnyDirectionContext* fullEdgeAnyDirection();

  class  AbbreviatedEdgePatternContext : public antlr4::ParserRuleContext {
  public:
    AbbreviatedEdgePatternContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LEFT_ARROW();
    antlr4::tree::TerminalNode *TILDE();
    antlr4::tree::TerminalNode *RIGHT_ARROW();
    antlr4::tree::TerminalNode *LEFT_ARROW_TILDE();
    antlr4::tree::TerminalNode *TILDE_RIGHT_ARROW();
    antlr4::tree::TerminalNode *LEFT_MINUS_RIGHT();
    antlr4::tree::TerminalNode *MINUS_SIGN();

   
  };

  AbbreviatedEdgePatternContext* abbreviatedEdgePattern();

  class  ParenthesizedPathPatternExpressionContext : public antlr4::ParserRuleContext {
  public:
    ParenthesizedPathPatternExpressionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LEFT_PAREN();
    PathPatternExpressionContext *pathPatternExpression();
    antlr4::tree::TerminalNode *RIGHT_PAREN();
    SubpathVariableDeclarationContext *subpathVariableDeclaration();
    PathModePrefixContext *pathModePrefix();
    ParenthesizedPathPatternWhereClauseContext *parenthesizedPathPatternWhereClause();

   
  };

  ParenthesizedPathPatternExpressionContext* parenthesizedPathPatternExpression();

  class  SubpathVariableDeclarationContext : public antlr4::ParserRuleContext {
  public:
    SubpathVariableDeclarationContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    SubpathVariableContext *subpathVariable();
    antlr4::tree::TerminalNode *EQUALS_OPERATOR();

   
  };

  SubpathVariableDeclarationContext* subpathVariableDeclaration();

  class  ParenthesizedPathPatternWhereClauseContext : public antlr4::ParserRuleContext {
  public:
    ParenthesizedPathPatternWhereClauseContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *WHERE();
    SearchConditionContext *searchCondition();

   
  };

  ParenthesizedPathPatternWhereClauseContext* parenthesizedPathPatternWhereClause();

  class  LabelExpressionContext : public antlr4::ParserRuleContext {
  public:
    LabelExpressionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
   
    LabelExpressionContext() = default;
    void copyFrom(LabelExpressionContext *context);
    using antlr4::ParserRuleContext::copyFrom;

    virtual size_t getRuleIndex() const override;

   
  };

  class  LabelExpressionNegationContext : public LabelExpressionContext {
  public:
    LabelExpressionNegationContext(LabelExpressionContext *ctx);

    antlr4::tree::TerminalNode *EXCLAMATION_MARK();
    LabelExpressionContext *labelExpression();
  };

  class  LabelExpressionDisjunctionContext : public LabelExpressionContext {
  public:
    LabelExpressionDisjunctionContext(LabelExpressionContext *ctx);

    std::vector<LabelExpressionContext *> labelExpression();
    LabelExpressionContext* labelExpression(size_t i);
    antlr4::tree::TerminalNode *VERTICAL_BAR();
  };

  class  LabelExpressionParenthesizedContext : public LabelExpressionContext {
  public:
    LabelExpressionParenthesizedContext(LabelExpressionContext *ctx);

    antlr4::tree::TerminalNode *LEFT_PAREN();
    LabelExpressionContext *labelExpression();
    antlr4::tree::TerminalNode *RIGHT_PAREN();
  };

  class  LabelExpressionWildcardContext : public LabelExpressionContext {
  public:
    LabelExpressionWildcardContext(LabelExpressionContext *ctx);

    antlr4::tree::TerminalNode *PERCENT();
  };

  class  LabelExpressionConjunctionContext : public LabelExpressionContext {
  public:
    LabelExpressionConjunctionContext(LabelExpressionContext *ctx);

    std::vector<LabelExpressionContext *> labelExpression();
    LabelExpressionContext* labelExpression(size_t i);
    antlr4::tree::TerminalNode *AMPERSAND();
  };

  class  LabelExpressionNameContext : public LabelExpressionContext {
  public:
    LabelExpressionNameContext(LabelExpressionContext *ctx);

    LabelNameContext *labelName();
  };

  LabelExpressionContext* labelExpression();
  LabelExpressionContext* labelExpression(int precedence);
  class  PathVariableReferenceContext : public antlr4::ParserRuleContext {
  public:
    PathVariableReferenceContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    BindingVariableReferenceContext *bindingVariableReference();

   
  };

  PathVariableReferenceContext* pathVariableReference();

  class  ElementVariableReferenceContext : public antlr4::ParserRuleContext {
  public:
    ElementVariableReferenceContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    BindingVariableReferenceContext *bindingVariableReference();

   
  };

  ElementVariableReferenceContext* elementVariableReference();

  class  GraphPatternQuantifierContext : public antlr4::ParserRuleContext {
  public:
    GraphPatternQuantifierContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *ASTERISK();
    antlr4::tree::TerminalNode *PLUS_SIGN();
    FixedQuantifierContext *fixedQuantifier();
    GeneralQuantifierContext *generalQuantifier();

   
  };

  GraphPatternQuantifierContext* graphPatternQuantifier();

  class  FixedQuantifierContext : public antlr4::ParserRuleContext {
  public:
    FixedQuantifierContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LEFT_BRACE();
    UnsignedIntegerContext *unsignedInteger();
    antlr4::tree::TerminalNode *RIGHT_BRACE();

   
  };

  FixedQuantifierContext* fixedQuantifier();

  class  GeneralQuantifierContext : public antlr4::ParserRuleContext {
  public:
    GeneralQuantifierContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LEFT_BRACE();
    antlr4::tree::TerminalNode *COMMA();
    antlr4::tree::TerminalNode *RIGHT_BRACE();
    LowerBoundContext *lowerBound();
    UpperBoundContext *upperBound();

   
  };

  GeneralQuantifierContext* generalQuantifier();

  class  LowerBoundContext : public antlr4::ParserRuleContext {
  public:
    LowerBoundContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    UnsignedIntegerContext *unsignedInteger();

   
  };

  LowerBoundContext* lowerBound();

  class  UpperBoundContext : public antlr4::ParserRuleContext {
  public:
    UpperBoundContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    UnsignedIntegerContext *unsignedInteger();

   
  };

  UpperBoundContext* upperBound();

  class  SimplifiedPathPatternExpressionContext : public antlr4::ParserRuleContext {
  public:
    SimplifiedPathPatternExpressionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    SimplifiedDefaultingLeftContext *simplifiedDefaultingLeft();
    SimplifiedDefaultingUndirectedContext *simplifiedDefaultingUndirected();
    SimplifiedDefaultingRightContext *simplifiedDefaultingRight();
    SimplifiedDefaultingLeftOrUndirectedContext *simplifiedDefaultingLeftOrUndirected();
    SimplifiedDefaultingUndirectedOrRightContext *simplifiedDefaultingUndirectedOrRight();
    SimplifiedDefaultingLeftOrRightContext *simplifiedDefaultingLeftOrRight();
    SimplifiedDefaultingAnyDirectionContext *simplifiedDefaultingAnyDirection();

   
  };

  SimplifiedPathPatternExpressionContext* simplifiedPathPatternExpression();

  class  SimplifiedDefaultingLeftContext : public antlr4::ParserRuleContext {
  public:
    SimplifiedDefaultingLeftContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LEFT_MINUS_SLASH();
    SimplifiedContentsContext *simplifiedContents();
    antlr4::tree::TerminalNode *SLASH_MINUS();

   
  };

  SimplifiedDefaultingLeftContext* simplifiedDefaultingLeft();

  class  SimplifiedDefaultingUndirectedContext : public antlr4::ParserRuleContext {
  public:
    SimplifiedDefaultingUndirectedContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *TILDE_SLASH();
    SimplifiedContentsContext *simplifiedContents();
    antlr4::tree::TerminalNode *SLASH_TILDE();

   
  };

  SimplifiedDefaultingUndirectedContext* simplifiedDefaultingUndirected();

  class  SimplifiedDefaultingRightContext : public antlr4::ParserRuleContext {
  public:
    SimplifiedDefaultingRightContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *MINUS_SLASH();
    SimplifiedContentsContext *simplifiedContents();
    antlr4::tree::TerminalNode *SLASH_MINUS_RIGHT();

   
  };

  SimplifiedDefaultingRightContext* simplifiedDefaultingRight();

  class  SimplifiedDefaultingLeftOrUndirectedContext : public antlr4::ParserRuleContext {
  public:
    SimplifiedDefaultingLeftOrUndirectedContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LEFT_TILDE_SLASH();
    SimplifiedContentsContext *simplifiedContents();
    antlr4::tree::TerminalNode *SLASH_TILDE();

   
  };

  SimplifiedDefaultingLeftOrUndirectedContext* simplifiedDefaultingLeftOrUndirected();

  class  SimplifiedDefaultingUndirectedOrRightContext : public antlr4::ParserRuleContext {
  public:
    SimplifiedDefaultingUndirectedOrRightContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *TILDE_SLASH();
    SimplifiedContentsContext *simplifiedContents();
    antlr4::tree::TerminalNode *SLASH_TILDE_RIGHT();

   
  };

  SimplifiedDefaultingUndirectedOrRightContext* simplifiedDefaultingUndirectedOrRight();

  class  SimplifiedDefaultingLeftOrRightContext : public antlr4::ParserRuleContext {
  public:
    SimplifiedDefaultingLeftOrRightContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LEFT_MINUS_SLASH();
    SimplifiedContentsContext *simplifiedContents();
    antlr4::tree::TerminalNode *SLASH_MINUS_RIGHT();

   
  };

  SimplifiedDefaultingLeftOrRightContext* simplifiedDefaultingLeftOrRight();

  class  SimplifiedDefaultingAnyDirectionContext : public antlr4::ParserRuleContext {
  public:
    SimplifiedDefaultingAnyDirectionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *MINUS_SLASH();
    SimplifiedContentsContext *simplifiedContents();
    antlr4::tree::TerminalNode *SLASH_MINUS();

   
  };

  SimplifiedDefaultingAnyDirectionContext* simplifiedDefaultingAnyDirection();

  class  SimplifiedContentsContext : public antlr4::ParserRuleContext {
  public:
    SimplifiedContentsContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    SimplifiedTermContext *simplifiedTerm();
    SimplifiedPathUnionContext *simplifiedPathUnion();
    SimplifiedMultisetAlternationContext *simplifiedMultisetAlternation();

   
  };

  SimplifiedContentsContext* simplifiedContents();

  class  SimplifiedPathUnionContext : public antlr4::ParserRuleContext {
  public:
    SimplifiedPathUnionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    std::vector<SimplifiedTermContext *> simplifiedTerm();
    SimplifiedTermContext* simplifiedTerm(size_t i);
    std::vector<antlr4::tree::TerminalNode *> VERTICAL_BAR();
    antlr4::tree::TerminalNode* VERTICAL_BAR(size_t i);

   
  };

  SimplifiedPathUnionContext* simplifiedPathUnion();

  class  SimplifiedMultisetAlternationContext : public antlr4::ParserRuleContext {
  public:
    SimplifiedMultisetAlternationContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    std::vector<SimplifiedTermContext *> simplifiedTerm();
    SimplifiedTermContext* simplifiedTerm(size_t i);
    std::vector<antlr4::tree::TerminalNode *> MULTISET_ALTERNATION_OPERATOR();
    antlr4::tree::TerminalNode* MULTISET_ALTERNATION_OPERATOR(size_t i);

   
  };

  SimplifiedMultisetAlternationContext* simplifiedMultisetAlternation();

  class  SimplifiedTermContext : public antlr4::ParserRuleContext {
  public:
    SimplifiedTermContext(antlr4::ParserRuleContext *parent, size_t invokingState);
   
    SimplifiedTermContext() = default;
    void copyFrom(SimplifiedTermContext *context);
    using antlr4::ParserRuleContext::copyFrom;

    virtual size_t getRuleIndex() const override;

   
  };

  class  SimplifiedFactorLowLabelContext : public SimplifiedTermContext {
  public:
    SimplifiedFactorLowLabelContext(SimplifiedTermContext *ctx);

    SimplifiedFactorLowContext *simplifiedFactorLow();
  };

  class  SimplifiedConcatenationLabelContext : public SimplifiedTermContext {
  public:
    SimplifiedConcatenationLabelContext(SimplifiedTermContext *ctx);

    SimplifiedTermContext *simplifiedTerm();
    SimplifiedFactorLowContext *simplifiedFactorLow();
  };

  SimplifiedTermContext* simplifiedTerm();
  SimplifiedTermContext* simplifiedTerm(int precedence);
  class  SimplifiedFactorLowContext : public antlr4::ParserRuleContext {
  public:
    SimplifiedFactorLowContext(antlr4::ParserRuleContext *parent, size_t invokingState);
   
    SimplifiedFactorLowContext() = default;
    void copyFrom(SimplifiedFactorLowContext *context);
    using antlr4::ParserRuleContext::copyFrom;

    virtual size_t getRuleIndex() const override;

   
  };

  class  SimplifiedConjunctionLabelContext : public SimplifiedFactorLowContext {
  public:
    SimplifiedConjunctionLabelContext(SimplifiedFactorLowContext *ctx);

    SimplifiedFactorLowContext *simplifiedFactorLow();
    antlr4::tree::TerminalNode *AMPERSAND();
    SimplifiedFactorHighContext *simplifiedFactorHigh();
  };

  class  SimplifiedFactorHighLabelContext : public SimplifiedFactorLowContext {
  public:
    SimplifiedFactorHighLabelContext(SimplifiedFactorLowContext *ctx);

    SimplifiedFactorHighContext *simplifiedFactorHigh();
  };

  SimplifiedFactorLowContext* simplifiedFactorLow();
  SimplifiedFactorLowContext* simplifiedFactorLow(int precedence);
  class  SimplifiedFactorHighContext : public antlr4::ParserRuleContext {
  public:
    SimplifiedFactorHighContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    SimplifiedTertiaryContext *simplifiedTertiary();
    SimplifiedQuantifiedContext *simplifiedQuantified();
    SimplifiedQuestionedContext *simplifiedQuestioned();

   
  };

  SimplifiedFactorHighContext* simplifiedFactorHigh();

  class  SimplifiedQuantifiedContext : public antlr4::ParserRuleContext {
  public:
    SimplifiedQuantifiedContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    SimplifiedTertiaryContext *simplifiedTertiary();
    GraphPatternQuantifierContext *graphPatternQuantifier();

   
  };

  SimplifiedQuantifiedContext* simplifiedQuantified();

  class  SimplifiedQuestionedContext : public antlr4::ParserRuleContext {
  public:
    SimplifiedQuestionedContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    SimplifiedTertiaryContext *simplifiedTertiary();
    antlr4::tree::TerminalNode *QUESTION_MARK();

   
  };

  SimplifiedQuestionedContext* simplifiedQuestioned();

  class  SimplifiedTertiaryContext : public antlr4::ParserRuleContext {
  public:
    SimplifiedTertiaryContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    SimplifiedDirectionOverrideContext *simplifiedDirectionOverride();
    SimplifiedSecondaryContext *simplifiedSecondary();

   
  };

  SimplifiedTertiaryContext* simplifiedTertiary();

  class  SimplifiedDirectionOverrideContext : public antlr4::ParserRuleContext {
  public:
    SimplifiedDirectionOverrideContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    SimplifiedOverrideLeftContext *simplifiedOverrideLeft();
    SimplifiedOverrideUndirectedContext *simplifiedOverrideUndirected();
    SimplifiedOverrideRightContext *simplifiedOverrideRight();
    SimplifiedOverrideLeftOrUndirectedContext *simplifiedOverrideLeftOrUndirected();
    SimplifiedOverrideUndirectedOrRightContext *simplifiedOverrideUndirectedOrRight();
    SimplifiedOverrideLeftOrRightContext *simplifiedOverrideLeftOrRight();
    SimplifiedOverrideAnyDirectionContext *simplifiedOverrideAnyDirection();

   
  };

  SimplifiedDirectionOverrideContext* simplifiedDirectionOverride();

  class  SimplifiedOverrideLeftContext : public antlr4::ParserRuleContext {
  public:
    SimplifiedOverrideLeftContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LEFT_ANGLE_BRACKET();
    SimplifiedSecondaryContext *simplifiedSecondary();

   
  };

  SimplifiedOverrideLeftContext* simplifiedOverrideLeft();

  class  SimplifiedOverrideUndirectedContext : public antlr4::ParserRuleContext {
  public:
    SimplifiedOverrideUndirectedContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *TILDE();
    SimplifiedSecondaryContext *simplifiedSecondary();

   
  };

  SimplifiedOverrideUndirectedContext* simplifiedOverrideUndirected();

  class  SimplifiedOverrideRightContext : public antlr4::ParserRuleContext {
  public:
    SimplifiedOverrideRightContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    SimplifiedSecondaryContext *simplifiedSecondary();
    antlr4::tree::TerminalNode *RIGHT_ANGLE_BRACKET();

   
  };

  SimplifiedOverrideRightContext* simplifiedOverrideRight();

  class  SimplifiedOverrideLeftOrUndirectedContext : public antlr4::ParserRuleContext {
  public:
    SimplifiedOverrideLeftOrUndirectedContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LEFT_ARROW_TILDE();
    SimplifiedSecondaryContext *simplifiedSecondary();

   
  };

  SimplifiedOverrideLeftOrUndirectedContext* simplifiedOverrideLeftOrUndirected();

  class  SimplifiedOverrideUndirectedOrRightContext : public antlr4::ParserRuleContext {
  public:
    SimplifiedOverrideUndirectedOrRightContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *TILDE();
    SimplifiedSecondaryContext *simplifiedSecondary();
    antlr4::tree::TerminalNode *RIGHT_ANGLE_BRACKET();

   
  };

  SimplifiedOverrideUndirectedOrRightContext* simplifiedOverrideUndirectedOrRight();

  class  SimplifiedOverrideLeftOrRightContext : public antlr4::ParserRuleContext {
  public:
    SimplifiedOverrideLeftOrRightContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LEFT_ANGLE_BRACKET();
    SimplifiedSecondaryContext *simplifiedSecondary();
    antlr4::tree::TerminalNode *RIGHT_ANGLE_BRACKET();

   
  };

  SimplifiedOverrideLeftOrRightContext* simplifiedOverrideLeftOrRight();

  class  SimplifiedOverrideAnyDirectionContext : public antlr4::ParserRuleContext {
  public:
    SimplifiedOverrideAnyDirectionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *MINUS_SIGN();
    SimplifiedSecondaryContext *simplifiedSecondary();

   
  };

  SimplifiedOverrideAnyDirectionContext* simplifiedOverrideAnyDirection();

  class  SimplifiedSecondaryContext : public antlr4::ParserRuleContext {
  public:
    SimplifiedSecondaryContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    SimplifiedPrimaryContext *simplifiedPrimary();
    SimplifiedNegationContext *simplifiedNegation();

   
  };

  SimplifiedSecondaryContext* simplifiedSecondary();

  class  SimplifiedNegationContext : public antlr4::ParserRuleContext {
  public:
    SimplifiedNegationContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *EXCLAMATION_MARK();
    SimplifiedPrimaryContext *simplifiedPrimary();

   
  };

  SimplifiedNegationContext* simplifiedNegation();

  class  SimplifiedPrimaryContext : public antlr4::ParserRuleContext {
  public:
    SimplifiedPrimaryContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    LabelNameContext *labelName();
    antlr4::tree::TerminalNode *LEFT_PAREN();
    SimplifiedContentsContext *simplifiedContents();
    antlr4::tree::TerminalNode *RIGHT_PAREN();

   
  };

  SimplifiedPrimaryContext* simplifiedPrimary();

  class  WhereClauseContext : public antlr4::ParserRuleContext {
  public:
    WhereClauseContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *WHERE();
    SearchConditionContext *searchCondition();

   
  };

  WhereClauseContext* whereClause();

  class  YieldClauseContext : public antlr4::ParserRuleContext {
  public:
    YieldClauseContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *YIELD();
    YieldItemListContext *yieldItemList();

   
  };

  YieldClauseContext* yieldClause();

  class  YieldItemListContext : public antlr4::ParserRuleContext {
  public:
    YieldItemListContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    std::vector<YieldItemContext *> yieldItem();
    YieldItemContext* yieldItem(size_t i);
    std::vector<antlr4::tree::TerminalNode *> COMMA();
    antlr4::tree::TerminalNode* COMMA(size_t i);

   
  };

  YieldItemListContext* yieldItemList();

  class  YieldItemContext : public antlr4::ParserRuleContext {
  public:
    YieldItemContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    YieldItemNameContext *yieldItemName();
    YieldItemAliasContext *yieldItemAlias();

   
  };

  YieldItemContext* yieldItem();

  class  YieldItemNameContext : public antlr4::ParserRuleContext {
  public:
    YieldItemNameContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    FieldNameContext *fieldName();

   
  };

  YieldItemNameContext* yieldItemName();

  class  YieldItemAliasContext : public antlr4::ParserRuleContext {
  public:
    YieldItemAliasContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *AS();
    BindingVariableContext *bindingVariable();

   
  };

  YieldItemAliasContext* yieldItemAlias();

  class  GroupByClauseContext : public antlr4::ParserRuleContext {
  public:
    GroupByClauseContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *GROUP();
    antlr4::tree::TerminalNode *BY();
    GroupingElementListContext *groupingElementList();

   
  };

  GroupByClauseContext* groupByClause();

  class  GroupingElementListContext : public antlr4::ParserRuleContext {
  public:
    GroupingElementListContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    std::vector<GroupingElementContext *> groupingElement();
    GroupingElementContext* groupingElement(size_t i);
    std::vector<antlr4::tree::TerminalNode *> COMMA();
    antlr4::tree::TerminalNode* COMMA(size_t i);
    EmptyGroupingSetContext *emptyGroupingSet();

   
  };

  GroupingElementListContext* groupingElementList();

  class  GroupingElementContext : public antlr4::ParserRuleContext {
  public:
    GroupingElementContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    BindingVariableReferenceContext *bindingVariableReference();

   
  };

  GroupingElementContext* groupingElement();

  class  EmptyGroupingSetContext : public antlr4::ParserRuleContext {
  public:
    EmptyGroupingSetContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LEFT_PAREN();
    antlr4::tree::TerminalNode *RIGHT_PAREN();

   
  };

  EmptyGroupingSetContext* emptyGroupingSet();

  class  OrderByClauseContext : public antlr4::ParserRuleContext {
  public:
    OrderByClauseContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *ORDER();
    antlr4::tree::TerminalNode *BY();
    SortSpecificationListContext *sortSpecificationList();

   
  };

  OrderByClauseContext* orderByClause();

  class  SortSpecificationListContext : public antlr4::ParserRuleContext {
  public:
    SortSpecificationListContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    std::vector<SortSpecificationContext *> sortSpecification();
    SortSpecificationContext* sortSpecification(size_t i);
    std::vector<antlr4::tree::TerminalNode *> COMMA();
    antlr4::tree::TerminalNode* COMMA(size_t i);

   
  };

  SortSpecificationListContext* sortSpecificationList();

  class  SortSpecificationContext : public antlr4::ParserRuleContext {
  public:
    SortSpecificationContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    SortKeyContext *sortKey();
    OrderingSpecificationContext *orderingSpecification();
    NullOrderingContext *nullOrdering();

   
  };

  SortSpecificationContext* sortSpecification();

  class  SortKeyContext : public antlr4::ParserRuleContext {
  public:
    SortKeyContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    AggregatingValueExpressionContext *aggregatingValueExpression();

   
  };

  SortKeyContext* sortKey();

  class  OrderingSpecificationContext : public antlr4::ParserRuleContext {
  public:
    OrderingSpecificationContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *ASC();
    antlr4::tree::TerminalNode *ASCENDING();
    antlr4::tree::TerminalNode *DESC();
    antlr4::tree::TerminalNode *DESCENDING();

   
  };

  OrderingSpecificationContext* orderingSpecification();

  class  NullOrderingContext : public antlr4::ParserRuleContext {
  public:
    NullOrderingContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *NULLS();
    antlr4::tree::TerminalNode *FIRST();
    antlr4::tree::TerminalNode *LAST();

   
  };

  NullOrderingContext* nullOrdering();

  class  LimitClauseContext : public antlr4::ParserRuleContext {
  public:
    LimitClauseContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LIMIT();
    NonNegativeIntegerSpecificationContext *nonNegativeIntegerSpecification();

   
  };

  LimitClauseContext* limitClause();

  class  OffsetClauseContext : public antlr4::ParserRuleContext {
  public:
    OffsetClauseContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    OffsetSynonymContext *offsetSynonym();
    NonNegativeIntegerSpecificationContext *nonNegativeIntegerSpecification();

   
  };

  OffsetClauseContext* offsetClause();

  class  OffsetSynonymContext : public antlr4::ParserRuleContext {
  public:
    OffsetSynonymContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *OFFSET();
    antlr4::tree::TerminalNode *SKIP_RESERVED_WORD();

   
  };

  OffsetSynonymContext* offsetSynonym();

  class  SchemaReferenceContext : public antlr4::ParserRuleContext {
  public:
    SchemaReferenceContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    AbsoluteCatalogSchemaReferenceContext *absoluteCatalogSchemaReference();
    RelativeCatalogSchemaReferenceContext *relativeCatalogSchemaReference();
    ReferenceParameterSpecificationContext *referenceParameterSpecification();

   
  };

  SchemaReferenceContext* schemaReference();

  class  AbsoluteCatalogSchemaReferenceContext : public antlr4::ParserRuleContext {
  public:
    AbsoluteCatalogSchemaReferenceContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *SOLIDUS();
    AbsoluteDirectoryPathContext *absoluteDirectoryPath();
    SchemaNameContext *schemaName();

   
  };

  AbsoluteCatalogSchemaReferenceContext* absoluteCatalogSchemaReference();

  class  CatalogSchemaParentAndNameContext : public antlr4::ParserRuleContext {
  public:
    CatalogSchemaParentAndNameContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    AbsoluteDirectoryPathContext *absoluteDirectoryPath();
    SchemaNameContext *schemaName();

   
  };

  CatalogSchemaParentAndNameContext* catalogSchemaParentAndName();

  class  RelativeCatalogSchemaReferenceContext : public antlr4::ParserRuleContext {
  public:
    RelativeCatalogSchemaReferenceContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    PredefinedSchemaReferenceContext *predefinedSchemaReference();
    RelativeDirectoryPathContext *relativeDirectoryPath();
    SchemaNameContext *schemaName();

   
  };

  RelativeCatalogSchemaReferenceContext* relativeCatalogSchemaReference();

  class  PredefinedSchemaReferenceContext : public antlr4::ParserRuleContext {
  public:
    PredefinedSchemaReferenceContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *HOME_SCHEMA();
    antlr4::tree::TerminalNode *CURRENT_SCHEMA();
    antlr4::tree::TerminalNode *PERIOD();

   
  };

  PredefinedSchemaReferenceContext* predefinedSchemaReference();

  class  AbsoluteDirectoryPathContext : public antlr4::ParserRuleContext {
  public:
    AbsoluteDirectoryPathContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *SOLIDUS();
    SimpleDirectoryPathContext *simpleDirectoryPath();

   
  };

  AbsoluteDirectoryPathContext* absoluteDirectoryPath();

  class  RelativeDirectoryPathContext : public antlr4::ParserRuleContext {
  public:
    RelativeDirectoryPathContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    std::vector<antlr4::tree::TerminalNode *> DOUBLE_PERIOD();
    antlr4::tree::TerminalNode* DOUBLE_PERIOD(size_t i);
    std::vector<antlr4::tree::TerminalNode *> SOLIDUS();
    antlr4::tree::TerminalNode* SOLIDUS(size_t i);
    SimpleDirectoryPathContext *simpleDirectoryPath();

   
  };

  RelativeDirectoryPathContext* relativeDirectoryPath();

  class  SimpleDirectoryPathContext : public antlr4::ParserRuleContext {
  public:
    SimpleDirectoryPathContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    std::vector<DirectoryNameContext *> directoryName();
    DirectoryNameContext* directoryName(size_t i);
    std::vector<antlr4::tree::TerminalNode *> SOLIDUS();
    antlr4::tree::TerminalNode* SOLIDUS(size_t i);

   
  };

  SimpleDirectoryPathContext* simpleDirectoryPath();

  class  GraphReferenceContext : public antlr4::ParserRuleContext {
  public:
    GraphReferenceContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    CatalogObjectParentReferenceContext *catalogObjectParentReference();
    GraphNameContext *graphName();
    DelimitedGraphNameContext *delimitedGraphName();
    HomeGraphContext *homeGraph();
    ReferenceParameterSpecificationContext *referenceParameterSpecification();

   
  };

  GraphReferenceContext* graphReference();

  class  CatalogGraphParentAndNameContext : public antlr4::ParserRuleContext {
  public:
    CatalogGraphParentAndNameContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    GraphNameContext *graphName();
    CatalogObjectParentReferenceContext *catalogObjectParentReference();

   
  };

  CatalogGraphParentAndNameContext* catalogGraphParentAndName();

  class  HomeGraphContext : public antlr4::ParserRuleContext {
  public:
    HomeGraphContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *HOME_PROPERTY_GRAPH();
    antlr4::tree::TerminalNode *HOME_GRAPH();

   
  };

  HomeGraphContext* homeGraph();

  class  GraphTypeReferenceContext : public antlr4::ParserRuleContext {
  public:
    GraphTypeReferenceContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    CatalogGraphTypeParentAndNameContext *catalogGraphTypeParentAndName();
    ReferenceParameterSpecificationContext *referenceParameterSpecification();

   
  };

  GraphTypeReferenceContext* graphTypeReference();

  class  CatalogGraphTypeParentAndNameContext : public antlr4::ParserRuleContext {
  public:
    CatalogGraphTypeParentAndNameContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    GraphTypeNameContext *graphTypeName();
    CatalogObjectParentReferenceContext *catalogObjectParentReference();

   
  };

  CatalogGraphTypeParentAndNameContext* catalogGraphTypeParentAndName();

  class  BindingTableReferenceContext : public antlr4::ParserRuleContext {
  public:
    BindingTableReferenceContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    CatalogObjectParentReferenceContext *catalogObjectParentReference();
    BindingTableNameContext *bindingTableName();
    DelimitedBindingTableNameContext *delimitedBindingTableName();
    ReferenceParameterSpecificationContext *referenceParameterSpecification();

   
  };

  BindingTableReferenceContext* bindingTableReference();

  class  ProcedureReferenceContext : public antlr4::ParserRuleContext {
  public:
    ProcedureReferenceContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    CatalogProcedureParentAndNameContext *catalogProcedureParentAndName();
    ReferenceParameterSpecificationContext *referenceParameterSpecification();

   
  };

  ProcedureReferenceContext* procedureReference();

  class  CatalogProcedureParentAndNameContext : public antlr4::ParserRuleContext {
  public:
    CatalogProcedureParentAndNameContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    ProcedureNameContext *procedureName();
    CatalogObjectParentReferenceContext *catalogObjectParentReference();

   
  };

  CatalogProcedureParentAndNameContext* catalogProcedureParentAndName();

  class  CatalogObjectParentReferenceContext : public antlr4::ParserRuleContext {
  public:
    CatalogObjectParentReferenceContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    SchemaReferenceContext *schemaReference();
    antlr4::tree::TerminalNode *SOLIDUS();
    std::vector<ObjectNameContext *> objectName();
    ObjectNameContext* objectName(size_t i);
    std::vector<antlr4::tree::TerminalNode *> PERIOD();
    antlr4::tree::TerminalNode* PERIOD(size_t i);

   
  };

  CatalogObjectParentReferenceContext* catalogObjectParentReference();

  class  ReferenceParameterSpecificationContext : public antlr4::ParserRuleContext {
  public:
    ReferenceParameterSpecificationContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *SUBSTITUTED_PARAMETER_REFERENCE();

   
  };

  ReferenceParameterSpecificationContext* referenceParameterSpecification();

  class  NestedGraphTypeSpecificationContext : public antlr4::ParserRuleContext {
  public:
    NestedGraphTypeSpecificationContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LEFT_BRACE();
    GraphTypeSpecificationBodyContext *graphTypeSpecificationBody();
    antlr4::tree::TerminalNode *RIGHT_BRACE();

   
  };

  NestedGraphTypeSpecificationContext* nestedGraphTypeSpecification();

  class  GraphTypeSpecificationBodyContext : public antlr4::ParserRuleContext {
  public:
    GraphTypeSpecificationBodyContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    ElementTypeListContext *elementTypeList();

   
  };

  GraphTypeSpecificationBodyContext* graphTypeSpecificationBody();

  class  ElementTypeListContext : public antlr4::ParserRuleContext {
  public:
    ElementTypeListContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    std::vector<ElementTypeSpecificationContext *> elementTypeSpecification();
    ElementTypeSpecificationContext* elementTypeSpecification(size_t i);
    std::vector<antlr4::tree::TerminalNode *> COMMA();
    antlr4::tree::TerminalNode* COMMA(size_t i);

   
  };

  ElementTypeListContext* elementTypeList();

  class  ElementTypeSpecificationContext : public antlr4::ParserRuleContext {
  public:
    ElementTypeSpecificationContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    NodeTypeSpecificationContext *nodeTypeSpecification();
    EdgeTypeSpecificationContext *edgeTypeSpecification();

   
  };

  ElementTypeSpecificationContext* elementTypeSpecification();

  class  NodeTypeSpecificationContext : public antlr4::ParserRuleContext {
  public:
    NodeTypeSpecificationContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    NodeTypePatternContext *nodeTypePattern();
    NodeTypePhraseContext *nodeTypePhrase();

   
  };

  NodeTypeSpecificationContext* nodeTypeSpecification();

  class  NodeTypePatternContext : public antlr4::ParserRuleContext {
  public:
    NodeTypePatternContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LEFT_PAREN();
    antlr4::tree::TerminalNode *RIGHT_PAREN();
    NodeSynonymContext *nodeSynonym();
    NodeTypeNameContext *nodeTypeName();
    LocalNodeTypeAliasContext *localNodeTypeAlias();
    NodeTypeFillerContext *nodeTypeFiller();
    antlr4::tree::TerminalNode *TYPE();

   
  };

  NodeTypePatternContext* nodeTypePattern();

  class  NodeTypePhraseContext : public antlr4::ParserRuleContext {
  public:
    NodeTypePhraseContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    NodeSynonymContext *nodeSynonym();
    NodeTypePhraseFillerContext *nodeTypePhraseFiller();
    antlr4::tree::TerminalNode *TYPE();
    antlr4::tree::TerminalNode *AS();
    LocalNodeTypeAliasContext *localNodeTypeAlias();

   
  };

  NodeTypePhraseContext* nodeTypePhrase();

  class  NodeTypePhraseFillerContext : public antlr4::ParserRuleContext {
  public:
    NodeTypePhraseFillerContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    NodeTypeNameContext *nodeTypeName();
    NodeTypeFillerContext *nodeTypeFiller();

   
  };

  NodeTypePhraseFillerContext* nodeTypePhraseFiller();

  class  NodeTypeFillerContext : public antlr4::ParserRuleContext {
  public:
    NodeTypeFillerContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    NodeTypeKeyLabelSetContext *nodeTypeKeyLabelSet();
    NodeTypeImpliedContentContext *nodeTypeImpliedContent();

   
  };

  NodeTypeFillerContext* nodeTypeFiller();

  class  LocalNodeTypeAliasContext : public antlr4::ParserRuleContext {
  public:
    LocalNodeTypeAliasContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    RegularIdentifierContext *regularIdentifier();

   
  };

  LocalNodeTypeAliasContext* localNodeTypeAlias();

  class  NodeTypeImpliedContentContext : public antlr4::ParserRuleContext {
  public:
    NodeTypeImpliedContentContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    NodeTypeLabelSetContext *nodeTypeLabelSet();
    NodeTypePropertyTypesContext *nodeTypePropertyTypes();

   
  };

  NodeTypeImpliedContentContext* nodeTypeImpliedContent();

  class  NodeTypeKeyLabelSetContext : public antlr4::ParserRuleContext {
  public:
    NodeTypeKeyLabelSetContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *IMPLIES();
    LabelSetPhraseContext *labelSetPhrase();

   
  };

  NodeTypeKeyLabelSetContext* nodeTypeKeyLabelSet();

  class  NodeTypeLabelSetContext : public antlr4::ParserRuleContext {
  public:
    NodeTypeLabelSetContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    LabelSetPhraseContext *labelSetPhrase();

   
  };

  NodeTypeLabelSetContext* nodeTypeLabelSet();

  class  NodeTypePropertyTypesContext : public antlr4::ParserRuleContext {
  public:
    NodeTypePropertyTypesContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    PropertyTypesSpecificationContext *propertyTypesSpecification();

   
  };

  NodeTypePropertyTypesContext* nodeTypePropertyTypes();

  class  EdgeTypeSpecificationContext : public antlr4::ParserRuleContext {
  public:
    EdgeTypeSpecificationContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    EdgeTypePatternContext *edgeTypePattern();
    EdgeTypePhraseContext *edgeTypePhrase();

   
  };

  EdgeTypeSpecificationContext* edgeTypeSpecification();

  class  EdgeTypePatternContext : public antlr4::ParserRuleContext {
  public:
    EdgeTypePatternContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    EdgeTypePatternDirectedContext *edgeTypePatternDirected();
    EdgeTypePatternUndirectedContext *edgeTypePatternUndirected();
    EdgeSynonymContext *edgeSynonym();
    EdgeTypeNameContext *edgeTypeName();
    EdgeKindContext *edgeKind();
    antlr4::tree::TerminalNode *TYPE();

   
  };

  EdgeTypePatternContext* edgeTypePattern();

  class  EdgeTypePhraseContext : public antlr4::ParserRuleContext {
  public:
    EdgeTypePhraseContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    EdgeKindContext *edgeKind();
    EdgeSynonymContext *edgeSynonym();
    EdgeTypePhraseFillerContext *edgeTypePhraseFiller();
    EndpointPairPhraseContext *endpointPairPhrase();
    antlr4::tree::TerminalNode *TYPE();

   
  };

  EdgeTypePhraseContext* edgeTypePhrase();

  class  EdgeTypePhraseFillerContext : public antlr4::ParserRuleContext {
  public:
    EdgeTypePhraseFillerContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    EdgeTypeNameContext *edgeTypeName();
    EdgeTypeFillerContext *edgeTypeFiller();

   
  };

  EdgeTypePhraseFillerContext* edgeTypePhraseFiller();

  class  EdgeTypeFillerContext : public antlr4::ParserRuleContext {
  public:
    EdgeTypeFillerContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    EdgeTypeKeyLabelSetContext *edgeTypeKeyLabelSet();
    EdgeTypeImpliedContentContext *edgeTypeImpliedContent();

   
  };

  EdgeTypeFillerContext* edgeTypeFiller();

  class  EdgeTypeImpliedContentContext : public antlr4::ParserRuleContext {
  public:
    EdgeTypeImpliedContentContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    EdgeTypeLabelSetContext *edgeTypeLabelSet();
    EdgeTypePropertyTypesContext *edgeTypePropertyTypes();

   
  };

  EdgeTypeImpliedContentContext* edgeTypeImpliedContent();

  class  EdgeTypeKeyLabelSetContext : public antlr4::ParserRuleContext {
  public:
    EdgeTypeKeyLabelSetContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *IMPLIES();
    LabelSetPhraseContext *labelSetPhrase();

   
  };

  EdgeTypeKeyLabelSetContext* edgeTypeKeyLabelSet();

  class  EdgeTypeLabelSetContext : public antlr4::ParserRuleContext {
  public:
    EdgeTypeLabelSetContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    LabelSetPhraseContext *labelSetPhrase();

   
  };

  EdgeTypeLabelSetContext* edgeTypeLabelSet();

  class  EdgeTypePropertyTypesContext : public antlr4::ParserRuleContext {
  public:
    EdgeTypePropertyTypesContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    PropertyTypesSpecificationContext *propertyTypesSpecification();

   
  };

  EdgeTypePropertyTypesContext* edgeTypePropertyTypes();

  class  EdgeTypePatternDirectedContext : public antlr4::ParserRuleContext {
  public:
    EdgeTypePatternDirectedContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    EdgeTypePatternPointingRightContext *edgeTypePatternPointingRight();
    EdgeTypePatternPointingLeftContext *edgeTypePatternPointingLeft();

   
  };

  EdgeTypePatternDirectedContext* edgeTypePatternDirected();

  class  EdgeTypePatternPointingRightContext : public antlr4::ParserRuleContext {
  public:
    EdgeTypePatternPointingRightContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    SourceNodeTypeReferenceContext *sourceNodeTypeReference();
    ArcTypePointingRightContext *arcTypePointingRight();
    DestinationNodeTypeReferenceContext *destinationNodeTypeReference();

   
  };

  EdgeTypePatternPointingRightContext* edgeTypePatternPointingRight();

  class  EdgeTypePatternPointingLeftContext : public antlr4::ParserRuleContext {
  public:
    EdgeTypePatternPointingLeftContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    DestinationNodeTypeReferenceContext *destinationNodeTypeReference();
    ArcTypePointingLeftContext *arcTypePointingLeft();
    SourceNodeTypeReferenceContext *sourceNodeTypeReference();

   
  };

  EdgeTypePatternPointingLeftContext* edgeTypePatternPointingLeft();

  class  EdgeTypePatternUndirectedContext : public antlr4::ParserRuleContext {
  public:
    EdgeTypePatternUndirectedContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    SourceNodeTypeReferenceContext *sourceNodeTypeReference();
    ArcTypeUndirectedContext *arcTypeUndirected();
    DestinationNodeTypeReferenceContext *destinationNodeTypeReference();

   
  };

  EdgeTypePatternUndirectedContext* edgeTypePatternUndirected();

  class  ArcTypePointingRightContext : public antlr4::ParserRuleContext {
  public:
    ArcTypePointingRightContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *MINUS_LEFT_BRACKET();
    EdgeTypeFillerContext *edgeTypeFiller();
    antlr4::tree::TerminalNode *BRACKET_RIGHT_ARROW();

   
  };

  ArcTypePointingRightContext* arcTypePointingRight();

  class  ArcTypePointingLeftContext : public antlr4::ParserRuleContext {
  public:
    ArcTypePointingLeftContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LEFT_ARROW_BRACKET();
    EdgeTypeFillerContext *edgeTypeFiller();
    antlr4::tree::TerminalNode *RIGHT_BRACKET_MINUS();

   
  };

  ArcTypePointingLeftContext* arcTypePointingLeft();

  class  ArcTypeUndirectedContext : public antlr4::ParserRuleContext {
  public:
    ArcTypeUndirectedContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *TILDE_LEFT_BRACKET();
    EdgeTypeFillerContext *edgeTypeFiller();
    antlr4::tree::TerminalNode *RIGHT_BRACKET_TILDE();

   
  };

  ArcTypeUndirectedContext* arcTypeUndirected();

  class  SourceNodeTypeReferenceContext : public antlr4::ParserRuleContext {
  public:
    SourceNodeTypeReferenceContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LEFT_PAREN();
    SourceNodeTypeAliasContext *sourceNodeTypeAlias();
    antlr4::tree::TerminalNode *RIGHT_PAREN();
    NodeTypeFillerContext *nodeTypeFiller();

   
  };

  SourceNodeTypeReferenceContext* sourceNodeTypeReference();

  class  DestinationNodeTypeReferenceContext : public antlr4::ParserRuleContext {
  public:
    DestinationNodeTypeReferenceContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LEFT_PAREN();
    DestinationNodeTypeAliasContext *destinationNodeTypeAlias();
    antlr4::tree::TerminalNode *RIGHT_PAREN();
    NodeTypeFillerContext *nodeTypeFiller();

   
  };

  DestinationNodeTypeReferenceContext* destinationNodeTypeReference();

  class  EdgeKindContext : public antlr4::ParserRuleContext {
  public:
    EdgeKindContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *DIRECTED();
    antlr4::tree::TerminalNode *UNDIRECTED();

   
  };

  EdgeKindContext* edgeKind();

  class  EndpointPairPhraseContext : public antlr4::ParserRuleContext {
  public:
    EndpointPairPhraseContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *CONNECTING();
    EndpointPairContext *endpointPair();

   
  };

  EndpointPairPhraseContext* endpointPairPhrase();

  class  EndpointPairContext : public antlr4::ParserRuleContext {
  public:
    EndpointPairContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    EndpointPairDirectedContext *endpointPairDirected();
    EndpointPairUndirectedContext *endpointPairUndirected();

   
  };

  EndpointPairContext* endpointPair();

  class  EndpointPairDirectedContext : public antlr4::ParserRuleContext {
  public:
    EndpointPairDirectedContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    EndpointPairPointingRightContext *endpointPairPointingRight();
    EndpointPairPointingLeftContext *endpointPairPointingLeft();

   
  };

  EndpointPairDirectedContext* endpointPairDirected();

  class  EndpointPairPointingRightContext : public antlr4::ParserRuleContext {
  public:
    EndpointPairPointingRightContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LEFT_PAREN();
    SourceNodeTypeAliasContext *sourceNodeTypeAlias();
    ConnectorPointingRightContext *connectorPointingRight();
    DestinationNodeTypeAliasContext *destinationNodeTypeAlias();
    antlr4::tree::TerminalNode *RIGHT_PAREN();

   
  };

  EndpointPairPointingRightContext* endpointPairPointingRight();

  class  EndpointPairPointingLeftContext : public antlr4::ParserRuleContext {
  public:
    EndpointPairPointingLeftContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LEFT_PAREN();
    DestinationNodeTypeAliasContext *destinationNodeTypeAlias();
    antlr4::tree::TerminalNode *LEFT_ARROW();
    SourceNodeTypeAliasContext *sourceNodeTypeAlias();
    antlr4::tree::TerminalNode *RIGHT_PAREN();

   
  };

  EndpointPairPointingLeftContext* endpointPairPointingLeft();

  class  EndpointPairUndirectedContext : public antlr4::ParserRuleContext {
  public:
    EndpointPairUndirectedContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LEFT_PAREN();
    SourceNodeTypeAliasContext *sourceNodeTypeAlias();
    ConnectorUndirectedContext *connectorUndirected();
    DestinationNodeTypeAliasContext *destinationNodeTypeAlias();
    antlr4::tree::TerminalNode *RIGHT_PAREN();

   
  };

  EndpointPairUndirectedContext* endpointPairUndirected();

  class  ConnectorPointingRightContext : public antlr4::ParserRuleContext {
  public:
    ConnectorPointingRightContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *TO();
    antlr4::tree::TerminalNode *RIGHT_ARROW();

   
  };

  ConnectorPointingRightContext* connectorPointingRight();

  class  ConnectorUndirectedContext : public antlr4::ParserRuleContext {
  public:
    ConnectorUndirectedContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *TO();
    antlr4::tree::TerminalNode *TILDE();

   
  };

  ConnectorUndirectedContext* connectorUndirected();

  class  SourceNodeTypeAliasContext : public antlr4::ParserRuleContext {
  public:
    SourceNodeTypeAliasContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    RegularIdentifierContext *regularIdentifier();

   
  };

  SourceNodeTypeAliasContext* sourceNodeTypeAlias();

  class  DestinationNodeTypeAliasContext : public antlr4::ParserRuleContext {
  public:
    DestinationNodeTypeAliasContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    RegularIdentifierContext *regularIdentifier();

   
  };

  DestinationNodeTypeAliasContext* destinationNodeTypeAlias();

  class  LabelSetPhraseContext : public antlr4::ParserRuleContext {
  public:
    LabelSetPhraseContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LABEL();
    LabelNameContext *labelName();
    antlr4::tree::TerminalNode *LABELS();
    LabelSetSpecificationContext *labelSetSpecification();
    IsOrColonContext *isOrColon();

   
  };

  LabelSetPhraseContext* labelSetPhrase();

  class  LabelSetSpecificationContext : public antlr4::ParserRuleContext {
  public:
    LabelSetSpecificationContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    std::vector<LabelNameContext *> labelName();
    LabelNameContext* labelName(size_t i);
    std::vector<antlr4::tree::TerminalNode *> AMPERSAND();
    antlr4::tree::TerminalNode* AMPERSAND(size_t i);

   
  };

  LabelSetSpecificationContext* labelSetSpecification();

  class  PropertyTypesSpecificationContext : public antlr4::ParserRuleContext {
  public:
    PropertyTypesSpecificationContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LEFT_BRACE();
    antlr4::tree::TerminalNode *RIGHT_BRACE();
    PropertyTypeListContext *propertyTypeList();

   
  };

  PropertyTypesSpecificationContext* propertyTypesSpecification();

  class  PropertyTypeListContext : public antlr4::ParserRuleContext {
  public:
    PropertyTypeListContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    std::vector<PropertyTypeContext *> propertyType();
    PropertyTypeContext* propertyType(size_t i);
    std::vector<antlr4::tree::TerminalNode *> COMMA();
    antlr4::tree::TerminalNode* COMMA(size_t i);

   
  };

  PropertyTypeListContext* propertyTypeList();

  class  PropertyTypeContext : public antlr4::ParserRuleContext {
  public:
    PropertyTypeContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    PropertyNameContext *propertyName();
    PropertyValueTypeContext *propertyValueType();
    TypedContext *typed();

   
  };

  PropertyTypeContext* propertyType();

  class  PropertyValueTypeContext : public antlr4::ParserRuleContext {
  public:
    PropertyValueTypeContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    ValueTypeContext *valueType();

   
  };

  PropertyValueTypeContext* propertyValueType();

  class  BindingTableTypeContext : public antlr4::ParserRuleContext {
  public:
    BindingTableTypeContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *TABLE();
    FieldTypesSpecificationContext *fieldTypesSpecification();
    antlr4::tree::TerminalNode *BINDING();

   
  };

  BindingTableTypeContext* bindingTableType();

  class  ValueTypeContext : public antlr4::ParserRuleContext {
  public:
    ValueTypeContext(antlr4::ParserRuleContext *parent, size_t invokingState);
   
    ValueTypeContext() = default;
    void copyFrom(ValueTypeContext *context);
    using antlr4::ParserRuleContext::copyFrom;

    virtual size_t getRuleIndex() const override;

   
  };

  class  DynamicPropertyValueTypeLabelContext : public ValueTypeContext {
  public:
    DynamicPropertyValueTypeLabelContext(ValueTypeContext *ctx);

    antlr4::tree::TerminalNode *PROPERTY();
    antlr4::tree::TerminalNode *VALUE();
    antlr4::tree::TerminalNode *ANY();
    NotNullContext *notNull();
  };

  class  ClosedDynamicUnionTypeAtl1Context : public ValueTypeContext {
  public:
    ClosedDynamicUnionTypeAtl1Context(ValueTypeContext *ctx);

    antlr4::tree::TerminalNode *ANY();
    antlr4::tree::TerminalNode *LEFT_ANGLE_BRACKET();
    std::vector<ValueTypeContext *> valueType();
    ValueTypeContext* valueType(size_t i);
    antlr4::tree::TerminalNode *RIGHT_ANGLE_BRACKET();
    antlr4::tree::TerminalNode *VALUE();
    std::vector<antlr4::tree::TerminalNode *> VERTICAL_BAR();
    antlr4::tree::TerminalNode* VERTICAL_BAR(size_t i);
  };

  class  ClosedDynamicUnionTypeAtl2Context : public ValueTypeContext {
  public:
    ClosedDynamicUnionTypeAtl2Context(ValueTypeContext *ctx);

    std::vector<ValueTypeContext *> valueType();
    ValueTypeContext* valueType(size_t i);
    antlr4::tree::TerminalNode *VERTICAL_BAR();
  };

  class  PathValueTypeLabelContext : public ValueTypeContext {
  public:
    PathValueTypeLabelContext(ValueTypeContext *ctx);

    PathValueTypeContext *pathValueType();
  };

  class  ListValueTypeAlt3Context : public ValueTypeContext {
  public:
    ListValueTypeAlt3Context(ValueTypeContext *ctx);

    ListValueTypeNameContext *listValueTypeName();
    antlr4::tree::TerminalNode *LEFT_BRACKET();
    MaxLengthContext *maxLength();
    antlr4::tree::TerminalNode *RIGHT_BRACKET();
    NotNullContext *notNull();
  };

  class  ListValueTypeAlt2Context : public ValueTypeContext {
  public:
    ListValueTypeAlt2Context(ValueTypeContext *ctx);

    ValueTypeContext *valueType();
    ListValueTypeNameContext *listValueTypeName();
    antlr4::tree::TerminalNode *LEFT_BRACKET();
    MaxLengthContext *maxLength();
    antlr4::tree::TerminalNode *RIGHT_BRACKET();
    NotNullContext *notNull();
  };

  class  ListValueTypeAlt1Context : public ValueTypeContext {
  public:
    ListValueTypeAlt1Context(ValueTypeContext *ctx);

    ListValueTypeNameContext *listValueTypeName();
    antlr4::tree::TerminalNode *LEFT_ANGLE_BRACKET();
    ValueTypeContext *valueType();
    antlr4::tree::TerminalNode *RIGHT_ANGLE_BRACKET();
    antlr4::tree::TerminalNode *LEFT_BRACKET();
    MaxLengthContext *maxLength();
    antlr4::tree::TerminalNode *RIGHT_BRACKET();
    NotNullContext *notNull();
  };

  class  PredefinedTypeLabelContext : public ValueTypeContext {
  public:
    PredefinedTypeLabelContext(ValueTypeContext *ctx);

    PredefinedTypeContext *predefinedType();
  };

  class  RecordTypeLabelContext : public ValueTypeContext {
  public:
    RecordTypeLabelContext(ValueTypeContext *ctx);

    RecordTypeContext *recordType();
  };

  class  OpenDynamicUnionTypeLabelContext : public ValueTypeContext {
  public:
    OpenDynamicUnionTypeLabelContext(ValueTypeContext *ctx);

    antlr4::tree::TerminalNode *ANY();
    antlr4::tree::TerminalNode *VALUE();
    NotNullContext *notNull();
  };

  ValueTypeContext* valueType();
  ValueTypeContext* valueType(int precedence);
  class  TypedContext : public antlr4::ParserRuleContext {
  public:
    TypedContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *DOUBLE_COLON();
    antlr4::tree::TerminalNode *TYPED();

   
  };

  TypedContext* typed();

  class  PredefinedTypeContext : public antlr4::ParserRuleContext {
  public:
    PredefinedTypeContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    BooleanTypeContext *booleanType();
    CharacterStringTypeContext *characterStringType();
    ByteStringTypeContext *byteStringType();
    NumericTypeContext *numericType();
    TemporalTypeContext *temporalType();
    ReferenceValueTypeContext *referenceValueType();
    ImmaterialValueTypeContext *immaterialValueType();

   
  };

  PredefinedTypeContext* predefinedType();

  class  BooleanTypeContext : public antlr4::ParserRuleContext {
  public:
    BooleanTypeContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *BOOL();
    antlr4::tree::TerminalNode *BOOLEAN();
    NotNullContext *notNull();

   
  };

  BooleanTypeContext* booleanType();

  class  CharacterStringTypeContext : public antlr4::ParserRuleContext {
  public:
    CharacterStringTypeContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *STRING();
    antlr4::tree::TerminalNode *LEFT_PAREN();
    MaxLengthContext *maxLength();
    antlr4::tree::TerminalNode *RIGHT_PAREN();
    NotNullContext *notNull();
    MinLengthContext *minLength();
    antlr4::tree::TerminalNode *COMMA();
    antlr4::tree::TerminalNode *CHAR();
    FixedLengthContext *fixedLength();
    antlr4::tree::TerminalNode *VARCHAR();

   
  };

  CharacterStringTypeContext* characterStringType();

  class  ByteStringTypeContext : public antlr4::ParserRuleContext {
  public:
    ByteStringTypeContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *BYTES();
    antlr4::tree::TerminalNode *LEFT_PAREN();
    MaxLengthContext *maxLength();
    antlr4::tree::TerminalNode *RIGHT_PAREN();
    NotNullContext *notNull();
    MinLengthContext *minLength();
    antlr4::tree::TerminalNode *COMMA();
    antlr4::tree::TerminalNode *BINARY();
    FixedLengthContext *fixedLength();
    antlr4::tree::TerminalNode *VARBINARY();

   
  };

  ByteStringTypeContext* byteStringType();

  class  MinLengthContext : public antlr4::ParserRuleContext {
  public:
    MinLengthContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    UnsignedIntegerContext *unsignedInteger();

   
  };

  MinLengthContext* minLength();

  class  MaxLengthContext : public antlr4::ParserRuleContext {
  public:
    MaxLengthContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    UnsignedIntegerContext *unsignedInteger();

   
  };

  MaxLengthContext* maxLength();

  class  FixedLengthContext : public antlr4::ParserRuleContext {
  public:
    FixedLengthContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    UnsignedIntegerContext *unsignedInteger();

   
  };

  FixedLengthContext* fixedLength();

  class  NumericTypeContext : public antlr4::ParserRuleContext {
  public:
    NumericTypeContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    ExactNumericTypeContext *exactNumericType();
    ApproximateNumericTypeContext *approximateNumericType();

   
  };

  NumericTypeContext* numericType();

  class  ExactNumericTypeContext : public antlr4::ParserRuleContext {
  public:
    ExactNumericTypeContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    BinaryExactNumericTypeContext *binaryExactNumericType();
    DecimalExactNumericTypeContext *decimalExactNumericType();

   
  };

  ExactNumericTypeContext* exactNumericType();

  class  BinaryExactNumericTypeContext : public antlr4::ParserRuleContext {
  public:
    BinaryExactNumericTypeContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    SignedBinaryExactNumericTypeContext *signedBinaryExactNumericType();
    UnsignedBinaryExactNumericTypeContext *unsignedBinaryExactNumericType();

   
  };

  BinaryExactNumericTypeContext* binaryExactNumericType();

  class  SignedBinaryExactNumericTypeContext : public antlr4::ParserRuleContext {
  public:
    SignedBinaryExactNumericTypeContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *INT8();
    NotNullContext *notNull();
    antlr4::tree::TerminalNode *INT16();
    antlr4::tree::TerminalNode *INT32();
    antlr4::tree::TerminalNode *INT64();
    antlr4::tree::TerminalNode *INT128();
    antlr4::tree::TerminalNode *INT256();
    antlr4::tree::TerminalNode *SMALLINT();
    antlr4::tree::TerminalNode *INT();
    antlr4::tree::TerminalNode *LEFT_PAREN();
    PrecisionContext *precision();
    antlr4::tree::TerminalNode *RIGHT_PAREN();
    antlr4::tree::TerminalNode *BIGINT();
    VerboseBinaryExactNumericTypeContext *verboseBinaryExactNumericType();
    antlr4::tree::TerminalNode *SIGNED();

   
  };

  SignedBinaryExactNumericTypeContext* signedBinaryExactNumericType();

  class  UnsignedBinaryExactNumericTypeContext : public antlr4::ParserRuleContext {
  public:
    UnsignedBinaryExactNumericTypeContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *UINT8();
    NotNullContext *notNull();
    antlr4::tree::TerminalNode *UINT16();
    antlr4::tree::TerminalNode *UINT32();
    antlr4::tree::TerminalNode *UINT64();
    antlr4::tree::TerminalNode *UINT128();
    antlr4::tree::TerminalNode *UINT256();
    antlr4::tree::TerminalNode *USMALLINT();
    antlr4::tree::TerminalNode *UINT();
    antlr4::tree::TerminalNode *LEFT_PAREN();
    PrecisionContext *precision();
    antlr4::tree::TerminalNode *RIGHT_PAREN();
    antlr4::tree::TerminalNode *UBIGINT();
    antlr4::tree::TerminalNode *UNSIGNED();
    VerboseBinaryExactNumericTypeContext *verboseBinaryExactNumericType();

   
  };

  UnsignedBinaryExactNumericTypeContext* unsignedBinaryExactNumericType();

  class  VerboseBinaryExactNumericTypeContext : public antlr4::ParserRuleContext {
  public:
    VerboseBinaryExactNumericTypeContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *INTEGER8();
    NotNullContext *notNull();
    antlr4::tree::TerminalNode *INTEGER16();
    antlr4::tree::TerminalNode *INTEGER32();
    antlr4::tree::TerminalNode *INTEGER64();
    antlr4::tree::TerminalNode *INTEGER128();
    antlr4::tree::TerminalNode *INTEGER256();
    antlr4::tree::TerminalNode *SMALL();
    antlr4::tree::TerminalNode *INTEGER();
    antlr4::tree::TerminalNode *LEFT_PAREN();
    PrecisionContext *precision();
    antlr4::tree::TerminalNode *RIGHT_PAREN();
    antlr4::tree::TerminalNode *BIG();

   
  };

  VerboseBinaryExactNumericTypeContext* verboseBinaryExactNumericType();

  class  DecimalExactNumericTypeContext : public antlr4::ParserRuleContext {
  public:
    DecimalExactNumericTypeContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *DECIMAL();
    antlr4::tree::TerminalNode *DEC();
    antlr4::tree::TerminalNode *LEFT_PAREN();
    PrecisionContext *precision();
    antlr4::tree::TerminalNode *RIGHT_PAREN();
    antlr4::tree::TerminalNode *COMMA();
    ScaleContext *scale();
    NotNullContext *notNull();

   
  };

  DecimalExactNumericTypeContext* decimalExactNumericType();

  class  PrecisionContext : public antlr4::ParserRuleContext {
  public:
    PrecisionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    UnsignedDecimalIntegerContext *unsignedDecimalInteger();

   
  };

  PrecisionContext* precision();

  class  ScaleContext : public antlr4::ParserRuleContext {
  public:
    ScaleContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    UnsignedDecimalIntegerContext *unsignedDecimalInteger();

   
  };

  ScaleContext* scale();

  class  ApproximateNumericTypeContext : public antlr4::ParserRuleContext {
  public:
    ApproximateNumericTypeContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *FLOAT16();
    NotNullContext *notNull();
    antlr4::tree::TerminalNode *FLOAT32();
    antlr4::tree::TerminalNode *FLOAT64();
    antlr4::tree::TerminalNode *FLOAT128();
    antlr4::tree::TerminalNode *FLOAT256();
    antlr4::tree::TerminalNode *FLOAT();
    antlr4::tree::TerminalNode *LEFT_PAREN();
    PrecisionContext *precision();
    antlr4::tree::TerminalNode *RIGHT_PAREN();
    antlr4::tree::TerminalNode *COMMA();
    ScaleContext *scale();
    antlr4::tree::TerminalNode *REAL();
    antlr4::tree::TerminalNode *DOUBLE();
    antlr4::tree::TerminalNode *PRECISION();

   
  };

  ApproximateNumericTypeContext* approximateNumericType();

  class  TemporalTypeContext : public antlr4::ParserRuleContext {
  public:
    TemporalTypeContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    TemporalInstantTypeContext *temporalInstantType();
    TemporalDurationTypeContext *temporalDurationType();

   
  };

  TemporalTypeContext* temporalType();

  class  TemporalInstantTypeContext : public antlr4::ParserRuleContext {
  public:
    TemporalInstantTypeContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    DatetimeTypeContext *datetimeType();
    LocaldatetimeTypeContext *localdatetimeType();
    DateTypeContext *dateType();
    TimeTypeContext *timeType();
    LocaltimeTypeContext *localtimeType();

   
  };

  TemporalInstantTypeContext* temporalInstantType();

  class  DatetimeTypeContext : public antlr4::ParserRuleContext {
  public:
    DatetimeTypeContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *ZONED();
    antlr4::tree::TerminalNode *DATETIME();
    NotNullContext *notNull();
    antlr4::tree::TerminalNode *TIMESTAMP();
    antlr4::tree::TerminalNode *WITH();
    antlr4::tree::TerminalNode *TIME();
    antlr4::tree::TerminalNode *ZONE();

   
  };

  DatetimeTypeContext* datetimeType();

  class  LocaldatetimeTypeContext : public antlr4::ParserRuleContext {
  public:
    LocaldatetimeTypeContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LOCAL();
    antlr4::tree::TerminalNode *DATETIME();
    NotNullContext *notNull();
    antlr4::tree::TerminalNode *TIMESTAMP();
    antlr4::tree::TerminalNode *WITHOUT();
    antlr4::tree::TerminalNode *TIME();
    antlr4::tree::TerminalNode *ZONE();

   
  };

  LocaldatetimeTypeContext* localdatetimeType();

  class  DateTypeContext : public antlr4::ParserRuleContext {
  public:
    DateTypeContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *DATE();
    NotNullContext *notNull();

   
  };

  DateTypeContext* dateType();

  class  TimeTypeContext : public antlr4::ParserRuleContext {
  public:
    TimeTypeContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *ZONED();
    std::vector<antlr4::tree::TerminalNode *> TIME();
    antlr4::tree::TerminalNode* TIME(size_t i);
    NotNullContext *notNull();
    antlr4::tree::TerminalNode *WITH();
    antlr4::tree::TerminalNode *ZONE();

   
  };

  TimeTypeContext* timeType();

  class  LocaltimeTypeContext : public antlr4::ParserRuleContext {
  public:
    LocaltimeTypeContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LOCAL();
    std::vector<antlr4::tree::TerminalNode *> TIME();
    antlr4::tree::TerminalNode* TIME(size_t i);
    NotNullContext *notNull();
    antlr4::tree::TerminalNode *WITHOUT();
    antlr4::tree::TerminalNode *ZONE();

   
  };

  LocaltimeTypeContext* localtimeType();

  class  TemporalDurationTypeContext : public antlr4::ParserRuleContext {
  public:
    TemporalDurationTypeContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *DURATION();
    antlr4::tree::TerminalNode *LEFT_PAREN();
    TemporalDurationQualifierContext *temporalDurationQualifier();
    antlr4::tree::TerminalNode *RIGHT_PAREN();
    NotNullContext *notNull();

   
  };

  TemporalDurationTypeContext* temporalDurationType();

  class  TemporalDurationQualifierContext : public antlr4::ParserRuleContext {
  public:
    TemporalDurationQualifierContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *YEAR();
    antlr4::tree::TerminalNode *TO();
    antlr4::tree::TerminalNode *MONTH();
    antlr4::tree::TerminalNode *DAY();
    antlr4::tree::TerminalNode *SECOND();

   
  };

  TemporalDurationQualifierContext* temporalDurationQualifier();

  class  ReferenceValueTypeContext : public antlr4::ParserRuleContext {
  public:
    ReferenceValueTypeContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    GraphReferenceValueTypeContext *graphReferenceValueType();
    BindingTableReferenceValueTypeContext *bindingTableReferenceValueType();
    NodeReferenceValueTypeContext *nodeReferenceValueType();
    EdgeReferenceValueTypeContext *edgeReferenceValueType();

   
  };

  ReferenceValueTypeContext* referenceValueType();

  class  ImmaterialValueTypeContext : public antlr4::ParserRuleContext {
  public:
    ImmaterialValueTypeContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    NullTypeContext *nullType();
    EmptyTypeContext *emptyType();

   
  };

  ImmaterialValueTypeContext* immaterialValueType();

  class  NullTypeContext : public antlr4::ParserRuleContext {
  public:
    NullTypeContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *NULL_KW();

   
  };

  NullTypeContext* nullType();

  class  EmptyTypeContext : public antlr4::ParserRuleContext {
  public:
    EmptyTypeContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *NULL_KW();
    NotNullContext *notNull();
    antlr4::tree::TerminalNode *NOTHING();

   
  };

  EmptyTypeContext* emptyType();

  class  GraphReferenceValueTypeContext : public antlr4::ParserRuleContext {
  public:
    GraphReferenceValueTypeContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    OpenGraphReferenceValueTypeContext *openGraphReferenceValueType();
    ClosedGraphReferenceValueTypeContext *closedGraphReferenceValueType();

   
  };

  GraphReferenceValueTypeContext* graphReferenceValueType();

  class  ClosedGraphReferenceValueTypeContext : public antlr4::ParserRuleContext {
  public:
    ClosedGraphReferenceValueTypeContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *GRAPH();
    NestedGraphTypeSpecificationContext *nestedGraphTypeSpecification();
    antlr4::tree::TerminalNode *PROPERTY();
    NotNullContext *notNull();

   
  };

  ClosedGraphReferenceValueTypeContext* closedGraphReferenceValueType();

  class  OpenGraphReferenceValueTypeContext : public antlr4::ParserRuleContext {
  public:
    OpenGraphReferenceValueTypeContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *ANY();
    antlr4::tree::TerminalNode *GRAPH();
    antlr4::tree::TerminalNode *PROPERTY();
    NotNullContext *notNull();

   
  };

  OpenGraphReferenceValueTypeContext* openGraphReferenceValueType();

  class  BindingTableReferenceValueTypeContext : public antlr4::ParserRuleContext {
  public:
    BindingTableReferenceValueTypeContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    BindingTableTypeContext *bindingTableType();
    NotNullContext *notNull();

   
  };

  BindingTableReferenceValueTypeContext* bindingTableReferenceValueType();

  class  NodeReferenceValueTypeContext : public antlr4::ParserRuleContext {
  public:
    NodeReferenceValueTypeContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    OpenNodeReferenceValueTypeContext *openNodeReferenceValueType();
    ClosedNodeReferenceValueTypeContext *closedNodeReferenceValueType();

   
  };

  NodeReferenceValueTypeContext* nodeReferenceValueType();

  class  ClosedNodeReferenceValueTypeContext : public antlr4::ParserRuleContext {
  public:
    ClosedNodeReferenceValueTypeContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    NodeTypeSpecificationContext *nodeTypeSpecification();
    NotNullContext *notNull();

   
  };

  ClosedNodeReferenceValueTypeContext* closedNodeReferenceValueType();

  class  OpenNodeReferenceValueTypeContext : public antlr4::ParserRuleContext {
  public:
    OpenNodeReferenceValueTypeContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    NodeSynonymContext *nodeSynonym();
    antlr4::tree::TerminalNode *ANY();
    NotNullContext *notNull();

   
  };

  OpenNodeReferenceValueTypeContext* openNodeReferenceValueType();

  class  EdgeReferenceValueTypeContext : public antlr4::ParserRuleContext {
  public:
    EdgeReferenceValueTypeContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    OpenEdgeReferenceValueTypeContext *openEdgeReferenceValueType();
    ClosedEdgeReferenceValueTypeContext *closedEdgeReferenceValueType();

   
  };

  EdgeReferenceValueTypeContext* edgeReferenceValueType();

  class  ClosedEdgeReferenceValueTypeContext : public antlr4::ParserRuleContext {
  public:
    ClosedEdgeReferenceValueTypeContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    EdgeTypeSpecificationContext *edgeTypeSpecification();
    NotNullContext *notNull();

   
  };

  ClosedEdgeReferenceValueTypeContext* closedEdgeReferenceValueType();

  class  OpenEdgeReferenceValueTypeContext : public antlr4::ParserRuleContext {
  public:
    OpenEdgeReferenceValueTypeContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    EdgeSynonymContext *edgeSynonym();
    antlr4::tree::TerminalNode *ANY();
    NotNullContext *notNull();

   
  };

  OpenEdgeReferenceValueTypeContext* openEdgeReferenceValueType();

  class  PathValueTypeContext : public antlr4::ParserRuleContext {
  public:
    PathValueTypeContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *PATH();
    NotNullContext *notNull();

   
  };

  PathValueTypeContext* pathValueType();

  class  ListValueTypeNameContext : public antlr4::ParserRuleContext {
  public:
    ListValueTypeNameContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    ListValueTypeNameSynonymContext *listValueTypeNameSynonym();

   
  };

  ListValueTypeNameContext* listValueTypeName();

  class  ListValueTypeNameSynonymContext : public antlr4::ParserRuleContext {
  public:
    ListValueTypeNameSynonymContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LIST();
    antlr4::tree::TerminalNode *ARRAY();

   
  };

  ListValueTypeNameSynonymContext* listValueTypeNameSynonym();

  class  RecordTypeContext : public antlr4::ParserRuleContext {
  public:
    RecordTypeContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *RECORD();
    antlr4::tree::TerminalNode *ANY();
    NotNullContext *notNull();
    FieldTypesSpecificationContext *fieldTypesSpecification();

   
  };

  RecordTypeContext* recordType();

  class  FieldTypesSpecificationContext : public antlr4::ParserRuleContext {
  public:
    FieldTypesSpecificationContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LEFT_BRACE();
    antlr4::tree::TerminalNode *RIGHT_BRACE();
    FieldTypeListContext *fieldTypeList();

   
  };

  FieldTypesSpecificationContext* fieldTypesSpecification();

  class  FieldTypeListContext : public antlr4::ParserRuleContext {
  public:
    FieldTypeListContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    std::vector<FieldTypeContext *> fieldType();
    FieldTypeContext* fieldType(size_t i);
    std::vector<antlr4::tree::TerminalNode *> COMMA();
    antlr4::tree::TerminalNode* COMMA(size_t i);

   
  };

  FieldTypeListContext* fieldTypeList();

  class  NotNullContext : public antlr4::ParserRuleContext {
  public:
    NotNullContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *NOT();
    antlr4::tree::TerminalNode *NULL_KW();

   
  };

  NotNullContext* notNull();

  class  FieldTypeContext : public antlr4::ParserRuleContext {
  public:
    FieldTypeContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    FieldNameContext *fieldName();
    ValueTypeContext *valueType();
    TypedContext *typed();

   
  };

  FieldTypeContext* fieldType();

  class  SearchConditionContext : public antlr4::ParserRuleContext {
  public:
    SearchConditionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    BooleanValueExpressionContext *booleanValueExpression();

   
  };

  SearchConditionContext* searchCondition();

  class  PredicateContext : public antlr4::ParserRuleContext {
  public:
    PredicateContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    ExistsPredicateContext *existsPredicate();
    NullPredicateContext *nullPredicate();
    ValueTypePredicateContext *valueTypePredicate();
    DirectedPredicateContext *directedPredicate();
    LabeledPredicateContext *labeledPredicate();
    SourceDestinationPredicateContext *sourceDestinationPredicate();
    All_differentPredicateContext *all_differentPredicate();
    SamePredicateContext *samePredicate();
    Property_existsPredicateContext *property_existsPredicate();

   
  };

  PredicateContext* predicate();

  class  CompOpContext : public antlr4::ParserRuleContext {
  public:
    CompOpContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *EQUALS_OPERATOR();
    antlr4::tree::TerminalNode *NOT_EQUALS_OPERATOR();
    antlr4::tree::TerminalNode *LEFT_ANGLE_BRACKET();
    antlr4::tree::TerminalNode *RIGHT_ANGLE_BRACKET();
    antlr4::tree::TerminalNode *LESS_THAN_OR_EQUALS_OPERATOR();
    antlr4::tree::TerminalNode *GREATER_THAN_OR_EQUALS_OPERATOR();

   
  };

  CompOpContext* compOp();

  class  ExistsPredicateContext : public antlr4::ParserRuleContext {
  public:
    ExistsPredicateContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *EXISTS();
    antlr4::tree::TerminalNode *LEFT_BRACE();
    GraphPatternContext *graphPattern();
    antlr4::tree::TerminalNode *RIGHT_BRACE();
    antlr4::tree::TerminalNode *LEFT_PAREN();
    antlr4::tree::TerminalNode *RIGHT_PAREN();
    MatchStatementBlockContext *matchStatementBlock();
    NestedQuerySpecificationContext *nestedQuerySpecification();

   
  };

  ExistsPredicateContext* existsPredicate();

  class  NullPredicateContext : public antlr4::ParserRuleContext {
  public:
    NullPredicateContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    ValueExpressionPrimaryContext *valueExpressionPrimary();
    NullPredicatePart2Context *nullPredicatePart2();

   
  };

  NullPredicateContext* nullPredicate();

  class  NullPredicatePart2Context : public antlr4::ParserRuleContext {
  public:
    NullPredicatePart2Context(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *IS();
    antlr4::tree::TerminalNode *NULL_KW();
    antlr4::tree::TerminalNode *NOT();

   
  };

  NullPredicatePart2Context* nullPredicatePart2();

  class  ValueTypePredicateContext : public antlr4::ParserRuleContext {
  public:
    ValueTypePredicateContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    ValueExpressionPrimaryContext *valueExpressionPrimary();
    ValueTypePredicatePart2Context *valueTypePredicatePart2();

   
  };

  ValueTypePredicateContext* valueTypePredicate();

  class  ValueTypePredicatePart2Context : public antlr4::ParserRuleContext {
  public:
    ValueTypePredicatePart2Context(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *IS();
    TypedContext *typed();
    ValueTypeContext *valueType();
    antlr4::tree::TerminalNode *NOT();

   
  };

  ValueTypePredicatePart2Context* valueTypePredicatePart2();

  class  NormalizedPredicatePart2Context : public antlr4::ParserRuleContext {
  public:
    NormalizedPredicatePart2Context(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *IS();
    antlr4::tree::TerminalNode *NORMALIZED();
    antlr4::tree::TerminalNode *NOT();
    NormalFormContext *normalForm();

   
  };

  NormalizedPredicatePart2Context* normalizedPredicatePart2();

  class  DirectedPredicateContext : public antlr4::ParserRuleContext {
  public:
    DirectedPredicateContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    ElementVariableReferenceContext *elementVariableReference();
    DirectedPredicatePart2Context *directedPredicatePart2();

   
  };

  DirectedPredicateContext* directedPredicate();

  class  DirectedPredicatePart2Context : public antlr4::ParserRuleContext {
  public:
    DirectedPredicatePart2Context(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *IS();
    antlr4::tree::TerminalNode *DIRECTED();
    antlr4::tree::TerminalNode *NOT();

   
  };

  DirectedPredicatePart2Context* directedPredicatePart2();

  class  LabeledPredicateContext : public antlr4::ParserRuleContext {
  public:
    LabeledPredicateContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    ElementVariableReferenceContext *elementVariableReference();
    LabeledPredicatePart2Context *labeledPredicatePart2();

   
  };

  LabeledPredicateContext* labeledPredicate();

  class  LabeledPredicatePart2Context : public antlr4::ParserRuleContext {
  public:
    LabeledPredicatePart2Context(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    IsLabeledOrColonContext *isLabeledOrColon();
    LabelExpressionContext *labelExpression();

   
  };

  LabeledPredicatePart2Context* labeledPredicatePart2();

  class  IsLabeledOrColonContext : public antlr4::ParserRuleContext {
  public:
    IsLabeledOrColonContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *IS();
    antlr4::tree::TerminalNode *LABELED();
    antlr4::tree::TerminalNode *NOT();
    antlr4::tree::TerminalNode *COLON();

   
  };

  IsLabeledOrColonContext* isLabeledOrColon();

  class  SourceDestinationPredicateContext : public antlr4::ParserRuleContext {
  public:
    SourceDestinationPredicateContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    NodeReferenceContext *nodeReference();
    SourcePredicatePart2Context *sourcePredicatePart2();
    DestinationPredicatePart2Context *destinationPredicatePart2();

   
  };

  SourceDestinationPredicateContext* sourceDestinationPredicate();

  class  NodeReferenceContext : public antlr4::ParserRuleContext {
  public:
    NodeReferenceContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    ElementVariableReferenceContext *elementVariableReference();

   
  };

  NodeReferenceContext* nodeReference();

  class  SourcePredicatePart2Context : public antlr4::ParserRuleContext {
  public:
    SourcePredicatePart2Context(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *IS();
    antlr4::tree::TerminalNode *SOURCE();
    antlr4::tree::TerminalNode *OF();
    EdgeReferenceContext *edgeReference();
    antlr4::tree::TerminalNode *NOT();

   
  };

  SourcePredicatePart2Context* sourcePredicatePart2();

  class  DestinationPredicatePart2Context : public antlr4::ParserRuleContext {
  public:
    DestinationPredicatePart2Context(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *IS();
    antlr4::tree::TerminalNode *DESTINATION();
    antlr4::tree::TerminalNode *OF();
    EdgeReferenceContext *edgeReference();
    antlr4::tree::TerminalNode *NOT();

   
  };

  DestinationPredicatePart2Context* destinationPredicatePart2();

  class  EdgeReferenceContext : public antlr4::ParserRuleContext {
  public:
    EdgeReferenceContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    ElementVariableReferenceContext *elementVariableReference();

   
  };

  EdgeReferenceContext* edgeReference();

  class  All_differentPredicateContext : public antlr4::ParserRuleContext {
  public:
    All_differentPredicateContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *ALL_DIFFERENT();
    antlr4::tree::TerminalNode *LEFT_PAREN();
    std::vector<ElementVariableReferenceContext *> elementVariableReference();
    ElementVariableReferenceContext* elementVariableReference(size_t i);
    std::vector<antlr4::tree::TerminalNode *> COMMA();
    antlr4::tree::TerminalNode* COMMA(size_t i);
    antlr4::tree::TerminalNode *RIGHT_PAREN();

   
  };

  All_differentPredicateContext* all_differentPredicate();

  class  SamePredicateContext : public antlr4::ParserRuleContext {
  public:
    SamePredicateContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *SAME();
    antlr4::tree::TerminalNode *LEFT_PAREN();
    std::vector<ElementVariableReferenceContext *> elementVariableReference();
    ElementVariableReferenceContext* elementVariableReference(size_t i);
    std::vector<antlr4::tree::TerminalNode *> COMMA();
    antlr4::tree::TerminalNode* COMMA(size_t i);
    antlr4::tree::TerminalNode *RIGHT_PAREN();

   
  };

  SamePredicateContext* samePredicate();

  class  Property_existsPredicateContext : public antlr4::ParserRuleContext {
  public:
    Property_existsPredicateContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *PROPERTY_EXISTS();
    antlr4::tree::TerminalNode *LEFT_PAREN();
    ElementVariableReferenceContext *elementVariableReference();
    antlr4::tree::TerminalNode *COMMA();
    PropertyNameContext *propertyName();
    antlr4::tree::TerminalNode *RIGHT_PAREN();

   
  };

  Property_existsPredicateContext* property_existsPredicate();

  class  ValueExpressionContext : public antlr4::ParserRuleContext {
  public:
    ValueExpressionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
   
    ValueExpressionContext() = default;
    void copyFrom(ValueExpressionContext *context);
    using antlr4::ParserRuleContext::copyFrom;

    virtual size_t getRuleIndex() const override;

   
  };

  class  ConjunctiveExprAltContext : public ValueExpressionContext {
  public:
    ConjunctiveExprAltContext(ValueExpressionContext *ctx);

    std::vector<ValueExpressionContext *> valueExpression();
    ValueExpressionContext* valueExpression(size_t i);
    antlr4::tree::TerminalNode *AND();
  };

  class  PropertyGraphExprAltContext : public ValueExpressionContext {
  public:
    PropertyGraphExprAltContext(ValueExpressionContext *ctx);

    antlr4::tree::TerminalNode *GRAPH();
    GraphExpressionContext *graphExpression();
    antlr4::tree::TerminalNode *PROPERTY();
  };

  class  MultDivExprAltContext : public ValueExpressionContext {
  public:
    MultDivExprAltContext(ValueExpressionContext *ctx);

    antlr4::Token *operator_ = nullptr;
    std::vector<ValueExpressionContext *> valueExpression();
    ValueExpressionContext* valueExpression(size_t i);
    antlr4::tree::TerminalNode *ASTERISK();
    antlr4::tree::TerminalNode *SOLIDUS();
  };

  class  BindingTableExprAltContext : public ValueExpressionContext {
  public:
    BindingTableExprAltContext(ValueExpressionContext *ctx);

    antlr4::tree::TerminalNode *TABLE();
    BindingTableExpressionContext *bindingTableExpression();
    antlr4::tree::TerminalNode *BINDING();
  };

  class  SignedExprAltContext : public ValueExpressionContext {
  public:
    SignedExprAltContext(ValueExpressionContext *ctx);

    antlr4::Token *sign = nullptr;
    ValueExpressionContext *valueExpression();
    antlr4::tree::TerminalNode *PLUS_SIGN();
    antlr4::tree::TerminalNode *MINUS_SIGN();
  };

  class  IsNotExprAltContext : public ValueExpressionContext {
  public:
    IsNotExprAltContext(ValueExpressionContext *ctx);

    ValueExpressionContext *valueExpression();
    antlr4::tree::TerminalNode *IS();
    TruthValueContext *truthValue();
    antlr4::tree::TerminalNode *NOT();
  };

  class  NormalizedPredicateExprAltContext : public ValueExpressionContext {
  public:
    NormalizedPredicateExprAltContext(ValueExpressionContext *ctx);

    ValueExpressionContext *valueExpression();
    NormalizedPredicatePart2Context *normalizedPredicatePart2();
  };

  class  NotExprAltContext : public ValueExpressionContext {
  public:
    NotExprAltContext(ValueExpressionContext *ctx);

    antlr4::tree::TerminalNode *NOT();
    ValueExpressionContext *valueExpression();
  };

  class  ValueFunctionExprAltContext : public ValueExpressionContext {
  public:
    ValueFunctionExprAltContext(ValueExpressionContext *ctx);

    ValueFunctionContext *valueFunction();
  };

  class  ConcatenationExprAltContext : public ValueExpressionContext {
  public:
    ConcatenationExprAltContext(ValueExpressionContext *ctx);

    std::vector<ValueExpressionContext *> valueExpression();
    ValueExpressionContext* valueExpression(size_t i);
    antlr4::tree::TerminalNode *CONCATENATION_OPERATOR();
  };

  class  DisjunctiveExprAltContext : public ValueExpressionContext {
  public:
    DisjunctiveExprAltContext(ValueExpressionContext *ctx);

    antlr4::Token *operator_ = nullptr;
    std::vector<ValueExpressionContext *> valueExpression();
    ValueExpressionContext* valueExpression(size_t i);
    antlr4::tree::TerminalNode *OR();
    antlr4::tree::TerminalNode *XOR();
  };

  class  ComparisonExprAltContext : public ValueExpressionContext {
  public:
    ComparisonExprAltContext(ValueExpressionContext *ctx);

    std::vector<ValueExpressionContext *> valueExpression();
    ValueExpressionContext* valueExpression(size_t i);
    CompOpContext *compOp();
  };

  class  PrimaryExprAltContext : public ValueExpressionContext {
  public:
    PrimaryExprAltContext(ValueExpressionContext *ctx);

    ValueExpressionPrimaryContext *valueExpressionPrimary();
  };

  class  AddSubtractExprAltContext : public ValueExpressionContext {
  public:
    AddSubtractExprAltContext(ValueExpressionContext *ctx);

    antlr4::Token *operator_ = nullptr;
    std::vector<ValueExpressionContext *> valueExpression();
    ValueExpressionContext* valueExpression(size_t i);
    antlr4::tree::TerminalNode *PLUS_SIGN();
    antlr4::tree::TerminalNode *MINUS_SIGN();
  };

  class  PredicateExprAltContext : public ValueExpressionContext {
  public:
    PredicateExprAltContext(ValueExpressionContext *ctx);

    PredicateContext *predicate();
  };

  ValueExpressionContext* valueExpression();
  ValueExpressionContext* valueExpression(int precedence);
  class  ValueFunctionContext : public antlr4::ParserRuleContext {
  public:
    ValueFunctionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    NumericValueFunctionContext *numericValueFunction();
    DatetimeSubtractionContext *datetimeSubtraction();
    DatetimeValueFunctionContext *datetimeValueFunction();
    DurationValueFunctionContext *durationValueFunction();
    CharacterOrByteStringFunctionContext *characterOrByteStringFunction();
    ListValueFunctionContext *listValueFunction();

   
  };

  ValueFunctionContext* valueFunction();

  class  BooleanValueExpressionContext : public antlr4::ParserRuleContext {
  public:
    BooleanValueExpressionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    ValueExpressionContext *valueExpression();

   
  };

  BooleanValueExpressionContext* booleanValueExpression();

  class  CharacterOrByteStringFunctionContext : public antlr4::ParserRuleContext {
  public:
    CharacterOrByteStringFunctionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    SubCharacterOrByteStringContext *subCharacterOrByteString();
    TrimSingleCharacterOrByteStringContext *trimSingleCharacterOrByteString();
    FoldCharacterStringContext *foldCharacterString();
    TrimMultiCharacterCharacterStringContext *trimMultiCharacterCharacterString();
    NormalizeCharacterStringContext *normalizeCharacterString();

   
  };

  CharacterOrByteStringFunctionContext* characterOrByteStringFunction();

  class  SubCharacterOrByteStringContext : public antlr4::ParserRuleContext {
  public:
    SubCharacterOrByteStringContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LEFT_PAREN();
    ValueExpressionContext *valueExpression();
    antlr4::tree::TerminalNode *COMMA();
    StringLengthContext *stringLength();
    antlr4::tree::TerminalNode *RIGHT_PAREN();
    antlr4::tree::TerminalNode *LEFT();
    antlr4::tree::TerminalNode *RIGHT();

   
  };

  SubCharacterOrByteStringContext* subCharacterOrByteString();

  class  TrimSingleCharacterOrByteStringContext : public antlr4::ParserRuleContext {
  public:
    TrimSingleCharacterOrByteStringContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *TRIM();
    antlr4::tree::TerminalNode *LEFT_PAREN();
    TrimOperandsContext *trimOperands();
    antlr4::tree::TerminalNode *RIGHT_PAREN();

   
  };

  TrimSingleCharacterOrByteStringContext* trimSingleCharacterOrByteString();

  class  FoldCharacterStringContext : public antlr4::ParserRuleContext {
  public:
    FoldCharacterStringContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LEFT_PAREN();
    ValueExpressionContext *valueExpression();
    antlr4::tree::TerminalNode *RIGHT_PAREN();
    antlr4::tree::TerminalNode *UPPER();
    antlr4::tree::TerminalNode *LOWER();

   
  };

  FoldCharacterStringContext* foldCharacterString();

  class  TrimMultiCharacterCharacterStringContext : public antlr4::ParserRuleContext {
  public:
    TrimMultiCharacterCharacterStringContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LEFT_PAREN();
    std::vector<ValueExpressionContext *> valueExpression();
    ValueExpressionContext* valueExpression(size_t i);
    antlr4::tree::TerminalNode *RIGHT_PAREN();
    antlr4::tree::TerminalNode *BTRIM();
    antlr4::tree::TerminalNode *LTRIM();
    antlr4::tree::TerminalNode *RTRIM();
    antlr4::tree::TerminalNode *COMMA();

   
  };

  TrimMultiCharacterCharacterStringContext* trimMultiCharacterCharacterString();

  class  NormalizeCharacterStringContext : public antlr4::ParserRuleContext {
  public:
    NormalizeCharacterStringContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *NORMALIZE();
    antlr4::tree::TerminalNode *LEFT_PAREN();
    ValueExpressionContext *valueExpression();
    antlr4::tree::TerminalNode *RIGHT_PAREN();
    antlr4::tree::TerminalNode *COMMA();
    NormalFormContext *normalForm();

   
  };

  NormalizeCharacterStringContext* normalizeCharacterString();

  class  NodeReferenceValueExpressionContext : public antlr4::ParserRuleContext {
  public:
    NodeReferenceValueExpressionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    ValueExpressionPrimaryContext *valueExpressionPrimary();

   
  };

  NodeReferenceValueExpressionContext* nodeReferenceValueExpression();

  class  EdgeReferenceValueExpressionContext : public antlr4::ParserRuleContext {
  public:
    EdgeReferenceValueExpressionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    ValueExpressionPrimaryContext *valueExpressionPrimary();

   
  };

  EdgeReferenceValueExpressionContext* edgeReferenceValueExpression();

  class  AggregatingValueExpressionContext : public antlr4::ParserRuleContext {
  public:
    AggregatingValueExpressionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    ValueExpressionContext *valueExpression();

   
  };

  AggregatingValueExpressionContext* aggregatingValueExpression();

  class  ValueExpressionPrimaryContext : public antlr4::ParserRuleContext {
  public:
    ValueExpressionPrimaryContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    ParenthesizedValueExpressionContext *parenthesizedValueExpression();
    AggregateFunctionContext *aggregateFunction();
    UnsignedValueSpecificationContext *unsignedValueSpecification();
    PathValueConstructorContext *pathValueConstructor();
    ValueQueryExpressionContext *valueQueryExpression();
    CaseExpressionContext *caseExpression();
    CastSpecificationContext *castSpecification();
    Element_idFunctionContext *element_idFunction();
    LetValueExpressionContext *letValueExpression();
    BindingVariableReferenceContext *bindingVariableReference();
    ValueExpressionPrimaryContext *valueExpressionPrimary();
    antlr4::tree::TerminalNode *PERIOD();
    PropertyNameContext *propertyName();

   
  };

  ValueExpressionPrimaryContext* valueExpressionPrimary();
  ValueExpressionPrimaryContext* valueExpressionPrimary(int precedence);
  class  ParenthesizedValueExpressionContext : public antlr4::ParserRuleContext {
  public:
    ParenthesizedValueExpressionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LEFT_PAREN();
    ValueExpressionContext *valueExpression();
    antlr4::tree::TerminalNode *RIGHT_PAREN();

   
  };

  ParenthesizedValueExpressionContext* parenthesizedValueExpression();

  class  NonParenthesizedValueExpressionPrimaryContext : public antlr4::ParserRuleContext {
  public:
    NonParenthesizedValueExpressionPrimaryContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    NonParenthesizedValueExpressionPrimarySpecialCaseContext *nonParenthesizedValueExpressionPrimarySpecialCase();
    BindingVariableReferenceContext *bindingVariableReference();

   
  };

  NonParenthesizedValueExpressionPrimaryContext* nonParenthesizedValueExpressionPrimary();

  class  NonParenthesizedValueExpressionPrimarySpecialCaseContext : public antlr4::ParserRuleContext {
  public:
    NonParenthesizedValueExpressionPrimarySpecialCaseContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    AggregateFunctionContext *aggregateFunction();
    UnsignedValueSpecificationContext *unsignedValueSpecification();
    PathValueConstructorContext *pathValueConstructor();
    ValueExpressionPrimaryContext *valueExpressionPrimary();
    antlr4::tree::TerminalNode *PERIOD();
    PropertyNameContext *propertyName();
    ValueQueryExpressionContext *valueQueryExpression();
    CaseExpressionContext *caseExpression();
    CastSpecificationContext *castSpecification();
    Element_idFunctionContext *element_idFunction();
    LetValueExpressionContext *letValueExpression();

   
  };

  NonParenthesizedValueExpressionPrimarySpecialCaseContext* nonParenthesizedValueExpressionPrimarySpecialCase();

  class  UnsignedValueSpecificationContext : public antlr4::ParserRuleContext {
  public:
    UnsignedValueSpecificationContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    UnsignedLiteralContext *unsignedLiteral();
    GeneralValueSpecificationContext *generalValueSpecification();

   
  };

  UnsignedValueSpecificationContext* unsignedValueSpecification();

  class  NonNegativeIntegerSpecificationContext : public antlr4::ParserRuleContext {
  public:
    NonNegativeIntegerSpecificationContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    UnsignedIntegerContext *unsignedInteger();
    DynamicParameterSpecificationContext *dynamicParameterSpecification();

   
  };

  NonNegativeIntegerSpecificationContext* nonNegativeIntegerSpecification();

  class  GeneralValueSpecificationContext : public antlr4::ParserRuleContext {
  public:
    GeneralValueSpecificationContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    DynamicParameterSpecificationContext *dynamicParameterSpecification();
    antlr4::tree::TerminalNode *SESSION_USER();

   
  };

  GeneralValueSpecificationContext* generalValueSpecification();

  class  DynamicParameterSpecificationContext : public antlr4::ParserRuleContext {
  public:
    DynamicParameterSpecificationContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *GENERAL_PARAMETER_REFERENCE();

   
  };

  DynamicParameterSpecificationContext* dynamicParameterSpecification();

  class  LetValueExpressionContext : public antlr4::ParserRuleContext {
  public:
    LetValueExpressionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LET();
    LetVariableDefinitionListContext *letVariableDefinitionList();
    antlr4::tree::TerminalNode *IN();
    ValueExpressionContext *valueExpression();
    antlr4::tree::TerminalNode *END();

   
  };

  LetValueExpressionContext* letValueExpression();

  class  ValueQueryExpressionContext : public antlr4::ParserRuleContext {
  public:
    ValueQueryExpressionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *VALUE();
    NestedQuerySpecificationContext *nestedQuerySpecification();

   
  };

  ValueQueryExpressionContext* valueQueryExpression();

  class  CaseExpressionContext : public antlr4::ParserRuleContext {
  public:
    CaseExpressionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    CaseAbbreviationContext *caseAbbreviation();
    CaseSpecificationContext *caseSpecification();

   
  };

  CaseExpressionContext* caseExpression();

  class  CaseAbbreviationContext : public antlr4::ParserRuleContext {
  public:
    CaseAbbreviationContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *NULLIF();
    antlr4::tree::TerminalNode *LEFT_PAREN();
    std::vector<ValueExpressionContext *> valueExpression();
    ValueExpressionContext* valueExpression(size_t i);
    std::vector<antlr4::tree::TerminalNode *> COMMA();
    antlr4::tree::TerminalNode* COMMA(size_t i);
    antlr4::tree::TerminalNode *RIGHT_PAREN();
    antlr4::tree::TerminalNode *COALESCE();

   
  };

  CaseAbbreviationContext* caseAbbreviation();

  class  CaseSpecificationContext : public antlr4::ParserRuleContext {
  public:
    CaseSpecificationContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    SimpleCaseContext *simpleCase();
    SearchedCaseContext *searchedCase();

   
  };

  CaseSpecificationContext* caseSpecification();

  class  SimpleCaseContext : public antlr4::ParserRuleContext {
  public:
    SimpleCaseContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *CASE();
    CaseOperandContext *caseOperand();
    antlr4::tree::TerminalNode *END();
    std::vector<SimpleWhenClauseContext *> simpleWhenClause();
    SimpleWhenClauseContext* simpleWhenClause(size_t i);
    ElseClauseContext *elseClause();

   
  };

  SimpleCaseContext* simpleCase();

  class  SearchedCaseContext : public antlr4::ParserRuleContext {
  public:
    SearchedCaseContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *CASE();
    antlr4::tree::TerminalNode *END();
    std::vector<SearchedWhenClauseContext *> searchedWhenClause();
    SearchedWhenClauseContext* searchedWhenClause(size_t i);
    ElseClauseContext *elseClause();

   
  };

  SearchedCaseContext* searchedCase();

  class  SimpleWhenClauseContext : public antlr4::ParserRuleContext {
  public:
    SimpleWhenClauseContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *WHEN();
    WhenOperandListContext *whenOperandList();
    antlr4::tree::TerminalNode *THEN();
    ResultContext *result();

   
  };

  SimpleWhenClauseContext* simpleWhenClause();

  class  SearchedWhenClauseContext : public antlr4::ParserRuleContext {
  public:
    SearchedWhenClauseContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *WHEN();
    SearchConditionContext *searchCondition();
    antlr4::tree::TerminalNode *THEN();
    ResultContext *result();

   
  };

  SearchedWhenClauseContext* searchedWhenClause();

  class  ElseClauseContext : public antlr4::ParserRuleContext {
  public:
    ElseClauseContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *ELSE();
    ResultContext *result();

   
  };

  ElseClauseContext* elseClause();

  class  CaseOperandContext : public antlr4::ParserRuleContext {
  public:
    CaseOperandContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    NonParenthesizedValueExpressionPrimaryContext *nonParenthesizedValueExpressionPrimary();
    ElementVariableReferenceContext *elementVariableReference();

   
  };

  CaseOperandContext* caseOperand();

  class  WhenOperandListContext : public antlr4::ParserRuleContext {
  public:
    WhenOperandListContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    std::vector<WhenOperandContext *> whenOperand();
    WhenOperandContext* whenOperand(size_t i);
    std::vector<antlr4::tree::TerminalNode *> COMMA();
    antlr4::tree::TerminalNode* COMMA(size_t i);

   
  };

  WhenOperandListContext* whenOperandList();

  class  WhenOperandContext : public antlr4::ParserRuleContext {
  public:
    WhenOperandContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    NonParenthesizedValueExpressionPrimaryContext *nonParenthesizedValueExpressionPrimary();
    CompOpContext *compOp();
    ValueExpressionContext *valueExpression();
    NullPredicatePart2Context *nullPredicatePart2();
    ValueTypePredicatePart2Context *valueTypePredicatePart2();
    NormalizedPredicatePart2Context *normalizedPredicatePart2();
    DirectedPredicatePart2Context *directedPredicatePart2();
    LabeledPredicatePart2Context *labeledPredicatePart2();
    SourcePredicatePart2Context *sourcePredicatePart2();
    DestinationPredicatePart2Context *destinationPredicatePart2();

   
  };

  WhenOperandContext* whenOperand();

  class  ResultContext : public antlr4::ParserRuleContext {
  public:
    ResultContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    ResultExpressionContext *resultExpression();
    NullLiteralContext *nullLiteral();

   
  };

  ResultContext* result();

  class  ResultExpressionContext : public antlr4::ParserRuleContext {
  public:
    ResultExpressionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    ValueExpressionContext *valueExpression();

   
  };

  ResultExpressionContext* resultExpression();

  class  CastSpecificationContext : public antlr4::ParserRuleContext {
  public:
    CastSpecificationContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *CAST();
    antlr4::tree::TerminalNode *LEFT_PAREN();
    CastOperandContext *castOperand();
    antlr4::tree::TerminalNode *AS();
    CastTargetContext *castTarget();
    antlr4::tree::TerminalNode *RIGHT_PAREN();

   
  };

  CastSpecificationContext* castSpecification();

  class  CastOperandContext : public antlr4::ParserRuleContext {
  public:
    CastOperandContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    ValueExpressionContext *valueExpression();
    NullLiteralContext *nullLiteral();

   
  };

  CastOperandContext* castOperand();

  class  CastTargetContext : public antlr4::ParserRuleContext {
  public:
    CastTargetContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    ValueTypeContext *valueType();

   
  };

  CastTargetContext* castTarget();

  class  AggregateFunctionContext : public antlr4::ParserRuleContext {
  public:
    AggregateFunctionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *COUNT();
    antlr4::tree::TerminalNode *LEFT_PAREN();
    antlr4::tree::TerminalNode *ASTERISK();
    antlr4::tree::TerminalNode *RIGHT_PAREN();
    GeneralSetFunctionContext *generalSetFunction();
    BinarySetFunctionContext *binarySetFunction();

   
  };

  AggregateFunctionContext* aggregateFunction();

  class  GeneralSetFunctionContext : public antlr4::ParserRuleContext {
  public:
    GeneralSetFunctionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    GeneralSetFunctionTypeContext *generalSetFunctionType();
    antlr4::tree::TerminalNode *LEFT_PAREN();
    ValueExpressionContext *valueExpression();
    antlr4::tree::TerminalNode *RIGHT_PAREN();
    SetQuantifierContext *setQuantifier();

   
  };

  GeneralSetFunctionContext* generalSetFunction();

  class  BinarySetFunctionContext : public antlr4::ParserRuleContext {
  public:
    BinarySetFunctionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    BinarySetFunctionTypeContext *binarySetFunctionType();
    antlr4::tree::TerminalNode *LEFT_PAREN();
    DependentValueExpressionContext *dependentValueExpression();
    antlr4::tree::TerminalNode *COMMA();
    IndependentValueExpressionContext *independentValueExpression();
    antlr4::tree::TerminalNode *RIGHT_PAREN();

   
  };

  BinarySetFunctionContext* binarySetFunction();

  class  GeneralSetFunctionTypeContext : public antlr4::ParserRuleContext {
  public:
    GeneralSetFunctionTypeContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *AVG();
    antlr4::tree::TerminalNode *COUNT();
    antlr4::tree::TerminalNode *MAX();
    antlr4::tree::TerminalNode *MIN();
    antlr4::tree::TerminalNode *SUM();
    antlr4::tree::TerminalNode *COLLECT_LIST();
    antlr4::tree::TerminalNode *STDDEV_SAMP();
    antlr4::tree::TerminalNode *STDDEV_POP();

   
  };

  GeneralSetFunctionTypeContext* generalSetFunctionType();

  class  SetQuantifierContext : public antlr4::ParserRuleContext {
  public:
    SetQuantifierContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *DISTINCT();
    antlr4::tree::TerminalNode *ALL();

   
  };

  SetQuantifierContext* setQuantifier();

  class  BinarySetFunctionTypeContext : public antlr4::ParserRuleContext {
  public:
    BinarySetFunctionTypeContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *PERCENTILE_CONT();
    antlr4::tree::TerminalNode *PERCENTILE_DISC();

   
  };

  BinarySetFunctionTypeContext* binarySetFunctionType();

  class  DependentValueExpressionContext : public antlr4::ParserRuleContext {
  public:
    DependentValueExpressionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    NumericValueExpressionContext *numericValueExpression();
    SetQuantifierContext *setQuantifier();

   
  };

  DependentValueExpressionContext* dependentValueExpression();

  class  IndependentValueExpressionContext : public antlr4::ParserRuleContext {
  public:
    IndependentValueExpressionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    NumericValueExpressionContext *numericValueExpression();

   
  };

  IndependentValueExpressionContext* independentValueExpression();

  class  Element_idFunctionContext : public antlr4::ParserRuleContext {
  public:
    Element_idFunctionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *ELEMENT_ID();
    antlr4::tree::TerminalNode *LEFT_PAREN();
    ElementVariableReferenceContext *elementVariableReference();
    antlr4::tree::TerminalNode *RIGHT_PAREN();

   
  };

  Element_idFunctionContext* element_idFunction();

  class  BindingVariableReferenceContext : public antlr4::ParserRuleContext {
  public:
    BindingVariableReferenceContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    BindingVariableContext *bindingVariable();

   
  };

  BindingVariableReferenceContext* bindingVariableReference();

  class  PathValueExpressionContext : public antlr4::ParserRuleContext {
  public:
    PathValueExpressionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    ValueExpressionContext *valueExpression();

   
  };

  PathValueExpressionContext* pathValueExpression();

  class  PathValueConstructorContext : public antlr4::ParserRuleContext {
  public:
    PathValueConstructorContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    PathValueConstructorByEnumerationContext *pathValueConstructorByEnumeration();

   
  };

  PathValueConstructorContext* pathValueConstructor();

  class  PathValueConstructorByEnumerationContext : public antlr4::ParserRuleContext {
  public:
    PathValueConstructorByEnumerationContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *PATH();
    antlr4::tree::TerminalNode *LEFT_BRACKET();
    PathElementListContext *pathElementList();
    antlr4::tree::TerminalNode *RIGHT_BRACKET();

   
  };

  PathValueConstructorByEnumerationContext* pathValueConstructorByEnumeration();

  class  PathElementListContext : public antlr4::ParserRuleContext {
  public:
    PathElementListContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    PathElementListStartContext *pathElementListStart();
    std::vector<PathElementListStepContext *> pathElementListStep();
    PathElementListStepContext* pathElementListStep(size_t i);

   
  };

  PathElementListContext* pathElementList();

  class  PathElementListStartContext : public antlr4::ParserRuleContext {
  public:
    PathElementListStartContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    NodeReferenceValueExpressionContext *nodeReferenceValueExpression();

   
  };

  PathElementListStartContext* pathElementListStart();

  class  PathElementListStepContext : public antlr4::ParserRuleContext {
  public:
    PathElementListStepContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    std::vector<antlr4::tree::TerminalNode *> COMMA();
    antlr4::tree::TerminalNode* COMMA(size_t i);
    EdgeReferenceValueExpressionContext *edgeReferenceValueExpression();
    NodeReferenceValueExpressionContext *nodeReferenceValueExpression();

   
  };

  PathElementListStepContext* pathElementListStep();

  class  ListValueExpressionContext : public antlr4::ParserRuleContext {
  public:
    ListValueExpressionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    ValueExpressionContext *valueExpression();

   
  };

  ListValueExpressionContext* listValueExpression();

  class  ListValueFunctionContext : public antlr4::ParserRuleContext {
  public:
    ListValueFunctionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    TrimListFunctionContext *trimListFunction();
    ElementsFunctionContext *elementsFunction();

   
  };

  ListValueFunctionContext* listValueFunction();

  class  TrimListFunctionContext : public antlr4::ParserRuleContext {
  public:
    TrimListFunctionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *TRIM();
    antlr4::tree::TerminalNode *LEFT_PAREN();
    ListValueExpressionContext *listValueExpression();
    antlr4::tree::TerminalNode *COMMA();
    NumericValueExpressionContext *numericValueExpression();
    antlr4::tree::TerminalNode *RIGHT_PAREN();

   
  };

  TrimListFunctionContext* trimListFunction();

  class  ElementsFunctionContext : public antlr4::ParserRuleContext {
  public:
    ElementsFunctionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *ELEMENTS();
    antlr4::tree::TerminalNode *LEFT_PAREN();
    PathValueExpressionContext *pathValueExpression();
    antlr4::tree::TerminalNode *RIGHT_PAREN();

   
  };

  ElementsFunctionContext* elementsFunction();

  class  ListValueConstructorContext : public antlr4::ParserRuleContext {
  public:
    ListValueConstructorContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    ListValueConstructorByEnumerationContext *listValueConstructorByEnumeration();

   
  };

  ListValueConstructorContext* listValueConstructor();

  class  ListValueConstructorByEnumerationContext : public antlr4::ParserRuleContext {
  public:
    ListValueConstructorByEnumerationContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LEFT_BRACKET();
    antlr4::tree::TerminalNode *RIGHT_BRACKET();
    ListValueTypeNameContext *listValueTypeName();
    ListElementListContext *listElementList();

   
  };

  ListValueConstructorByEnumerationContext* listValueConstructorByEnumeration();

  class  ListElementListContext : public antlr4::ParserRuleContext {
  public:
    ListElementListContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    std::vector<ListElementContext *> listElement();
    ListElementContext* listElement(size_t i);
    std::vector<antlr4::tree::TerminalNode *> COMMA();
    antlr4::tree::TerminalNode* COMMA(size_t i);

   
  };

  ListElementListContext* listElementList();

  class  ListElementContext : public antlr4::ParserRuleContext {
  public:
    ListElementContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    ValueExpressionContext *valueExpression();

   
  };

  ListElementContext* listElement();

  class  RecordConstructorContext : public antlr4::ParserRuleContext {
  public:
    RecordConstructorContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    FieldsSpecificationContext *fieldsSpecification();
    antlr4::tree::TerminalNode *RECORD();

   
  };

  RecordConstructorContext* recordConstructor();

  class  FieldsSpecificationContext : public antlr4::ParserRuleContext {
  public:
    FieldsSpecificationContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LEFT_BRACE();
    antlr4::tree::TerminalNode *RIGHT_BRACE();
    FieldListContext *fieldList();

   
  };

  FieldsSpecificationContext* fieldsSpecification();

  class  FieldListContext : public antlr4::ParserRuleContext {
  public:
    FieldListContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    std::vector<FieldContext *> field();
    FieldContext* field(size_t i);
    std::vector<antlr4::tree::TerminalNode *> COMMA();
    antlr4::tree::TerminalNode* COMMA(size_t i);

   
  };

  FieldListContext* fieldList();

  class  FieldContext : public antlr4::ParserRuleContext {
  public:
    FieldContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    FieldNameContext *fieldName();
    antlr4::tree::TerminalNode *COLON();
    ValueExpressionContext *valueExpression();

   
  };

  FieldContext* field();

  class  TruthValueContext : public antlr4::ParserRuleContext {
  public:
    TruthValueContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *BOOLEAN_LITERAL();

   
  };

  TruthValueContext* truthValue();

  class  NumericValueExpressionContext : public antlr4::ParserRuleContext {
  public:
    antlr4::Token *sign = nullptr;
    antlr4::Token *operator_ = nullptr;
    NumericValueExpressionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    std::vector<NumericValueExpressionContext *> numericValueExpression();
    NumericValueExpressionContext* numericValueExpression(size_t i);
    antlr4::tree::TerminalNode *PLUS_SIGN();
    antlr4::tree::TerminalNode *MINUS_SIGN();
    ValueExpressionPrimaryContext *valueExpressionPrimary();
    NumericValueFunctionContext *numericValueFunction();
    antlr4::tree::TerminalNode *ASTERISK();
    antlr4::tree::TerminalNode *SOLIDUS();

   
  };

  NumericValueExpressionContext* numericValueExpression();
  NumericValueExpressionContext* numericValueExpression(int precedence);
  class  NumericValueFunctionContext : public antlr4::ParserRuleContext {
  public:
    NumericValueFunctionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    LengthExpressionContext *lengthExpression();
    CardinalityExpressionContext *cardinalityExpression();
    AbsoluteValueExpressionContext *absoluteValueExpression();
    ModulusExpressionContext *modulusExpression();
    TrigonometricFunctionContext *trigonometricFunction();
    GeneralLogarithmFunctionContext *generalLogarithmFunction();
    CommonLogarithmContext *commonLogarithm();
    NaturalLogarithmContext *naturalLogarithm();
    ExponentialFunctionContext *exponentialFunction();
    PowerFunctionContext *powerFunction();
    SquareRootContext *squareRoot();
    FloorFunctionContext *floorFunction();
    CeilingFunctionContext *ceilingFunction();

   
  };

  NumericValueFunctionContext* numericValueFunction();

  class  LengthExpressionContext : public antlr4::ParserRuleContext {
  public:
    LengthExpressionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    CharLengthExpressionContext *charLengthExpression();
    ByteLengthExpressionContext *byteLengthExpression();
    PathLengthExpressionContext *pathLengthExpression();

   
  };

  LengthExpressionContext* lengthExpression();

  class  CardinalityExpressionContext : public antlr4::ParserRuleContext {
  public:
    CardinalityExpressionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *CARDINALITY();
    antlr4::tree::TerminalNode *LEFT_PAREN();
    CardinalityExpressionArgumentContext *cardinalityExpressionArgument();
    antlr4::tree::TerminalNode *RIGHT_PAREN();
    antlr4::tree::TerminalNode *SIZE();
    ListValueExpressionContext *listValueExpression();

   
  };

  CardinalityExpressionContext* cardinalityExpression();

  class  CardinalityExpressionArgumentContext : public antlr4::ParserRuleContext {
  public:
    CardinalityExpressionArgumentContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    ValueExpressionContext *valueExpression();

   
  };

  CardinalityExpressionArgumentContext* cardinalityExpressionArgument();

  class  CharLengthExpressionContext : public antlr4::ParserRuleContext {
  public:
    CharLengthExpressionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LEFT_PAREN();
    CharacterStringValueExpressionContext *characterStringValueExpression();
    antlr4::tree::TerminalNode *RIGHT_PAREN();
    antlr4::tree::TerminalNode *CHAR_LENGTH();
    antlr4::tree::TerminalNode *CHARACTER_LENGTH();

   
  };

  CharLengthExpressionContext* charLengthExpression();

  class  ByteLengthExpressionContext : public antlr4::ParserRuleContext {
  public:
    ByteLengthExpressionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LEFT_PAREN();
    ByteStringValueExpressionContext *byteStringValueExpression();
    antlr4::tree::TerminalNode *RIGHT_PAREN();
    antlr4::tree::TerminalNode *BYTE_LENGTH();
    antlr4::tree::TerminalNode *OCTET_LENGTH();

   
  };

  ByteLengthExpressionContext* byteLengthExpression();

  class  PathLengthExpressionContext : public antlr4::ParserRuleContext {
  public:
    PathLengthExpressionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *PATH_LENGTH();
    antlr4::tree::TerminalNode *LEFT_PAREN();
    PathValueExpressionContext *pathValueExpression();
    antlr4::tree::TerminalNode *RIGHT_PAREN();

   
  };

  PathLengthExpressionContext* pathLengthExpression();

  class  AbsoluteValueExpressionContext : public antlr4::ParserRuleContext {
  public:
    AbsoluteValueExpressionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *ABS();
    antlr4::tree::TerminalNode *LEFT_PAREN();
    ValueExpressionContext *valueExpression();
    antlr4::tree::TerminalNode *RIGHT_PAREN();

   
  };

  AbsoluteValueExpressionContext* absoluteValueExpression();

  class  ModulusExpressionContext : public antlr4::ParserRuleContext {
  public:
    ModulusExpressionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *MOD();
    antlr4::tree::TerminalNode *LEFT_PAREN();
    NumericValueExpressionDividendContext *numericValueExpressionDividend();
    antlr4::tree::TerminalNode *COMMA();
    NumericValueExpressionDivisorContext *numericValueExpressionDivisor();
    antlr4::tree::TerminalNode *RIGHT_PAREN();

   
  };

  ModulusExpressionContext* modulusExpression();

  class  NumericValueExpressionDividendContext : public antlr4::ParserRuleContext {
  public:
    NumericValueExpressionDividendContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    NumericValueExpressionContext *numericValueExpression();

   
  };

  NumericValueExpressionDividendContext* numericValueExpressionDividend();

  class  NumericValueExpressionDivisorContext : public antlr4::ParserRuleContext {
  public:
    NumericValueExpressionDivisorContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    NumericValueExpressionContext *numericValueExpression();

   
  };

  NumericValueExpressionDivisorContext* numericValueExpressionDivisor();

  class  TrigonometricFunctionContext : public antlr4::ParserRuleContext {
  public:
    TrigonometricFunctionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    TrigonometricFunctionNameContext *trigonometricFunctionName();
    antlr4::tree::TerminalNode *LEFT_PAREN();
    NumericValueExpressionContext *numericValueExpression();
    antlr4::tree::TerminalNode *RIGHT_PAREN();

   
  };

  TrigonometricFunctionContext* trigonometricFunction();

  class  TrigonometricFunctionNameContext : public antlr4::ParserRuleContext {
  public:
    TrigonometricFunctionNameContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *SIN();
    antlr4::tree::TerminalNode *COS();
    antlr4::tree::TerminalNode *TAN();
    antlr4::tree::TerminalNode *COT();
    antlr4::tree::TerminalNode *SINH();
    antlr4::tree::TerminalNode *COSH();
    antlr4::tree::TerminalNode *TANH();
    antlr4::tree::TerminalNode *ASIN();
    antlr4::tree::TerminalNode *ACOS();
    antlr4::tree::TerminalNode *ATAN();
    antlr4::tree::TerminalNode *DEGREES();
    antlr4::tree::TerminalNode *RADIANS();

   
  };

  TrigonometricFunctionNameContext* trigonometricFunctionName();

  class  GeneralLogarithmFunctionContext : public antlr4::ParserRuleContext {
  public:
    GeneralLogarithmFunctionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LOG_KW();
    antlr4::tree::TerminalNode *LEFT_PAREN();
    GeneralLogarithmBaseContext *generalLogarithmBase();
    antlr4::tree::TerminalNode *COMMA();
    GeneralLogarithmArgumentContext *generalLogarithmArgument();
    antlr4::tree::TerminalNode *RIGHT_PAREN();

   
  };

  GeneralLogarithmFunctionContext* generalLogarithmFunction();

  class  GeneralLogarithmBaseContext : public antlr4::ParserRuleContext {
  public:
    GeneralLogarithmBaseContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    NumericValueExpressionContext *numericValueExpression();

   
  };

  GeneralLogarithmBaseContext* generalLogarithmBase();

  class  GeneralLogarithmArgumentContext : public antlr4::ParserRuleContext {
  public:
    GeneralLogarithmArgumentContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    NumericValueExpressionContext *numericValueExpression();

   
  };

  GeneralLogarithmArgumentContext* generalLogarithmArgument();

  class  CommonLogarithmContext : public antlr4::ParserRuleContext {
  public:
    CommonLogarithmContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LOG10();
    antlr4::tree::TerminalNode *LEFT_PAREN();
    NumericValueExpressionContext *numericValueExpression();
    antlr4::tree::TerminalNode *RIGHT_PAREN();

   
  };

  CommonLogarithmContext* commonLogarithm();

  class  NaturalLogarithmContext : public antlr4::ParserRuleContext {
  public:
    NaturalLogarithmContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LN();
    antlr4::tree::TerminalNode *LEFT_PAREN();
    NumericValueExpressionContext *numericValueExpression();
    antlr4::tree::TerminalNode *RIGHT_PAREN();

   
  };

  NaturalLogarithmContext* naturalLogarithm();

  class  ExponentialFunctionContext : public antlr4::ParserRuleContext {
  public:
    ExponentialFunctionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *EXP();
    antlr4::tree::TerminalNode *LEFT_PAREN();
    NumericValueExpressionContext *numericValueExpression();
    antlr4::tree::TerminalNode *RIGHT_PAREN();

   
  };

  ExponentialFunctionContext* exponentialFunction();

  class  PowerFunctionContext : public antlr4::ParserRuleContext {
  public:
    PowerFunctionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *POWER();
    antlr4::tree::TerminalNode *LEFT_PAREN();
    NumericValueExpressionBaseContext *numericValueExpressionBase();
    antlr4::tree::TerminalNode *COMMA();
    NumericValueExpressionExponentContext *numericValueExpressionExponent();
    antlr4::tree::TerminalNode *RIGHT_PAREN();

   
  };

  PowerFunctionContext* powerFunction();

  class  NumericValueExpressionBaseContext : public antlr4::ParserRuleContext {
  public:
    NumericValueExpressionBaseContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    NumericValueExpressionContext *numericValueExpression();

   
  };

  NumericValueExpressionBaseContext* numericValueExpressionBase();

  class  NumericValueExpressionExponentContext : public antlr4::ParserRuleContext {
  public:
    NumericValueExpressionExponentContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    NumericValueExpressionContext *numericValueExpression();

   
  };

  NumericValueExpressionExponentContext* numericValueExpressionExponent();

  class  SquareRootContext : public antlr4::ParserRuleContext {
  public:
    SquareRootContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *SQRT();
    antlr4::tree::TerminalNode *LEFT_PAREN();
    NumericValueExpressionContext *numericValueExpression();
    antlr4::tree::TerminalNode *RIGHT_PAREN();

   
  };

  SquareRootContext* squareRoot();

  class  FloorFunctionContext : public antlr4::ParserRuleContext {
  public:
    FloorFunctionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *FLOOR();
    antlr4::tree::TerminalNode *LEFT_PAREN();
    NumericValueExpressionContext *numericValueExpression();
    antlr4::tree::TerminalNode *RIGHT_PAREN();

   
  };

  FloorFunctionContext* floorFunction();

  class  CeilingFunctionContext : public antlr4::ParserRuleContext {
  public:
    CeilingFunctionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LEFT_PAREN();
    NumericValueExpressionContext *numericValueExpression();
    antlr4::tree::TerminalNode *RIGHT_PAREN();
    antlr4::tree::TerminalNode *CEIL();
    antlr4::tree::TerminalNode *CEILING();

   
  };

  CeilingFunctionContext* ceilingFunction();

  class  CharacterStringValueExpressionContext : public antlr4::ParserRuleContext {
  public:
    CharacterStringValueExpressionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    ValueExpressionContext *valueExpression();

   
  };

  CharacterStringValueExpressionContext* characterStringValueExpression();

  class  ByteStringValueExpressionContext : public antlr4::ParserRuleContext {
  public:
    ByteStringValueExpressionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    ValueExpressionContext *valueExpression();

   
  };

  ByteStringValueExpressionContext* byteStringValueExpression();

  class  TrimOperandsContext : public antlr4::ParserRuleContext {
  public:
    TrimOperandsContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    TrimCharacterOrByteStringSourceContext *trimCharacterOrByteStringSource();
    antlr4::tree::TerminalNode *FROM();
    TrimSpecificationContext *trimSpecification();
    TrimCharacterOrByteStringContext *trimCharacterOrByteString();

   
  };

  TrimOperandsContext* trimOperands();

  class  TrimCharacterOrByteStringSourceContext : public antlr4::ParserRuleContext {
  public:
    TrimCharacterOrByteStringSourceContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    ValueExpressionContext *valueExpression();

   
  };

  TrimCharacterOrByteStringSourceContext* trimCharacterOrByteStringSource();

  class  TrimSpecificationContext : public antlr4::ParserRuleContext {
  public:
    TrimSpecificationContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LEADING();
    antlr4::tree::TerminalNode *TRAILING();
    antlr4::tree::TerminalNode *BOTH();

   
  };

  TrimSpecificationContext* trimSpecification();

  class  TrimCharacterOrByteStringContext : public antlr4::ParserRuleContext {
  public:
    TrimCharacterOrByteStringContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    ValueExpressionContext *valueExpression();

   
  };

  TrimCharacterOrByteStringContext* trimCharacterOrByteString();

  class  NormalFormContext : public antlr4::ParserRuleContext {
  public:
    NormalFormContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *NFC();
    antlr4::tree::TerminalNode *NFD();
    antlr4::tree::TerminalNode *NFKC();
    antlr4::tree::TerminalNode *NFKD();

   
  };

  NormalFormContext* normalForm();

  class  StringLengthContext : public antlr4::ParserRuleContext {
  public:
    StringLengthContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    NumericValueExpressionContext *numericValueExpression();

   
  };

  StringLengthContext* stringLength();

  class  DatetimeValueExpressionContext : public antlr4::ParserRuleContext {
  public:
    DatetimeValueExpressionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    ValueExpressionContext *valueExpression();

   
  };

  DatetimeValueExpressionContext* datetimeValueExpression();

  class  DatetimeValueFunctionContext : public antlr4::ParserRuleContext {
  public:
    DatetimeValueFunctionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    DateFunctionContext *dateFunction();
    TimeFunctionContext *timeFunction();
    DatetimeFunctionContext *datetimeFunction();
    LocaltimeFunctionContext *localtimeFunction();
    LocaldatetimeFunctionContext *localdatetimeFunction();

   
  };

  DatetimeValueFunctionContext* datetimeValueFunction();

  class  DateFunctionContext : public antlr4::ParserRuleContext {
  public:
    DateFunctionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *CURRENT_DATE();
    antlr4::tree::TerminalNode *DATE();
    antlr4::tree::TerminalNode *LEFT_PAREN();
    antlr4::tree::TerminalNode *RIGHT_PAREN();
    DateFunctionParametersContext *dateFunctionParameters();

   
  };

  DateFunctionContext* dateFunction();

  class  TimeFunctionContext : public antlr4::ParserRuleContext {
  public:
    TimeFunctionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *CURRENT_TIME();
    antlr4::tree::TerminalNode *ZONED_TIME();
    antlr4::tree::TerminalNode *LEFT_PAREN();
    antlr4::tree::TerminalNode *RIGHT_PAREN();
    TimeFunctionParametersContext *timeFunctionParameters();

   
  };

  TimeFunctionContext* timeFunction();

  class  LocaltimeFunctionContext : public antlr4::ParserRuleContext {
  public:
    LocaltimeFunctionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LOCAL_TIME();
    antlr4::tree::TerminalNode *LEFT_PAREN();
    antlr4::tree::TerminalNode *RIGHT_PAREN();
    TimeFunctionParametersContext *timeFunctionParameters();

   
  };

  LocaltimeFunctionContext* localtimeFunction();

  class  DatetimeFunctionContext : public antlr4::ParserRuleContext {
  public:
    DatetimeFunctionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *CURRENT_TIMESTAMP();
    antlr4::tree::TerminalNode *ZONED_DATETIME();
    antlr4::tree::TerminalNode *LEFT_PAREN();
    antlr4::tree::TerminalNode *RIGHT_PAREN();
    DatetimeFunctionParametersContext *datetimeFunctionParameters();

   
  };

  DatetimeFunctionContext* datetimeFunction();

  class  LocaldatetimeFunctionContext : public antlr4::ParserRuleContext {
  public:
    LocaldatetimeFunctionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LOCAL_TIMESTAMP();
    antlr4::tree::TerminalNode *LOCAL_DATETIME();
    antlr4::tree::TerminalNode *LEFT_PAREN();
    antlr4::tree::TerminalNode *RIGHT_PAREN();
    DatetimeFunctionParametersContext *datetimeFunctionParameters();

   
  };

  LocaldatetimeFunctionContext* localdatetimeFunction();

  class  DateFunctionParametersContext : public antlr4::ParserRuleContext {
  public:
    DateFunctionParametersContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    DateStringContext *dateString();
    RecordConstructorContext *recordConstructor();

   
  };

  DateFunctionParametersContext* dateFunctionParameters();

  class  TimeFunctionParametersContext : public antlr4::ParserRuleContext {
  public:
    TimeFunctionParametersContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    TimeStringContext *timeString();
    RecordConstructorContext *recordConstructor();

   
  };

  TimeFunctionParametersContext* timeFunctionParameters();

  class  DatetimeFunctionParametersContext : public antlr4::ParserRuleContext {
  public:
    DatetimeFunctionParametersContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    DatetimeStringContext *datetimeString();
    RecordConstructorContext *recordConstructor();

   
  };

  DatetimeFunctionParametersContext* datetimeFunctionParameters();

  class  DurationValueExpressionContext : public antlr4::ParserRuleContext {
  public:
    DurationValueExpressionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    ValueExpressionContext *valueExpression();

   
  };

  DurationValueExpressionContext* durationValueExpression();

  class  DatetimeSubtractionContext : public antlr4::ParserRuleContext {
  public:
    DatetimeSubtractionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *DURATION_BETWEEN();
    antlr4::tree::TerminalNode *LEFT_PAREN();
    DatetimeSubtractionParametersContext *datetimeSubtractionParameters();
    antlr4::tree::TerminalNode *RIGHT_PAREN();
    TemporalDurationQualifierContext *temporalDurationQualifier();

   
  };

  DatetimeSubtractionContext* datetimeSubtraction();

  class  DatetimeSubtractionParametersContext : public antlr4::ParserRuleContext {
  public:
    DatetimeSubtractionParametersContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    DatetimeValueExpression1Context *datetimeValueExpression1();
    antlr4::tree::TerminalNode *COMMA();
    DatetimeValueExpression2Context *datetimeValueExpression2();

   
  };

  DatetimeSubtractionParametersContext* datetimeSubtractionParameters();

  class  DatetimeValueExpression1Context : public antlr4::ParserRuleContext {
  public:
    DatetimeValueExpression1Context(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    DatetimeValueExpressionContext *datetimeValueExpression();

   
  };

  DatetimeValueExpression1Context* datetimeValueExpression1();

  class  DatetimeValueExpression2Context : public antlr4::ParserRuleContext {
  public:
    DatetimeValueExpression2Context(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    DatetimeValueExpressionContext *datetimeValueExpression();

   
  };

  DatetimeValueExpression2Context* datetimeValueExpression2();

  class  DurationValueFunctionContext : public antlr4::ParserRuleContext {
  public:
    DurationValueFunctionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    DurationFunctionContext *durationFunction();
    AbsoluteValueExpressionContext *absoluteValueExpression();

   
  };

  DurationValueFunctionContext* durationValueFunction();

  class  DurationFunctionContext : public antlr4::ParserRuleContext {
  public:
    DurationFunctionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *DURATION();
    antlr4::tree::TerminalNode *LEFT_PAREN();
    DurationFunctionParametersContext *durationFunctionParameters();
    antlr4::tree::TerminalNode *RIGHT_PAREN();

   
  };

  DurationFunctionContext* durationFunction();

  class  DurationFunctionParametersContext : public antlr4::ParserRuleContext {
  public:
    DurationFunctionParametersContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    DurationStringContext *durationString();
    RecordConstructorContext *recordConstructor();

   
  };

  DurationFunctionParametersContext* durationFunctionParameters();

  class  ObjectNameContext : public antlr4::ParserRuleContext {
  public:
    ObjectNameContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    IdentifierContext *identifier();

   
  };

  ObjectNameContext* objectName();

  class  ObjectNameOrBindingVariableContext : public antlr4::ParserRuleContext {
  public:
    ObjectNameOrBindingVariableContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    RegularIdentifierContext *regularIdentifier();

   
  };

  ObjectNameOrBindingVariableContext* objectNameOrBindingVariable();

  class  DirectoryNameContext : public antlr4::ParserRuleContext {
  public:
    DirectoryNameContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    IdentifierContext *identifier();

   
  };

  DirectoryNameContext* directoryName();

  class  SchemaNameContext : public antlr4::ParserRuleContext {
  public:
    SchemaNameContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    IdentifierContext *identifier();

   
  };

  SchemaNameContext* schemaName();

  class  GraphNameContext : public antlr4::ParserRuleContext {
  public:
    GraphNameContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    RegularIdentifierContext *regularIdentifier();
    DelimitedGraphNameContext *delimitedGraphName();

   
  };

  GraphNameContext* graphName();

  class  DelimitedGraphNameContext : public antlr4::ParserRuleContext {
  public:
    DelimitedGraphNameContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *DOUBLE_QUOTED_CHARACTER_SEQUENCE();
    antlr4::tree::TerminalNode *ACCENT_QUOTED_CHARACTER_SEQUENCE();

   
  };

  DelimitedGraphNameContext* delimitedGraphName();

  class  GraphTypeNameContext : public antlr4::ParserRuleContext {
  public:
    GraphTypeNameContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    IdentifierContext *identifier();

   
  };

  GraphTypeNameContext* graphTypeName();

  class  NodeTypeNameContext : public antlr4::ParserRuleContext {
  public:
    NodeTypeNameContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    IdentifierContext *identifier();

   
  };

  NodeTypeNameContext* nodeTypeName();

  class  EdgeTypeNameContext : public antlr4::ParserRuleContext {
  public:
    EdgeTypeNameContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    IdentifierContext *identifier();

   
  };

  EdgeTypeNameContext* edgeTypeName();

  class  BindingTableNameContext : public antlr4::ParserRuleContext {
  public:
    BindingTableNameContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    RegularIdentifierContext *regularIdentifier();
    DelimitedBindingTableNameContext *delimitedBindingTableName();

   
  };

  BindingTableNameContext* bindingTableName();

  class  DelimitedBindingTableNameContext : public antlr4::ParserRuleContext {
  public:
    DelimitedBindingTableNameContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *DOUBLE_QUOTED_CHARACTER_SEQUENCE();
    antlr4::tree::TerminalNode *ACCENT_QUOTED_CHARACTER_SEQUENCE();

   
  };

  DelimitedBindingTableNameContext* delimitedBindingTableName();

  class  ProcedureNameContext : public antlr4::ParserRuleContext {
  public:
    ProcedureNameContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    IdentifierContext *identifier();

   
  };

  ProcedureNameContext* procedureName();

  class  LabelNameContext : public antlr4::ParserRuleContext {
  public:
    LabelNameContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    IdentifierContext *identifier();

   
  };

  LabelNameContext* labelName();

  class  PropertyNameContext : public antlr4::ParserRuleContext {
  public:
    PropertyNameContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    IdentifierContext *identifier();

   
  };

  PropertyNameContext* propertyName();

  class  FieldNameContext : public antlr4::ParserRuleContext {
  public:
    FieldNameContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    IdentifierContext *identifier();

   
  };

  FieldNameContext* fieldName();

  class  ElementVariableContext : public antlr4::ParserRuleContext {
  public:
    ElementVariableContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    BindingVariableContext *bindingVariable();

   
  };

  ElementVariableContext* elementVariable();

  class  PathVariableContext : public antlr4::ParserRuleContext {
  public:
    PathVariableContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    BindingVariableContext *bindingVariable();

   
  };

  PathVariableContext* pathVariable();

  class  SubpathVariableContext : public antlr4::ParserRuleContext {
  public:
    SubpathVariableContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    RegularIdentifierContext *regularIdentifier();

   
  };

  SubpathVariableContext* subpathVariable();

  class  BindingVariableContext : public antlr4::ParserRuleContext {
  public:
    BindingVariableContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    RegularIdentifierContext *regularIdentifier();

   
  };

  BindingVariableContext* bindingVariable();

  class  UnsignedLiteralContext : public antlr4::ParserRuleContext {
  public:
    UnsignedLiteralContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    UnsignedNumericLiteralContext *unsignedNumericLiteral();
    GeneralLiteralContext *generalLiteral();

   
  };

  UnsignedLiteralContext* unsignedLiteral();

  class  GeneralLiteralContext : public antlr4::ParserRuleContext {
  public:
    GeneralLiteralContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *BOOLEAN_LITERAL();
    CharacterStringLiteralContext *characterStringLiteral();
    antlr4::tree::TerminalNode *BYTE_STRING_LITERAL();
    TemporalLiteralContext *temporalLiteral();
    DurationLiteralContext *durationLiteral();
    NullLiteralContext *nullLiteral();
    ListLiteralContext *listLiteral();
    RecordLiteralContext *recordLiteral();

   
  };

  GeneralLiteralContext* generalLiteral();

  class  TemporalLiteralContext : public antlr4::ParserRuleContext {
  public:
    TemporalLiteralContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    DateLiteralContext *dateLiteral();
    TimeLiteralContext *timeLiteral();
    DatetimeLiteralContext *datetimeLiteral();

   
  };

  TemporalLiteralContext* temporalLiteral();

  class  DateLiteralContext : public antlr4::ParserRuleContext {
  public:
    DateLiteralContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *DATE();
    DateStringContext *dateString();

   
  };

  DateLiteralContext* dateLiteral();

  class  TimeLiteralContext : public antlr4::ParserRuleContext {
  public:
    TimeLiteralContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *TIME();
    TimeStringContext *timeString();

   
  };

  TimeLiteralContext* timeLiteral();

  class  DatetimeLiteralContext : public antlr4::ParserRuleContext {
  public:
    DatetimeLiteralContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    DatetimeStringContext *datetimeString();
    antlr4::tree::TerminalNode *DATETIME();
    antlr4::tree::TerminalNode *TIMESTAMP();

   
  };

  DatetimeLiteralContext* datetimeLiteral();

  class  ListLiteralContext : public antlr4::ParserRuleContext {
  public:
    ListLiteralContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    ListValueConstructorByEnumerationContext *listValueConstructorByEnumeration();

   
  };

  ListLiteralContext* listLiteral();

  class  RecordLiteralContext : public antlr4::ParserRuleContext {
  public:
    RecordLiteralContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    RecordConstructorContext *recordConstructor();

   
  };

  RecordLiteralContext* recordLiteral();

  class  IdentifierContext : public antlr4::ParserRuleContext {
  public:
    IdentifierContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    RegularIdentifierContext *regularIdentifier();
    antlr4::tree::TerminalNode *DOUBLE_QUOTED_CHARACTER_SEQUENCE();
    antlr4::tree::TerminalNode *ACCENT_QUOTED_CHARACTER_SEQUENCE();

   
  };

  IdentifierContext* identifier();

  class  RegularIdentifierContext : public antlr4::ParserRuleContext {
  public:
    RegularIdentifierContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *REGULAR_IDENTIFIER();
    NonReservedWordsContext *nonReservedWords();

   
  };

  RegularIdentifierContext* regularIdentifier();

  class  TimeZoneStringContext : public antlr4::ParserRuleContext {
  public:
    TimeZoneStringContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    CharacterStringLiteralContext *characterStringLiteral();

   
  };

  TimeZoneStringContext* timeZoneString();

  class  CharacterStringLiteralContext : public antlr4::ParserRuleContext {
  public:
    CharacterStringLiteralContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *SINGLE_QUOTED_CHARACTER_SEQUENCE();
    antlr4::tree::TerminalNode *DOUBLE_QUOTED_CHARACTER_SEQUENCE();

   
  };

  CharacterStringLiteralContext* characterStringLiteral();

  class  UnsignedNumericLiteralContext : public antlr4::ParserRuleContext {
  public:
    UnsignedNumericLiteralContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    ExactNumericLiteralContext *exactNumericLiteral();
    ApproximateNumericLiteralContext *approximateNumericLiteral();

   
  };

  UnsignedNumericLiteralContext* unsignedNumericLiteral();

  class  ExactNumericLiteralContext : public antlr4::ParserRuleContext {
  public:
    ExactNumericLiteralContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *UNSIGNED_DECIMAL_IN_SCIENTIFIC_NOTATION_WITH_EXACT_NUMBER_SUFFIX();
    antlr4::tree::TerminalNode *UNSIGNED_DECIMAL_IN_COMMON_NOTATION_WITH_EXACT_NUMBER_SUFFIX();
    antlr4::tree::TerminalNode *UNSIGNED_DECIMAL_IN_COMMON_NOTATION_WITHOUT_SUFFIX();
    antlr4::tree::TerminalNode *UNSIGNED_DECIMAL_INTEGER_WITH_EXACT_NUMBER_SUFFIX();
    UnsignedIntegerContext *unsignedInteger();

   
  };

  ExactNumericLiteralContext* exactNumericLiteral();

  class  ApproximateNumericLiteralContext : public antlr4::ParserRuleContext {
  public:
    ApproximateNumericLiteralContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *UNSIGNED_DECIMAL_IN_SCIENTIFIC_NOTATION_WITH_APPROXIMATE_NUMBER_SUFFIX();
    antlr4::tree::TerminalNode *UNSIGNED_DECIMAL_IN_SCIENTIFIC_NOTATION_WITHOUT_SUFFIX();
    antlr4::tree::TerminalNode *UNSIGNED_DECIMAL_IN_COMMON_NOTATION_WITH_APPROXIMATE_NUMBER_SUFFIX();
    antlr4::tree::TerminalNode *UNSIGNED_DECIMAL_INTEGER_WITH_APPROXIMATE_NUMBER_SUFFIX();

   
  };

  ApproximateNumericLiteralContext* approximateNumericLiteral();

  class  UnsignedIntegerContext : public antlr4::ParserRuleContext {
  public:
    UnsignedIntegerContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *UNSIGNED_DECIMAL_INTEGER();
    antlr4::tree::TerminalNode *UNSIGNED_HEXADECIMAL_INTEGER();
    antlr4::tree::TerminalNode *UNSIGNED_OCTAL_INTEGER();
    antlr4::tree::TerminalNode *UNSIGNED_BINARY_INTEGER();

   
  };

  UnsignedIntegerContext* unsignedInteger();

  class  UnsignedDecimalIntegerContext : public antlr4::ParserRuleContext {
  public:
    UnsignedDecimalIntegerContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *UNSIGNED_DECIMAL_INTEGER();

   
  };

  UnsignedDecimalIntegerContext* unsignedDecimalInteger();

  class  NullLiteralContext : public antlr4::ParserRuleContext {
  public:
    NullLiteralContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *NULL_KW();

   
  };

  NullLiteralContext* nullLiteral();

  class  DateStringContext : public antlr4::ParserRuleContext {
  public:
    DateStringContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    CharacterStringLiteralContext *characterStringLiteral();

   
  };

  DateStringContext* dateString();

  class  TimeStringContext : public antlr4::ParserRuleContext {
  public:
    TimeStringContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    CharacterStringLiteralContext *characterStringLiteral();

   
  };

  TimeStringContext* timeString();

  class  DatetimeStringContext : public antlr4::ParserRuleContext {
  public:
    DatetimeStringContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    CharacterStringLiteralContext *characterStringLiteral();

   
  };

  DatetimeStringContext* datetimeString();

  class  DurationLiteralContext : public antlr4::ParserRuleContext {
  public:
    DurationLiteralContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *DURATION();
    DurationStringContext *durationString();

   
  };

  DurationLiteralContext* durationLiteral();

  class  DurationStringContext : public antlr4::ParserRuleContext {
  public:
    DurationStringContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    CharacterStringLiteralContext *characterStringLiteral();

   
  };

  DurationStringContext* durationString();

  class  NodeSynonymContext : public antlr4::ParserRuleContext {
  public:
    NodeSynonymContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *NODE();
    antlr4::tree::TerminalNode *VERTEX();

   
  };

  NodeSynonymContext* nodeSynonym();

  class  EdgesSynonymContext : public antlr4::ParserRuleContext {
  public:
    EdgesSynonymContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *EDGES();
    antlr4::tree::TerminalNode *RELATIONSHIPS();

   
  };

  EdgesSynonymContext* edgesSynonym();

  class  EdgeSynonymContext : public antlr4::ParserRuleContext {
  public:
    EdgeSynonymContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *EDGE();
    antlr4::tree::TerminalNode *RELATIONSHIP();

   
  };

  EdgeSynonymContext* edgeSynonym();

  class  NonReservedWordsContext : public antlr4::ParserRuleContext {
  public:
    NonReservedWordsContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *ACYCLIC();
    antlr4::tree::TerminalNode *BINDING();
    antlr4::tree::TerminalNode *BINDINGS();
    antlr4::tree::TerminalNode *CONNECTING();
    antlr4::tree::TerminalNode *DESTINATION();
    antlr4::tree::TerminalNode *DIFFERENT();
    antlr4::tree::TerminalNode *DIRECTED();
    antlr4::tree::TerminalNode *EDGE();
    antlr4::tree::TerminalNode *EDGES();
    antlr4::tree::TerminalNode *ELEMENT();
    antlr4::tree::TerminalNode *ELEMENTS();
    antlr4::tree::TerminalNode *FIRST();
    antlr4::tree::TerminalNode *GRAPH();
    antlr4::tree::TerminalNode *GROUPS();
    antlr4::tree::TerminalNode *KEEP();
    antlr4::tree::TerminalNode *LABEL();
    antlr4::tree::TerminalNode *LABELED();
    antlr4::tree::TerminalNode *LABELS();
    antlr4::tree::TerminalNode *LAST();
    antlr4::tree::TerminalNode *NFC();
    antlr4::tree::TerminalNode *NFD();
    antlr4::tree::TerminalNode *NFKC();
    antlr4::tree::TerminalNode *NFKD();
    antlr4::tree::TerminalNode *NO();
    antlr4::tree::TerminalNode *NODE();
    antlr4::tree::TerminalNode *NORMALIZED();
    antlr4::tree::TerminalNode *ONLY();
    antlr4::tree::TerminalNode *ORDINALITY();
    antlr4::tree::TerminalNode *PROPERTY();
    antlr4::tree::TerminalNode *READ();
    antlr4::tree::TerminalNode *RELATIONSHIP();
    antlr4::tree::TerminalNode *RELATIONSHIPS();
    antlr4::tree::TerminalNode *REPEATABLE();
    antlr4::tree::TerminalNode *SHORTEST();
    antlr4::tree::TerminalNode *SIMPLE();
    antlr4::tree::TerminalNode *SOURCE();
    antlr4::tree::TerminalNode *TABLE();
    antlr4::tree::TerminalNode *TO();
    antlr4::tree::TerminalNode *TRAIL();
    antlr4::tree::TerminalNode *TRANSACTION();
    antlr4::tree::TerminalNode *TYPE();
    antlr4::tree::TerminalNode *UNDIRECTED();
    antlr4::tree::TerminalNode *VERTEX();
    antlr4::tree::TerminalNode *WALK();
    antlr4::tree::TerminalNode *WITHOUT();
    antlr4::tree::TerminalNode *WRITE();
    antlr4::tree::TerminalNode *ZONE();

   
  };

  NonReservedWordsContext* nonReservedWords();


  bool sempred(antlr4::RuleContext *_localctx, size_t ruleIndex, size_t predicateIndex) override;

  bool compositeQueryExpressionSempred(CompositeQueryExpressionContext *_localctx, size_t predicateIndex);
  bool labelExpressionSempred(LabelExpressionContext *_localctx, size_t predicateIndex);
  bool simplifiedTermSempred(SimplifiedTermContext *_localctx, size_t predicateIndex);
  bool simplifiedFactorLowSempred(SimplifiedFactorLowContext *_localctx, size_t predicateIndex);
  bool valueTypeSempred(ValueTypeContext *_localctx, size_t predicateIndex);
  bool valueExpressionSempred(ValueExpressionContext *_localctx, size_t predicateIndex);
  bool valueExpressionPrimarySempred(ValueExpressionPrimaryContext *_localctx, size_t predicateIndex);
  bool numericValueExpressionSempred(NumericValueExpressionContext *_localctx, size_t predicateIndex);

  // By default the static state used to implement the parser is lazily initialized during the first
  // call to the constructor. You can call this function if you wish to initialize the static state
  // ahead of time.
  static void initialize();

private:
};

