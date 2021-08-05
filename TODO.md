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
    "declaration": {
        "type": "vec3Array",
        "name": "test5",
        "value": [
            {
                "type": "vec3",
                "name": null,
                "value": {
                    "x": "~85.3",
                    "y": "^-5.4",
                    "z": "0"
                }
            },
            {
                "type": "vec3",
                "name": null,
                "value": {
                    "x": "43",
                    "y": "~0.4",
                    "z": "^2.6"
                }
            }
        ]
    },
    "for-loop": {
        "elemVarName": "value",
        "arrayRefName": "test5",
        "statementBlock": [
            {
                "type": "literal",
                "value": "setblock ${value} minecraft:obsidian"
            },
            {
                "type": "literal",
                "value": "tp @p ${value}"
            }
        ]
    }
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
    "declaration": {
        "type": "entityArray",
        "name": "test2",
        "value": [
            {
                "type": "entity",
                "name": null,
                "value": "@a[scores={someObjective=3}]"
            },
            {
                "type": "entity",
                "name": null,
                "value": "@e[tag=TestTag,r=5]"
            }
        ]
    },
    "declaration": {
        "type": "vec3Array",
        "name": "test4",
        "value": [
            {
                "type": "vec3",
                "name": null,
                "value": {
                    "x": "~0.23",
                    "y": "^45.4",
                    "z": "-25"
                }
            },
            {
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
    "declaration": {
        "type": "vec3Array",
        "name": "test5",
        "value": [
            {
                "type": "vec3",
                "name": null,
                "value": {
                    "x": "~85.3",
                    "y": "^-5.4",
                    "z": "0"
                }
            },
            {
                "type": "vec3",
                "name": null,
                "value": {
                    "x": "43",
                    "y": "~0.4",
                    "z": "^2.6"
                }
            }
        ]
    },
    "function": {
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
        "statementBlock": [
            {
                "type": "for-loop",
                "value": {
                    "elemVarName": "value",
                    "arrayRefName": "list",
                    "statementBlock": [
                        {
                            "type": "literal",
                            "value": "execute ${value} test4[0] fill ${coords1} ${corrds2} air"
                        }
                    ]
                }
            }
        ]
    }
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