export const locales=await fetch('../assets/locales.json').then(r=>r.json());
export let language='it';
try{language=localStorage.getItem('sky-hopper-language')==='en'?'en':'it';}catch{}
const listeners=new Set();
export const t=key=>locales[language][key]??`[${key}]`;
export function translateDOM(){
  document.documentElement.lang=language;
  document.querySelectorAll('[data-i18n]').forEach(el=>el.textContent=t(el.dataset.i18n));
  document.querySelectorAll('[data-i18n-aria]').forEach(el=>el.setAttribute('aria-label',t(el.dataset.i18nAria)));
  const selector=document.querySelector('#language');if(selector)selector.value=language;
}
export function setLanguage(value){
  language=value==='en'?'en':'it';try{localStorage.setItem('sky-hopper-language',language);}catch{}
  translateDOM();for(const listener of listeners)listener(language);
}
export function onLanguageChange(listener){listeners.add(listener);}
translateDOM();
document.querySelector('#language')?.addEventListener('change',e=>setLanguage(e.target.value));
