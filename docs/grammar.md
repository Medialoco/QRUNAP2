# Implemented grammar

This is the QRUNAP2 0.1 parser contract, not a complete reconstructed QNAP2 grammar. Evidence for individual spellings is in the [inventory](language-inventory.md). Restrictions and required sections below are implementation choices.

```text
program = /DECLARE/ QUEUE identifier { , identifier } ;
          station { station }
          /CONTROL/ TMAX = number ;
          /EXEC/ BEGIN SIMUL ; END ; /END/
station = /STATION/ NAME = identifier ; { property }
property = TYPE = SOURCE ;
         | TYPE = SERVER [ , SINGLE | , MULTIPLE ( integer ) ] ;
         | SERVICE = ( EXP | CST ) ( number ) ;
         | TRANSIT = ( identifier | OUT ) ;
         | INIT = integer ;
```

The parentheses around EXP/CST alternatives in this notation are grouping notation; concrete code is `SERVICE=EXP(2);` or `SERVICE=CST(2);`. SERVICE and TRANSIT are required once per station. TYPE defaults to a single server, INIT to zero. Property order after NAME is free; duplicate properties are errors. Every declared name needs a definition. OUT is reserved. A source cannot receive a route or have nonzero INIT.

Identifiers start with an ASCII letter and continue with letters, digits or underscores. Case is ignored. `&` begins a comment ending at the next newline. Numbers are unsigned decimal literals, with an optional decimal point. Names, sections, properties and numeric formats outside this subset are rejected.

Historical lexical rules, section repetition, implicit declarations, general expressions and full SERVICE bodies remain research tasks.
