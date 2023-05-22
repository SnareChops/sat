package parse

import (
	"fmt"

	"github.com/SnareChops/sat/expression"
	"github.com/SnareChops/sat/options"
	"github.com/SnareChops/sat/token"
	"github.com/SnareChops/sat/types"
)

func parse_multi_ref(tokens *token.Tokens) (types.Expression, error) {
	if *options.Trace {
		fmt.Printf("parse_multi_ref\n\t%v\n---\n", tokens)
	}
	loc := tokens.Loc()
	refs := []string{}
	for {
		tok, ok := tokens.Token()
		if !ok {
			return expression.MultiRef{loc, refs}, nil
		}
		switch tok.(type) {
		case token.Comma:
			tokens.Advance()
		case token.Symbol:
			ref, err := parse_ref(tokens)
			if err != nil {
				return nil, err
			}
			r := ref.(expression.Ref)
			refs = append(refs, r.Value)
		default:
			return expression.MultiRef{loc, refs}, nil
		}
	}
}
