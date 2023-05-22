package parse

import (
	"fmt"

	"github.com/SnareChops/sat/options"
	"github.com/SnareChops/sat/token"
	"github.com/SnareChops/sat/types"
)

func parse_expression(tokens *token.Tokens, scope *types.Scope) (types.Expression, error) {
	if *options.Trace {
		fmt.Printf("parse_expression\n\t%v\n---\n", tokens)
	}
	if special, left, right := tokens.SplitOnFirstSpecial(); special >= 0 {
		switch special {
		case token.ASSIGN:
			return parse_assign(left, right, scope)
		case token.EQUALITY:
			return parse_equality(left, right, scope)
		default:
			return nil, ParseError(left.Loc(), "Unknown special token")
		}
	} else {
		tok, ok := tokens.Token()
		if !ok {
			return nil, ParseError(tokens.Loc(), "expected expression")
		}
		switch tok.(type) {
		case token.Symbol:
			return parse_symbol(tokens, scope)
		case token.Number:
			return parse_number(tokens)
		case token.String:
			return parse_string(tokens)
		case token.Object:
			return parse_object(tokens, scope)
		case token.Array:
			return parse_array(tokens, scope)
		default:
			return nil, ParseError(tok.Location(), "unknown token type")
		}
	}
}
