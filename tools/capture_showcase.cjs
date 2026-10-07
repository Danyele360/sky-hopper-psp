// Genuine PSP framebuffer captures; uses an isolated demonstration profile.
const fs = require('node:fs');
const path = require('node:path');
const zlib = require('node:zlib');
const crypto = require('node:crypto');
const session = '.tools/github-showcase';
const out = 'docs/showcase';
const sleep = ms => new Promise(resolve => setTimeout(resolve, ms));
const keys = ['best','bank','owned','skin','world','upgrade','version',
  'duration1','duration2','duration3','duration4','duration5','duration6',
  'range','cooldown','yield','stock1','stock2','stock5','launch','language',
  'runs','distance','coins','pickups','goal1','goal2','goal3','tier1','tier2','tier3','relics'];
function seedProfile() {
  const profile = [1840,846,7,0,0,1,2,2,1,2,1,3,1,2,1,2,5,4,3,3,1,
    24,18720,438,62,520,9,2,1,2,1,75];
  const head=Buffer.alloc(8);head.write('RCFG');head.writeUInt16LE(1,4);head.writeUInt16LE(keys.length,6);
  const chunks=[head];
  keys.forEach((key,i)=>{const k=Buffer.from(key),b=Buffer.alloc(k.length+8);
    b[0]=k.length;k.copy(b,1);b[k.length+1]=2;b.writeUInt16LE(4,k.length+2);
    b.writeUInt32LE(profile[i],k.length+4);chunks.push(b);});
  fs.mkdirSync(session,{recursive:true});fs.mkdirSync(out,{recursive:true});
  fs.copyFileSync('dist/SkyHopper.EBOOT.PBP',`${session}/SkyHopper.EBOOT.PBP`);
  fs.writeFileSync(`${session}/ppsspp.ini`, '[General]\nFirstRun=False\nRemoteDebuggerOnStartup=True\nRemoteDebuggerLocal=True\nRemoteISOPort=9234\nPauseWhenMinimized=False\n[Graphics]\nGraphicsBackend=0\nInternalResolution=2\nShowFPSCounter=0\n[Sound]\nEnable=False\n');
  fs.writeFileSync(`${session}/sky-hopper-save.bin`,Buffer.concat(chunks));
  fs.writeFileSync(`${session}/profile.json`,JSON.stringify(Object.fromEntries(keys.map((k,i)=>[k,profile[i]])),null,2));
}
function crc32(b){let c=0xffffffff;for(const v of b){c^=v;for(let i=0;i<8;i++)c=(c>>>1)^((c&1)?0xedb88320:0);}return (c^0xffffffff)>>>0;}
function chunk(type,data){const tag=Buffer.from(type),b=Buffer.alloc(data.length+12);b.writeUInt32BE(data.length);tag.copy(b,4);data.copy(b,8);b.writeUInt32BE(crc32(Buffer.concat([tag,data])),b.length-4);return b;}
function png(rgba){const header=Buffer.alloc(13);header.writeUInt32BE(480);header.writeUInt32BE(272,4);header[8]=8;header[9]=6;
  const rows=Buffer.alloc((480*4+1)*272);for(let y=0;y<272;y++)rgba.copy(rows,y*(480*4+1)+1,y*480*4,(y+1)*480*4);
  return Buffer.concat([Buffer.from('89504e470d0a1a0a','hex'),chunk('IHDR',header),chunk('IDAT',zlib.deflateSync(rows)),chunk('IEND',Buffer.alloc(0))]);}
