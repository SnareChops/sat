package parse

import (
	"fmt"

	"github.com/SnareChops/sat/expression"
	"github.com/SnareChops/sat/options"
	"github.com/SnareChops/sat/primitive"
	"github.com/SnareChops/sat/token"
	"github.com/SnareChops/sat/types"
)

func parse_object(tokens *token.Tokens, scope *types.Scope) (types.Expression, error) {
	if *options.Trace {
		fmt.Printf("parse_obect\n\t%v\n\t%v\n---\n", tokens, scope)
	}
	tok, ok := tokens.Token()
	if !ok {
		return nil, ParseError(tokens.Loc(), "expected object")
	}
	tokens.Advance()
	sok := tok.(token.Object)
	value := map[string]types.Expression{}
	for key, prop := range sok.Props {
		exp, err := parse_expression(prop, scope)
		if err != nil {
			return nil, err
		}
		value[key] = exp
	}
	return expression.Primitive{sok.Loc, primitive.Object{value}}, nil
}
