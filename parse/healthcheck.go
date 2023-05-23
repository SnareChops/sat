package parse

import (
	"fmt"

	"github.com/SnareChops/sat/expression"
	"github.com/SnareChops/sat/options"
	"github.com/SnareChops/sat/token"
	"github.com/SnareChops/sat/types"
)

func parse_healthcheck(loc types.Location, tokens *token.Tokens, scope *types.Scope) (expression.Healthcheck, error) {
	if *options.Trace {
		fmt.Printf("parse_healthcheck\n\t%v\n\t%v\n---\n", loc, tokens)
	}
	tok, ok := tokens.Token()
	if !ok {
		return expression.Healthcheck{}, ParseError(loc, "expected expression for healthcheck")
	}
	pipe, ok := tok.(token.Pipe)
	if !ok {
		return expression.Healthcheck{}, ParseError(loc, "expected pipe for healthcheck")
	}
	exp, err := parse_expression(pipe.Tokens, scope)
	if err != nil {
		return expression.Healthcheck{}, err
	}
	return expression.Healthcheck{loc, exp}, nil
}
