# Markdown Table Extension: ColGroup -- PoC

This repo is a POC for an extension to the Markdown table (aka GFM table) to write tables with more than one level of columns.

> [!CAUTION]
> This is still work-in-progress meaning that the POC is currently not functional.


## Syntax

Extended Markdown table are written like so:
```md
|main column 1|main column 2              |main column 3                                       |
|             |---------------------------|---------------------------------------------------:|
|             |[sub column 1|sub column 2]|[sub column 3|sub column 4                         ]|
|             |[            |            ]|[            |:-----------------------------------:]|
|             |[            |            ]|[            |[sub sub column 1 | sub sub column 2]]|
|-------------|[:----------:|------------]|[-----------:|[----------------:|:---------------:]]|
|cell 1       |[cell 2      |cell 3      ]|[cell 4      |[cell 5           | cell 6          ]]|
|cell 7       |[cell 8      |cell 9      ]|[cell 10     |[cell 11          | cell 12         ]]|
```

And produce such table :
![](res/demo-table-rendering.png)


## Backward Compatibility Issue

**None.**

_Classic Markdown table syntax is a subset of the extended syntax._
