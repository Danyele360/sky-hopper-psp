# Build PSP 0.2 — 7 ottobre 2026

Su richiesta dell'utente è stata compilata **solo la versione PSP**.
L'anteprima WebAssembly resta alla build precedente. Toolchain
nightly-2026-04-13 e SDK ad92131218406308170a3344786865415aef2995.

Controlli completati:

- Tabelle bonus, economia, catalogo, zoom e ITA/ENG sincronizzati coi JSON.
- Formattazione Rust conforme; **30 test Rust superati**.
- 22 test dei modelli JavaScript superati.
- `cargo psp --release --locked` riuscito in release.
- Metadati XMB aggiornati a versione 0.20; modulo PSP versione 1.1.
- Avvio nativo in PPSSPP 1.20.4, menu ITA/ENG, guida, obiettivi, store e scorte.
- Corsa e salti con camera 80%, HUD compatto, pausa con framebuffer immutato.
- Salvataggio precedente copiato nella cartella isolata di prova: 6 chiavi
  migrate a 32, dati precedenti conservati, lingua e statistiche nuove scritte.
- Intestazione PBP, metadati SFO, PNG XMB, modulo PSP e integrità dello ZIP validati.

Il test usa una copia autonoma dell'EBOOT nella cartella `.tools/psp-v02-check`.
Le immagini sono catture della VRAM emulata, non del modello browser. Il percorso
PSP/GAME esterno alla Memory Stick può portare PPSSPP a caricare un modulo da
un'altra cartella: la verifica definitiva usa `SkyHopper.EBOOT.PBP` autonomo.
L'emulatore è stato fermato al termine; audio disattivato durante il test.
Non sono state misurate le prestazioni e non è stato verificato il nuovo EBOOT
su una console fisica. L'utente aveva confermato il funzionamento della versione
precedente sulla propria PSP.

Report e schermate: `native-checks.json`, `menu-ita.png`, `menu-eng.png`,
`help-eng.png`, `goals-eng.png`, `store-eng.png`, `capsules-eng.png`,
`gameplay-80.png`, `paused-eng.png`.

Distribuzione:

- `dist/PSP/GAME/SKYHOPPER/EBOOT.PBP` — installazione su console.
- `dist/SkyHopper.EBOOT.PBP` — apertura diretta in PPSSPP.
- `dist/SkyHopper-PSP-v0.2.zip` — cartella PSP, istruzioni ITA/ENG e licenza SDK.
- `dist/release-v0.2.json` — dimensioni e SHA-256 del pacchetto.

Il pacchetto non contiene salvataggi di prova. Copiare l'aggiornamento conservando
il proprio `sky-hopper-save.bin`.
