package parse

import (
	"fmt"

	"github.com/SnareChops/sat/expression"
	"github.com/SnareChops/sat/options"
	"github.com/SnareChops/sat/token"
	"github.com/SnareChops/sat/types"
)

func parse_test(loc types.Location, tokens *token.Tokens, scope *types.Scope) (types.Expression, error) {
	if *options.Trace {
		fmt.Printf("parse_test\n\t%v\n\t%v\n---\n", loc, tokens)
	}
	tok, ok := tokens.Token()
	if !ok {
		return nil, ParseError(loc, "expected test name")
	}
	nameToken, ok := tok.(token.String)
	if !ok {
		return nil, ParseError(tok.Location(), "expected test name to be a string")
	}
	tokens.Advance()
	tok, ok = tokens.Token()
	if !ok {
		return nil, ParseError(loc, "expected block for test")
	}
	blockToken, ok := tok.(token.Block)
	if !ok {
		return nil, ParseError(tok.Location(), "expected test block")
	}
	block, err := parse_block(blockToken.Loc, blockToken.Tokens, scope)
	if err != nil {
		return nil, err
	}
	return expression.Test{loc, nameToken.Value, block}, nil
}
