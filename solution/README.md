# Filler Game: AI Territory Domination

Bienvenue dans **Filler**, un jeu algorithmique stratégique où deux robots s'affrontent pour dominer un champ de bataille 2D appelé *Anfield*. L'objectif est simple : conquérir la plus grande zone en plaçant des pièces aux formes aléatoires, tout en respectant des règles strictes. Votre défi ? Construire un robot IA capable de battre tous nos robots—sauf le légendaire

---

## 🌟 Fonctionnalités

- **Gameplay Dynamique** : Des pièces générées aléatoirement pour rendre chaque partie unique.
- **Stratégie au Tour Par Tour** : Les robots placent leurs pièces en alternance.
- **Défi IA Complexe** : Affrontez des robots IA avancés dans Docker.
- **Cartes Personnalisables** : Utilisez les cartes fournies ou créez les vôtres.
- **Système de Points** : Dominez l'Anfield en revendiquant la plus grande zone.
- **Objectifs Bonus** :
  - Créer un visualiseur graphique.
  - Construire un bot capable de battre **Terminator**.

---

## 🚀 Installation et Lancement

### Installation
1. Téléchargez et dézippez le dossier **docker_image** via --> https://assets.01-edu.org/filler/filler.zip

2. Naviguez dans le dossier et construisez l'image Docker :  
   ```bash
   docker build -t filler .
3. Lancez le conteneur Docker .

        --> docker run -v "$(pwd)/solution":/filler/solution -it filler

## Construction de la Solution
    Une fois dans le conteneur Docker, construisez la solution en utilisant Cargo :

       -->  cd solution/solution/src
       -->  cargo build --release

## Lancer une Partie

### Depuis le conteneur :
   **## entre les robots** Exp:: ./game_engine -f maps/map01 -p1 m1_robots/bender -p2 m1_robots/terminator
   
   **## ou entre l' etudiant et les robots** Exp:: ./game_engine -f maps/map01 -p1 solution/target/release/solution -p2 m1_robots/terminator


## Options Disponibles
    -f : Spécifie le fichier de la carte.
    -p1, -p2 : Chemins vers les exécutables des joueurs.
    -s : Utilise une graine aléatoire.
    -t : Définit un délai d'exécution (par défaut : 10 secondes).


# 🧠 Développement de l'IA

## Implémentation de la Logique
- Lire l'Anfield et les détails de la pièce.
- Déterminer les placements valides selon les règles.
- Optimiser pour maximiser la domination de la surface.
## Script Joueur
Répondre aux entrées du moteur avec les coordonnées.
## Gestion des Erreurs
Toujours retourner un résultat valide (0 0\n si aucun coup n'est possible).

# 📋 Exemple

## Anfield Initial
    Anfield 20 15:
        01234567890123456789
    000 ....................
    001 ....................
    002 .........@..........
    003 ....................
    004 ....................
    005 ....................
    006 ....................
    007 ....................
    008 ....................
    009 ....................
    010 ....................
    011 ....................
    012 .........$..........
    013 ....................
    014 ....................
    Piece 4 1:
    .OO.

## Réponse du Joueur
    7 2\n