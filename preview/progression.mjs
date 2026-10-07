// Local model of source changes. No PSP/WASM compilation or execution.
export class Progression {
  constructor(design,words=[]){
    this.design=design;this.levels=Array(6).fill(0);this.skills=Array(3).fill(0);
    this.stock=Array(3).fill(0);this.launch=0;this.language=0;this.runs=0;
    this.totals=Array(3).fill(0);this.goals=Array(3).fill(0);this.tiers=Array(3).fill(0);this.relics=0;
    const w=Array(32).fill(0);words.slice(0,32).forEach((value,i)=>{if(Number.isInteger(value)&&value>=0&&value<=0xffffffff)w[i]=value;});
    [this.best,this.bank]=w;this.owned=w[2]&7;this.skin=Math.min(2,w[3]);
    if(this.skin&&!(this.owned&(1<<(this.skin-1))))this.skin=0;
    this.world=this.owned&4?Math.min(1,w[4]):0;this.upgraded=Math.min(1,w[5]);
    if(w[6]===2){
      this.levels=w.slice(7,13).map(v=>Math.min(3,v));this.skills=w.slice(13,16).map(v=>Math.min(3,v));
      this.stock=w.slice(16,19).map(v=>Math.min(9,v));this.launch=Math.min(3,w[19]);this.language=Math.min(1,w[20]);
      if(this.launch&&!this.stock[this.launch-1])this.launch=0;
      this.runs=w[21];this.totals=w.slice(22,25);this.tiers=w.slice(28,31);
      this.goals=w.slice(25,28).map((v,i)=>Math.min(v,this.target(i)-1));this.relics=w[31]&511;
    }
  }
  words(){return [this.best,this.bank,this.owned,this.skin,this.world,this.upgraded,2,...this.levels,...this.skills,...this.stock,this.launch,this.language,this.runs,...this.totals,...this.goals,...this.tiers,this.relics];}
  level(item){return item.kind==='duration'?this.levels[item.index]:item.kind==='skill'?this.skills[item.index]:item.kind==='capsule'?this.stock[item.index]:(this.owned>>item.index)&1;}
  cost(item){return item.cost+(['duration','skill'].includes(item.kind)?this.level(item)*item.step:0);}
  purchase(index){
    const item=this.design.items[index];if(!item)return 3;
    const level=this.level(item);
    if(item.kind==='cosmetic'&&level){
      if(item.index<2){const skin=item.index+1;this.skin=this.skin===skin?0:skin;return this.skin?4:5;}
      this.world=1-this.world;return this.world?4:5;
    }
    if(level>=item.max)return 3;
    const cost=this.cost(item);if(this.bank<cost)return 2;
    this.bank-=cost;
    if(item.kind==='duration')this.levels[item.index]++;
    else if(item.kind==='skill')this.skills[item.index]++;
    else if(item.kind==='capsule')this.stock[item.index]++;
    else{this.owned|=1<<item.index;if(item.index<2)this.skin=item.index+1;else this.world=1;}
    return 1;
  }
  arm(slot){if(slot<0||slot>=3||!this.stock[slot])return false;this.launch=this.launch===slot+1?0:slot+1;return true;}
  beginRun(){
    if(this.launch<1||this.launch>3)return 0;
    const slot=this.launch-1;if(!this.stock[slot]){this.launch=0;return 0;}
    this.stock[slot]--;const id=[1,2,5][slot];if(!this.stock[slot])this.launch=0;return id;
  }
  target(i){const m=this.design.missions[i];return m.target+(this.tiers[i]%4)*m.step;}
  reward(i){return this.design.missions[i].reward+Math.min(20,Math.floor(this.tiers[i]/4))*5;}
  bankRun({distance,coins,pickups,score}){
    const add=(a,b)=>Math.min(0xffffffff,a+b);this.runs=add(this.runs,1);this.best=Math.max(this.best,score);
    const stats=[distance,coins,pickups];let rewards=0,completed=0;
    for(let i=0;i<3;i++){
      this.totals[i]=add(this.totals[i],stats[i]);this.goals[i]=add(this.goals[i],stats[i]);
      const target=this.target(i);
      if(this.goals[i]>=target){
        rewards+=this.reward(i);this.relics|=1<<(i*3+(this.tiers[i]%3));
        this.tiers[i]=add(this.tiers[i],1);this.goals[i]=Math.min(this.goals[i]-target,this.target(i)-1);completed|=1<<i;
      }
    }
    const earned=add(coins,Math.floor(coins*this.skills[2]*this.design.coin_yield_percent/100));
    this.bank=add(add(this.bank,earned),rewards);return {earned,rewards,completed};
  }
  dashSeconds(){return 3-this.skills[1]*this.design.dash_recovery_step;}
  magnetRadius(){return 125+this.skills[0]*this.design.magnet_range_step;}
  bonusExtra(id){return this.levels[id-1]*(id===5?this.design.jet_duration_step:this.design.duration_step);}
}
