package compiler

type TemplateValidateFunc func(string) error

type Template struct {
	Name    string
	Type    string
	Pattern string
}

var (
	AliasEntity Template = Template{
		Name:    "Alias",
		Type:    "entity",
		Pattern: `#alias\sentity\s\w([\w0-9_\-]*)\s@[apse](\[(([a-zA-Z]\w*=(\w*|\{([a-zA-Z]\w*=\w*)\})),?)*\])?`,
	}
	AliasVector Template = Template{
		Name:    "Alias",
		Type:    "vector",
		Pattern: `#alias\svector\s\w([\w0-9_\-]*)\s((\~|\^)?\-?[0-9]*\s){2}((\~|\^)?\-?[0-9]*\s*)`,
	}
)
