package parse

import (
	"fmt"

	"github.com/SnareChops/sat/expression"
	"github.com/SnareChops/sat/options"
	"github.com/SnareChops/sat/primitive"
	"github.com/SnareChops/sat/token"
	"github.com/SnareChops/sat/types"
)

func parse_symbol(tokens *token.Tokens, scope *types.Scope) (types.Expression, error) {
	if *options.Trace {
		fmt.Printf("parse_symbol\n\t%v\n---\n", tokens)
	}
	if next, ok := tokens.Next(); ok {
		switch next.(type) {
		case token.Dot:
			return parse_ref(tokens)
		case token.Comma:
			return parse_multi_ref(tokens)
		}
	}
	tok, ok := tokens.Token()
	if !ok {
		return nil, ParseError(tokens.Loc(), "expected symbol")
	}
	symbol := tok.(token.Symbol)
	switch symbol.Value {
	case "true":
		return expression.Primitive{symbol.Loc, primitive.Boolean{true}}, nil
	case "false":
		return expression.Primitive{symbol.Loc, primitive.Boolean{false}}, nil
	case "test":
		tokens.Advance()
		return parse_test(symbol.Loc, tokens, scope)
	case "assert":
		tokens.Advance()
		return parse_assert(symbol.Loc, tokens, scope)
	case "get":
		tokens.Advance()
		return parse_get(symbol.Loc, tokens, scope)
	case "run":
		tokens.Advance()
		return parse_run(symbol.Loc, tokens, scope)
	case "spawn":
		tokens.Advance()
		return parse_spawn(symbol.Loc, tokens, scope)
	case "kill":
		tokens.Advance()
		return parse_kill(symbol.Loc, tokens, scope)
	case "healthcheck":
		tokens.Advance()
		return parse_healthcheck(symbol.Loc, tokens, scope)
	default:
		return parse_ref(tokens)
	}
}
