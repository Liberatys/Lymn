# Lymn


A research project that aims to bring understanding to the internal workings of a RDMS by writing 
a version based on the findings of different literature and proven principles ranging from sqlite to mariadb

Because this is a side project, I'll name all parts of the system in a funny way, because I can.

Containing parts of the system:

- [ ] SQL-Parser {Ocarina}
- [ ] SQL-Sanatizer
- [ ] Buffer-Manager


## SQL-Parser {Ocarina}

Ocarina is the sql compiler/parser/processor that is turing an sql statement into a query plan
that then can be executed by the query executor.

Ocarina also performs a basic optimisation of the query. The query is later optimized again by another
part of the system to ensure that queries run at an reasonable speed in the system.

[Ocarina docu](./docu/ocarina.md)
