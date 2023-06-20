# Kata Triple Triads

## Initialisation de la partie
- 2 joueurs requis
- Chaque joueur possèdes un deck de carte généré aléatoirement.
- Chaque joueur est représenté par une couleur
- Chaque joueur doit choisir 5 cartes de son deck.
- Les cartes sont composé de 4 valeurs situées aux 4 points cardinaux pouvant aller de 1 -> 9 et 10 étant A.
- Les cartes ne peuvent pas être tournées.
- Le plateau de jeu est constitué de 3 lignes et 3 colonnes devant contenir les cartes jouées
- Le joueur de départ est choisi au hasard
- Choix des règles :
    - Défaut :
        - La main adverse est cachée
        - Chacun joue son tour à tour de rôle
        - Une carte capture la carte adverse si elle est plus forte que celle de l'adversaire sur son côté en opposition.
        - Capture c'est le fait de gagné la carte adversaire en prenant la couleur du joueur qui a jouer la carte gagnante.
        - La victoire est déterminée quand le plateau est rempli au nombre de cartes capturées.
    - Optionelles:
        - Open : 
            - Les 2 joueurs jouent a main ouverte (les cartes sont visibles des 2 côtés)
        - Combo :
            - Si une carte est capturée et se trouve à côté d'une carte adverse non capturée et qu'elle est plus forte que celle-ci, alors la carte adverse est capturée.
        - Elemental : 
            - La carte obtient un bonus +1 contre une carte dont son élément lui est inférieur.
        - Similaire :
            - Si une carte  les mêmes valeurs au minimum (2 valeurs) que la carte opposé au même point cardinaux alors la carte opposé est capturée (la règles combo peut s'appliquée aussi, effet cumulée)

## Déroulement d'une partie
- Le joueur courant doit placer une carte sur le terrain et passe son tour.
- La fin de la partie est quand toute les cases du plateau de jeu sont remplie.


## Génération aléatoire d'une carte
La moyenne d'une carte doit ce situer entre 15-25.
- On détermine la valeur de la carte aléatoirement entre 15 et 25.
- On génère une première valeur entre 1 et 10.
- Tu soustraits ta première valeur a la valeur entre 15 et 25 (Vt)
- On génère une deuxième valeur entre 1 et (Vc-Vt-Cr) Ex: C1 = 1, Valeur carte = 18, alors 18 - 1 - 2 = min(15, 10)

## Implementation
- TDD
- Composition
- Rust : Tout est immutable utiliser les paradigmes fonctionnel.
- Raylib pour faire une interface graphique


## Textual display

  -------------------------
  |   3   |   1   |   2   |
3 | 1 o 1 | 2 x 3 | 2 o 2 |
  |   1   |   3   |   2   |
  -------------------------
  |   3   |       |       |
2 | 1 x 1 |       |       |
  |   1   |       |       |
  -------------------------
  |   3   |   1   |   2   |
1 | 1 o 1 | 2 x 3 | 2 o 2 |
  |   1   |   3   |   2   |
  -------------------------
      A       B       C

o: 4 cards | x: 3 cards