// No-build review model: source rules + existing graphics; not Rust physics.
import {PowerState} from './power-state.mjs';
import {Progression} from './progression.mjs';
import {t,language,setLanguage,onLanguageChange,translateDOM} from './i18n.mjs';
const loadImage=async path=>{const image=new Image();image.src=path;await image.decode();return image;};
const [sky,robot,world,font,rules,design]=await Promise.all([
  loadImage('../assets/runtime/sky.png'),loadImage('../assets/runtime/robot.png'),
  loadImage('../assets/runtime/world.png'),fetch('../assets/runtime/font.bin').then(r=>r.arrayBuffer()).then(b=>new Uint8Array(b)),
  fetch('../assets/power-rules.json').then(r=>r.json()),fetch('../assets/progression.json').then(r=>r.json()),
]);
let words=[];try{const saved=JSON.parse(localStorage.getItem('sky-hopper-review-v2'));if(Array.isArray(saved))words=saved;}catch{}
const profile=new Progression(design,words);profile.language=language==='en'?1:0;
let powers=new PowerState(rules),powerClock=false;
function tint(source,variant,background=false){
  const canvas=document.createElement('canvas');canvas.width=source.width;canvas.height=source.height;
  const ctx=canvas.getContext('2d');ctx.drawImage(source,0,0);
  const image=ctx.getImageData(0,0,canvas.width,canvas.height),d=image.data;
  for(let i=0;i<d.length;i+=4){
    if(background){d[i]=Math.min(255,Math.floor(d[i]/2)+Math.floor(d[i+2]/3));d[i+1]=Math.floor(d[i+1]*2/3);}
    else if(d[i+2]>d[i]){
      d[i]=Math.min(255,d[i]+(variant===1?80:120));
      if(variant===1)d[i+1]=Math.floor(d[i+1]*3/4);
      else{d[i+1]=Math.min(255,d[i+1]+30);d[i+2]=Math.floor(d[i+2]/2);}
    }
  }
  ctx.putImageData(image,0,0);return canvas;
}
const robotSkins=[robot,tint(robot,1),tint(robot,2)],nebulaSky=tint(sky,1,true);
const platforms=[
  {x:0,y:210,w:240,kind:0}, {x:275,y:189,w:148,kind:0},
  {x:463,y:168,w:137,kind:1}, {x:643,y:146,w:143,kind:0},
  {x:827,y:123,w:125,kind:0,hazard:true},
  {x:1002,y:105,w:143,kind:2}, {x:1193,y:67,w:137,kind:1,enemy:true},
  {x:1371,y:46,w:141,kind:0},
];
const runner={x:678,y:118};
let zoom=design.zoom,pose=0,animated=false,clock=0,last=performance.now();
let screen='playing',shop=0,notice=0,menuIndex=0,lastResult=null;
const views=[...document.querySelectorAll('[data-view]')];
const selected=document.querySelector('#selected'),native=document.querySelector('#native');

