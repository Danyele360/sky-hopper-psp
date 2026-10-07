import {t,language,setLanguage,onLanguageChange,translateDOM} from './i18n.mjs';
const canvas=document.querySelector('#game');
const ctx=canvas.getContext('2d',{alpha:false});
let api, keyboard=0, touch=0, gamepad=0, pending=0, muted=false, audio, musicTimer;
const mapping={Space:1,KeyX:1,ArrowLeft:2,KeyA:2,ArrowRight:4,KeyD:4,Enter:8,KeyP:8,KeyQ:16,KeyE:16,Escape:32,KeyC:32,ArrowUp:64,KeyW:64,ArrowDown:128,KeyS:128};
const heldKeys=new Set();
function refreshKeys(){keyboard=[...heldKeys].reduce((mask,key)=>mask|(mapping[key]||0),0);}
function sound(frequency,duration=0.09,type='triangle',volume=0.06,slide=0){
  if(muted||!audio||audio.state!=='running')return;
  const osc=audio.createOscillator(),gain=audio.createGain(),now=audio.currentTime;
  osc.type=type;osc.frequency.setValueAtTime(frequency,now);
  if(slide)osc.frequency.exponentialRampToValueAtTime(Math.max(20,frequency+slide),now+duration);
  gain.gain.setValueAtTime(volume,now);gain.gain.exponentialRampToValueAtTime(0.0001,now+duration);
  osc.connect(gain);gain.connect(audio.destination);osc.start(now);osc.stop(now+duration);
}
function unlockAudio(){
  if(!audio){
    audio=new AudioContext();
    let note=0;const melody=[261.63,329.63,392,523.25,392,329.63,293.66,392,246.94,329.63,392,493.88,392,329.63,293.66,246.94];
    musicTimer=setInterval(()=>{if(api&&api.stat(0)===1&&!muted){sound(melody[note++%melody.length],0.22,'triangle',0.018);}},240);
  }
  if(audio.state==='suspended')audio.resume();
}
function playEvents(events){
  if(events&1)sound(360,0.13,'square',0.025,350);
  if(events&2)sound(1100,0.07,'sine',0.08,300);
  if(events&4){sound(520,0.25,'triangle',0.06,700);}
  if(events&8)sound(170,0.16,'sawtooth',0.035,-70);
  if(events&16)sound(340,0.4,'triangle',0.07,-270);
}
document.addEventListener('keydown',event=>{
  if(event.target.closest?.('select,input,textarea')||(event.target.tagName==='BUTTON'&&!event.target.dataset.key))return;
  if(!(event.code in mapping))return;
  event.preventDefault();if(!event.repeat)pending|=mapping[event.code];heldKeys.add(event.code);refreshKeys();unlockAudio();
});
document.addEventListener('keyup',event=>{if(event.code in mapping){heldKeys.delete(event.code);refreshKeys();if(!event.target.closest?.('select,input,textarea'))event.preventDefault();}});
const pointers=new Map();
document.querySelectorAll('[data-key]').forEach(button=>{
  button.addEventListener('pointerdown',event=>{
    event.preventDefault();button.setPointerCapture(event.pointerId);pointers.set(event.pointerId,Number(button.dataset.key));pending|=Number(button.dataset.key);
    touch=[...pointers.values()].reduce((a,b)=>a|b,0);button.classList.add('active');unlockAudio();canvas.focus();
  });
  const release=event=>{pointers.delete(event.pointerId);touch=[...pointers.values()].reduce((a,b)=>a|b,0);button.classList.remove('active');};
  button.addEventListener('pointerup',release);button.addEventListener('pointercancel',release);button.addEventListener('lostpointercapture',release);
});
function pollGamepad(){
  gamepad=0;
  const pad=[...navigator.getGamepads()].find(Boolean);if(!pad)return;
  for(const [index,bit] of [[0,1],[1,32],[4,16],[5,16],[9,8],[12,64],[13,128],[14,2],[15,4]])if(pad.buttons[index]?.pressed)gamepad|=bit;
  if(pad.axes[0]<-0.35)gamepad|=2;if(pad.axes[0]>0.35)gamepad|=4;
  if(gamepad)unlockAudio();
}
function save(){
  if(!api.stat(7))return;
  const length=api.progress_len?api.progress_len():6;
  try{localStorage.setItem(length===32?'sky-hopper-v2':'sky-hopper-v1',JSON.stringify(Array.from({length},(_,i)=>api.stat(i+10))));api.mark_saved();}catch{}
}
const mute=document.querySelector('#mute');
mute.addEventListener('click',()=>{muted=!muted;mute.textContent=t(muted?'sound_off':'sound_on');mute.setAttribute('aria-pressed',String(muted));unlockAudio();});
document.querySelector('#fullscreen').addEventListener('click',()=>{
  if(document.fullscreenElement)document.exitFullscreen();else document.querySelector('.game-wrap').requestFullscreen().catch(()=>{});
});
let last=performance.now(),accumulator=0;
function clearInput(){heldKeys.clear();keyboard=0;pointers.clear();touch=0;gamepad=0;pending=0;document.querySelectorAll('.active').forEach(b=>b.classList.remove('active'));}
function pauseWhenHidden(){
  clearInput();if(api&&api.stat(0)===1){api.tick(0);api.tick(8);api.tick(0);}
  last=performance.now();accumulator=0;save();
}
window.addEventListener('blur',pauseWhenHidden);
document.addEventListener('visibilitychange',()=>{if(document.hidden)pauseWhenHidden();last=performance.now();});
window.addEventListener('pagehide',save);
function frame(now){
  accumulator+=Math.min((now-last)/1000,0.1);last=now;pollGamepad();
  while(accumulator>=1/60){api.tick(keyboard|touch|gamepad|pending);pending=0;playEvents(api.stat(4));accumulator-=1/60;}
  const pointer=api.render();
  const pixels=new Uint8ClampedArray(api.memory.buffer,pointer,480*272*4);
  ctx.putImageData(new ImageData(pixels,480,272),0,0);save();
  if(api.set_language&&api.stat(9)!==(language==='en'?1:0))setLanguage(api.stat(9)?'en':'it');
  requestAnimationFrame(frame);
}
try{
  const response=await fetch('sky-hopper.wasm');if(!response.ok)throw new Error(`File del gioco non disponibile (${response.status}).`);
  const result=await WebAssembly.instantiate(await response.arrayBuffer(),{});api=result.instance.exports;
  api.init((crypto.getRandomValues(new Uint32Array(1))[0])||42);
  try{
    const words=JSON.parse(localStorage.getItem(api.progress_len?'sky-hopper-v2':'sky-hopper-v1')||localStorage.getItem('sky-hopper-v1'));
    if(Array.isArray(words)&&[6,32].includes(words.length)&&words.every(v=>Number.isInteger(v)&&v>=0&&v<=0xffffffff)){
      if(api.load_word){for(let i=0;i<32;i++)api.load_word(i,words[i]||0);api.commit_progress();}
      else if(words.length===6)api.load_progress(...words);
    }
  }catch{}
  if(api.set_language){api.set_language(language==='en'?1:0);document.querySelector('#game-note').hidden=true;}
  document.querySelector('#loading').remove();canvas.focus();last=performance.now();requestAnimationFrame(frame);
}catch(error){
  document.querySelector('#loading').removeAttribute('data-i18n');
  document.querySelector('#loading').textContent=`${t('launch_error')} ${error.message}`;
}
const labels={'run':'run','idle':'idle','jump':'jump','double-jump':'double_jump','fall':'fall','land':'land','hurt':'hurt','jetpack':'power_5','boost':'power_3','victory':'victory','defeat':'defeat','coin':'coin','drone':'drone','spring':'spring','magnet':'power_1','shield':'power_2','boost-pickup':'power_3','super-jump':'power_4','jetpack-pickup':'power_5','slow-motion':'power_6'};
for(const [name,key] of Object.entries(labels)){
  const figure=document.createElement('figure'),img=document.createElement('img'),caption=document.createElement('figcaption');
  img.src=`../assets/animations/${name}.gif`;img.alt=t(key);img.dataset.locale=key;img.loading='lazy';caption.dataset.i18n=key;caption.textContent=t(key);figure.append(img,caption);document.querySelector('#animations').append(figure);
}
onLanguageChange(()=>{
  mute.textContent=t(muted?'sound_off':'sound_on');
  document.querySelectorAll('[data-locale]').forEach(img=>img.alt=t(img.dataset.locale));
  if(api?.set_language)api.set_language(language==='en'?1:0);
});
translateDOM();
