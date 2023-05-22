package parse

import (
	"fmt"

	"github.com/SnareChops/sat/options"
	"github.com/SnareChops/sat/token"
	"github.com/SnareChops/sat/types"
)

func ParseError(loc types.Location, message string) error {
	return fmt.Errorf("%s: %s", loc.ToString(), message)
}

func Parse(tokens []*token.Tokens) (types.Program, error) {
	if *options.Trace {
		fmt.Printf("Parse\n\t%v\n---\n", tokens)
	}
	expressions := []types.Expression{}
	scope := types.NewScope(nil)
	for _, toks := range tokens {
		expression, err := parse_expression(toks, scope)
		if err != nil {
			return types.Program{}, err
		}
		expressions = append(expressions, expression)
	}
	return types.Program{expressions, scope}, nil
}
