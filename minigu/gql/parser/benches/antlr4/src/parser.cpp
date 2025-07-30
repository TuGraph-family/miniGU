#include "GQLLexer.h"
#include "GQLParser.h"
#include "antlr4-runtime.h"

using namespace antlr4;

extern "C" void parse_gql(const char *input) {
  ANTLRInputStream stream(input);
  GQLLexer lexer(&stream);
  CommonTokenStream tokens(&lexer);
  GQLParser parser(&tokens);
  // Prevent the compiler from optimizing out the result.
  volatile auto result = parser.gqlProgram();
}
