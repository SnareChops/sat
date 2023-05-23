package run

import (
	"fmt"

	"github.com/SnareChops/sat/expression"
	"github.com/SnareChops/sat/options"
	"github.com/SnareChops/sat/types"
)

func run_test(feedback *Feedback, exp expression.Test) error {
	if *options.Trace {
		fmt.Printf("run_test\n\t%v\n---\n", exp)
	}
	for _, e := range exp.Block.Contents {
		_, err := run_expression(e, exp.Block.Scope)
		if err != nil {
			return err
		}
	}
	failed := []types.Location{}
	for _, ass := range exp.Block.Scope.Asserts {
		if !ass.Passed {
			failed = append(failed, ass.Loc)
		}
	}
	if len(failed) > 0 {
		feedback.addTestFailed(exp.Name, failed)
	} else {
		feedback.addTestPassed(exp.Name)
	}
	return nil
}
