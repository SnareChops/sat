package parse

import (
	"fmt"

	"github.com/SnareChops/sat/expression"
	"github.com/SnareChops/sat/options"
	"github.com/SnareChops/sat/token"
	"github.com/SnareChops/sat/types"
)

func parse_assert(loc types.Location, tokens *token.Tokens, scope *types.Scope) (types.Expression, error) {
	if *options.Trace {
		fmt.Printf("parse_assert\n\t%v\n\t%v\n---\n", loc, tokens)
	}
	tok, ok := tokens.Token()
	if !ok {
		return nil, ParseError(tokens.Loc(), "expected pipe")
	}
	pipe, ok := tok.(token.Pipe)
	if !ok {
		return nil, ParseError(tok.Location(), "expected pipe")
	}
	exp, err := parse_expression(pipe.Tokens, scope)
	if err != nil {
		return nil, err
	}
	return expression.Assert{loc, exp}, nil
}
