// Compose GitHub presentation layouts around unmodified native PSP captures.
const {launchBrowser}=require('./browser.cjs');
const fs=require('node:fs');
const path=require('node:path');
const dir='docs/showcase';
const img=name=>`data:image/png;base64,${fs.readFileSync(`${dir}/${name}.png`).toString('base64')}`;
const base=`*{box-sizing:border-box}body{margin:0;font-family:'Segoe UI',Arial,sans-serif;color:#ecf8ff;background:#071426}
  .page{position:relative;overflow:hidden;background:radial-gradient(ellipse at 85% 0,#163c60 0,transparent 55%),linear-gradient(125deg,#071629,#071020)}
  .kicker{font-size:18px;letter-spacing:3px;color:#73dcff;font-weight:650}.badge{border:1px solid #4084a0;background:#102b43;padding:9px 16px;border-radius:40px;color:#b9efff;font-size:18px;letter-spacing:1px}
  h1{margin:10px 0 2px;font-weight:750;letter-spacing:-3px;line-height:1.06}p{margin:0;color:#b5cddd;line-height:1.45}
  .frame{padding:5px;border:1px solid #396481;background:#041020;box-shadow:0 20px 45px #0005}
  img{display:block;image-rendering:pixelated;width:100%;height:auto}.label{font-size:16px;font-weight:650;letter-spacing:1.5px;color:#8ed9f4;margin-bottom:10px}
  .foot{font-size:16px;color:#86a8bd;letter-spacing:.4px}.feature{border-top:1px solid #2c4f68;padding-top:16px;color:#d9f6ff;font-size:23px;font-weight:550}
  .accent{color:#69d8ff}.rule{height:1px;background:#28465d}
`;
const cover=`<div class="page" style="width:1600px;height:1000px;padding:48px 58px">
  <div style="display:flex;justify-content:space-between;align-items:center"><div class="kicker">PSP HOMEBREW · BUILT WITH RUST</div><div class="badge">v0.2 &nbsp; / &nbsp; ITA + ENG</div></div>
  <h1 style="font-size:86px">SKY HOPPER</h1><p style="font-size:28px">Oltre le Nuvole <span class="accent">/</span> Beyond the Clouds</p>
  <div style="display:grid;grid-template-columns:972px 492px;gap:20px;margin-top:32px">
    <div><div class="label">80% PANORAMA · COMPACT BONUS HUD</div><div class="frame"><img src="${img('gameplay-bonuses-eng')}"></div></div>
    <div><div class="label">PERMANENT UPGRADES + RUN GOALS</div><div class="frame"><img src="${img('hangar-upgrades-eng')}"></div>
    <div class="frame" style="margin-top:16px"><img src="${img('goals-eng')}"></div></div>
  </div>
  <div style="position:absolute;left:58px;right:58px;bottom:66px;display:grid;grid-template-columns:repeat(4,1fr);gap:24px">
    <div class="feature">Stacking bonuses</div><div class="feature">Stockable capsules</div><div class="feature">Missions & collection</div><div class="feature">Unlockable styles</div>
  </div>
  <div class="foot" style="position:absolute;left:58px;bottom:24px">PSP v0.2 running in PPSSPP · Native game screenshots · Demonstration save · October 2026</div>
</div>`;
const cards=[['gameplay-boost-slow-eng','01 / Endless ascent','80% panorama, coins and simultaneous bonus timers'],
  ['gameplay-nebula-ita','02 / Nebula style','Unlockable robot and sky, Italian HUD'],
  ['menu-eng','03 / ITA + ENG','Language selection directly in the game'],
  ['hangar-upgrades-eng','04 / Permanent upgrades','Individual power durations with three levels'],
  ['hangar-skills-eng','05 / Lasting progression','Magnet range, dash cooldown and coin yield'],
  ['hangar-capsules-eng','06 / Supplies','Stock capsules and equip a starting bonus'],
  ['goals-eng','07 / Goals & collection','Progress across runs and collect nine relics'],
  ['risultati-ita','08 / Another horizon','Banked coins and mission rewards after a run']];
const gallery=`<div class="page" style="width:2080px;padding:48px 54px 40px">
  <div class="kicker">SKY HOPPER / PSP v0.2 / OCTOBER 2026</div><h1 style="font-size:64px;margin:16px 0 12px">Above the clouds, one run at a time.</h1>
  <p style="font-size:24px;margin-bottom:34px">Actual PSP game captures · ITA / ENG · 480 × 272 native resolution</p>
  <div style="display:grid;grid-template-columns:repeat(2,972px);gap:30px 28px">
    ${cards.map(([file,title,caption])=>`<div><div class="label" style="font-size:22px;letter-spacing:0">${title}</div><div class="frame"><img src="${img(file)}"></div><p style="font-size:22px;margin-top:10px">${caption}</p></div>`).join('')}
  </div><div class="rule" style="margin:34px 0 18px"></div><div class="foot" style="font-size:20px">Captured in PPSSPP 1.20.4 using an isolated demonstration save with staged resources and unlocks.</div>
</div>`;
const social=`<div class="page" style="width:1280px;height:640px;padding:24px 34px">
  <div style="display:flex;justify-content:space-between"><div class="kicker">PSP HOMEBREW · RUST</div><div class="badge" style="font-size:16px;padding:7px 14px">v0.2 · ITA / ENG</div></div>
  <h1 style="font-size:54px;margin-top:6px">SKY HOPPER</h1><p style="font-size:22px">Oltre le Nuvole / Beyond the Clouds</p>
  <div style="display:grid;grid-template-columns:700px 492px;gap:20px;margin-top:12px">
    <div class="frame"><img src="${img('gameplay-bonuses-eng')}"></div>
    <div><div class="frame"><img src="${img('hangar-upgrades-eng')}"></div>
      <p style="font-size:24px;color:#ddf4ff;margin-top:20px;line-height:1.55">Stacking bonuses<br>Permanent upgrades<br>Missions & collection</p>
    </div>
  </div><div class="foot" style="position:absolute;bottom:14px;left:34px;font-size:13px">Actual PSP game in PPSSPP · Demonstration save</div>
</div>`;
(async()=>{const browser=await launchBrowser();
  try{for(const [name,body,width,height] of [['cover',cover,1600,1000],['gallery',gallery,2080,3000],['social-preview',social,1280,640]]){
    const page=await browser.newPage({viewport:{width,height},deviceScaleFactor:1});
    await page.setContent(`<!doctype html><meta charset="utf-8"><style>${base}</style>${body}`);
    await page.evaluate(()=>Promise.all([...document.images].map(image=>image.decode())));
    const overflowing=await page.evaluate(()=>[...document.querySelectorAll('img')].some(i=>i.getBoundingClientRect().right>document.documentElement.clientWidth));
    if(overflowing)throw Error(`${name} overflows`);
    await page.locator('.page').screenshot({path:`${dir}/${name}.png`});
    await page.close();console.log(`Composed ${name}`);
  }}finally{await browser.close();}
})().catch(e=>{console.error(e);process.exitCode=1;});
