# Retrouver QNAP2, découvrir QRUNAP2

[Read in English](guide.md)

Vous pouvez commencer sans vous souvenir de la syntaxe. Dans l'atelier, cliquez sur **Exécuter cet exemple**, changez un nombre, puis relancez. Le modèle s'exécute localement dans votre navigateur. L'anglais est la langue par défaut ; le bouton FR donne accès à l'interface française.

## 1. À quoi cela sert-il ?

Imaginez un guichet recevant des demandes. Elles arrivent, attendent si le guichet est occupé, sont traitées puis repartent. Vous cherchez à connaître leur attente, l'occupation du guichet et l'intérêt d'ajouter un deuxième agent. Le même raisonnement s'applique aux processeurs, disques, ateliers ou flux de transport.

Un **client** est l'objet traité : requête, tâche, personne ou message. Une **station** associe un service à sa file d'attente. Un **serveur** est une unité de capacité de traitement dans cette station. Deux serveurs peuvent partager une file. Un **réseau** relie les stations par des règles de circulation des clients.

QNAP2 associait ces objets à un langage algorithmique et à des commandes d'exécution. Il permettait aussi des attributs de clients, des synchronisations et des comportements programmés. Les [sources historiques](sources.md), notamment S3 et S8, montrent cette richesse.

## 2. Relire un petit modèle

Voici un exemple pédagogique original dans le sous-ensemble pris en charge, pas la transcription d'un manuel.

```text
/DECLARE/
QUEUE INPUT, DESK;
/STATION/
NAME=INPUT;
TYPE=SOURCE;
SERVICE=EXP(2);
TRANSIT=DESK;
/STATION/
NAME=DESK;
TYPE=SERVER,SINGLE;
SERVICE=EXP(1);
TRANSIT=OUT;
/CONTROL/
TMAX=10000;
/EXEC/
BEGIN
SIMUL;
END;
/END/
```

| Ce qu'on lit | Ce que cela signifie |
| --- | --- |
| /DECLARE/ QUEUE INPUT, DESK; | Nommer les stations. QUEUE désigne ici l'objet station, pas seulement les clients qui attendent. |
| /STATION/ NAME=INPUT; | Commencer la description de INPUT. |
| TYPE=SOURCE; | Créer des clients provenant de l'extérieur. |
| SERVICE=EXP(2); sur INPUT | Espacer les arrivées selon une loi exponentielle de moyenne 2. |
| TRANSIT=DESK; | Envoyer chaque nouveau client vers DESK. |
| TYPE=SERVER,SINGLE; | Traiter un client à la fois ; les autres attendent. |
| SERVICE=EXP(1); sur DESK | Donner à chaque service une durée exponentielle de moyenne 1. |
| TRANSIT=OUT; | Faire sortir le client du réseau après son service. |
| /CONTROL/ TMAX=10000; | Arrêter après 10 000 unités de temps simulé. |
| /EXEC/ BEGIN SIMUL; END; | Lancer la simulation. |
| /END/ | Terminer le programme source. |

`EXP(2)` indique une **durée moyenne de 2**, pas deux arrivées par unité de temps. Le taux correspondant est 1/2. Cette convention est explicitée dans la thèse de Place, page imprimée III-84 ([S9](sources.md)) ; sa vérification dans le manuel complet 9.3 reste à faire. `CST(2)` désigne une durée constante de 2. Choisissez une même unité partout : secondes, minutes ou autre.

Pour une SOURCE, SERVICE décrit l'espacement des arrivées. Pour un SERVER, il décrit le temps de traitement. C'est un repère essentiel pour relire un ancien modèle.

## 3. Que fait le bouton Exécuter ?

Rust analyse et vérifie le texte, construit le réseau puis avance une horloge virtuelle d'événement en événement. Il n'attend pas 10 000 secondes réelles. Une arrivée ou une fin de service est un événement. Le moteur mesure l'occupation, les files et les visites terminées.

La graine détermine les tirages aléatoires. Une même graine permet de reproduire l'expérience ; une autre donne une nouvelle réalisation. Le générateur est nouveau : il ne reproduit pas les anciens tirages de QNAP2.

## 4. Lire les résultats

| Indicateur | Sens concret |
| --- | --- |
| Occupation | Part du temps disponible des serveurs consacrée au travail. |
| L | Nombre moyen dans la station, service compris. |
| Lq | Nombre moyen en attente, hors service. |
| W | Temps moyen dans la station pour les visites terminées : attente et service. |
| Wq | Attente moyenne de ces visites terminées. |
| Débit | Visites terminées par unité de temps. |

Dans le premier exemple, les valeurs théoriques à long terme sont une occupation de 50 %, L=1 et W=2. Une simulation fluctue autour de ces valeurs. Une station peut avoir une capacité moyenne disponible et néanmoins faire attendre, car arrivées et services sont irréguliers.

