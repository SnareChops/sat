package run

import (
	"fmt"

	"github.com/SnareChops/sat/expression"
	"github.com/SnareChops/sat/options"
	"github.com/SnareChops/sat/primitive"
	"github.com/SnareChops/sat/types"
)

func RunAssignment(assert expression.Assignment, scope *types.Scope) (types.Primitive, error) {
	if *options.Trace {
		fmt.Printf("run_asssignment\n\t%v\n---\n", assert)
	}
	switch assert.Left.(type) {
	case expression.Ref:
		ref := assert.Left.(expression.Ref)
		prim, err := run_expression(assert.Right, scope)
		if err != nil {
			return nil, err
		}
		scope.Vars[ref.Value] = prim
		return prim, nil
	case expression.MultiRef:
		multi := assert.Left.(expression.MultiRef)
		prim, err := run_expression(assert.Right, scope)
		if err != nil {
			return nil, err
		}
		values, ok := prim.(primitive.Multi)
		if !ok {
			return nil, RunError(assert.Right.Location(), "expected multi value expression")
		}
		if len(multi.Refs) > len(values.Value) {
			return nil, RunError(assert.Left.Location(), "expression returns fewer values than variables specified")
		}
		for i, ref := range multi.Refs {
			scope.Vars[ref] = values.Value[i]
		}
		return values.Value[0], nil
	default:
		return nil, RunError(assert.Left.Location(), "invalid assignment")
	}
}
