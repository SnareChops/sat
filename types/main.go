package types

import (
	"fmt"
	"strings"
)

type Location struct {
	File string
	Row  int
	Col  int
}

func (it *Location) ToString() string {
	return fmt.Sprintf("%s:%d:%d", it.File, it.Row, it.Col)
}

type Expression interface {
	Location() Location
	Debug(int) string
}

type Primitive interface {
	Debug(int) string
}

type Program struct {
	Expressions []Expression
	Scope       *Scope
}

func (self Program) Debug() string {
	out := []string{}
	for _, exp := range self.Expressions {
		out = append(out, exp.Debug(1))
	}
	return fmt.Sprintf("%s", strings.Join(out, "\n"))
}

type Scope struct {
	Parent  *Scope
	Vars    map[string]Primitive
	Asserts []AssertResult
}

func NewScope(parent *Scope) *Scope {
	return &Scope{
		Parent:  parent,
		Vars:    map[string]Primitive{},
		Asserts: []AssertResult{},
	}
}

type AssertResult struct {
	Loc    Location
	Passed bool
}
