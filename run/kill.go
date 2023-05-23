package run

import (
	"fmt"

	"github.com/SnareChops/sat/expression"
	"github.com/SnareChops/sat/options"
	"github.com/SnareChops/sat/primitive"
	"github.com/SnareChops/sat/types"
)

func run_kill(exp expression.Kill, scope *types.Scope) (types.Primitive, error) {
	if *options.Trace {
		fmt.Printf("run_kill\n\t%v\n\t%v\n---\n", exp, scope)
	}
	prim, err := run_expression(exp.Handle, scope)
	if err != nil {
		return nil, err
	}
	proc, ok := prim.(primitive.Process)
	if !ok {
		return nil, RunError(exp.Location(), "expected process reference for kill expression")
	}
	err = proc.Value.Process.Kill()
	if err != nil {
		return nil, RunError(exp.Location(), err.Error())
	}
	return primitive.Void{}, nil
}
