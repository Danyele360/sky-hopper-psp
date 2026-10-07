// Verifies the local review model, without compiling/executing Rust or WASM.
import test from 'node:test';
import assert from 'node:assert/strict';
import {readFile} from 'node:fs/promises';
import {PowerState} from '../preview/power-state.mjs';

const rules=JSON.parse(await readFile(new URL('../assets/power-rules.json',import.meta.url),'utf8'));
const create=()=>new PowerState(rules);
const active=state=>rules.powers.filter(p=>state.has(p.id)).map(p=>p.id);

test('All distinct pairs coexist except boost/jetpack, in both pickup orders',()=>{
  for(let first=1;first<=6;first++)for(let last=1;last<=6;last++){
    if(first===last)continue;
    const state=create();state.collect(first);state.collect(last);
    const conflict=(first===3&&last===5)||(first===5&&last===3);
    assert.deepEqual(active(state),conflict?[last]:[first,last].sort((a,b)=>a-b));
    assert.equal(state.suspended(4),state.has(4)&&state.has(5));
  }
});
test('Replacing either movement bonus preserves four compatible bonuses',()=>{
  for(const [first,last] of [[3,5],[5,3]]){
    const state=create();for(const id of [1,2,4,6,first])state.collect(id);
    state.tick(2);const before=[1,2,4,6].map(id=>state.remaining(id));
    state.collect(last);
    assert.deepEqual(active(state),[1,2,4,6,last].sort((a,b)=>a-b));
    assert.deepEqual([1,2,4,6].map(id=>state.remaining(id)),before);
  }
});
test('Repeated pickup refreshes its own duration without stacking strength/time',()=>{
  const state=create();state.collect(1);state.collect(2);state.tick(3);
  state.collect(1);state.collect(1);
  assert.equal(state.remaining(1),8);assert.equal(state.remaining(2),5);
});
test('Expiry and shield consumption preserve unrelated bonuses',()=>{
  const state=create();state.collect(1);state.tick(3);state.collect(2);state.collect(6);
  assert.equal(state.tick(5),1);assert.deepEqual(active(state),[2,6]);
  assert.equal(state.consume(2),true);assert.equal(state.consume(2),false);
  assert.deepEqual(active(state),[6]);assert.equal(state.remaining(6),3);
});
test('Super jump timer pauses during flight and resumes at the exact expiry boundary',()=>{
  const state=create();state.collect(4);state.collect(5);
  state.tick(2);assert.equal(state.remaining(4),8);assert.equal(state.remaining(5),2.5);
  assert.equal(state.effective(4),false);
  assert.equal(state.tick(3),16);assert.equal(state.remaining(4),7.5);
  assert.equal(state.effective(4),true);
});
test('Collecting super jump during flight retains its entire duration',()=>{
  const state=create();state.collect(5);state.tick(3);state.collect(4);
  state.tick(1.5);assert.equal(state.remaining(4),8);assert.equal(state.has(5),false);
  state.tick(1);assert.equal(state.remaining(4),7);
});
test('Upgrade extends each bonus and a weaker repeat never shortens an existing timer',()=>{
  const state=create();for(const id of [1,2,4,5,6])state.collect(id,true);
  assert.equal(state.remaining(5),6.5);assert.equal(state.remaining(1),10);
  state.collect(1,false);assert.equal(state.remaining(1),10);assert.equal(state.duration(1),10);
  assert.equal(state.tick(6.5),16);assert.equal(state.remaining(4),10);
  assert.equal(state.remaining(1),3.5);
});
test('Large timestep expires all bonuses including previously suspended super jump',()=>{
  const state=create();for(const id of [1,2,4,5,6])state.collect(id);
  assert.equal(state.tick(20),0b111011);assert.equal(state.mask(),0);
});
test('Pause/zero elapsed time and invalid inputs do not alter state',()=>{
  const state=create();state.collect(1);const before=Array.from(state.timers);
  for(const id of [0,7,255,NaN,1.5]){state.collect(id);assert.equal(state.has(id),false);}
  for(const dt of [0,-1,NaN,Infinity])assert.equal(state.tick(dt),0);
  assert.deepEqual(Array.from(state.timers),before);
});
