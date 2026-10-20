# Language inventory

Evidence levels: **attested** means visible in a historical listing; **explained** means its meaning is explicitly described by a source; **QRUNAP2 policy** means an implementation choice awaiting historical verification. See [sources](sources.md).

| Feature | Evidence / version / location | Meaning and v0.1 support |
| --- | --- | --- |
| /DECLARE/, QUEUE | S8, V9.Rb, line 1 | Attested; one declaration containing a comma-separated list. |
| /STATION/, NAME | S8, lines 3–5 | Attested; one definition for every declared station. |
| SERVICE=EXP(number) | S8, lines 3–5; S9, III-84 | Syntax attested, mean parameter explicitly explained in S9 (older V03). Implemented; full 9.3 verification pending. |
| SERVICE=CST(number) | S9, III-84–85 | Fixed duration explained; supported as a simple SERVICE expression. Exact 9.3 syntax verification pending. |
| TRANSIT=station | S8, lines 3–5 | Attested; deterministic destination. |
| OUT | S10, routing grammar | Exit destination corroborated; supported. |
| TYPE=SOURCE | S3, model listing | Attested; repeated arrivals implemented; first arrival after one sampled delay is QRUNAP2 policy. |
| TYPE=SERVER,MULTIPLE(n) | S3, model listing | Attested; shared FIFO queue with n servers implemented. |
| TYPE=SERVER,SINGLE | S10, station grammar | Corroborated; one server implemented. Omitted TYPE means one server in QRUNAP2; manual validation pending. |
| INIT=n | S8, line 3 | Initial population attested; initialization order is QRUNAP2 policy. |
| /CONTROL/, TMAX | S8, line 7 | Attested; finite time horizon implemented. |
| /EXEC/, BEGIN, SIMUL, END, /END/ | S8, lines 10–18 | Attested; only one SIMUL instruction implemented. |
| & comments | S9, historical listings | Line comments; ASCII identifiers are case-insensitive in QRUNAP2. Full lexical rules await S1. |
| INTEGER, REAL, :=, GET, PRINT | S8, lines 1, 11–14 | Historical, not implemented. |
| OPTION=SIMPAR, ESTIM=REPLI, ACCURACY | S8, lines 7–8 | Historical replication controls, not implemented. |
| FLAG, REF CUSTOMER, WAIT, UNSET, IF | S3, model listing | Historical algorithmic objects/control, not implemented. |
| FIFO scheduling | QRUNAP2 policy | Implemented runtime rule; no scheduling keyword accepted yet. |
| Other solvers | S1 abstract attests multiple analysis algorithms | Not implemented; no invented historical command names. |
