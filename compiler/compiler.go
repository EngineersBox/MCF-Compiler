package compiler

import (
	"flag"
	"fmt"
	"log"
	"os"
	"strings"

	"github.com/EngineersBox/ModularCLI/cli"
)

var commands = map[string]cli.SubCommand{
	"compile": {
		ErrorHandler: flag.ExitOnError,
		Flags: []*cli.Flag{
			{
				Type:         cli.TypeString,
				Name:         "verbosity",
				DefaultValue: "INFO",
				HelpMsg:      "Level of verbosity to apply [INFO,WARNING,ERROR]",
				Required:     false,
				ValidateFunc: func(arg cli.TypedArgument) error {
					var lowerVerbosity string = strings.ToLower(*arg.GetString())
					if lowerVerbosity != "info" && lowerVerbosity != "warning" && lowerVerbosity != "error" {
						return fmt.Errorf("Invalid verbosity [" + lowerVerbosity + "], must be one of INFO, WARNING, ERROR")
					}
					return nil
				},
			},
			{
				Type:         cli.TypeBool,
				Name:         "recursive",
				DefaultValue: false,
				HelpMsg:      "Whether to compile .mcfunction files in nested directories [default: false]",
				Required:     false,
			},
		},
		Parameters: []*cli.Parameter{
			{
				Type:     cli.TypeString,
				Name:     "path",
				Position: 0,
				ValidateFunc: func(param cli.Parameter) error {
					if _, err := os.Stat(*param.GetString()); os.IsNotExist(err) {
						return fmt.Errorf("Path does not exist")
					}
					return nil
				},
			},
		},
	},
}

func handleError(err error) {
	if err != nil {
		log.Fatal(err)
	}
}

func main() {
	compilerCli, err := cli.CreateCLI(commands)
	handleError(err)

	handleError(compilerCli.Parse())
}
