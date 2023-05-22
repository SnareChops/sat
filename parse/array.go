package parse

import (
	"fmt"

	"github.com/SnareChops/sat/expression"
	"github.com/SnareChops/sat/options"
	"github.com/SnareChops/sat/primitive"
	"github.com/SnareChops/sat/token"
	"github.com/SnareChops/sat/types"
)

func parse_array(tokens *token.Tokens, scope *types.Scope) (types.Expression, error) {
	if *options.Trace {
		fmt.Printf("parse_array\n\t%v\n\t%v\n---\n", tokens, scope)
	}
	tok, ok := tokens.Token()
	if !ok {
		return nil, ParseError(tokens.Loc(), "expected array")
	}
	tokens.Advance()
	sok := tok.(token.Array)
	value := []types.Expression{}
	for _, item := range sok.Items {
		exp, err := parse_expression(item, scope)
		if err != nil {
			return nil, err
		}
		value = append(value, exp)
	}
	return expression.Primitive{sok.Loc, primitive.Array{value}}, nil
}
