# 80%, HUD compatto, progressione e ITA/ENG

Aggiornamento successivo: la versione PSP 0.2 è stata compilata su richiesta,
con 30 test Rust e verifica nativa PPSSPP superati. Vedi `../psp-v0.2/NOTE.md`.
Questa nota conserva la documentazione della verifica prima della build.
La WASM resta alla versione precedente.

Revisione del 7 ottobre 2026. L'utente ha scelto la vista all'80% e chiesto
progressione che dia senso alle corse ripetute, store, risorse accumulabili e
due lingue. I sorgenti Rust sono aggiornati. Su sua richiesta non sono stati
compilati Rust, WASM o PSP.

## Prova locale

Apri http://127.0.0.1:8787/preview/camera.html con il server dell'anteprima acceso.
La scena usa gli asset esistenti e modelli JavaScript, non la nuova fisica Rust.
Puoi scegliere lingua e schermata, raccogliere bonus, avviare i timer, provare
la pausa e navigare l'hangar. Le altre scale restano disponibili per confronto.

Per lo store, aggiungi monete di prova, seleziona una miglioria e compra.
Nelle scorte compra una o più capsule, equipaggiane una e prova la partenza:
si consuma solo una scorta del tipo scelto. Il pulsante fine corsa simula
800 unità di distanza, 16 monete e 3 bonus per verificare premi e collezione.
Gli acquisti della verifica sono separati dai salvataggi del gioco.

La pagina consente controlli da tastiera quando il canvas è selezionato:
frecce per voci e categorie, X/Invio per acquisto, Q/E per equipaggiare la
capsula, P per pausa, Esc per menu. Il gioco PSP usa le frecce, X, L/R e Cerchio.
La lingua nativa si sceglie nel menu con X sulla voce ITA/ENG.

## Cambiamenti nei sorgenti

- Zoom del mondo 0,8 attorno alla posizione del personaggio; 600 unità
  visibili. Simulazione e collisioni mantengono le unità originali.
- HUD: chip punteggio e monete, indicatore dash, cinque bonus al massimo
  in una fila di icone da 18 pixel con countdown e barra. Le superfici opache
  dell'HUD, senza suggerimento iniziale, occupano circa il 6% del display
  contro circa il 20% dei precedenti cinque riquadri completi.
- Dettagli dei bonus in pausa, con timer congelati e super salto in attesa.
- Hangar: 15 voci tra nove migliorie, tre capsule e tre aspetti. Limiti,
  prezzo successivo e azione equipaggia/rimuovi visibili.
- Tre obiettivi che progrediscono tra le corse e nove distintivi conservati.
- Nuovo seed a ogni corsa conclusa e raccolte dei bonus scelte dal generatore.
- Testi condivisi ITA/ENG e preferenza salvata; UI web e galleria tradotte.
- Font esteso con percentuale, punto interrogativo e punteggiatura. Si può
  rigenerare solo il font con `python tools/prepare_font.py`, senza compilare
  il gioco o ricreare texture e animazioni.
- Migrazione del salvataggio da 6 a 32 valori, conservando monete, aspetti,
  selezioni valide e vecchio acquisto globale +2 s. Le chiavi PSP precedenti
  restano valide. La futura WASM legge tutti i nuovi valori prima di applicarli.

Le fonti e le motivazioni delle scelte sono in `../replayability.md`.

## Verifiche eseguite

```sh
node tools/check_powers.mjs
node tools/check_progression.mjs
python tools/sync_power_rules.py --check
python tools/sync_design.py --check
node tools/check_design_review.cjs
```

22 test del modello locale superati: combinazioni, rinnovo, scadenze,
sospensione, prezzi, acquisti massimi, fondi insufficienti, scorte,
equipaggiamento, premi, collezione e migrazione. Le costanti Rust dei bonus,
catalogo, economia, zoom e traduzioni corrispondono ai JSON della verifica.

La verifica Chrome ha controllato le schermate a 480x272, entrambe le lingue,
pausa, acquisti, ricaricamento, consumo di una sola capsula, obiettivi,
controlli da tastiera e layout a 390 pixel, senza errori JavaScript.
Report: `browser-checks.json`. Gli screenshot sono PNG nativi, senza upscaling.

Il gioco WASM precedente si avvia ancora e mantiene input e pausa nel nuovo
contenitore web. Il suo canvas resta quello della build precedente, inclusa
la lingua interna italiana: la pagina lo indica chiaramente.

`rustfmt` ha analizzato e formattato i sorgenti senza compilarli. I test Rust
aggiunti restano da eseguire con la futura build. Le prove qui non dimostrano
la compilabilità, le prestazioni PSP o il bilanciamento del gameplay aggiornato.

## File utili

| Percorso | Contenuto |
| --- | --- |
| `hud-80-ita.png`, `hud-80-eng.png` | Nuovo HUD all'80% |
| `countdown-3s.png` | Scadenze imminenti |
| `pausa-ita.png`, `pause-eng.png` | Dettagli e timer in pausa |
| `store-migliorie-ita.png`, `store-upgrades-eng.png` | Store e livello acquistato |
| `store-scorte-ita.png` | Due capsule scudo, una scelta iniziale |
| `store-style-eng.png` | Aspetti e azione equipaggia/rimuovi |
| `obiettivi-collezione-ita.png`, `goals-collection-eng.png` | Progressione e distintivi |
| `menu-eng.png`, `help-eng.png`, `risultati-ita.png` | Menu, guida e ricompense |
| `mobile-eng.png` | Layout mobile completo |

EBOOT e WASM invariati, SHA-256:

- EBOOT: `9be73d22cf6cc3d75d55b4665df0eae43fe6f81ee108fd6a00b2955e4a642100`
- WASM: `da4781e10a53ae5ab66cb4daecbed46e6c0ac3d5b61aedb1f26583e95356211f`
