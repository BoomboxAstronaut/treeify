# Treeify

Convert word lists into a regex prefix trees
---

Prefix tree regex patterns offer substantial performance benefits when searching for multiple words in large data sets.


Input:

Alan
Albert
Alessandro
Alexander
Alexis
Ali
Alice
Alma
Amanda
Amber
Amelia
Amy

Standard Regex Pattern:

\b(?:Alan|Albert|Alessandro|Alexander|Alexis|Ali|Alice|Alma|Amanda|Amber|Amelia|Amy)\b

Prefix-Tree Regex Pattern:

\b(?:A(?:l(?:an|bert|e(?:ssandro|x(?:ander|is))|i(?:ce)?|ma)|m(?:anda|ber|elia|y)))\b

Performance
-----

A few test showing the performance difference between a standard regex pattern and a prefix-tree regex pattern. 

Tests performed using gnu grep to search through wikipedia given an input list of several hundred words.


| No. Input Words | Standard Regex        | Prefix Tree           | Performance Gain |
| --------------- | ---------------------:| ---------------------:| ----------------:|
| 246 Words       |                       |                       |                  |
| Matches Found   |         11813664      |         11813664      |                  |
| real            |         2m44.197s     |         0m50.577s     |          + ~220% |
| user            |         2m41.844s     |         0m43.084s     |                  |
| sys             |         0m1.891s      |         0m1.918s      |                  |
|                 |                       |                       |                  |
| 221 Words       |                       |                       |                  |
| Matches Found   |         11189391      |         11189391      |                  |
| real            |         2m30.296s     |         0m50.527s     |          + ~190% |
| user            |         2m28.254s     |         0m44.390s     |                  |
| sys             |         0m1.846s      |         0m1.807s      |                  |
|                 |                       |                       |                  |
| 456 Words       |                       |                       |                  |
| Matches Found   |         18785993      |         18785993      |                  |
| real            |         5m0.686s      |         0m50.346s     |          + ~490% |
| user            |         4m58.284s     |         0m45.282s     |                  |
| sys             |         0m2.015s      |         0m1.912s      |                  |




