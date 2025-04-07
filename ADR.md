# Architecture Decision Record (ADR)

## 1. Utilisation du Pattern Trait pour les Robots

### Contexte
Besoin d'implémenter différents types de robots (Explorer et Harvester) avec des comportements communs et spécifiques.

### Décision
Utilisation d'un trait `Robot` comme abstraction principale avec des implémentations spécifiques pour chaque type de robot.

### Conséquences
- ✅ Extensibilité : Facilité d'ajout de nouveaux types de robots
- ✅ Réutilisation du code : Comportements communs définis dans le trait
- ✅ Polymorphisme : Manipulation uniforme des différents types de robots
- ❌ Complexité accrue pour la gestion des types dynamiques

### Code Example
```rust
pub trait Robot: Any {
    fn new(x: usize, y: usize, id: usize) -> Self where Self: Sized;
    fn get_position(&self) -> (usize, usize);
    fn update(&mut self, map: &mut Map);
    // ...
}
```

## 2. Gestion de la Concurrence avec Arc et RwLock

### Contexte
Nécessité de partager l'état de la simulation entre plusieurs threads de robots tout en garantissant la sécurité des accès.

### Décision
- Utilisation de `Arc<RwLock<Map>>` pour la carte partagée
- `Arc<Mutex<>>` pour les ressources partagées
- `Arc<AtomicBool>` pour les flags de contrôle

### Conséquences
- ✅ Thread-safety garantie
- ✅ Accès concurrent sécurisé à la carte
- ✅ Prévention des data races
- ❌ Overhead de performance dû aux verrous
- ❌ Risque potentiel de deadlocks

### Code Example
```rust
pub struct Simulation {
    pub map: Arc<RwLock<Map>>,
    pub energy_count: Arc<Mutex<u32>>,
    pub running: Arc<AtomicBool>,
    // ...
}
```

## 3. Interface Graphique avec Iced

### Contexte
Besoin d'une interface graphique réactive pour visualiser et contrôler la simulation.

### Décision
Utilisation du framework Iced pour l'interface utilisateur.

### Conséquences
- ✅ API Rust native et type-safe
- ✅ Modèle de mise à jour réactif
- ✅ Performance et légèreté
- ❌ Écosystème moins mature que d'autres frameworks

### Code Example
```rust
impl Application for MapWindow {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    // ...
}
```

## 4. Génération Procédurale de l'Environnement

### Contexte
Nécessité de créer un environnement varié et intéressant pour la simulation.

### Décision
- Utilisation de l'algorithme de Perlin Noise pour la génération du terrain
- Placement aléatoire des ressources avec des contraintes

### Conséquences
- ✅ Environnements généré en fonction d'une seed
- ✅ Distribution naturelle des ressources
- ✅ Contrôle sur la densité des éléments
- ❌ Paramétrage complexe pour des résultats optimaux

### Code Example
```rust
fn generate_terrain(&mut self) {
    let perlin: Perlin = Perlin::new(self.seed);
    // ...
}
```

## 5. Architecture Modulaire

### Contexte
Besoin d'organiser le code de manière maintenable et évolutive.

### Décision
Structure du projet en modules distincts :
- `environment/` : Gestion de l'environnement
- `robots/` : Logique des robots
- `simulation/` : Gestion de la simulation
- `windows/` : Interface utilisateur

### Conséquences
- ✅ Séparation claire des responsabilités
- ✅ Facilité de maintenance
- ❌ Nécessité de gérer les dépendances entre modules

## 6. Gestion des États des Robots

### Contexte
Les robots doivent gérer différents états et transitions.

### Décision
Utilisation d'une énumération pour les états des robots :
```rust
pub enum RobotState {
    Exploring,
    Harvesting,
    ReturningToBase,
    Reporting,
    Idle,
}
```

### Conséquences
- ✅ États clairement définis et exhaustifs
- ✅ Transitions d'états type-safe
- ✅ Facilité d'ajout de nouveaux états
- ❌ Complexité de gestion des transitions

## 7. Communication Inter-Robots

### Contexte
Besoin de coordination entre les robots explorateurs et récolteurs.

### Décision
Utilisation d'une file d'attente thread-safe pour les ressources découvertes :
```rust
pub located_resources: Arc<Mutex<VecDeque<Vec<(usize, usize, Resource)>>>>
```

### Conséquences
- ✅ Communication asynchrone
- ✅ Découplage entre explorateurs et récolteurs
- ✅ Gestion ordonnée des ressources
- ❌ Complexité de la synchronisation 