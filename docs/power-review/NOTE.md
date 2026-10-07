# Bonus cumulabili: verifica locale del 7 ottobre 2026

Questa nota documenta la prima verifica dei bonus. Lo stato attuale include
vista all'80%, HUD compatto, ITA/ENG e progressione: vedi `../design-review/NOTE.md`.
`tools/check_power_review.cjs` ora richiama la verifica completa aggiornata.

Il precedente stato conteneva un solo bonus e un solo timer. I sorgenti Rust ora
usano sei timer indipendenti. Una nuova raccolta sostituisce solo eventuali
bonus esplicitamente incompatibili. Il renderer mostra una scheda per ciascun
bonus conservato, con icona, nome, tempo e barra di durata.

## Regole adottate

| Bonus | Durata base | Compatibilità |
| --- | --- | --- |
| Magnete | 8 s | Tutti |
| Scudo | 8 s, un urto | Tutti |
| Boost | 8 s | Sostituisce il jetpack |
| Super salto | 8 s | Conservato durante il jetpack, effetto e timer sospesi |
| Jetpack | 4,5 s | Sostituisce il boost |
| Slow motion | 8 s | Tutti; rallenta l'ambiente, conserva la velocità del personaggio |

Le incompatibilità sono una scelta iniziale modificabile in
`assets/power-rules.json`. Si possono avere cinque bonus conservati insieme.
Una seconda raccolta dello stesso bonus rinnova la sua durata completa, senza
sommare secondi o moltiplicare l'intensità. Non accorcia un timer più lungo già
attivo. Il miglioramento nell'hangar aggiunge due secondi alle nuove raccolte.

Un urto consuma solo lo scudo e conserva gli altri bonus. Il boost protegge senza
consumare lo scudo. Raccogliere un bonus compatibile o rinnovare il jetpack
non riavvia il volo. All'uscita dal volo, anche per sostituzione col boost,
si mantiene la transizione protetta già prevista dal gioco. La pausa congela
tutti i timer.

## Cosa è stato verificato

Aprire http://127.0.0.1:8787/preview/camera.html attraverso il server locale.
La pagina consente raccolte manuali, urto, avanzamento di 1 o 5 secondi e
avvio/arresto dei timer. Usa gli asset esistenti e un modello JavaScript delle
regole, senza simulare la fisica del gioco.

- `node tools/check_powers.mjs`: 9 test passati del modello locale, incluse
  tutte le 30 coppie distinte in entrambi gli ordini, rinnovo, scadenze,
  conservazione del super salto, miglioramento e input non validi.
- `python tools/sync_power_rules.py --check`: costanti Rust corrispondenti
  alla tabella JSON usata nella verifica. Lo script senza `--check` rigenera
  le costanti dopo una modifica delle regole.
- `node tools/check_power_review.cjs`: combinazioni, rinnovo, scudo, conflitti,
  sospensione/ripresa, durate migliorate, timer, animazioni e layout mobile
  verificati in Chrome, senza errori JavaScript. Report in `browser-checks.json`.
- `rustfmt --edition 2021 crates/game/src/lib.rs crates/wasm/src/lib.rs`:
  parsing e formattazione dei sorgenti, senza compilazione.

I nuovi test Rust sono stati scritti ma non eseguiti: richiederebbero una
compilazione. La corrispondenza delle costanti non dimostra l'esecuzione della
nuova logica Rust; serviranno test e una nuova build quando verranno richiesti.
La verifica sulla PSP riferita dall'utente riguarda l'eseguibile precedente.

## Immagini della verifica

- `compatibili.png`: magnete, scudo, boost, super salto e slow motion.
- `volo.png`: jetpack al posto del boost, super salto in attesa.
- `super-salto-ripreso.png`: jetpack terminato, super salto di nuovo disponibile.
- `scudo-consumato.png`: solo lo scudo rimosso, magnete conservato.
- `pagina.png` e `mobile.png`: schermata completa dei controlli.

L'inquadratura all'85% nelle immagini è solo quella dello studio locale:
non è stata applicata al renderer Rust e resta da scegliere.

SHA-256 invariati: nessuna nuova build PSP o WASM.

| File | SHA-256 |
| --- | --- |
| `dist/PSP/GAME/SKYHOPPER/EBOOT.PBP` | `9be73d22cf6cc3d75d55b4665df0eae43fe6f81ee108fd6a00b2955e4a642100` |
| `preview/sky-hopper.wasm` | `da4781e10a53ae5ab66cb4daecbed46e6c0ac3d5b61aedb1f26583e95356211f` |
