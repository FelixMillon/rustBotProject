# 🚀 RustBotProject
  
Ce projet est un jeu de simulation minimaliste où des éclaireurs et des récolteurs collectent des ressources sur une carte générée aléatoirement.  
Il est composé d'une API en **Rust (Axum)** pour la logique du jeu, et d'une interface en **React** pour la visualisation en temps réel.  
  
---  
  
## 🧱 Technologies utilisées  
  
- 🦀 [Rust](https://www.rust-lang.org/) avec [Axum](https://docs.rs/axum/latest/axum/) pour le backend  
- ⚛️ [React](https://reactjs.org/) pour le frontend  
- 🐳 [Docker Compose](https://docs.docker.com/compose/) pour l'orchestration des services  
  
---  
  
## 🗂️ Structure du projet  
  
```
.
├── back/                  # Code source backend (Rust + Axum)
│   ├── src/
│   ├── Cargo.toml
│   └── Dockerfile
├── front/                 # Interface utilisateur (React)
│   ├── public/
│   ├── src/
│   ├── package.json
│   └── Dockerfile
├── docker-compose.yml     # Orchestration des services
└── README.md
```  
  
---  
  
## ⚙️ Installation locale sans Docker  
  
### 🧪 Prérequis  
  
- Rust >= 1.80 : https://www.rust-lang.org/tools/install  
- Node.js >= 16 : https://nodejs.org/
  
### 🔧 Backend
  
```bash
cd back
cargo run
```  
  
> L’API sera disponible sur `http://localhost:3001`  
  
### 🌐 Frontend  
  
```bash
cd front
npm install
npm start
```  
  
> L’interface sera disponible sur `http://localhost:3000`  
  
---  
  
## 🐳 Démarrage avec Docker  
  
### 🧱 Build & Lancement  
  
```bash
docker compose up --build
```  
  
- Frontend : http://localhost:3000  
- Backend : http://localhost:3001  
  
---  
  
## 📡 Endpoints API (Rust / Axum)  
  
| Méthode | Endpoint   | Description                                         |
|---------|------------|-----------------------------------------------------|
| GET     | `/state`   | Récupère l'état actuel de la partie                 |
| POST    | `/start`   | Démarre une nouvelle partie                         |
| POST    | `/reset`   | Réinitialise la partie avec de nouveaux paramètres  |
| POST    | `/stop`    | Stoppe la partie en cours                           |
  
### Exemple de payload `/reset` ou `/start` :  
  
```json
{
  "columns": 20,
  "rows": 20,
  "gatherers": 5,
  "scouts": 3,
  "resources": 15,
  "seed": 123,
  "empty_display": " ",
  "obstacle_display": "8",
  "base_display": "#"
}
```  
  
---  
  
## 🎮 Gameplay & Interface  
  
- La carte est affichée sous forme de grille.  
- Icônes utilisées :  
  - 🏰 : Base  
  - 👷 / 👨‍🌾 : Récolteur  
  - 🛰️ / 🕵️‍♂️ : Éclaireur  
  - 💎 : Cristal  
  - ⚡️ : Énergie  
  - 🌳 : Obstacle  
- Contrôles :  
  - ▶️ / ⏸️ : Pause ou reprise de la simulation  
  - ⏪ / ⏩ : Modifier la vitesse de simulation  
  - 🚀 : Démarrer la partie  
  - 🔄 : Réinitialiser  
  - ❌ : Stopper  
  
---  
  
## 🛠️ Personnalisation  
  
Les paramètres comme la taille de la carte, le nombre d’agents ou encore le seed aléatoire peuvent être modifiés depuis l’interface via un popup.  
  
---  
  
## 🧪 Tests  
  
Pour l’instant, les tests unitaires peuvent être ajoutés dans les fichiers du backend avec :  
  
```bash
cargo test
```  
  
---  
  
## 📦 Roadmap  
  
- ✅ Simulation basique en temps réel  
- ✅ Gestion multi-agents (scouts / gatherers)  
- ✅ Interface visuelle dynamique  
- ⏳ IA des agents plus avancée  
- ⏳ Animation frontend  
- ⏳ Persistence via base de données  
- ⏳ Authentification & sauvegarde  
  
---  
  
## 🧑‍💻 Auteurs  
  
- 💡 Projet conçu par Felix MILLON, Elone MACCIONI, Ilyes BOULKRINAT, Ian GALMICHE
- 🛠️ Contributions bienvenues !  
  
---  
  
## 📜 Licence  
  
Ce projet est sous licence MIT. Voir le fichier `LICENSE` pour plus d'informations.  
