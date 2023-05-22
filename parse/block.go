package parse

import (
	"fmt"

	"github.com/SnareChops/sat/expression"
	"github.com/SnareChops/sat/options"
	"github.com/SnareChops/sat/token"
	"github.com/SnareChops/sat/types"
)

func parse_block(loc types.Location, tokens []*token.Tokens, scope *types.Scope) (expression.Block, error) {
	if *options.Trace {
		fmt.Printf("parse_block\n\t%v\n---\n", tokens)
	}
	expressions := []types.Expression{}
	for _, toks := range tokens {
		exp, err := parse_expression(toks, scope)
		if err != nil {
			return expression.Block{}, err
		}
		expressions = append(expressions, exp)
	}
	return expression.NewBlock(loc, expressions, scope), nil
}
