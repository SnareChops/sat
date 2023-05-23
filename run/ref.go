package run

import (
	"fmt"
	"strconv"
	"strings"

	"github.com/SnareChops/sat/expression"
	"github.com/SnareChops/sat/options"
	"github.com/SnareChops/sat/primitive"
	"github.com/SnareChops/sat/types"
)

func RunRef(exp expression.Ref, scope *types.Scope) (types.Primitive, error) {
	if *options.Trace {
		fmt.Printf("run_ref\n\t%v\n\t%v\n---\n", exp, scope)
	}
	if !strings.Contains(exp.Value, ".") {
		if val, ok := scope.Vars[exp.Value]; ok {
			return val, nil
		}
		panic("todo")
	} else {
		parts := strings.Split(exp.Value, ".")
		first := parts[0]
		next := parts[1]
		if val, ok := scope.Vars[first]; ok {
			switch val.(type) {
			case primitive.Object:
				return run_expression(val.(primitive.Object).Value[next], scope)
			case primitive.Array:
				index, err := strconv.ParseInt(next, 10, 64)
				if err != nil {
					return nil, err
				}
				if len(val.(primitive.Array).Value) < int(index) {
					return nil, RunError(exp.Location(), "array index ref out of range")
				}
				return run_expression(val.(primitive.Array).Value[index], scope)
			}
		} else {
			return nil, RunError(exp.Loc, "expression")
		}
	}
	panic("")
}
