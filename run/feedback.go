package run

import (
	"fmt"
	"strings"

	"github.com/SnareChops/sat/types"
)

type Feedback struct {
	output []string
}

func (self *Feedback) addTestPassed(name string) {
	self.output = append(self.output, fmt.Sprintf("\t✓\t%s", name))
}

func (self *Feedback) addTestFailed(name string, failed []types.Location) {
	asserts := ""
	for _, fail := range failed {
		asserts += fmt.Sprintf("\n\t\t- assert %s", fail.ToString())
	}
	self.output = append(self.output, fmt.Sprintf("\t❌\t%s%s", name, asserts))
}

func (self *Feedback) ToString() string {
	return strings.Join(self.output, "\n")
}
