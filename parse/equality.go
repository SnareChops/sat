package parse

import (
	"fmt"

	"github.com/SnareChops/sat/expression"
	"github.com/SnareChops/sat/options"
	"github.com/SnareChops/sat/token"
	"github.com/SnareChops/sat/types"
)

func parse_equality(left *token.Tokens, right *token.Tokens, scope *types.Scope) (types.Expression, error) {
	if *options.Trace {
		fmt.Printf("parse_equality\n\t%v\n\t%v\n---\n", left, right)
	}
	left_expression, err := parse_expression(left, scope)
	if err != nil {
		return nil, err
	}
	right_expression, err := parse_expression(right, scope)
	if err != nil {
		return nil, err
	}
	return expression.Equality{left_expression.Location(), left_expression, right_expression}, nil
}
