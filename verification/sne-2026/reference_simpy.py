"""Independent SimPy Resource/process models for QRUNAP2 verification.
No QRUNAP2 parser, calendar, statistics routine or random stream is reused.
Includes all events at T via peek/step, unlike run(until=T).
"""
import json, math, pathlib, random, statistics, subprocess, sys, platform
import simpy
ROOT=pathlib.Path(__file__).resolve().parents[2]
OUT=pathlib.Path(__file__).parent/'reference-results'; OUT.mkdir(exist_ok=True)
KEYS=['mean_number','mean_queue','utilization','throughput','mean_wait','mean_response']

class Station:
    def __init__(self,env,capacity,duration):
        self.env=env; self.r=simpy.Resource(env,capacity=capacity); self.duration=duration
        self.n=0;self.busy=0;self.last=0.;self.area=0.;self.barea=0.;self.done=0;self.wait=0.;self.response=0.
    def update(self,t):
        self.area+=(t-self.last)*self.n; self.barea+=(t-self.last)*self.busy;self.last=t
    def visit(self):
        self.update(self.env.now);self.n+=1;arrival=self.env.now
        with self.r.request() as req:
            yield req
            self.update(self.env.now);self.busy+=1; wait=self.env.now-arrival
            yield self.env.timeout(self.duration())
            self.update(self.env.now); self.busy-=1; self.n-=1; self.done+=1
            self.wait+=wait; self.response+=self.env.now-arrival
    def report(self,T):
        self.update(T)
        return dict(mean_number=self.area/T,mean_queue=(self.area-self.barea)/T,
                    utilization=self.barea/T/self.r.capacity,throughput=self.done/T,
                    mean_wait=self.wait/self.done if self.done else None,
                    mean_response=self.response/self.done if self.done else None,
                    completed=self.done,remaining=self.n)

def simulate(T,arrival=None,capacity=1,service=None,closed=False,population=40):
    env=simpy.Environment(); stats={'initial':population if closed else 0,'generated':0,'exited':0}
    stations=[Station(env,capacity,service)]
    if closed: stations.append(Station(env,1,service))
    def client():
        while True:
            for station in stations: yield from station.visit()
            if not closed: stats['exited']+=1;return
    def source():
        while True:
            yield env.timeout(arrival());stats['generated']+=1;env.process(client())
    if closed:
        for _ in range(population): env.process(client())
    else: env.process(source())
    while env.peek()<=T: env.step()
    result=[station.report(T) for station in stations]
    stats['remaining']=sum(r['remaining'] for r in result)
    assert stats['initial']+stats['generated']==stats['exited']+stats['remaining']
    return dict(**stats,stations=result)

def deterministic():
    results=[]
    # Exact finite-horizon cases, deliberately including time ties and overload.
    for name,ia,sv,cap,T in [
        ('dd1-light',2,1,1,20),('dd1-ties',1,1,1,20),
        ('dd1-overload',1,2,1,20),('dd2-full',1,2,2,20),
        ('dd2-overload',1,3,2,20),('dd3',1,2,3,20),
        ('dd1-before-arrival',2,1,1,1),('dd1-at-completion',2,1,1,3)]:
        source=f'''/DECLARE/ QUEUE INPUT, DESK;
/STATION/ NAME=INPUT; TYPE=SOURCE; SERVICE=CST({ia}); TRANSIT=DESK;
/STATION/ NAME=DESK; TYPE=SERVER,MULTIPLE({cap}); SERVICE=CST({sv}); TRANSIT=OUT;
/CONTROL/ TMAX={T}; /EXEC/ BEGIN SIMUL; END; /END/'''
        model=OUT/f'{name}.qnp';model.write_text(source+'\n')
        native=json.loads(subprocess.check_output([str(ROOT/'target/release/qrunap2'),'run',str(model),'42']))
        ref=simulate(T,lambda:ia,cap,lambda:sv)
        for key in ['initial','generated','exited','remaining']:assert native[key]==ref[key],(name,key)
        for key in KEYS+['completed','remaining']:
            a=native['stations'][1][key];b=ref['stations'][0][key]
            assert a==b or (a is not None and b is not None and abs(a-b)<=1e-10*max(1,abs(a))),(name,key,a,b)
        results.append({'case':name,'qrunap2':native['stations'][1],'simpy':ref,'pass':True})
    (OUT/'deterministic.json').write_text(json.dumps(results,indent=2)+'\n')
    print('PASS 8 deterministic cross-engine cases',flush=True)

def stochastic():
    rows=[];summaries=[]
    configs=[(f'mm1-{r}',r,1,False) for r in [.25,.5,.8,.95]]+[('mm2',4/3,2,False),('closed40',0,1,True)]
    for name,rate,cap,closed in configs:
        samples=[]
        for seed in range(1,33):
            rng=random.Random(seed) # Python MT, intentionally different from QRUNAP2 xorshift32.
            report=simulate(200000,lambda:rng.expovariate(rate),cap,lambda:rng.expovariate(1),closed)
            if closed: assert abs(sum(s['mean_number'] for s in report['stations'])-40)<1e-9
            (OUT/f'{name}-seed{seed:02}.json').write_text(json.dumps(report,indent=2)+'\n')
            sample=report['stations'][0];samples.append(sample);rows.append(dict(model=name,seed=seed,**sample))
        summary={'model':name,'n':32,'mean':{k:statistics.mean(s[k] for s in samples) for k in KEYS},'sd':{k:statistics.stdev(s[k] for s in samples) for k in KEYS}}
        summaries.append(summary);print(name,json.dumps(summary),flush=True)
    (OUT/'summary.json').write_text(json.dumps(summaries,indent=2)+'\n')
    (OUT/'measurements.json').write_text(json.dumps(rows,indent=2)+'\n')
    (OUT/'environment.json').write_text(json.dumps({'python':sys.version,'simpy':simpy.__version__,'platform':platform.platform(),'T':200000,'seeds':list(range(1,33)),'random':'random.Random (MT19937), expovariate; distinct seeds are not guaranteed independent substreams.','warmup':0},indent=2)+'\n')

if __name__=='__main__':
    deterministic()
    if '--deterministic-only' not in sys.argv: stochastic()
