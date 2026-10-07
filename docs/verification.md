# Verifica — 7 ottobre 2026

Questa pagina documenta la prima versione 0.1. La nuova build **PSP 0.2** ha
superato 30 test Rust e la verifica nativa: risultati in `psp-v0.2/NOTE.md`.
L'anteprima WASM conserva la versione 0.1.

La build PSP release è stata compilata con nightly-2026-04-13 e SDK
AndrewAltimit/rust-psp al commit ad92131218406308170a3344786865415aef2995.
Il pacchetto EBOOT ha intestazione PBP valida, metadati XMB, PNG di icona e fondale,
e modulo eseguibile ELF/PRX. Dimensione: 1.401.853 byte.

- **12 test Rust superati**: generazione del percorso su 79 seed e migliaia di
  piattaforme, traversata automatica di 60 secondi, doppio salto con rilevamento
  della pressione, pausa, magnete, scudo monouso, uscita dal jetpack, acquisti,
  validazione del salvataggio, accreditamento singolo delle monete, clipping e
  rendering di tutte le modalità e tutti i power-up.
- **Browser Chrome / WASM**: caricamento, salto, doppio salto, dash, pausa con
  immagine stabile, hangar, persistenza locale, 20 animazioni in galleria e layout
  mobile 390 px senza overflow. Nessun errore JavaScript. Dettagli in
  `captures/browser-check.json`.
- **PPSSPP 1.20.4 / EBOOT nativo**: avvio del modulo `Sky Hopper PSP`, thread
  principale e audio, input PSP, corsa e salti, pausa e hangar. Catture reali del
  framebuffer nella VRAM emulata in `captures/psp-*.png`. Dettagli in
  `captures/psp-check.json`. Il salvataggio RCFG viene scritto con successo.
- **Formattazione Rust**: `cargo fmt --all -- --check` superato.

L'emulatore è stato eseguito con finestra nascosta durante la verifica. Per il
renderer software la cattura legge la VRAM con stride di 512 pixel e ne esporta
i 480 pixel visibili. Le immagini provengono dal codice PSP eseguito in PPSSPP,
non dall'anteprima WASM. Non sono foto di hardware fisico.

Limiti: nessuna prova su PSP-1000 fisica, nessuna misura affidabile del frame rate
su console, bilanciamento iniziale dei livelli e dei bonus. Le skin e il secondo
mondo sono varianti cromatiche, non nuovi set di disegni. Le animazioni del
personaggio usano 16 pose e particelle/scie animate dal motore.
