# Verifica dell'inquadratura

Aggiornamento: l'utente ha scelto **80%**. La scala è ora impostata nei sorgenti
e nella nuova verifica locale; gli screenshot sotto documentano il confronto
precedente. Vedi `../design-review/NOTE.md` per lo stato corrente. Nessuna build.

Su richiesta dell'utente, le nuove verifiche si svolgono localmente prima di
ricompilare. L'utente riferisce che il gioco funziona sulla propria PSP.
Durante questa verifica non sono stati compilati Rust, WASM o EBOOT.

Aprire http://127.0.0.1:8787/preview/camera.html, con il server dell'anteprima acceso.
La pagina confronta una stessa scena di prova con gli asset esistenti:

| Scala del mondo | Larghezza visibile | Percorso davanti al robot |
| --- | --- | --- |
| 100% | 480 unità | 365 unità |
| 85% | circa 565 unità | circa 429 unità |
| 75% | 640 unità | circa 487 unità |

HUD, display 480 × 272 e posizione sullo schermo del robot sono costanti.
Si possono confrontare corsa, salto, doppio salto e jetpack, anche alla dimensione
nativa del display. È uno studio visuale; non applica lo zoom al gioco esistente
e non verifica fisica o collisioni a una nuova scala.

Verifica locale: caricamento degli asset, tre scale, selezione delle pose e
animazione senza errori JavaScript. Screenshot salvati in questa cartella.

SHA-256 invariati durante il lavoro:

- EBOOT: `9be73d22cf6cc3d75d55b4665df0eae43fe6f81ee108fd6a00b2955e4a642100`
- WASM: `da4781e10a53ae5ab66cb4daecbed46e6c0ac3d5b61aedb1f26583e95356211f`

Il confronto originale resta conservato per riferimento.
