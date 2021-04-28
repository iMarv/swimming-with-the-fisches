# Vorgeschichte
Ein bis zwei mal die Woche bewegen sich ein [Arbeitskollege](https://github.com/Kjarrigan) und ich aus unserer Komfortzone und beschäftigen uns mit Technologien, welche wir sonst so auf der Arbeit nicht verwenden.

[Kjarrigan](https://github.com/Kjarrigan) versucht sich an der Spieleprogrammierung und hat sich an einem möglichst simplen Regelwerk versucht, das er digital nachbauen möchte. Die Regeln seiner Wahl stehen bei ihm zu Hause im Regal, es geht um das Spiel **[Tempo, kleine Fische](https://www.ravensburger.de/produkte/spiele/mitbringspiele/tempo-kleine-fische-23334/index.html)**. Das Spiel wurde schon diverse male mit dem eigenem Nachwuchs gespielt, ist also bekannt.

Die Regeln in Kurzform (die Spielanleitung ist ansonsten unter dem Link oben zu finden):
- Es gibt **vier Fische** in **vier Farben**
- Es gibt **zwei Angler** in **zwei Farben**
- Ein sechsseitiger Würfel mit jeweils den Farben der Spielfiguren
- Das Spielfeld besteht aus **einem Boot-Feld**, **11 Fluss-Feldern** und einem **Meer-Feld**
- Das Boot wird an einem äußeren Ende des Spielfelds platziert, das Meer am anderen und die 11 Fluss Felder dazwischen
- Alle Fische werden auf das Feld in der Mitte gelegt
- Die Spieler legen sich fest welche Partei gewinnt, das Boot oder die Fische
- Es wird gewürfelt, die Farbe die gewürfelt wird darf ein Feld nach vorne wandern. Wird eine Boot-Farbe gewürfelt, wird das Stück Fluss vor dem Boot entfernt, das Boot rückt vor
- Erreicht ein Fisch das Meer, so ist er frei, erreicht das Boot ein Feld auf dem Fische sind, so werden diese gefangen
- Wird nun eine Farbe eines freien Fisches gewürfelt, so darf ein beliebiger anderer Fisch bewegt werden
- Wird die Farbe eines gefangenen Fisches gewürfelt, so darf das Boot voran schreiten
- Es wird so lange gewürfelt bis alle Fische entweder gefangen sind, oder frei
- Das Boot gewinnt, wenn es mindestens drei Fische gefangen hat, die Fische gewinnen wenn mindestens drei die Freiheit erreicht haben, ein Unentschieden wird gewertet wenn gleich viele Fische in der Freiheit und gefangen sind

Während des spielens ist bereits aufgefallen, dass das Boot gefühlt häufiger gewinnt als die Fische.

# "Forschung"
Klar, hier wird nicht wirklich geforscht, aber verfolgen wir mal einen forschungsähnlichen Ansatz. Wir haben die Hypothese: **Das Boot hat höhere Gewinnchancen als die Fische**

Um nicht selber würfeln zu müssen werden das Spiel und seine Regeln in Programmcode umgesetzt. Zusätzlich zu den gegebenen Regeln wird festgelegt, dass wir **immer den Fisch der am weitesten zurück liegt bewegen**, wenn die Farbe eines bereits freien Fisches gewürfelt wird¹.

*Die Implementation der Spielmechanik ist in diesem Projekt in der `src/main.rs` zu finden, wo die Funktion `tick` eine Spielrunde simuliert.*

Nun wurden 100.000 Runden mit dem Programm simuliert, welche zu folgendem Ergebnis kommen:

| Siege Boot | Boot%  | Siege Fisch | Fisch% | Unentschieden | Tie%   |
| ---------- | ------ | ----------- | ------ | ------------- | ------ |
| 52941      | 52.94% | 35703       | 35.70% | 11356         | 11.36% |

Eindeutig abzulesen ist: **Das Boot gewinnt knapp jedes zweite Spiel, während die Fische nur knapp alle drei Spiele gewinnen**

Die Hypothese ist somit also bestätigt, das Boot gewinnt häufiger.

# Lösungsfindung

Dass das Boot in **fünf von zehn** Fällen gewinnt erschien uns als unfair, jeder Spieler mit dem Wissen über diesen Fakt kann Enkel, Neffen, Nichten oder eigene Kinder über den Tisch ziehen, indem sie immer auf das Boot wetten.

Bei der Lösungsfindung sei festgelegt, dass wir nur mit den vorhandenen Materialien arbeiten und das Spielfeld selbst nicht verändern, also weder Flussstücke hinzufügen, noch entfernen.

Der erste spontane Versuch das Spiel auszugleichen war, **zwei Fischen einen Vorsprung von einem Feld zu geben**. Die Ergebnisse einer Simulation von 100.000 Spielen führte zu entsprechendem Ergebnis:

| Siege Boot | Boot%  | Siege Fisch | Fisch% | Unentschieden | Tie%   |
| ---------- | ------ | ----------- | ------ | ------------- | ------ |
| 41222      | 41.22% | 46582       | 46.58% | 12196         | 12.20% |

Eine Verbesserung ist sichtbar. Anstatt dass das Boot eine ~17% höhere Gewinnchance hat, haben die Fische nun eine ~5% höhere Gewinnchance. Diese Annäherung ist bereits sehr gut, uns interessiert jedoch ob dies noch besser geht.

Um Lebenszeit und Nerven zu sparen wurde weiterer Code geschrieben, welcher alle möglichen Kombinationen von Fischpositionen generiert. Hierbei sei gesagt dass es bei 4 Fischen und 11 Feldern in der Theorie 11⁴ mögliche Kombinationen gibt, in der Realität es aber egal ist welcher Fisch auf einem Feld liegt. Das heißt, ob der blaue Fisch alleine auf dem ersten Feld liegt und alle anderen auf dem zweiten, oder der gelbe auf dem ersten Feld liegt und alle anderen auf dem zweiten macht spieltechnisch keinen Unterschied. Entfernt man alle Duplikate ergeben sich **1001 Mögliche Positionen zum platzieren der Fische**.

Für jede dieser Positionen wurden 100.000 Spiele simuliert. Die Ergebnisse dieser Simulation sind [hier](https://gist.github.com/iMarv/e403b8d3a6b76bd823bdb8c395d1deab) zu finden.

# Auswertung & Lösungsvorschlag
Bei der Festlegung auf einen Spielaufbau sind  verschiedene Dinge zu beachten.

Das Spiel soll nicht "kaputt" gemacht werden. Als Paradebeispiel gilt hier das Top 1 Ergebnis unserer [Ergebnisliste](https://gist.github.com/iMarv/e403b8d3a6b76bd823bdb8c395d1deab):

```
0100001100010 | B: 40126 ( 40.13%, R: 15) | F: 39933 ( 39.93%, R: 21) | T: 19941 ( 19.94%, R: 19)
```

Die Gewinnchancen sind hier nahezu optimal ausgeglichen, jedoch wird das Regelwerk ad absurdum geführt. Wir haben einen Fisch der in den allermeisten Fällen **direkt geangelt** wird und wir haben einen Fisch die in den allermeisten Fällen **direkt ins Meer** entflieht. Der Kern des Spiels dreht sich also nur um die verbleibenden zwei Fische. Wir wollen dieses Spiel mit vier Fischen spielen, daher ist dieser Aufbau vielleicht **stochastisch am besten**, entfernt sich jedoch **zu weit vom Spielgedanken**.

Weiterhin möchten wir das Spiel auch mit Kindern spielen. Nehmen wir hier **Platz 10** als Beispiel:

```
0001001101000 | B: 43312 ( 43.31%, R: 17) | F: 39654 ( 39.65%, R: 26) | T: 17034 ( 17.03%, R: 22)
```

Die Fische sind relativ gut verteilt, keiner rettet sich direkt, keiner wird direkt geangelt. Mit ~4% Unterschied sind die Siegchancen auch noch relativ fair verteilt.

Hier greift eher ein **praktisches Problem**: Bis man den Kindern erklärt hat warum man die Fische in diesem Muster aufgebaut hat, hätte man schon die ersten 5 Runden spielen können — vorausgesetzt, dass die Kinder nach der Erklärung noch Interesse an dem Spiel haben.

## Vorschläge
Orientiert an diesen Ansätzen bieten sich folgende Kombinationen an um das Spiel ausgeglichener zu gestalten:

```
0000002200000 | B: 41222 ( 41.22%, R: 19) | F: 46582 ( 46.58%, R: 27) | T: 12196 ( 12.20%, R: 25)
```
Überraschung! Unser handgepickter Lösungsversuch stellt sich als Platz 19 heraus! Wir haben nur knapp 5% Unterschied bei der Gewinnchance und das Spielfeld sieht auch ganz vernünftig aus.

```
0000011110000 | B: 40258 ( 40.26%, R: 18) | F: 46109 ( 46.11%, R: 26) | T: 13633 ( 13.63%, R: 24)
```
Platz 26, rund 6% Unterschied und die Chance dass die Kinder diesen Aufbau selber schon versucht haben sind gleichermaßen hoch.

```
0000101101000 | B: 38647 ( 38.65%, R: 17) | F: 45338 ( 45.34%, R: 26) | T: 16015 ( 16.02%, R: 23)
```
Platz 36, 7% Unterschied und auch ein schickes Muster welches nicht groß erklärt werden muss.

# Schlusswort
Ein sehr interessantes Dienstagabendprojekt. Es war sehr überraschend wie stark sich kleine Änderungen im Spielfeldaufbau auf das Gesamtergebnis ausgewirkt haben.

Zusätzlich ist es auch interessant zu beobachten wie sich die durchschnittliche Rundenzahl entwickelt, welche benötigt wird um entsprechende Spielenden zu erreichen (diese wurde in der Zusammenfassung nicht erwähnt, ich empfehle aber mal einen Blick in die Daten, Dort sind auch spannende Sprünge zu beobachten).

# Bonus
## Uninteressantes
- Der in den Regeln empfohlene Aufbau befindet sich in der sortierten und gefilterten Liste auf Platz 70
- 100.000 Simulationen sind bewusst gewählt, bei 10.000 hatten wir noch zu viele Abweichungen zwischen den Ergebnissen
- Prozentwerte anzuzeigen wenn die Anzahl der Simulationen ein vielfaches von 10 ist, macht wenig Spaß, hilft aber trotzdem
- Dass der Quellcode ein wilder Mix aus deutsch und englisch ist, ist als Gag zu verstehen
- Das ganze Projekt ist vermutlich völlig overengineered
## Unethisches
Eher unauffällige Fischverteilungen, zumindest so unauffällig dass die Kinder das nicht hinterfragen. Einfach entsprechend der Wettvorlieben der Kinder einen Aufbau wählen wählen:

**Pro Boot**:
```
0001101100000 | B: 73619 ( 73.62%, R: 16) | F: 15603 ( 15.60%, R: 32) | T: 10778 ( 10.78%, R: 26)
```

```
0000111100000 | B: 63795 ( 63.80%, R: 18) | F: 24645 ( 24.65%, R: 30) | T: 11560 ( 11.56%, R: 26)
```

**Pro Fisch**:
```
0000101010100 | B: 24163 ( 24.16%, R: 17) | F: 57417 ( 57.42%, R: 23) | T: 18420 ( 18.42%, R: 20)
```

```
0000001111000 | B: 18808 ( 18.81%, R: 18) | F: 69898 ( 69.90%, R: 22) | T: 11294 ( 11.29%, R: 22)
```



---
¹ *Zur Vereinfachung. Theoretisch kann man die Gewinnchancen der Fische noch minimal erhöhen indem man "schlaue" Züge einführt, z.B. dass man wenn zwei Fische bereits gerettet sind versucht den nächsten Fisch noch über die Ziellinie zu bringen*

