# QRUNAP2 user guide

[Lire en français](guide.fr.md)

Select an example, click **Run this example**, then edit the model to compare results. Simulation runs locally in your browser.

## 1. What problem does it solve?

Imagine a desk receiving requests. Requests arrive, wait if the desk is busy, receive service, then leave. You want to know how long they wait, how busy the desk is, and whether adding a second desk helps. The same ideas describe processors, disks, manufacturing stations or transport flows.

A **customer** is the thing being processed: a request, job, person or message. A **station** is a service facility with its waiting queue. A **server** is one unit of processing capacity within that station. Two servers can share a single queue. A **network** connects stations by routing customers between them.

QNAP2 used these objects together with an algorithmic language and execution controls. Historical programs could express considerably more than a sequence of queues: customer attributes, synchronization and programmed behavior also existed. See the [historical evidence](sources.md), especially S3 and S8.

## 2. Read a small model

This is an original teaching example in the supported subset, not a transcription from a manual.

```text
/DECLARE/
QUEUE INPUT, DESK;
/STATION/
NAME=INPUT;
TYPE=SOURCE;
SERVICE=EXP(2);
TRANSIT=DESK;
/STATION/
NAME=DESK;
TYPE=SERVER,SINGLE;
SERVICE=EXP(1);
TRANSIT=OUT;
/CONTROL/
TMAX=10000;
/EXEC/
BEGIN
SIMUL;
END;
/END/
```

| Read this | Understand this |
| --- | --- |
| /DECLARE/ QUEUE INPUT, DESK; | Give names to the network's stations. QUEUE names the station object, not just the people waiting. |
| /STATION/ NAME=INPUT; | Start the description of INPUT. |
| TYPE=SOURCE; | Generate customers from outside the network. |
| SERVICE=EXP(2); on INPUT | Wait an exponentially distributed interval of mean 2 between arrivals. |
| TRANSIT=DESK; | Send each new customer to DESK. |
| TYPE=SERVER,SINGLE; | DESK can serve one customer at a time. Others wait. |
| SERVICE=EXP(1); on DESK | Each service has an exponential duration of mean 1. |
| TRANSIT=OUT; | A completed customer leaves the network. |
| /CONTROL/ TMAX=10000; | End the experiment after 10,000 units of simulated time. |
| /EXEC/ BEGIN SIMUL; END; | Run the simulation. |
| /END/ | Finish the input program. |

`EXP(2)` means a **mean duration of 2**, not two arrivals per unit time. Its corresponding rate is 1/2. The mean convention is explicitly described in Place's thesis, printed III-84 ([S9](sources.md)); the full 9.3 manual still needs verification. `CST(2)` means a fixed duration of 2. Use one consistent time unit throughout: seconds, minutes or another unit you choose.

For a SOURCE, SERVICE describes the spacing of arrivals. For a SERVER, it describes processing time. That distinction is essential when rereading an old model.

## 3. What happens when you click Run?

Rust parses and checks the text, builds the network, then advances a virtual clock from event to event. It does not wait 10,000 real seconds. An event is an arrival or a service completion. The engine records occupied servers, queue sizes and completed visits as it goes.

The seed controls the random draws. Repeat with the same seed to reproduce the experiment; change it to obtain another realization. QRUNAP2 uses a new random generator, so the same seed cannot recreate an old QNAP2 run.

## 4. Read the results

| Quantity | Plain-language meaning |
| --- | --- |
| Utilization | Fraction of available server time spent working. |
| L | Average number in the station, including those being served. |
| Lq | Average number waiting, excluding those being served. |
| W | Average time at the station for completed visits: waiting plus service. |
| Wq | Average waiting time for those completed visits. |
| Throughput | Completed visits per unit time. |

In the first example, the theoretical long-run utilization is 50%, L is 1, and W is 2 time units. Simulation results fluctuate. Waiting can occur at a station with spare average capacity because arrivals and services are irregular.

These reports include the startup period. Visits unfinished at the horizon do not enter W or Wq. There are no confidence intervals yet; a single run is an experiment, not a precision guarantee. See [measurement definitions](semantics.md).

## 5. Five experiments to rebuild intuition

| Example | What to try | What to notice |
| --- | --- | --- |
| First queue | Run unchanged, then change the seed. | Similar average behavior, different random paths. |
| Constant durations | Arrival CST(2), service CST(1), horizon 20. | No waiting: 10 arrivals, 9 exits, one customer still in service. |
| Two servers | A shared queue with MULTIPLE(2), arrivals EXP(0.75). | Capacity is two simultaneous services, not twice the speed of each service. |
| Overloaded desk | Arrivals EXP(0.8), one server with EXP(1). | Demand exceeds capacity. The queue grows; a longer run does not create equilibrium. |
| INRIA 1995, adapted | 40 customers circulate A → B → A. | A closed network needs initial customers, not an external source. Population stays 40. |

The last example is based on the topology of an actual [INRIA execution listing](https://www-sop.inria.fr/sloop/qnaprep/output.html). Its interactive input and parallel replications have been replaced; the [change record](sources.md) explains the adaptation. It is not an unchanged historical program.

## 6. The historical language has three layers

Keep these layers separate when opening an old QNAP2 file:

| Layer | Its role | Reading cues |
| --- | --- | --- |
| Network description | Declare objects and describe stations. | QUEUE, NAME, TYPE, SERVICE, TRANSIT |
| Algorithmic behavior | Compute values and describe behavior beyond a simple service duration. | Variables, assignment, conditions, blocks, customer references and synchronization |
| Execution controls | Prepare and launch an experiment, interact with the user and request results. | /CONTROL/, /EXEC/, GET, PRINT, SIMUL |

In the historical INRIA listing, `=` introduces station/control properties while `:=` assigns a value inside executable instructions. INTEGER and REAL declare numeric variables; BEGIN and END delimit an instruction block. GET reads a value and PRINT displays information. These are useful reading cues, but **variable declarations, assignments and interactive instructions are not supported by this playground yet**. See [S8](sources.md).

A historical SERVICE could contain an algorithm instead of just a duration. A customer reference identifies a particular customer; flags support synchronization between activities. The urban-flow program illustrates this richer style ([S3](sources.md)). Those mechanisms need their own semantics; replacing them with fixed delays would change the model.

Simulation produces observations from generated event histories. An analytical method computes results from mathematical equations under particular assumptions. Historical QNAP2 offered both; this QRUNAP2 release implements simulation only. A mathematical formula shown in the guide is a teaching reference, not an implemented historical solver.

## 7. What has been adapted in QRUNAP2?

Historical spellings remain visible: QUEUE, STATION, SERVICE, TRANSIT and SIMUL. The implementation is new Rust code. That same core runs from the command line or as WebAssembly inside a browser worker. GitHub Pages serves static files; no simulation server or account is required for visitors.

The web editor, seed field, JSON export, English/French interface, resource limits and error messages are modern tooling. FIFO ordering, source startup and simultaneous-event ordering are explicit QRUNAP2 policies while historical details are investigated.

This release handles small open and closed networks, constant/exponential durations and single/multiple servers. Variables, loops, IF, references, flags, customer classes, probabilistic routes and the historical analytical solvers are not implemented. Unsupported input produces an error. Consult the [compatibility matrix](compatibility.md) before pasting an old program.

## 8. A practical way to resume

Start with the first queue, change only the arrival interval, and watch utilization and waiting. Then add a second server. Finally, explore the closed historical network and slow down B by increasing its service mean. Explain each result in ordinary words before changing another parameter. That is enough to recover the central modeling ideas before tackling historical algorithms and synchronization.
