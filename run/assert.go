package run

import (
	"fmt"

	"github.com/SnareChops/sat/expression"
	"github.com/SnareChops/sat/options"
	"github.com/SnareChops/sat/primitive"
	"github.com/SnareChops/sat/types"
)

func RunAssert(assert expression.Assert, scope *types.Scope) (types.Primitive, error) {
	if *options.Trace {
		fmt.Printf("run_assert\n\t%v\n\t%v\n---\n", assert, scope)
	}
	prim, err := run_expression(assert.Value, scope)
	if err != nil {
		return nil, err
	}
	b, ok := prim.(primitive.Boolean)
	if !ok {
		return nil, RunError(assert.Loc, "expected bool value as expression result for assert")
	}
	scope.Asserts = append(scope.Asserts, types.AssertResult{assert.Loc, b.Value})
	return b, nil
}
