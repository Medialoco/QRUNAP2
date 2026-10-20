# Sources and evidence

[Français : synthèse](guide.fr.md) · [English guide](guide.md)

Access checked on 19 October 2026. Historical versions differ. This register records evidence, not a claim of full QNAP2 9.3 conformance. No proprietary manual or implementation is redistributed.

| ID | Reference and location | Access and use |
| --- | --- | --- |
| S1 | Simulog, QNAP2 Version 9.3 User's Guide (1996), [Manualzilla](https://manualzilla.com/doc/6892832/qnap2-version-9.3-user-s-guide), [alternate mirror](https://manualzz.com/doc/4169276/simulog-qnap2-software-user-s-guide) | Landing page and abstract available; full viewer unavailable, PDF endpoint returned 404, alternate mirror inaccessible. Exact 9.3 rules remain unverified. No local archive. |
| S2 | M. Véran and D. Potier, QNAP2: A Portable Environment for Queueing Systems Modelling, INRIA RR-0314 (1984), [HAL](https://hal.inria.fr/inria-00076243) | Retrieval failed. Bibliographic entry from project plan; not used to establish syntax. |
| S3 | L. Pasini and S. Feliziani, Networks of Queues for the Simulation of Urban Flow Systems: An Intermodal Traffic System (2004), [author-uploaded paper](https://www.researchgate.net/publication/242279825_NETWORKS_OF_QUEUES_FOR_THE_SIMULATION_OF_URBAN_FLOW_SYSTEMS_AN_INTERMODAL_TRAFFIC_SYSTEM), model listing | Full text accessible. Attests SOURCE, SERVER,MULTIPLE(20), algorithmic SERVICE and synchronization. QNAP2 version unspecified. No redistribution of the full program. |
| S4 | Carleton QNAP2 [ShortGuide](https://www.sce.carleton.ca/courses/94511/handouts03/qnap-instructions/ShortGuide) | Retrieval failed. Not used as evidence. |
| S8 | Philippe Mussi / INRIA SLOOP, [QNAPREP toy example output](https://www-sop.inria.fr/sloop/qnaprep/output.html), 1995, source lines 1–18 and result table | Accessible first-party execution listing, QNAP2 V9.Rb (1993). One complete historical program located. An independently written topology reconstruction is provided, with changes recorded below. Original license not stated. |
| S9 | Jean-Marie Place, Les performances d'une architecture d'ordinateur destinée au support de la communication par message, doctoral thesis, Lille (1990), [university PDF](https://pepite-depot.univ-lille.fr/LIBRE/Th_Num/1990/50376-1990-23.pdf), printed III-84–85 | Downloaded temporarily for text inspection; not included in repository. III-84 explicitly explains EXP's mean parameter and CST's fixed duration. Listing on III-85 identifies QNAP2 V03 (October 1984). Corroboration across versions is still needed for 9.3. |
| S10 | [A Lexical and Syntactical Analyzer for the Exporting of QNAP to PMIF](https://www.researchgate.net/publication/366446100_A_Lexical_and_Syntactical_Analyzer_for_the_Exporting_of_QNAP_to_the_Performance_Model_Interchange_Format_PMIF), grammar §3 | Research implementation grammar corroborates station capacities and OUT. This is a restricted exporter grammar, not the QNAP2 specification. Publication metadata/version require further checking. |

## Historical example and adaptation

The INRIA listing describes a closed network with 40 customers, two stations, and exponential services. The downloadable example reconstructs that topology. It sets both service means to 1, adds explicit server types, and retains a horizon of 1,000. Interactive variable input and parallel replication controls are replaced by one seeded run. It is **not the unchanged original program**, and its results must not be compared bit for bit with INRIA's replicated output.

Only one complete historical program has been identified and inspected for this release. The three complete historical fixtures requested by M0 have not been achieved. The other runnable examples are original teaching models, explicitly labelled as such. S3 and S9 demonstrate richer historical behavior which this release does not execute.

## Outstanding research

Recover the full 9.3 user guide and reference manual; confirm defaults, source activation, event ties and statistics; obtain redistributable complete historical models; document exact solver commands and admissibility. A broken source is an open research item, not evidence of absence.