Le démarrage est inclus dans les moyennes. Les visites inachevées à l'arrêt ne contribuent pas à W ni Wq. Aucun intervalle de confiance n'est encore calculé. Voir les [définitions techniques en anglais](semantics.md).

## 5. Cinq expériences pour retrouver l'intuition

| Exemple | Manipulation | Ce qu'il faut observer |
| --- | --- | --- |
| Première file | Exécuter puis changer la graine. | Des moyennes proches, des trajectoires différentes. |
| Durées constantes | Arrivées CST(2), service CST(1), horizon 20. | Aucune attente : 10 arrivées, 9 sorties, un client encore en service. |
| Deux serveurs | MULTIPLE(2), arrivées EXP(0.75). | Deux services simultanés ; chaque service conserve sa durée. |
| Guichet surchargé | Arrivées EXP(0.8), un serveur EXP(1). | La demande dépasse la capacité ; prolonger ne crée pas d'équilibre. |
| INRIA 1995, adapté | 40 clients circulent A → B → A. | Un réseau fermé démarre avec des clients initiaux et conserve sa population. |

Le dernier exemple reconstruit la topologie d'une véritable [exécution publiée par l'INRIA](https://www-sop.inria.fr/sloop/qnaprep/output.html). La saisie interactive et les réplications parallèles sont remplacées. Les [modifications sont documentées](sources.md) : ce n'est pas le programme original inchangé.

## 6. Le langage historique comporte trois niveaux

Pour ouvrir un ancien fichier QNAP2, distinguez ces niveaux :

| Niveau | Son rôle | Repères de lecture |
| --- | --- | --- |
| Description du réseau | Déclarer les objets et décrire les stations. | QUEUE, NAME, TYPE, SERVICE, TRANSIT |
| Comportement algorithmique | Calculer et exprimer un comportement plus riche qu'une simple durée. | Variables, affectation, conditions, blocs, références de clients et synchronisation |
| Pilotage de l'exécution | Préparer et lancer une expérience, dialoguer et demander des résultats. | /CONTROL/, /EXEC/, GET, PRINT, SIMUL |

Dans le programme historique de l'INRIA, `=` introduit les propriétés des stations ou du contrôle, tandis que `:=` affecte une valeur dans les instructions exécutables. INTEGER et REAL déclarent des variables numériques ; BEGIN et END délimitent un bloc. GET lit une valeur et PRINT affiche des informations. Ce sont des repères de lecture, mais **les variables, affectations et instructions interactives ne sont pas encore prises en charge dans cet atelier**. Voir [S8](sources.md).

Un SERVICE historique pouvait contenir un algorithme au lieu d'une durée. Une référence de client désigne un client particulier ; les drapeaux permettent de synchroniser des activités. Le programme de circulation urbaine illustre ce style plus riche ([S3](sources.md)). Remplacer ces mécanismes par des durées fixes changerait le modèle.

Une simulation produit des observations à partir d'une succession d'événements. Une méthode analytique calcule des résultats par des équations sous certaines hypothèses. QNAP2 proposait les deux ; cette version de QRUNAP2 implémente uniquement la simulation. Une formule dans le guide est une référence pédagogique, pas un solveur historique implémenté.

## 7. Comment QNAP2 est-il adapté ?

Les noms historiques restent visibles : QUEUE, STATION, SERVICE, TRANSIT et SIMUL. Le code est réécrit en Rust. Le même moteur fonctionne en ligne de commande ou en WebAssembly dans une tâche de fond du navigateur. GitHub Pages héberge des fichiers statiques ; les visiteurs n'ont besoin ni de serveur de simulation ni de compte.

L'éditeur web, la graine, l'export JSON, les deux langues, les limites de ressources et les diagnostics sont des outils modernes. L'ordre FIFO, le démarrage des sources et l'ordre des événements simultanés sont des règles explicites de QRUNAP2 dont les détails historiques restent à vérifier.

Cette version traite de petits réseaux ouverts et fermés, avec durées constantes ou exponentielles et serveurs simples ou multiples. Variables, boucles, IF, références, drapeaux, classes de clients, routage probabiliste et solveurs analytiques historiques ne sont pas implémentés. Une construction non prise en charge produit une erreur. La [matrice de compatibilité en anglais](compatibility.md) précise les limites.

## 8. Reprendre concrètement

Commencez par la première file, modifiez uniquement l'espacement des arrivées et observez l'occupation et l'attente. Ajoutez ensuite un deuxième serveur. Explorez enfin le réseau fermé et ralentissez B en augmentant sa durée moyenne de service. Expliquez le résultat avec vos propres mots avant de changer un autre paramètre : les idées centrales reviennent ainsi avant les algorithmes et synchronisations historiques.
