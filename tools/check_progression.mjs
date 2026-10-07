import test from 'node:test';
import assert from 'node:assert/strict';
import {readFile} from 'node:fs/promises';
import {Progression} from '../preview/progression.mjs';
import {PowerState} from '../preview/power-state.mjs';
const load=async path=>JSON.parse(await readFile(new URL(path,import.meta.url),'utf8'));
const design=await load('../assets/progression.json'),locales=await load('../assets/locales.json'),rules=await load('../assets/power-rules.json');
const font=await readFile(new URL('../assets/runtime/font.bin',import.meta.url));
const create=words=>new Progression(design,words);

test('Every player-facing key exists in both languages; design camera is 80%',()=>{
  assert.deepEqual(Object.keys(locales.it).sort(),Object.keys(locales.en).sort());
  for(const dictionary of Object.values(locales))for(const value of Object.values(dictionary)){
    assert.ok(value.length>0);
    for(const char of value.toUpperCase())if(char!==' '){const code=char.charCodeAt(0);assert.ok(font.subarray(code*7,code*7+7).some(Boolean),`Missing glyph ${char}`);}
  }
  for(const item of design.items)for(const dictionary of Object.values(locales))assert.ok(dictionary[item.key]);
  assert.equal(design.zoom,.8);assert.equal(480/design.zoom,600);
});
test('Legacy coins, skins and all-bonus duration upgrade migrate without loss',()=>{
  const p=create([100,200,7,2,1,1]);
  assert.deepEqual([p.best,p.bank,p.owned,p.skin,p.world,p.upgraded],[100,200,7,2,1,1]);
  assert.deepEqual(p.levels,Array(6).fill(0));assert.equal(p.words().length,32);
  assert.deepEqual(create(p.words()).words(),p.words());
});
test('Bad save values are bounded and locked equipment cannot be selected',()=>{
  const w=Array(32).fill(0xffffffff);w[2]=0;w[6]=2;
  const p=create(w);assert.equal(p.skin,0);assert.equal(p.world,0);
  assert.deepEqual(p.levels,Array(6).fill(3));assert.deepEqual(p.stock,[9,9,9]);
  assert.equal(p.language,1);assert.equal(p.relics,511);
  for(let i=0;i<3;i++)assert.ok(p.goals[i]<p.target(i));
  w.fill(0,16,19);assert.equal(create(w).launch,0,'Empty capsule slot cannot stay equipped after loading');
});
test('Purchases with insufficient coins and invalid selection do not mutate state',()=>{
  const p=create(),before=p.words();assert.equal(p.purchase(0),2);
  assert.equal(p.purchase(255),3);assert.deepEqual(p.words(),before);
});
test('All permanent upgrades have increasing exact prices, independent levels and a hard cap',()=>{
  for(let i=0;i<9;i++){
    const p=create(),item=design.items[i];p.bank=10000;let spent=0;
    for(let level=0;level<3;level++){
      const cost=item.cost+item.step*level;assert.equal(p.cost(item),cost);
      assert.equal(p.purchase(i),1);spent+=cost;assert.equal(p.level(item),level+1);
    }
    assert.equal(p.bank,10000-spent);const before=p.words();
    assert.equal(p.purchase(i),3);assert.deepEqual(p.words(),before);
  }
});
test('Each duration level changes only its own collected bonus; legacy upgrade still stacks',()=>{
  for(let id=1;id<=6;id++){
    const p=create();p.bank=10000;p.purchase(id-1);p.purchase(id-1);
    const state=new PowerState(rules);state.collect(id,true,p.bonusExtra(id));
    assert.equal(state.remaining(id),rules.powers[id-1].seconds+2+(id===5?1:2));
    const other=id===1?2:1;state.collect(other,false,p.bonusExtra(other));assert.equal(state.remaining(other),8);
  }
});
test('Capsules accumulate without auto-consumption, can be unequipped and consume exactly one armed supply',()=>{
  const p=create();p.bank=10000;
  for(let i=9;i<12;i++)for(let j=0;j<9;j++)assert.equal(p.purchase(i),1);
  const before=p.words();assert.equal(p.purchase(9),3);assert.deepEqual(p.words(),before);
  assert.equal(p.beginRun(),0);assert.deepEqual(p.stock,[9,9,9]);
  assert.equal(p.arm(1),true);assert.equal(p.arm(1),true);assert.equal(p.launch,0);
  assert.equal(p.arm(2),true);assert.equal(p.beginRun(),5);assert.deepEqual(p.stock,[9,9,8]);
  assert.equal(p.arm(8),false);
});
test('Last capsule clears armed slot while other supplies remain',()=>{
  const p=create();p.bank=100;p.purchase(10);p.purchase(9);p.arm(1);
  assert.equal(p.beginRun(),2);assert.equal(p.launch,0);assert.deepEqual(p.stock,[1,0,0]);
  assert.equal(p.beginRun(),0);assert.equal(p.arm(1),false);
});
test('Owned cosmetics can be equipped and removed without a second charge',()=>{
  const p=create();p.bank=1000;
  assert.equal(p.purchase(12),1);assert.equal(p.skin,1);assert.equal(p.bank,940);
  assert.equal(p.purchase(12),5);assert.equal(p.skin,0);assert.equal(p.bank,940);
  assert.equal(p.purchase(12),4);assert.equal(p.skin,1);assert.equal(p.bank,940);
  assert.equal(p.purchase(14),1);assert.equal(p.world,1);const bank=p.bank;
  assert.equal(p.purchase(14),5);assert.equal(p.world,0);assert.equal(p.bank,bank);
});
test('Short runs contribute to goals; completion rewards and relics are credited once',()=>{
  const p=create();const short={distance:300,coins:6,pickups:1,score:50};
  assert.deepEqual(p.bankRun(short),{earned:6,rewards:0,completed:0});
  assert.deepEqual(p.bankRun(short),{earned:6,rewards:60,completed:7});
  assert.equal(p.bank,72);assert.equal(p.relics,0b001001001);assert.deepEqual(p.tiers,[1,1,1]);
  assert.deepEqual(p.bankRun({distance:0,coins:0,pickups:0,score:0}),{earned:0,rewards:0,completed:0});
});
test('Goal surplus carries into next tier, with at most one payout per objective per run',()=>{
  const p=create();assert.equal(p.bankRun({distance:100000,coins:10000,pickups:1000,score:1}).rewards,60);
  assert.deepEqual(p.tiers,[1,1,1]);for(let i=0;i<3;i++)assert.equal(p.goals[i],p.target(i)-1);
});
test('Upgrades improve magnet range, dash recovery and final coin yield, with 32-word roundtrip',()=>{
  const p=create();p.bank=10000;for(let i=6;i<=8;i++)for(let j=0;j<3;j++)p.purchase(i);
  p.language=1;p.purchase(10);p.arm(1);
  assert.equal(p.magnetRadius(),185);assert.ok(Math.abs(p.dashSeconds()-2.1)<1e-9);
  assert.deepEqual(p.bankRun({distance:800,coins:16,pickups:3,score:236}),{earned:20,rewards:60,completed:7});
  assert.deepEqual(create(p.words()).words(),p.words());
});
test('Repeated normal runs unlock all nine collection pieces and preserve purchases across reloads',()=>{
  let p=create();for(let i=0;i<15;i++){p.bankRun({distance:800,coins:16,pickups:3,score:236+i});p=create(p.words());}
  assert.equal(p.relics,511);assert.equal(p.runs,15);assert.equal(p.totals[0],12000);
  assert.ok(p.bank>=design.items[0].cost);p.purchase(0);const saved=p.words();
  assert.deepEqual(create(saved).words(),saved);
});
