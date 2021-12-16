package main

import (
	"fmt"
	"os"

	"github.com/Broyojo/lang/lexer"
	"github.com/Broyojo/lang/token"
)

func main() {
	text, err := os.ReadFile("dev.rs")
	if err != nil {
		panic(err)
	}
	source := string(text)
	l := lexer.New(source)

	t := l.NextToken()

	for t.Type != token.EOF {
		fmt.Printf("{Type: %s, Literal: %s}\n", t.Type, t.Literal)
		t = l.NextToken()
	}
}