function glyphs(ctx,x,y,text,color='#ecf9ff',scale=1){
  ctx.fillStyle=color;
  for(const ch of text.toUpperCase()){
    const code=ch.charCodeAt(0);
    for(let row=0;row<7;row++)for(let col=0;col<5;col++)if(font[code*7+row]&(1<<(4-col)))ctx.fillRect(x+col*scale,y+row*scale,scale,scale);
    x+=6*scale;
  }
}
function render(canvas,z,view='playing'){
  const ctx=canvas.getContext('2d',{alpha:false});ctx.imageSmoothingEnabled=false;
  ctx.setTransform(1,0,0,1,0,0);ctx.drawImage(profile.world?nebulaSky:sky,0,0);
  // Anchor the runner at the same screen position in every view, revealing more
  // route ahead rather than shrinking a screenshot and adding empty borders.
  const originX=115-runner.x*z,originY=210-(runner.y+28)*z;
  ctx.setTransform(z,0,0,z,originX,originY);
  function sprite(image,index,x,y,w,h){const cell=image===robot?48:64;ctx.drawImage(image,index%4*cell,Math.floor(index/4)*cell,cell,cell,x,y,w,h);}
  for(const p of platforms){
    const worldClock=clock*(powers.effective(6)?.35:1);
    const top=p.y+(p.kind===1?Math.sin(worldClock*1.4)*8:0);
    const frame=p.kind===1?1:p.kind===2?2:0;
    sprite(world,frame,p.x,top-(p.kind===2?25:4),p.w,p.kind===2?83:p.kind===1?58:75);
    if(p.hazard)sprite(world,7,p.x+p.w-45,top-22,35,24);
    if(p.enemy){
      const ex=p.x+p.w/2+Math.sin(worldClock*2)*22,ey=top-58+Math.sin(worldClock*3)*10;
      sprite(world,14,ex-19,ey-18,38,36);
    }
    if(p.x>runner.x)for(let i=0;i<4;i++){
      const frame=[4,5,6,5][Math.floor(clock*8+i)%4];
      sprite(world,frame,p.x+18+i*24-8,top-40-Math.sin(i*1.04)*14,16,20);
    }
  }
  const powerX=platforms[4].x+platforms[4].w*0.56;
  sprite(world,9,powerX-17,platforms[4].y-61+Math.sin(clock*3)*3,34,34);
  sprite(world,3,platforms[5].x+platforms[5].w*.65-14,platforms[5].y-25,28,28);
  const frame=powers.effective(5)?12:powers.effective(3)?13:animated&&pose===0?Math.floor(clock*12)%4:pose;
  const lift=frame===5||frame===6?56:frame===12?80:0;
  const cx=runner.x+12,cy=runner.y+14-lift;
  function ring(radius,color){ctx.strokeStyle=color;ctx.lineWidth=2;ctx.beginPath();ctx.arc(cx,cy,radius,0,2*Math.PI);ctx.stroke();}
  if(powers.effective(2)){ring(25,'#85ebffc8');ring(28,'#59d4ff46');}
  if(powers.effective(1)){const r=29+Math.sin(clock*8)*4;ring(r,'#9a7effb4');ring(r+12,'#70d5ff41');}
  if(powers.effective(4))ring(22,'#d581ff96');
  const selectedRobot=robotSkins[profile.skin];
  ctx.drawImage(selectedRobot,frame%4*48,Math.floor(frame/4)*48,48,48,runner.x-12,runner.y-18-lift,48,48);
  ctx.setTransform(1,0,0,1,0,0);
  if(powers.effective(6)){ctx.fillStyle='rgba(72,170,220,.094)';ctx.fillRect(0,0,480,272);}

  drawHUD(ctx);
  if(view==='paused')drawPause(ctx);
  if(view==='shop')drawStore(ctx);
  if(view==='journal')drawJournal(ctx);
  if(view==='menu')drawMenu(ctx);
  if(view==='help')drawHelp(ctx);
  if(view==='result')drawResult(ctx);
}
const colors={white:'#ecf9ff',cyan:'#5bdfff',gold:'#ffdd64',gray:'#9bacc2',red:'#ff706b'};
function panel(ctx,x,y,w,h){ctx.fillStyle='rgba(4,15,37,.88)';ctx.fillRect(x,y,w,h);ctx.strokeStyle='#3694d1';ctx.strokeRect(x+.5,y+.5,w-1,h-1);}
function icon(ctx,id,x,y,size=18){const index=id+7;ctx.drawImage(world,index%4*64,Math.floor(index/4)*64,64,64,x,y,size,size);}
function center(ctx,y,text,scale=1,color=colors.white){glyphs(ctx,(480-text.length*6*scale)/2,y,text,color,scale);}
function overlay(ctx){ctx.fillStyle='rgba(4,17,39,.86)';ctx.fillRect(0,0,480,272);panel(ctx,8,8,464,248);}
function wrap(ctx,x,y,text,columns,color=colors.cyan){
  let line='',row=0;for(const word of text.split(' ')){
    if(line&&line.length+word.length+1>columns){glyphs(ctx,x,y+row++*12,line,color);line='';}
    line+=(line?' ':'')+word;
  }glyphs(ctx,x,y+row*12,line,color);
}
function drawHUD(ctx){
  panel(ctx,6,6,104,18);glyphs(ctx,11,12,`${t('score')} 00236`);
  panel(ctx,404,6,70,18);ctx.drawImage(world,0,64,64,64,408,7,12,15);glyphs(ctx,423,12,'5',colors.gold);
  panel(ctx,6,28,46,18);glyphs(ctx,10,33,'L/R',colors.cyan);ctx.fillStyle=colors.cyan;ctx.fillRect(32,32,15,8);
  const active=rules.powers.filter(rule=>powers.has(rule.id));
  active.forEach((rule,slot)=>{
    const x=474-active.length*35+slot*35,remaining=powers.remaining(rule.id),held=powers.suspended(rule.id);
    const color=held?colors.gray:remaining<=3?(remaining<1.5&&Math.floor(clock*6)%2===0?colors.red:colors.gold):colors.cyan;
    panel(ctx,x,28,32,22);icon(ctx,rule.id,x+1,30);
    glyphs(ctx,x+20,35,held?'II':String(Math.ceil(remaining)),color);
    ctx.fillStyle='#1d3c5b';ctx.fillRect(x+2,47,28,2);ctx.fillStyle=color;
    ctx.fillRect(x+2,47,Math.floor(Math.min(1,remaining/powers.duration(rule.id))*28),2);
  });
}
function drawPause(ctx){
  ctx.fillStyle='rgba(3,10,26,.6)';ctx.fillRect(0,0,480,272);panel(ctx,74,30,332,216);
  center(ctx,42,t('pause'),2);center(ctx,65,`${t('height')} 9    ${t('score')} 236`,1,colors.gold);
  glyphs(ctx,90,87,t('active_bonus'),colors.cyan);
  const active=rules.powers.filter(p=>powers.has(p.id));
  active.forEach((p,i)=>{const y=102+i*16;icon(ctx,p.id,90,y-3,15);glyphs(ctx,111,y,t(`power_${p.id}`));glyphs(ctx,258,y,powers.suspended(p.id)?t('suspended'):`${powers.remaining(p.id).toFixed(1)} s`,colors.cyan);});
  if(!active.length)glyphs(ctx,90,109,t('no_bonus'));
  center(ctx,193,t('help_conflict'),1,colors.gray);center(ctx,213,t('resume'),1,colors.cyan);center(ctx,230,t('leave'));
}
function description(item){return item.kind==='duration'?(item.index===4?'jet_desc':'duration_desc'):item.kind==='skill'?['magnet_desc','dash_desc','coin_desc'][item.index]:item.kind==='capsule'?'capsule_desc':'cosmetic_desc';}
function effect(item){
  const level=profile.level(item);
  if(item.kind==='duration')return `+${(level*(item.index===4?design.jet_duration_step:design.duration_step)).toFixed(1)} s`;
  if(item.kind==='skill')return [String(profile.magnetRadius()),`${profile.dashSeconds().toFixed(1)} s`,`+${profile.skills[2]*10}%`][item.index];
  if(item.kind==='capsule')return `${level} / ${item.max}`;
  return t(level?'unlocked':'locked');
}
function cosmeticAction(item){return t((item.index<2?profile.skin===item.index+1:profile.world===1)?'unequip':'equip');}
function drawStore(ctx){
  overlay(ctx);glyphs(ctx,20,21,t('store'),colors.white,2);glyphs(ctx,314,26,`${t('coins')} ${profile.bank}`,colors.gold);
  const item=design.items[shop],category=item.category;
  ['upgrades','stock','style'].forEach((key,i)=>{const x=20+i*147;ctx.fillStyle=i===category?'#184d7b':'#0a2036';ctx.fillRect(x,43,137,18);glyphs(ctx,x+7,48,t(key),i===category?colors.white:colors.cyan);});
  const [start,end]=[[0,9],[9,12],[12,15]][category],first=start+Math.max(0,shop-start-4);
  for(let i=first;i<Math.min(end,first+5);i++){
    const y=75+(i-first)*26,it=design.items[i];if(i===shop){ctx.fillStyle='#184d7b';ctx.fillRect(18,y-6,245,23);}
    glyphs(ctx,24,y,t(it.key));glyphs(ctx,216,y,`${profile.level(it)}/${it.max}`,profile.level(it)===it.max?colors.cyan:colors.gold);
  }
  if(end-start>5)glyphs(ctx,22,208,`${shop-start+1} / ${end-start}`,colors.cyan);
  ctx.fillStyle='#3694d1';ctx.fillRect(274,71,1,145);
  wrap(ctx,286,76,t(item.key),28,colors.white);wrap(ctx,286,102,t(description(item)),28);
  glyphs(ctx,286,143,effect(item),colors.gold,2);
  const full=profile.level(item)>=item.max;
  glyphs(ctx,286,170,item.kind==='cosmetic'&&full?cosmeticAction(item):full?t('max'):`${profile.cost(item)} ${t('coins')}`,!full&&profile.bank<profile.cost(item)?colors.red:colors.cyan);
  if(item.kind==='capsule')glyphs(ctx,286,187,`L/R ${t(profile.launch===item.index+1?'unequip_short':'equip_short')}`,colors.cyan);
  glyphs(ctx,22,224,t(['stored','purchased','need_coins','full','equipped','none'][notice]),notice===2?colors.gold:colors.cyan);
  glyphs(ctx,22,243,`${t('buy')} | <> ${t('choose')} | ${t('back')}`);
}
function drawJournal(ctx){
  overlay(ctx);glyphs(ctx,20,22,t('objectives'),colors.white,2);glyphs(ctx,310,27,`${t('runs')} ${profile.runs}`,colors.gold);
  design.missions.forEach((m,i)=>{
    const y=58+i*36;glyphs(ctx,22,y,t(m.key));glyphs(ctx,239,y,`${profile.goals[i]} / ${profile.target(i)}`,colors.cyan);glyphs(ctx,370,y,`+${profile.reward(i)}`,colors.gold);
    ctx.fillStyle='#1d3c5b';ctx.fillRect(22,y+14,425,3);ctx.fillStyle=colors.cyan;ctx.fillRect(22,y+14,425*profile.goals[i]/profile.target(i),3);
  });
  glyphs(ctx,22,174,`${t('collection')} ${profile.relics.toString(2).replaceAll('0','').length}/9`);
  for(let i=0;i<9;i++){const x=22+i*48;panel(ctx,x,191,39,32);if(profile.relics&(1<<i)){icon(ctx,[1,2,5][Math.floor(i/3)],x+7,194,25);glyphs(ctx,x+2,216,String(i%3+1),colors.gold);}else glyphs(ctx,x+17,202,'?',colors.gray);}
  glyphs(ctx,22,243,`${t('journal_hint')} | ${t('back')}`,colors.cyan);
}
function drawMenu(ctx){
  ctx.fillStyle='rgba(4,17,39,.55)';ctx.fillRect(0,0,480,272);panel(ctx,22,20,289,230);
  glyphs(ctx,38,33,'SKY HOPPER',colors.white,3);glyphs(ctx,40,61,t('subtitle'),colors.cyan);
  const keys=['play','store','journal','help','language'];keys.forEach((key,i)=>{const y=94+i*25;if(i===menuIndex){ctx.fillStyle='#154e79';ctx.fillRect(32,y-5,267,20);glyphs(ctx,38,y,'>',colors.gold);}glyphs(ctx,54,y,`${t(key)}${i===4?' ITA / ENG':''}`);});
  glyphs(ctx,40,229,`${t('best')} ${profile.best}  ${t('coins')} ${profile.bank}`,colors.gold);
  ctx.drawImage(robotSkins[profile.skin],0,96,48,48,328,84,112,112);center(ctx,258,`${t('confirm')}   ${t('choose')}`);
  glyphs(ctx,324,211,t('launch'),colors.cyan);glyphs(ctx,324,227,profile.launch?t(`power_${[1,2,5][profile.launch-1]}`):t('none'),colors.gold);
}
function drawHelp(ctx){
  overlay(ctx);glyphs(ctx,22,23,t('help'),colors.white,2);
  ['help_route','help_jump','help_left','help_right','help_dash','help_pause','help_jet','help_stack','help_conflict','help_save'].forEach((key,i)=>glyphs(ctx,22,57+i*17,t(key),i===0?colors.gold:colors.white));
  glyphs(ctx,22,243,t('back'),colors.cyan);
}
function drawResult(ctx){
  ctx.fillStyle='rgba(3,10,26,.6)';ctx.fillRect(0,0,480,272);panel(ctx,95,52,290,176);
  center(ctx,66,t('run_end'),2);center(ctx,99,`${t('score')} 236`,2,colors.gold);
  center(ctx,126,`${t('banked')} +${lastResult?.earned??0}   ${t('mission_reward')} +${lastResult?.rewards??0}`);
  ctx.drawImage(robotSkins[profile.skin],96,144,48,48,209,138,60,60);center(ctx,200,`${t('retry')}   ${t('back')}`,1,colors.cyan);
}
function save(){try{localStorage.setItem('sky-hopper-review-v2',JSON.stringify(profile.words()));}catch{}}
function updateBuildNote(){
  document.querySelector('[data-i18n="review_note"]').textContent=language==='en'
    ?'This scene uses a local model, not Rust physics. PSP 0.2 has been built and checked in PPSSPP. The playable WASM keeps the previous build.'
    :'Questa scena usa un modello locale, non la fisica Rust. PSP 0.2 e stata compilata e verificata in PPSSPP. La WASM giocabile resta alla build precedente.';
}
function drawAll(){
  render(selected,zoom,screen);render(native,zoom,screen);for(const canvas of views)render(canvas,Number(canvas.dataset.view));
  document.querySelector('#metrics').textContent=`Zoom ${Math.round(zoom*100)}% · ${Math.round(480/zoom)} ${t('metrics')} · ${Math.round((480-115)/zoom)} ${t('ahead')}`;
  const active=rules.powers.filter(p=>powers.has(p.id));
  document.querySelector('#power-status').textContent=active.length?active.map(p=>`${t(`power_${p.id}`)}: ${powers.remaining(p.id).toFixed(1)} s${powers.suspended(p.id)?` (${t('suspended')})`:''}`).join(' · '):t('no_bonus');
  document.querySelector('#profile-status').textContent=`${t('coins')}: ${profile.bank} · ${t('runs')}: ${profile.runs} · ${t('stock')}: ${profile.stock.join('/')} · ${t('collection')}: ${profile.relics.toString(2).replaceAll('0','').length}/9`;
}
function refreshStore(){
  const item=design.items[shop],category=item.category;
  document.querySelector('#store-controls').hidden=screen!=='shop';
  document.querySelectorAll('[data-category]').forEach(b=>b.setAttribute('aria-pressed',String(Number(b.dataset.category)===category)));
  const container=document.querySelector('#store-items');container.replaceChildren();
  design.items.forEach((it,i)=>{if(it.category!==category)return;
    const button=document.createElement('button');button.dataset.item=String(i);button.classList.toggle('selected',shop===i);button.setAttribute('aria-pressed',String(shop===i));
    const name=document.createElement('span'),detail=document.createElement('small');name.textContent=t(it.key);
    detail.textContent=`${profile.level(it)}/${it.max} · ${profile.level(it)>=it.max?t(it.kind==='cosmetic'?'equip':'max'):`${profile.cost(it)} ${t('coins')}`}`;
    button.append(name,detail);button.addEventListener('click',()=>{shop=i;notice=0;refreshStore();drawAll();});container.append(button);
  });
  document.querySelector('#store-detail').textContent=`${t(item.key)} · ${effect(item)} · ${t(description(item))}`;
  const buy=document.querySelector('#buy');buy.textContent=item.kind==='cosmetic'&&profile.level(item)?cosmeticAction(item):t('buy');buy.disabled=item.kind!=='cosmetic'&&profile.level(item)>=item.max;
  const equip=document.querySelector('#equip');equip.hidden=item.kind!=='capsule';equip.disabled=item.kind==='capsule'&&!profile.stock[item.index];equip.textContent=t(profile.launch===item.index+1?'unequip':'equip');
}
function selectScreen(value){screen=value;document.querySelectorAll('[data-screen]').forEach(b=>b.setAttribute('aria-pressed',String(b.dataset.screen===screen)));refreshStore();drawAll();}
function collectPower(id){
  const previous=powers.mask();powers.collect(id,profile.upgraded>0||document.querySelector('#power-upgrade').checked,profile.bonusExtra(id));
  const replaced=rules.powers.filter(p=>(previous&(1<<(p.id-1)))&&!powers.has(p.id));
  document.querySelector('#power-message').textContent=`${t('collected')}: ${t(`power_${id}`)}${replaced.length?` · ${t('replaced')}: ${replaced.map(p=>t(`power_${p.id}`)).join(', ')}`:''}`;drawAll();
}
function purchase(){notice=profile.purchase(shop);document.querySelector('#feedback').textContent=t(['stored','purchased','need_coins','full','equipped','none'][notice]);save();refreshStore();drawAll();}
function arm(){const item=design.items[shop];if(item.kind!=='capsule')return;notice=profile.arm(item.index)?(profile.launch?4:5):3;save();refreshStore();drawAll();}
document.querySelectorAll('[data-screen]').forEach(b=>b.addEventListener('click',()=>selectScreen(b.dataset.screen)));
document.querySelectorAll('[data-category]').forEach(b=>b.addEventListener('click',()=>{shop=[0,9,12][Number(b.dataset.category)];notice=0;refreshStore();drawAll();}));
document.querySelector('#buy').addEventListener('click',purchase);document.querySelector('#equip').addEventListener('click',arm);
document.querySelectorAll('[data-zoom]').forEach(button=>button.addEventListener('click',()=>{zoom=Number(button.dataset.zoom);document.querySelectorAll('[data-zoom]').forEach(b=>b.setAttribute('aria-pressed',String(b===button)));drawAll();}));
document.querySelectorAll('[data-pose]').forEach(button=>button.addEventListener('click',()=>{pose=Number(button.dataset.pose);document.querySelectorAll('[data-pose]').forEach(b=>b.setAttribute('aria-pressed',String(b===button)));drawAll();}));
document.querySelector('#motion').addEventListener('click',event=>{animated=!animated;event.currentTarget.setAttribute('aria-pressed',String(animated));event.currentTarget.textContent=t(animated?'stop_animate':'animate');drawAll();});
document.querySelectorAll('[data-power]').forEach(b=>b.addEventListener('click',()=>collectPower(Number(b.dataset.power))));
document.querySelectorAll('[data-combo]').forEach(b=>b.addEventListener('click',()=>{for(const id of b.dataset.combo.split(',').map(Number))collectPower(id);}));
document.querySelectorAll('[data-advance]').forEach(b=>b.addEventListener('click',()=>{if(screen==='playing')powers.tick(Number(b.dataset.advance));drawAll();}));
document.querySelector('#power-clock').addEventListener('click',event=>{powerClock=!powerClock;event.currentTarget.setAttribute('aria-pressed',String(powerClock));event.currentTarget.textContent=t(powerClock?'stop_timers':'start_timers');});
document.querySelector('#power-clear').addEventListener('click',()=>{powers=new PowerState(rules);document.querySelector('#power-message').textContent=t('no_bonus');drawAll();});
document.querySelector('#power-hit').addEventListener('click',()=>{document.querySelector('#power-message').textContent=t(powers.effective(3)?'hit_boost':powers.consume(2)?'hit_shield':'hit_none');drawAll();});
document.querySelector('#demo-wallet').addEventListener('click',()=>{profile.bank=Math.min(0xffffffff,profile.bank+100);save();document.querySelector('#feedback').textContent=t('save_local');refreshStore();drawAll();});
function startRun(){powers=new PowerState(rules);const id=profile.beginRun();if(id)collectPower(id);save();selectScreen('playing');document.querySelector('#feedback').textContent=`${t('launch')}: ${id?t(`power_${id}`):t('none')}`;}
document.querySelector('#start-run').addEventListener('click',startRun);
document.querySelector('#demo-run').addEventListener('click',()=>{lastResult=profile.bankRun({distance:800,coins:16,pickups:3,score:236});save();selectScreen('result');document.querySelector('#feedback').textContent=`${t('earned')} +${lastResult.earned} · ${t('mission_reward')} +${lastResult.rewards}`;});
onLanguageChange(()=>{profile.language=language==='en'?1:0;save();document.querySelector('#power-clock').textContent=t(powerClock?'stop_timers':'start_timers');document.querySelector('#motion').textContent=t(animated?'stop_animate':'animate');document.querySelector('#feedback').textContent='';document.querySelector('#power-message').textContent='';refreshStore();drawAll();updateBuildNote();});
selected.addEventListener('keydown',e=>{
  if(!['ArrowUp','ArrowDown','ArrowLeft','ArrowRight','Enter','KeyX','KeyQ','KeyE','Escape','KeyP'].includes(e.code))return;
  e.preventDefault();if(e.repeat)return;
  if(e.code==='Escape'){selectScreen('menu');return;}
  if(e.code==='KeyP'){selectScreen(screen==='paused'?'playing':'paused');return;}
  if(screen==='shop'){
    const category=design.items[shop].category,[start,end]=[[0,9],[9,12],[12,15]][category];
    if(e.code==='ArrowDown')shop=start+(shop-start+1)%(end-start);
    if(e.code==='ArrowUp')shop=start+(shop-start+end-start-1)%(end-start);
    if(e.code==='ArrowRight')shop=[0,9,12][(category+1)%3];
    if(e.code==='ArrowLeft')shop=[0,9,12][(category+2)%3];
    if(['KeyX','Enter'].includes(e.code))purchase();
    if(['KeyQ','KeyE'].includes(e.code))arm();
    refreshStore();drawAll();
  }else if(screen==='menu'){
    if(e.code==='ArrowDown')menuIndex=(menuIndex+1)%5;if(e.code==='ArrowUp')menuIndex=(menuIndex+4)%5;
    if(['KeyX','Enter'].includes(e.code)){if(menuIndex===4)setLanguage(language==='it'?'en':'it');else selectScreen(['playing','shop','journal','help'][menuIndex]);}drawAll();
  }else if(screen==='result'&&['KeyX','Enter'].includes(e.code))startRun();
});
selected.addEventListener('pointerdown',e=>{
  selected.focus();const r=selected.getBoundingClientRect(),x=(e.clientX-r.left)*480/r.width,y=(e.clientY-r.top)*272/r.height;
  if(screen==='shop'){
    if(y>=43&&y<61){shop=[0,9,12][Math.max(0,Math.min(2,Math.floor((x-20)/147)))];}
    else if(x<264&&y>=69&&y<200){const [start,end]=[[0,9],[9,12],[12,15]][design.items[shop].category],first=start+Math.max(0,shop-start-4);shop=Math.min(end-1,first+Math.floor((y-69)/26));}
    else if(x>=280&&y>=162&&y<182)purchase();
    else if(x>=280&&y>=183&&y<202)arm();
    refreshStore();drawAll();
  }
});
translateDOM();refreshStore();drawAll();updateBuildNote();
function frame(now){const dt=Math.min((now-last)/1000,.1);last=now;
  if(screen==='playing'){if(animated)clock+=dt;if(powerClock)powers.tick(dt);if(animated||powerClock)drawAll();}
  requestAnimationFrame(frame);
}requestAnimationFrame(frame);
