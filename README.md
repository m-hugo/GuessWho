# GuessWho
Projet L2

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
