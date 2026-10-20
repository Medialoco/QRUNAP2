"""Reproduce manuscript measurements; Python standard library only.
Run from repository root: python3 verification/sne-2026/run_native.py
No engine changes, no warm-up removal, descriptive sample SD only.
"""
import csv, json, pathlib, platform, statistics, subprocess, math
root=pathlib.Path(__file__).resolve().parents[2]
out=pathlib.Path(__file__).resolve().parent/'native-results'; out.mkdir(exist_ok=True)
subprocess.run(['cargo','build','--release','--locked'],cwd=root,check=True)
exe=root/'target/release/qrunap2'
configs=[]
for rate in [.25,.5,.8,.95]: configs.append((f'mm1-{rate}',rate,1))
configs.append(('mm2',4/3,2))
rows=[]; summaries=[]
for name,rate,cap in configs+[('closed40',0,1)]:
    if name=='closed40':
        src=(root/'examples/historical/inria-1995-adapted.qnp').read_text().replace('TMAX=1000;','TMAX=200000;')
        ref={'mean_number':20.,'mean_queue':20-40/41,'utilization':40/41,'throughput':40/41,'mean_response':20.5}
        station='A'
    else:
        src=f'''/DECLARE/ QUEUE INPUT, DESK;
/STATION/ NAME=INPUT; TYPE=SOURCE; SERVICE=EXP({1/rate:.16f}); TRANSIT=DESK;
/STATION/ NAME=DESK; TYPE=SERVER,MULTIPLE({cap}); SERVICE=EXP(1); TRANSIT=OUT;
/CONTROL/ TMAX=200000;
/EXEC/ BEGIN SIMUL; END;
/END/
'''
        rho=rate/cap; a=rate
        p0=1/(sum(a**k/math.factorial(k) for k in range(cap))+a**cap/(math.factorial(cap)*(1-rho)))
        pw=a**cap*p0/(math.factorial(cap)*(1-rho))
        lq=pw*rho/(1-rho)
        ref={'mean_number':lq+a,'mean_queue':lq,'utilization':rho,'throughput':rate,'mean_response':1+lq/rate}
        station='DESK'
    model=out/f'{name}.qnp'; model.write_text(src)
    samples=[]
    for seed in range(1,33):
        data=json.loads(subprocess.check_output([str(exe),'run',str(model),str(seed)]))
        (out/f'{name}-seed{seed:02}.json').write_text(json.dumps(data,indent=2)+'\n')
        assert data['initial']+data['generated']==data['exited']+data['remaining']
        if name=='closed40':
            assert data['remaining']==40
            assert abs(sum(s['mean_number'] for s in data['stations'])-40)<1e-9
        sample=next(s for s in data['stations'] if s['name']==station)
        rows.append(dict(model=name,seed=seed,**{k:sample[k] for k in ref}))
        samples.append(sample)
    summary={'model':name,'n':32,'reference':ref,'mean':{},'sd':{}}
    for key in ref:
        summary['mean'][key]=statistics.mean(s[key] for s in samples)
        summary['sd'][key]=statistics.stdev(s[key] for s in samples)
    summaries.append(summary)
with (out/'measurements.csv').open('w') as f:
    writer=csv.DictWriter(f,fieldnames=rows[0].keys(),lineterminator="\n");writer.writeheader();writer.writerows(rows)
(out/'summary.json').write_text(json.dumps(summaries,indent=2)+'\n')
(out/'environment.json').write_text(json.dumps({'commit':subprocess.check_output(['git','rev-parse','HEAD'],cwd=root,text=True).strip(),'rustc':subprocess.check_output(['rustc','-Vv'],text=True),'platform':platform.platform(),'python':platform.python_version(),'horizon':200000,'seeds':list(range(1,33)),'warmup':0,'note':'Descriptive SD; distinct seeds do not establish independent random substreams.'},indent=2)+'\n')
for s in summaries:
    print(s['model'], '; '.join(f'{k}={s["mean"][k]:.6f} SD={s["sd"][k]:.6f} ref={s["reference"][k]:.6f}' for k in ['mean_number','mean_response','utilization']))
