package main

import (
	"flag"
	"fmt"
	"io/ioutil"
	"os"
	"path"
	"strings"

	"github.com/SnareChops/sat/lex"
	"github.com/SnareChops/sat/options"
	"github.com/SnareChops/sat/parse"
	"github.com/SnareChops/sat/run"
)

func main() {
	lex_only := flag.Bool("lex", false, "run lexer only and output debug")
	parse_only := flag.Bool("parse", false, "run lexer and parser only and output debug")
	options.Trace = flag.Bool("trace", false, "output trace info to stdout")
	flag.Parse()

	if len(flag.Args()) < 1 {
		panic("expected file name to run")
	}
	cwd, err := os.Getwd()
	if err != nil {
		panic(err)
	}
	file, err := os.Open(path.Join(cwd, flag.Args()[0]))
	if err != nil {
		panic(err)
	}
	defer file.Close()
	contents, err := ioutil.ReadAll(file)
	if err != nil {
		panic(err)
	}
	lexed, err := lex.File(flag.Args()[0], string(contents))
	if err != nil {
		panic(err)
	}
	debug := []string{}
	for _, l := range lexed {
		debug = append(debug, l.Debug(0))
	}
	if *lex_only == true {
		f, _ := os.Create("debug.txt")
		f.Write([]byte(strings.Join(debug, "\n")))
		f.Close()
		os.Exit(0)
	}
	program, err := parse.Parse(lexed)
	if err != nil {
		panic(err)
	}
	if *parse_only == true {
		f, _ := os.Create("debug.txt")
		f.Write([]byte(program.Debug()))
		f.Close()
		os.Exit(0)
	}
	feedback, err := run.Run(program)
	if err != nil {
		panic(err)
	}
	fmt.Printf(feedback.ToString())
}
