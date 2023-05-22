package parse

import (
	"fmt"

	"github.com/SnareChops/sat/expression"
	"github.com/SnareChops/sat/options"
	"github.com/SnareChops/sat/token"
	"github.com/SnareChops/sat/types"
)

func parse_spawn(loc types.Location, tokens *token.Tokens, scope *types.Scope) (types.Expression, error) {
	if *options.Trace {
		fmt.Printf("parse_spawn\n\t%v\n\t%v\n---\n", loc, tokens)
	}
	tok, ok := tokens.Token()
	if !ok {
		return nil, ParseError(loc, "expected expression for spawn")
	}
	pipe, ok := tok.(token.Pipe)
	if !ok {
		return nil, ParseError(loc, "expected pipe for spawn")
	}
	exp, err := parse_expression(pipe.Tokens, scope)
	if err != nil {
		return nil, err
	}
	return expression.Spawn{loc, exp}, nil
}
