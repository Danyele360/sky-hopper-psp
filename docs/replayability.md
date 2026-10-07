# Sky Hopper: rendere interessante la prossima corsa

## Ricerca e scelte

La presentazione di Tom Cadwell a GDC descrive la padronanza del gioco come
motore della rigiocabilità dei roguelike. Per Sky Hopper questo suggerisce
obiettivi leggibili, controllo affidabile e combinazioni da sperimentare.
Non abbiamo consultato il filmato integrale: questo riferimento è la descrizione
pubblica della sessione. [GDC Vault, Level Up Your Game](https://www.gdcvault.com/play/1022119/Level-Up-Your-Game-The).

La descrizione pubblicata da Supergiant per Hades accosta tentativi variabili,
combinazioni di capacità e crescita permanente. È un riferimento utile per
separare ciò che cambia nella corsa da ciò che resta nel salvataggio.
[Hades, descrizione del produttore su Steam](https://store.steampowered.com/app/1145360/Hades/).

Da questi riferimenti ricaviamo una proposta per questo arcade: ogni corsa
deve offrire una scelta utile, progressi comprensibili e un motivo concreto
per provare ancora. È una scelta di design, non una garanzia che il gioco
risulti divertente: il ritmo e i prezzi andranno valutati giocando.

## Ciclo implementato nei sorgenti

1. Scegli una capsula iniziale oppure conserva le scorte.
2. Affronta un percorso con seed diverso e raccolte di bonus variabili.
3. Combina gli effetti compatibili e leggi le scadenze senza coprire il percorso.
4. A fine corsa conserva monete, avanzamento degli obiettivi e collezione.
5. Scegli una miglioria, una scorta o un aspetto, poi riparti rapidamente.

Le missioni contano distanza, monete e bonus raccolti anche attraverso più
partite. Una corsa breve contribuisce alla progressione. Il riepilogo separa
guadagno della corsa e ricompense degli obiettivi.

## Hangar e accumulo

| Categoria | Contenuto | Limite e beneficio |
| --- | --- | --- |
| Durate | Sei migliorie, una per bonus | 3 livelli: +1 s/livello; jetpack +0,5 s/livello |
| Capacità | Raggio magnete | Da 125 a 185 unità, 3 livelli |
| Capacità | Ricarica dash | Da 3 a 2,1 s, 3 livelli |
| Capacità | Guadagno monete | Fino a +30% delle monete della corsa, arrotondato per difetto |
| Scorte | Capsule magnete, scudo e jetpack | Fino a 9 per tipo; una sola scelta iniziale viene consumata |
| Stile | Robot Nebula, Robot Solar, cielo Nebula | Acquisto permanente, equipaggiabile e rimovibile |
| Collezione | Nove distintivi degli obiettivi | Tre serie, conservate tra le partite |

Le migliorie ordinarie costano 28, 53 e 78 monete; il jetpack 35, 65 e 95.
Le capacità costano 40, 75 e 110. Capsule: 12, 18 e 25 monete.
Aspetti: 60, 120 e 180. Il vecchio acquisto globale +2 s viene conservato.
I prezzi e i limiti sono in `assets/progression.json`.

I livelli sono limitati per conservare la necessità di imparare salti e
traiettorie. Le capsule aggiungono la scelta tra preparare la prossima corsa e
risparmiare per una miglioria. I bonus raccolti restano temporanei: ogni corsa
ricostruisce la propria combinazione.

Le soglie iniziali sono 600 unità di distanza, 12 monete e 2 bonus, con premi
di 15, 20 e 25 monete. I successivi tre obiettivi di ciascuna serie aumentano
la soglia, poi il ciclo riparte; ogni ciclo completo aumenta il premio, entro
un limite. Ogni corsa può completare un obiettivo per serie. L'eccedenza passa
al prossimo obiettivo fino a un punto dalla sua soglia; un nuovo contributo
è necessario per riscuotere ancora. I distintivi si sbloccano una sola volta.

## Informazioni nel momento utile

Durante la corsa compaiono solo punteggio, monete, disponibilità dash e bonus
compatti. Le icone mantengono il timer numerico; una piccola barra mostra la
proporzione di durata residua. Gli ultimi 3 secondi diventano gialli e gli
ultimi 1,5 possono diventare rossi. L'icona resta visibile. `II` indica un
effetto in attesa: super salto durante il jetpack.

Nomi, durate dettagliate e incompatibilità si leggono in pausa. Store,
obiettivi, collezione e scelta lingua restano nei menu. L'80% mostra 600
unità di mondo su 480 pixel, con HUD a scala nativa. I testi sono ITA/ENG.

## Come valutare se il ciclo regge

Il modello locale verifica contabilità e interfaccia, non l'interesse del
gameplay. Dopo le verifiche attuali e una futura compilazione, le prove
giocabili devono osservare: durata media delle corse, tempo al primo acquisto,
varietà delle capsule scelte, frequenza dei retry e leggibilità delle scadenze.
I numeri dell'economia sono un primo bilanciamento modificabile. La versione
PSP 0.2 è ora compilata e verificata in PPSSPP: vedi `psp-v0.2/NOTE.md`.
