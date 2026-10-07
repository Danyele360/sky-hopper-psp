// Local review model. Its rules are shared with the Rust source via the JSON file.
// Running this model does not execute or rebuild the PSP/WASM game.
export class PowerState {
  constructor(rules) {
    this.rules=rules;
    this.timers=new Float64Array(6);
    this.durations=new Float64Array(6);
  }
  valid(id){return Number.isInteger(id)&&id>=1&&id<=6;}
  has(id){return this.remaining(id)>0;}
  remaining(id){return this.valid(id)?this.timers[id-1]:0;}
  duration(id){return this.valid(id)?this.durations[id-1]:0;}
  mask(){return this.rules.powers.reduce((mask,p)=>mask|(this.has(p.id)?1<<(p.id-1):0),0);}
  suspended(id){return this.has(id)&&this.rules.powers[id-1].suspended_by.some(other=>this.has(other));}
  effective(id){return this.has(id)&&!this.suspended(id);}
  collect(id,upgraded=false,extraSeconds=0){
    if(!this.valid(id))return;
    const rule=this.rules.powers[id-1];
    for(const other of rule.incompatible)this.consume(other);
    const duration=rule.seconds+(upgraded?this.rules.upgrade_seconds:0)+(Number.isFinite(extraSeconds)?Math.min(3,Math.max(0,extraSeconds)):0);
    this.timers[id-1]=Math.max(this.timers[id-1],duration);
    this.durations[id-1]=this.timers[id-1];
  }
  consume(id){
    if(!this.has(id))return false;
    this.timers[id-1]=0;this.durations[id-1]=0;return true;
  }
  tick(dt){
    if(!Number.isFinite(dt)||dt<=0)return 0;
    const before=Array.from(this.timers);let expired=0;
    for(const rule of this.rules.powers){
      const index=rule.id-1;if(before[index]<=0)continue;
      const heldFor=Math.max(0,...rule.suspended_by.map(other=>before[other-1]));
      const elapsed=Math.max(0,dt-Math.min(dt,heldFor));
      this.timers[index]=Math.max(0,before[index]-elapsed);
      if(this.timers[index]===0){this.durations[index]=0;expired|=1<<index;}
    }
    return expired;
  }
}
