package parse

import (
	"fmt"

	"github.com/SnareChops/sat/expression"
	"github.com/SnareChops/sat/options"
	"github.com/SnareChops/sat/token"
	"github.com/SnareChops/sat/types"
)

func parse_ref(tokens *token.Tokens) (types.Expression, error) {
	if *options.Trace {
		fmt.Printf("parse_ref\n\t%v\n---\n", tokens)
	}
	symbol := ""
	tok, ok := tokens.Token()
	if !ok {
		return nil, ParseError(tokens.Loc(), "expected ref")
	}
	loc := tok.Location()
	for tok, ok := tokens.Token(); ok; tok, ok = tokens.Token() {
		switch tok.(type) {
		case token.Dot:
			tokens.Advance()
			symbol += "."
		case token.Symbol:
			tokens.Advance()
			symbol += tok.(token.Symbol).Value
		case token.Number:
			tokens.Advance()
			symbol += tok.(token.Number).Value
		default:
			return expression.Ref{loc, symbol}, nil
		}
	}
	return expression.Ref{loc, symbol}, nil
}
