// Run against the local PPSSPP debugger at port 9234. The game must already be open.
const fs=require('node:fs');
const assert=require('node:assert/strict');
const {execFileSync}=require('node:child_process');
fs.mkdirSync('docs/captures',{recursive:true});
const sleep=ms=>new Promise(resolve=>setTimeout(resolve,ms));
const ws=new WebSocket('ws://127.0.0.1:9234/debugger','debugger.ppsspp.org');
let ticket=0;
const requests=new Map();
ws.addEventListener('message',event=>{
  const data=JSON.parse(event.data);
  const id=data.ticket??[...requests].find(([,req])=>req.event===data.event)?.[0];
  const request=requests.get(id);
  if(request){requests.delete(id);clearTimeout(request.timeout);if(data.event==='error')request.reject(Error(data.message));else request.resolve(data);}
});
function request(event,args={}){
  return new Promise((resolve,reject)=>{
    const id=++ticket;
    const timeout=setTimeout(()=>{requests.delete(id);reject(Error(`Timeout: ${event}`));},15000);
    requests.set(id,{resolve,reject,timeout,event});ws.send(JSON.stringify({event,...args,ticket:id}));
  });
}
async function capture(name){
  // Software rendering writes directly to VRAM. Read a framebuffer with 512-pixel stride;
  // this also works with PPSSPP's window hidden, when its GPU has no output texture.
  const data=await request('memory.read',{address:0x04000000,size:512*272*4});
  const buffer=Buffer.from(data.base64,'base64');
  const pixels=Buffer.alloc(480*272*4);
  for(let row=0;row<272;row++)buffer.copy(pixels,row*480*4,row*512*4,(row*512+480)*4);
  assert.ok(new Set(pixels).size>50,'The PSP framebuffer must contain rendered artwork');
  fs.writeFileSync(`docs/captures/psp-${name}.rgba`,pixels);
  execFileSync(process.env.PYTHON || (process.platform === 'win32' ? 'python' : 'python3'),['-c',`from PIL import Image; from pathlib import Path; p=Path('docs/captures/psp-${name}.rgba'); Image.frombytes('RGBA',(480,272),p.read_bytes()).save(p.with_suffix('.png'))`]);
  return {width:480,height:272,source:'PPSSPP emulated VRAM'};
}
(async()=>{
  await new Promise((resolve,reject)=>{ws.addEventListener('open',resolve,{once:true});ws.addEventListener('error',reject,{once:true});});
  const version=await request('version');const status=await request('game.status');
  const cpu=await request('cpu.status');if(cpu.stepping)await request('cpu.resume');
  console.log(JSON.stringify({version,status}));
  await sleep(500);const menu=await capture('menu');
  await request('input.buttons.press',{button:'cross',duration:8});await sleep(450);
  await request('input.buttons.press',{button:'cross',duration:8});await sleep(250);
  await request('input.buttons.press',{button:'cross',duration:8});await sleep(180);
  const play=await capture('gameplay');
  await request('input.buttons.press',{button:'start',duration:8});await sleep(250);
  const pause=await capture('pause');
  await request('input.buttons.press',{button:'circle',duration:8});await sleep(300);
  await request('input.buttons.press',{button:'down',duration:8});await sleep(100);
  await request('input.buttons.press',{button:'cross',duration:8});await sleep(200);
  const hangar=await capture('hangar');
  fs.writeFileSync('docs/captures/psp-check.json',JSON.stringify({version,status,menu,play,pause,hangar,passed:true},null,2));
  console.log('PPSSPP boot, input, gameplay, pause and hangar screenshots captured.');ws.close();
})().catch(error=>{console.error(error);ws.close();process.exitCode=1;});
