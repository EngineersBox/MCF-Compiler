# TODO

1. Basic `let` declarations of any text as `let <NAME>: <TYPE> = <VALUE>;`
2. Replace templates (`${<NAME>}`) with corresponding let values
3. Add types for:
    * `entity`
    * `vec3`
4. Validate `entity` type declarations as `@[apse](\[(((([a-zA-Z]\w*=(\w*|\{([a-zA-Z]\w*=\w*)\}))),)*)?(([a-zA-Z]\w*=(\w*|\{([a-zA-Z]\w*=\w*)\})))?\])?`
5. Validate `vec3` type declarations as `vec3\(((\~|\^)?\-?[0-9]+(\.[0-9]*)?,(\s*)?){2}((\~|\^)?\-?[0-9]+(\.[0-9]*)?)\)`
6. Add array types for `T[]`:
    * `entity[]`: `\[((\s*)?@[apse](\[(((([a-zA-Z]\w*=(\w*|\{([a-zA-Z]\w*=\w*)\}))),)*)?(([a-zA-Z]\w*=(\w*|\{([a-zA-Z]\w*=\w*)\})))?\])?,)*(\s*)?\]`
    * `vec3[]`: `\[((\s*)?vec3\(((\~|\^)?\-?[0-9]+(\.[0-9]*)?,(\s*)?){2}((\~|\^)?\-?[0-9]+(\.[0-9]*)?)\),)*(\s*)?\]`
7. Split for loop declarations into three sections:
    * Element value name
    * Array to reference
    * Block of statements to perform replacement on and emplace
8. Create n-copies of the block statements and perform value replacements on them
9. 

## Statement Example Expansion
```
let test1: vec3[] = [
    vec3(~85.3, ^-5.4, 0),
    vec3(43, ~0.4, ^-2.6)
];
let test2: entity = @p[score={blocked=1}]
execute ${test2} ~ ~ ~ setblock ${test1[0]} minecraft:obsidian
```
Creates parser objects:
```
{
    "LUT": {
        "test1": {
            "element": "declaration",
            "type": "vec3Array",
            "value": {
                "0": {
                    "element": "arrayEntry",
                    "type": "vec3",
                    "value": {
                        "x": "~85.3",
                        "y": "^-5.4",
                        "z": "0"
                    }
                },
                "1": {
                    "element": "arrayEntry",
                    "type": "vec3",
                    "value": {
                        "x": "43",
                        "y": "~0.4",
                        "z": "^2.6"
                    }
                }
            }
        },
        "test2": {
            "element": "declaration",
            "type": "entity",
            "value": "@p[score={blocked=1}]"
        }
    },
    "blocks": [
        {
            "element": "statement",
            "type": "literal",
            "value": "execute ${test2} ~ ~ ~ setblock ${test1[1]} minecraft:obsidian",
            "templates": [
                {
                    "name": "test2",
                    "type": "*",
                    "attributes": null
                },
                {
                    "name": "test1",
                    "type": "array",
                    "attributes": {
                        "arrayIndex": 1
                    }
                }
            ]
        }
    ]
}
```
Which is then expanded to:
```
execute @p[score={blocked=1}] ~ ~ ~ setblock 43 ~0.4 ^-2.6 minecraft:obsidian
```

## For Loop Example Expansion
```
for <ELEM NAME> in <ARRAY REF> {
    ... <STATEMENTS>
}
```
For example:
```
let test5: vec3[] = [
    vec3(~85.3, ^-5.4, 0),
    vec3(43, ~0.4, ^-2.6)
];
for value in test5 {
    execute @p ~ ~ ~ setblock ${value} minecraft:obsidian
    tp @p ${value}
}
```
Creates parser objects:
```
{
    "LUT": {
        "test5": {
            "element": "declaration",
            "type": "vec3Array",
            "value": {
                "0": {
                    "element": "arrayEntry",
                    "type": "vec3",
                    "value": {
                        "x": "~85.3",
                        "y": "^-5.4",
                        "z": "0"
                    }
                },
                "1": {
                    "element": "arrayEntry",
                    "type": "vec3",
                    "value": {
                        "x": "43",
                        "y": "~0.4",
                        "z": "^2.6"
                    }
                }
            }
        }
    },
    "blocks": [
        {
            "element": "for",
            "type": null,
            "value: {
                "elemVarName": "value",
                "arrayRefName": "test5",
                "statements": [
                    {
                        "element": "statement",
                        "type": "literal",
                        "value": "setblock ${value} minecraft:obsidian",
                        templates: [
                            {
                                "name": "value",
                                "type": "*",
                                "attributes": null
                            }
                        ]
                    },
                    {
                        "element": "statement",
                        "type": "literal",
                        "value": "tp @p ${value}",
                        templates: [
                            {
                                "name": "value",
                                "type": "*",
                                "attributes": null
                            }
                        ]
                    }
                ]
            }
        }
    ]
}
```
Which is then exapanded to:
```
execute @p ~ ~ ~ setblock ~85.3 ^-5.4 0 minecraft:obsidian
tp @p ~85.3 ^-5.4 0
execute @p ~ ~ ~ setblock 43 ~0.4 ^-2.6 minecraft:obsidian
tp @p 43 ~0.4 ^-2.6
```

