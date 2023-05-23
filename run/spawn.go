package run

import (
	"fmt"
	"os/exec"
	"runtime"

	"github.com/SnareChops/sat/expression"
	"github.com/SnareChops/sat/options"
	"github.com/SnareChops/sat/primitive"
	"github.com/SnareChops/sat/types"
)

func run_spawn(exp expression.Spawn, scope *types.Scope) (primitive.Process, error) {
	if *options.Trace {
		fmt.Printf("run_spawn\n\t%v\n\t%v\n---\n", exp, scope)
	}
	prim, err := run_expression(exp.Command, scope)
	if err != nil {
		return primitive.Process{}, err
	}
	str, ok := prim.(primitive.String)
	if !ok {
		return primitive.Process{}, RunError(exp.Loc, "expected string value for run expression")
	}
	var cmd *exec.Cmd
	if runtime.GOOS == "windows" {
		cmd = exec.Command("cmd", "/C", str.Value)
	} else {
		cmd = exec.Command("sh", "-c", str.Value)
	}
	err = cmd.Start()
	if err != nil {
		return primitive.Process{}, RunError(exp.Loc, err.Error())
	}
	return primitive.Process{cmd}, nil
}