if(process.argv.includes('--prepare')){seedProfile();console.log('Isolated showcase profile ready');process.exit(0);}
let ticket=0;const requests=new Map();
const ws=new WebSocket('ws://127.0.0.1:9234/debugger','debugger.ppsspp.org');
ws.addEventListener('message',e=>{const d=JSON.parse(e.data),r=requests.get(d.ticket);if(r){requests.delete(d.ticket);clearTimeout(r.timeout);d.event==='error'?r.reject(Error(d.message)):r.resolve(d);}});
function request(event,args={}){return new Promise((resolve,reject)=>{const id=++ticket,timeout=setTimeout(()=>{requests.delete(id);reject(Error(event+' timed out'));},10000);requests.set(id,{resolve,reject,timeout});ws.send(JSON.stringify({event,...args,ticket:id}));});}
async function press(button){await request('input.buttons.press',{button,duration:12});await sleep(250);}
async function capture(name, scratch=false){const data=await request('memory.read',{address:0x04000000,size:512*272*4});
  const source=Buffer.from(data.base64,'base64'),pixels=Buffer.alloc(480*272*4);
  for(let row=0;row<272;row++)source.copy(pixels,row*480*4,row*512*4,(row*512+480)*4);
  const dir=scratch?`${session}/frames`:out;fs.mkdirSync(dir,{recursive:true});
  fs.writeFileSync(`${dir}/${name}.png`,png(pixels));console.log('Captured '+name);
}
(async()=>{try{
  await new Promise((resolve,reject)=>{ws.addEventListener('open',resolve,{once:true});ws.addEventListener('error',reject,{once:true});});
  const version=await request('version'),status=await request('game.status');
  if((await request('cpu.status')).stepping)await request('cpu.resume');await sleep(200);
  if(process.argv.includes('--skills')){
    // Continue from Italian pause, with Style selected in the hangar.
    await press('circle');await press('up');await press('cross');await press('down');
    await press('down');await press('cross');await press('right');
    for(let i=0;i<8;i++)await press('down');await capture('hangar-skills-eng');
  }else if(process.argv.includes('--extras')){
    // Continue from pause: record Italian views with correct category selection.
    await press('circle');await press('up');await press('cross');await press('down');await capture('menu-ita');
    await press('down');await press('cross');await press('left');await capture('hangar-migliorie-ita');
    await press('right');await capture('hangar-capsule-ita');
    await press('right');await press('cross'); // equip unlocked Nebula robot
    await press('down');await press('down');await press('cross'); // equip Nebula sky
    await capture('hangar-stile-ita');await press('circle');await press('up');await press('cross');
    for(let i=0;i<18;i++){
      if(i===0||i===8)await request('input.buttons.press',{button:'up',duration:8});
      await sleep(180);await capture(`nebula-${String(i).padStart(2,'0')}`,true);
    }
    // Let this run conclude naturally, then capture the native result screen.
    await sleep(16000);await capture('risultati-ita');
  }else if(process.argv.includes('--play')){
    await press('cross');
    // Follow the jetpack corridor with short vertical corrections. Capture a burst
    // for later editorial selection; these are actual running game frames.
    for(let i=0;i<32;i++){
      if(i===0||i===8||i===17)await request('input.buttons.press',{button:'up',duration:8});
      if(i>20&&i%3===0)await press('cross');
      await sleep(180);await capture(`flight-${String(i).padStart(2,'0')}`,true);
    }
    await press('start');await capture('pause-eng');
  }else{
    await capture('menu-eng');await press('down');await press('cross');await capture('hangar-upgrades-eng');
    for(let i=0;i<8;i++)await press('down');await capture('hangar-skills-eng');
    await press('right');await capture('hangar-capsules-eng');
    await press('right');await capture('hangar-style-eng');await press('circle');
    await press('down');await press('cross');await capture('goals-eng');await press('circle');
    await press('down');await press('down');await press('cross');await capture('menu-ita');
    await press('up');await press('up');await press('cross');await capture('obiettivi-ita');await press('circle');
    await press('up');await press('cross');await press('right');await capture('hangar-migliorie-ita');
    await press('right');await capture('hangar-capsule-ita');await press('circle');
    // Return to English, then select Play for the separate gameplay capture pass.
    for(let i=0;i<3;i++)await press('down');await press('cross');
    for(let i=0;i<4;i++)await press('up');
  }
  fs.writeFileSync(`${session}/capture-session.json`,JSON.stringify({date:'2026-10-08',version,status,
    buildSha256:crypto.createHash('sha256').update(fs.readFileSync(`${session}/SkyHopper.EBOOT.PBP`)).digest('hex'),
    framebuffer:{width:480,height:272,stride:512},profile:'Isolated demonstration save; unlocked items and resources staged',capture:'PPSSPP PSP framebuffer, no game rebuild'},null,2));
}finally{ws.close();}})().catch(e=>{console.error(e);process.exitCode=1;});
