# GuessWho
Projet L2

Pour lancer le generateur
1) installez une version recente de rust https://www.rust-lang.org/tools/install
2) ouvrez un terminal dans le dossier generator et tapez cargo run

un fichier json jj.json est fourni dans ce dossier et peut etre importé avec le bouton importer pour avoir une base à modifier

sinon vous pouvez commencer de zero (un attribut "Nom" est obligatoire)

pour ajouter des attributs, tapez du texte dans la boite Ajout de gauche et cliquez sur Create de gauche

pour ajouter des options pour un attribut, cliquez sur l'attribut, tapez du texte dans la boite Ajout de droite et cliquez sur Create de droite

Cliquez ensuite sur l'onglet Personnages pour remplir les cases de personnages avec leurs attributs à l'aide des menus déroulants

le bouton à gauche des menus permet d'importer les images qui doivent etre prealablement placées dans l'arborescence du projet

vous pouvez ensuite exporter votre planche dans un fichier avec le bouton Export et si vous l'enregistrez dans la racine du projet, elle sera automatiquement importée quand vous lancerez le jeu

====================================

Pour lancer le jeu
1) installez une version recente de rust https://www.rust-lang.org/tools/install
2) ouvrez un terminal dans ce dossier et tapez cargo run
3) amusez vous

les fichiers json valides inclus dans ce dossier sont automatiquement importés mais vous pouvez en charger d'autres pendant que le jeu tourne 

un fichier json valide ressemble à ça:

<pre>
{
  "attrs": {"nom":["nom1", "nom2"], "attribut1":["possibilité1", "possibilité2"], "attribut2":["possibilité1"]},
  "liste":[
    {
      "nom":"nom1",
      "attribut1": "possibilité1",
      "attribut2": "possibilité1",
      "image": "./personnages/nom1.png"
    },
    {
      "nom":"nom2",
      "attribut1": "possibilité2",
      "attribut2": "possibilité1",
      "image": "./personnages/nom2.png"
    },
    (22 autres "{}")
  ]
}</pre>

"attrs" contient tout les attributs et toutes les possibilités pour chaque attribut
"liste" contient 24 blocs, chacun contenant tous les attributs definis dans "attrs" suivi de "image" avec le chemin de l'image depuis ce dossier ou un chemin absolu

====================================

Pour lancer le serveur
ouvrez un terminal dans le dossier hello-rocket et tapez cargo run
si vous voulez joueur sur une ou plusieurs machine séparées de celle qui execute le serveur, assurer vous d'avoir acces au port 8000
si vous voulez lancer deux clients sur la meme machine vous pouvez juste dupliquer le dossier du projet et faire cargo run dans chaque dossier
