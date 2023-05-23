package run

import (
	"fmt"

	"github.com/SnareChops/sat/expression"
	"github.com/SnareChops/sat/options"
	"github.com/SnareChops/sat/types"
)

func RunError(loc types.Location, message string) error {
	return fmt.Errorf("%s: %s", loc.ToString(), message)
}

func Run(program types.Program) (*Feedback, error) {
	if *options.Trace {
		fmt.Printf("run\n\t%v\n---\n", program)
	}
	feedback := &Feedback{}
	for _, exp := range program.Expressions {
		var err error
		switch exp.(type) {
		case expression.Assignment:
			_, err = RunAssignment(exp.(expression.Assignment), program.Scope)
			if err != nil {
				return nil, err
			}
		case expression.Test:
			err = run_test(feedback, exp.(expression.Test))
			if err != nil {
				return nil, err
			}
		case expression.Assert:
			_, err = RunAssert(exp.(expression.Assert), program.Scope)
			if err != nil {
				return nil, err
			}
		case expression.Get:
			_, err = run_get(exp.(expression.Get), program.Scope)
			if err != nil {
				return nil, err
			}
		case expression.Run:
			_, err = run_command(exp.(expression.Run), program.Scope)
			if err != nil {
				return nil, err
			}
		case expression.Spawn:
			return nil, RunError(exp.Location(), "spawn at the root level must be used in an assignment")
			// _, err = run_spawn(exp.(expression.Spawn), program.Scope)
		case expression.Kill:
			_, err = run_kill(exp.(expression.Kill), program.Scope)
			if err != nil {
				return nil, err
			}
		case expression.Healthcheck:
			_, err = run_healthcheck(exp.(expression.Healthcheck), program.Scope)
			if err != nil {
				return nil, err
			}
		default:
			return nil, fmt.Errorf("unexpected expression type")
		}
		if err != nil {
			return nil, err
		}
	}
	return feedback, nil
}
