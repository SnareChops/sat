package run

import (
	"fmt"

	"github.com/SnareChops/sat/expression"
	"github.com/SnareChops/sat/options"
	"github.com/SnareChops/sat/types"
)

func run_expression(exp types.Expression, scope *types.Scope) (types.Primitive, error) {
	if *options.Trace {
		fmt.Printf("run_expression\n\t%v\n---\n", exp)
	}
	switch exp.(type) {
	case expression.Assignment:
		return RunAssignment(exp.(expression.Assignment), scope)
	case expression.Primitive:
		return exp.(expression.Primitive).Value, nil
	case expression.Equality:
		return run_equality(exp.(expression.Equality), scope)
	case expression.Ref:
		return RunRef(exp.(expression.Ref), scope)
	case expression.Assert:
		return RunAssert(exp.(expression.Assert), scope)
	case expression.Test:
		panic("")
	case expression.MultiRef:
		panic("")
	case expression.Get:
		return run_get(exp.(expression.Get), scope)
	case expression.Run:
		return run_command(exp.(expression.Run), scope)
	case expression.Spawn:
		return run_spawn(exp.(expression.Spawn), scope)
	case expression.Kill:
		return run_kill(exp.(expression.Kill), scope)
	default:
		return nil, RunError(exp.Location(), "unknown expression")
	}
}
