package parse

import (
	"fmt"

	"github.com/SnareChops/sat/expression"
	"github.com/SnareChops/sat/options"
	"github.com/SnareChops/sat/token"
	"github.com/SnareChops/sat/types"
)

func parse_get(loc types.Location, tokens *token.Tokens, scope *types.Scope) (types.Expression, error) {
	if *options.Trace {
		fmt.Printf("parse_get\n\t%v\n\t%v\n---\n", loc, tokens)
	}
	tok, ok := tokens.Token()
	if !ok {
		return nil, ParseError(loc, "expected piped expression for get")
	}
	pipe, ok := tok.(token.Pipe)
	if !ok {
		return nil, ParseError(loc, "expected pipe for kill")
	}
	exp, err := parse_expression(pipe.Tokens, scope)
	if err != nil {
		return nil, err
	}
	return expression.Get{loc, exp}, nil
}
