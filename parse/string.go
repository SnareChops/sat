package parse

import (
	"fmt"

	"github.com/SnareChops/sat/expression"
	"github.com/SnareChops/sat/options"
	"github.com/SnareChops/sat/primitive"
	"github.com/SnareChops/sat/token"
	"github.com/SnareChops/sat/types"
)

func parse_string(tokens *token.Tokens) (types.Expression, error) {
	if *options.Trace {
		fmt.Printf("parse_string\n\t%v\n---\n", tokens)
	}
	tok, ok := tokens.Token()
	if !ok {
		panic("")
	}
	tokens.Advance()
	sok := tok.(token.String)
	return expression.Primitive{sok.Loc, primitive.String{sok.Value}}, nil
}