## Function Example Expansion
```
let test2: entity[] = [
    @a[scores={someObjective=3}],
    @e[tag=TestTag,r=5]
];

let test4: vec3[] = [
    vec3(~0.23, ^45.4, -25),
    vec3(4.1, ~-50.4, ^8.23)
];

let test5: vec3[] = [
    vec3(~85.3, ^-5.4, 0),
    vec3(43, ~0.4, ^-2.6)
];

fn test7(list: entity[], coords1: vec3, coords2: vec3) {
    for value in list {
        execute ${value} test4[0] fill ${coords1} ${coords2} air
    }
}

${test7(test2, test5[0], test5[1])}
```
Creates parser objects:
```
{
    "LUT": {
        "test2": {
            "element": "declaration",
            "type": "entityArray",
            "value": {
                "0": {
                    "type": "entity",
                    "name": null,
                    "value": "@a[scores={someObjective=3}]"
                },
                "1": {
                    "type": "entity",
                    "name": null,
                    "value": "@e[tag=TestTag,r=5]"
                }
            }
        },
        "test4": {
            "element": "declaration",
            "type": "vec3Array",
            "value": [
                "0": {
                    "type": "vec3",
                    "name": null,
                    "value": {
                        "x": "~0.23",
                        "y": "^45.4",
                        "z": "-25"
                    }
                },
                "1": {
                    "type": "vec3",
                    "name": null,
                    "value": {
                        "x": "4.1",
                        "y": "~-50.4",
                        "z": "^8.23"
                    }
                }
            ]
        },
        "test5": {
            "element": "declaration",
            "type": "vec3Array",
            "value": [
                "0": {
                    "type": "vec3",
                    "name": null,
                    "value": {
                        "x": "~85.3",
                        "y": "^-5.4",
                        "z": "0"
                    }
                },
                "1": {
                    "type": "vec3",
                    "name": null,
                    "value": {
                        "x": "43",
                        "y": "~0.4",
                        "z": "^2.6"
                    }
                }
            ]
        }
    },
    "blocks": [
        {
            "element": "function",
            "name": "test7",
            "parameters: [
                {
                    "type": "entityArray",
                    "name": "list"
                },
                {
                    "type": "vec3",
                    "name": "coords1"
                },
                {
                    "type": "vec3",
                    "name": "coords2"
                }
            ],
            "statements": [
                {
                    "element": "for",
                    "type": null,
                    "value": {
                        "elemVarName": "value",
                        "arrayRefName": "list",
                        "statements": [
                            {
                                "element": "statement",
                                "type": "literal",
                                "value": "execute ${value} ${test4[0]} fill ${coords1} ${coords2} air",
                                "templates": [
                                    {
                                        "name": "value",
                                        "type": "*",
                                        "attributes": null
                                    },
                                    {
                                        "name": "test4",
                                        "type": "array",
                                        "attributes": {
                                            "arrayIndex": 0
                                        }
                                    },
                                    {
                                        "name": "coords1",
                                        "type": "*",
                                        "attributes": null
                                    },
                                    {
                                        "name": "coords2",
                                        "type": "*",
                                        "attributes": null
                                    }
                                ]
                            }
                        ]
                    }
                }
            ]
        },
        {
            "element": "statement",
            "type": "literal",
            "value": "${test7(test2, test5[0], test5[1])}",
            "templates": [
                {
                    "name": "test7",
                    "type": "function",
                    "attributes": {
                        "parameters": [
                            {
                                "name": "test2",
                                "type": "*",
                                "attributes": null
                            },
                            {
                                "name": "test5",
                                "type": "array",
                                "attributes": {
                                    "arrayIndex": 0
                                }
                            },
                            {
                                "name": "test5",
                                "type": "array",
                                "attributes": {
                                    "arrayIndex": 1
                                }
                            }
                        ]
                    }
                }
            ]
        }
    ]
}
```
Which is then expanded to
```
for value in test2 {
    execute ${value} test4[0] fill ${test5[0]} ${test5[1]} air
}
```
And then to
```
execute @a[scores={someObjective=3}] ~85.3 ^-5.4 0 fill ~85.3 ^-5.4 0 43 ~0.4 ^-2.6 air
execute @e[tag=TestTag,r=5] ~85.3 ^-5.4 0 fill ~85.3 ^-5.4 0 43 ~0.4 ^-2.6 air
```