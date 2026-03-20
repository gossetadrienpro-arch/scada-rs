# scada-rs

![CI](https://github.com/gossetadrienpro-arch/scada-rs/actions/workflows/ci.yml/badge.svg)

Backend SCADA écrit en Rust simulant un environnement industriel (PLC) via Modbus TCP, avec une API temps réel et des démonstrations d’attaques de cybersécurité OT.

---

## Objectif

Ce projet a pour but de :

* Comprendre et implémenter un protocole industriel bas niveau (Modbus TCP)
* Simuler un environnement OT réaliste avec un automate (PLC)
* Explorer des vulnérabilités courantes en cybersécurité industrielle
* Construire une architecture Rust modulaire et asynchrone

---

## Fonctionnalités

* Parser Modbus TCP from scratch
  Implémentation complète du parsing de trames binaires big-endian

* Simulateur de PLC
  Registres dynamiques avec valeurs générées aléatoirement

* Serveur TCP asynchrone
  Gestion de connexions multiples avec Tokio

* API REST
  Endpoint `GET /tags` retournant les données en JSON

* WebSocket
  Streaming de données en temps réel (rafraîchissement chaque seconde)

* Cybersécurité OT
  Détection de replay attack, rate limiting, audit logging

* Crate attacker
  Démonstration de plusieurs attaques Modbus réalistes

---

## Architecture

```
scada-rs/
├── Cargo.toml          # workspace root
├── crates/
│   ├── scada-core/     # types partagés (Tag, Value, Error)
│   ├── modbus/         # implémentation du protocole Modbus TCP
│   ├── simulator/      # simulateur de PLC
│   ├── server/         # API REST + WebSocket + TCP Modbus
│   └── attacker/       # démonstration d’attaques OT
├── rustfmt.toml
└── .gitignore
```

---

## Attaques OT démontrées

Ces attaques reproduisent des vulnérabilités réelles observées dans les systèmes industriels :

* Lecture non autorisée (function code 0x03)
  Accès aux registres sans authentification

* Écriture malveillante (function code 0x06)
  Modification de l’état du PLC

* Replay attack
  Rejeu de trames réseau capturées

---

## Lancer le projet

```bash
git clone https://github.com/gossetadrienpro-arch/scada-rs
cd scada-rs
cargo build

# Terminal 1
cargo run -p server

# Terminal 2
cargo run -p attacker
```

---

## Exemple d’utilisation

Récupération des tags via l’API REST :

```bash
curl http://localhost:3000/tags
```

Exemple de réponse :

```json
[
  {"id": 1, "name": "Température", "value": 850, "address": 40001},
  {"id": 2, "name": "Pression_1", "value": 10, "address": 40002}
]
```

---

## Stack technique

* Rust
* Tokio (runtime asynchrone)
* Axum (framework web)
* Serde (sérialisation JSON)
* Tracing (logging et observabilité)
* Thiserror (gestion des erreurs)
* Rand (génération de données)

---

## Perspectives d’amélioration

* Ajout d’un mécanisme d’authentification sur Modbus
* Interface graphique de supervision (SCADA UI)
* Persistance des données (base de données)
* Simulation de scénarios industriels plus complexes
* Détection d’anomalies basée sur le comportement

---