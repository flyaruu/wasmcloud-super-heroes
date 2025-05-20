use std::time::SystemTime;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{heroes::SqlHero, location::SqlLocation, villains::SqlVillain};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fighters {
    pub hero: SqlHero,
    pub villain: SqlVillain,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FightRequest {
    pub hero: SqlHero,
    pub villain: SqlVillain,
    pub location: SqlLocation,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FightResult {
    id: String,
    fight_date: String, // use timestamp data structure?
    winner_name: String,
    winner_level: i32,
    winner_powers: String,
    winner_picture: String,
    loser_name: String,
    loser_level: i32,
    loser_powers: String,
    loser_picture: String,
    winner_team: String,
    loser_team: String,
    location: SqlLocation,
}

#[derive(Clone, Copy)]
pub enum Winner {
    Heroes,
    Villains,
}

// {
//     "id": "67dd8fbe355b694fbec93dee",
//     "fightDate": "2025-03-21T16:11:42.316336007Z",
//     "winnerName": "Desghidorah (Rebirth of Mothra)",
//     "winnerLevel": 1000,
//     "winnerPowers": "Animal Attributes, Cold Resistance, Energy Beams, Fire Control, Fire Resistance, Flight, Large Size, Lava Manipulation, Longevity, Magma Manipulation, Natural Armor, Natural Weapons, Toxin and Disease Resistance, Electricity Resistance, Electrokinesis, Endurance, Energy Absorption, Energy Blasts, Enhanced Senses, Explosion Manipulation, Heat Generation, Invulnerability, Self-Sustenance, Spaceflight, Death Touch, Energy Resistance, Radiation Immunity, Terrakinesis, Transformation, Weather Control",
//     "winnerPicture": "https://raw.githubusercontent.com/quarkusio/quarkus-super-heroes/characterdata/images/desghidorah-(rebirth-of-mothra)-7621412497910022678.jpg",
//     "loserName": "Jarad",
//     "loserLevel": 719,
//     "loserPowers": "Dark Magic, Dexterity, Enhanced Hearing, Marksmanship, Necromancy, Reflexes, Toxin and Disease Resistance, Zombie Physiology, Accelerated Healing, Agility, Death Touch, Endurance, Enhanced Sight, Gadget Usage, Grappling/Climbing, Hunters Instinct, Jump, Longevity, Possession Resistance, Toxin and Disease Control, Weapon-based Powers, Acrobatics, Animal Control, Curse Manipulation, Curse Resistance, Energy Blasts, Energy Manipulation, Immortality, Indomitable Will, Information Analysis, Magic, Magic Resistance, Plant Control, Possession, Shadow Manipulation, Vitakinesis",
//     "loserPicture": "https://raw.githubusercontent.com/quarkusio/quarkus-super-heroes/characterdata/images/jarad-vod-savo--2332551804562092846.jpg",
//     "winnerTeam": "villains",
//     "loserTeam": "heroes",
//     "location": {
//         "name": "Tyr",
//         "description": "Located in the Tablelands, Tyr is one of the city-states in the harsh world of Athas. Tyr is known for its oppressive sorcerer-king, Kalak, who rules the city with an iron fist.",
//         "picture": "https://raw.githubusercontent.com/quarkusio/quarkus-super-heroes/characterdata/images/locations/tyr.jpg"
//     }
// }
