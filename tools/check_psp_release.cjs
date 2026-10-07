// Run the standalone release EBOOT in an isolated folder with a v1 save copy.
const fs=require('node:fs');
const assert=require('node:assert/strict');
const {execFileSync}=require('node:child_process');
const dir='docs/psp-v0.2';fs.mkdirSync(dir,{recursive:true});
const savePath='.tools/psp-v02-check/sky-hopper-save.bin';
function readSave(path){
  const b=fs.readFileSync(path);assert.equal(b.subarray(0,4).toString(),'RCFG');
  const count=b.readUInt16LE(6),values={};let offset=8;
  for(let i=0;i<count;i++){
    const keyLength=b[offset++],key=b.subarray(offset,offset+keyLength).toString();offset+=keyLength;
    const type=b[offset++],size=b.readUInt16LE(offset);offset+=2;
    assert.equal(type,2);assert.equal(size,4);values[key]=b.readUInt32LE(offset);offset+=size;
  }return values;
}
const legacy=readSave('.tools/psp-v02-check/legacy-save.bin');
const ws=new WebSocket('ws://127.0.0.1:9234/debugger','debugger.ppsspp.org');
let ticket=0;const requests=new Map();const sleep=ms=>new Promise(resolve=>setTimeout(resolve,ms));
ws.addEventListener('message',event=>{
  const data=JSON.parse(event.data),r=requests.get(data.ticket);
  if(r){requests.delete(data.ticket);clearTimeout(r.timeout);if(data.event==='error')r.reject(Error(data.message));else r.resolve(data);}
});
function request(event,args={}){
  return new Promise((resolve,reject)=>{const id=++ticket;
    const timeout=setTimeout(()=>{requests.delete(id);reject(Error(`Timeout ${event}`));},10000);
    requests.set(id,{resolve,reject,timeout});ws.send(JSON.stringify({event,...args,ticket:id}));
  });
}
async function press(button){await request('input.buttons.press',{button,duration:6});await sleep(180);}
async function capture(name){
  const data=await request('memory.read',{address:0x04000000,size:512*272*4});
  const source=Buffer.from(data.base64,'base64'),pixels=Buffer.alloc(480*272*4);
  for(let row=0;row<272;row++)source.copy(pixels,row*480*4,row*512*4,(row*512+480)*4);
  assert.ok(new Set(pixels).size>50);fs.writeFileSync(`${dir}/${name}.rgba`,pixels);
  execFileSync(process.env.PYTHON || (process.platform === 'win32' ? 'python' : 'python3'),['-c',"from PIL import Image; from pathlib import Path; import sys; p=Path(sys.argv[1]); Image.frombytes('RGBA',(480,272),p.read_bytes()).save(p.with_suffix('.png'))",`${dir}/${name}.rgba`]);
  return pixels;
}
(async()=>{
  try{
    await new Promise((resolve,reject)=>{ws.addEventListener('open',resolve,{once:true});ws.addEventListener('error',reject,{once:true});});
    const version=await request('version'),status=await request('game.status');
    const cpu=await request('cpu.status');if(cpu.stepping)await request('cpu.resume');
    await sleep(300);await capture('menu-ita');
    // Four down presses select the native language option in the new five-row menu.
    for(let i=0;i<4;i++)await press('down');await press('cross');await capture('menu-eng');
    const migrated=readSave(savePath);assert.equal(migrated.version,2);assert.equal(migrated.language,1);
    assert.equal(Object.keys(migrated).length,32);
    for(const key of ['best','bank','owned','skin','world','upgrade'])assert.equal(migrated[key],legacy[key]);
    await press('up');await press('cross');await capture('help-eng');await press('circle');
    await press('up');await press('cross');await capture('goals-eng');await press('circle');
    await press('up');await press('cross');await capture('store-eng');await press('right');await capture('capsules-eng');await press('circle');
    // Menu selection is still Hangar; up selects Play.
    await press('up');await press('cross');await press('cross');await press('cross');
    await capture('gameplay-80');await press('start');
    const frozen=await capture('paused-eng');await sleep(200);const later=await capture('paused-stable');
    assert.ok(frozen.equals(later),'Native PSP pause must freeze the framebuffer');
    await press('circle');const afterRun=readSave(savePath);assert.ok(afterRun.runs>=1);
    const report={passed:true,version,status,scope:'PSP EBOOT in PPSSPP, isolated game/save folder, audio muted',checks:['Native boot and PSP input','ITA/ENG menu, help, goals and store','Legacy 6-key save migrated to 32 keys with resources preserved','80% camera and compact HUD','Pause freezes framebuffer','New run statistics persisted'],legacy,migrated,afterRun};
    fs.writeFileSync(`${dir}/native-checks.json`,JSON.stringify(report,null,2));console.log(JSON.stringify(report,null,2));
  }finally{ws.close();}
})().catch(e=>{console.error(e);process.exitCode=1;});
