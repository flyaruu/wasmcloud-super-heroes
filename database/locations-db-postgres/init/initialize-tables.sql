DROP TABLE IF EXISTS locations;

CREATE TYPE location_type AS ENUM (
  'CITY',
  'PLANET',
  'PLACE',
  'ISLAND',
  'COUNTRY',
  'MOON'
);

CREATE TABLE locations (
  id SERIAL PRIMARY KEY,
  description TEXT,
  name VARCHAR(50) NOT NULL,
  picture VARCHAR(255),
  location_type location_type NOT NULL
);

ALTER TABLE locations ADD CONSTRAINT UK_name UNIQUE (name);

INSERT INTO locations(name, description, picture, location_type)
VALUES ('Gotham City', 'An American city rife with corruption and crime, the home of its iconic protector Batman.', 'https://raw.githubusercontent.com/quarkusio/quarkus-super-heroes/characterdata/images/locations/gotham_city.jpg', 'CITY');
INSERT INTO locations(name, description, picture, location_type)
VALUES ('Earth', 'Earth, our home planet, is the only place we know of so far that is inhabited by living things.', 'https://raw.githubusercontent.com/quarkusio/quarkus-super-heroes/characterdata/images/locations/earth.jpg', 'PLANET');
INSERT INTO locations(name, description, picture, location_type)
VALUES ('Krypton', 'An ancient world, Krypton was home to advanced civilization known as Kryptonians. In one reality they possessed superhuman strength among other abilities far beyond those of Earth humans. In the majority of realities, these powers would not appear unless the Kryptonian was offworld or under the light of an alien sun.', 'https://raw.githubusercontent.com/quarkusio/quarkus-super-heroes/characterdata/images/locations/krypton.jpg', 'PLANET');
INSERT INTO locations(name, description, picture, location_type)
VALUES ('Metropolis', 'Metropolis is depicted as being one of the largest and wealthiest cities in the world, having a population of 11 million citizens. In addition to Superman, the city has also been home to other superheroes, such as Booster Gold and Black Lightning.', 'https://raw.githubusercontent.com/quarkusio/quarkus-super-heroes/characterdata/images/locations/metropolis.jpg', 'CITY');
INSERT INTO locations(name, description, picture, location_type)
VALUES ('Alkali Lake', 'Alkali Lake is the location where the secret Weapon X facility run by Colonel Striker was located. Under the lake is a hidden and top secret facility where experiments were done to create Weapon X, and by extention Wolverine.', 'https://raw.githubusercontent.com/quarkusio/quarkus-super-heroes/characterdata/images/locations/Alkali_Lake.jpg', 'PLACE');
INSERT INTO locations(name, description, picture, location_type)
VALUES ('Asgard', 'One of the nine realms that serve as home of the Norse god superheros. A realm that resembles a grand, celestial city, Asgard is a place of towering golden spires, grand halls, and mythical landscapes.', 'https://raw.githubusercontent.com/quarkusio/quarkus-super-heroes/characterdata/images/locations/Asgard.jpg', 'PLACE');
INSERT INTO locations(name, description, picture, location_type)
VALUES ('The Savage Land', 'Located in Antarctica, it is a hidden prehistoric jungle inhabited by dinosaurs and primitive tribes. The Savage Land is a tropical oasis in the frozen continent, thanks to advanced technology and a mysterious energy source. It often serves as a setting for X-Men adventures due to its unique and dangerous environment.', 'https://raw.githubusercontent.com/quarkusio/quarkus-super-heroes/characterdata/images/locations/Savage_Land.jpg', 'PLACE');
INSERT INTO locations(name, description, picture, location_type)
VALUES ('Sakaar', 'A world located in a distant galaxy. Sakaar is a harsh and violent planet characterized by its constant conflict and divided society. It serves as the primary setting for the storyline, where the Hulk becomes a gladiator and eventually leads a rebellion against the oppressive rule of the Red King.', 'https://raw.githubusercontent.com/quarkusio/quarkus-super-heroes/characterdata/images/locations/Sakaar.jpg', 'PLANET');
INSERT INTO locations(name, description, picture, location_type)
VALUES ('The Death Star', 'The Death Star was the Empire''s ultimate weapon: a moon-sized space station with the ability to destroy an entire planet. But the Emperor and Imperial officers like Grand Moff Tarkin underestimated the tenacity of the Rebel Alliance, who refused to bow to this technological terror.', 'https://raw.githubusercontent.com/quarkusio/quarkus-super-heroes/characterdata/images/locations/Death_Star.png', 'PLACE');
INSERT INTO locations(name, description, picture, location_type)
VALUES ('Fortress of Solitude', 'The Fortress of Solitude is Clark Kent''s sanctuary in the Arctic, which holds information about his dead planet Krypton and allows him to communicate with the A.I.''s of his late ancestors. Clark began using the Fortress much more after becoming Superman. Becoming his occasional headquarters as Superman.', 'https://raw.githubusercontent.com/quarkusio/quarkus-super-heroes/characterdata/images/locations/Fortress_of_Solitude.jpg', 'PLACE');
INSERT INTO locations(name, description, picture, location_type)
VALUES ('Atlantis', 'An ancient underwater city, home to the Atlanteans. Ruled by King Arthur Curry, also known as Aquaman, Atlantis is a place of advanced technology and magic. The city is a beacon of hope for its inhabitants and is constantly under threat from both surface dwellers and internal strife.', 'https://raw.githubusercontent.com/quarkusio/quarkus-super-heroes/characterdata/images/locations/Atlantis.jpg', 'CITY');
INSERT INTO locations(name, description, picture, location_type)
VALUES ('Genosha', 'An island nation located off the east coast of Africa. Genosha was once a place of oppression for mutants, but after a revolution, it became a haven for the mutant population. However, its history is marred by conflict and tragedy.', 'https://raw.githubusercontent.com/quarkusio/quarkus-super-heroes/characterdata/images/locations/Genosha.jpg', 'ISLAND');
INSERT INTO locations(name, description, picture, location_type)
VALUES ('Hoth', 'Hoth is an ice planet located in the Outer Rim Territories, known for its freezing temperatures and barren landscape.', 'https://raw.githubusercontent.com/quarkusio/quarkus-super-heroes/characterdata/images/locations/hoth.png', 'PLANET');
INSERT INTO locations(name, description, picture, location_type)
VALUES ('Endor', 'Endor is a forested moon orbiting the gas giant planet of Endor in the Outer Rim Territories. The Ewok village, the Death Star shield generator bunker, and the lush forest terrain. Endor is the location of the Battle of Endor, a critical event in the Star Wars saga, where the Rebel Alliance defeated the Galactic Empire.', 'https://raw.githubusercontent.com/quarkusio/quarkus-super-heroes/characterdata/images/locations/endor.jpg', 'MOON');
INSERT INTO locations(name, description, picture, location_type)
VALUES ('Coruscant', 'Coruscant is a city-covered planet at the heart of the galaxy, serving as the political and cultural center of the Star Wars universe. The Jedi Temple, the Senate Building, and the massive cityscape covering the entire planet.', 'https://raw.githubusercontent.com/quarkusio/quarkus-super-heroes/characterdata/images/locations/coruscant.png', 'PLANET');
INSERT INTO locations(name, description, picture, location_type)
VALUES ('Tatooine', 'Tatooine is a desert planet located in the Outer Rim Territories, known for its twin suns and harsh environment.', 'https://raw.githubusercontent.com/quarkusio/quarkus-super-heroes/characterdata/images/locations/tatooine.jpg', 'PLANET');
INSERT INTO locations(name, description, picture, location_type)
VALUES ('Neverwinter', 'Neverwinter is a city situated on the Sword Coast of the Forgotten Realms campaign setting. Often referred to as the "City of Skilled Hands," Neverwinter is known for its resilience and recovery after the devastating cataclysm of Mount Hotenow.', 'https://raw.githubusercontent.com/quarkusio/quarkus-super-heroes/characterdata/images/locations/neverwinter.jpg', 'CITY');
INSERT INTO locations(name, description, picture, location_type)
VALUES ('Baldur''s Gate', 'Baldur''s Gate is a coastal city on the Sword Coast, part of the Forgotten Realms campaign setting. Known for its strategic location and thriving trade, Baldur''s Gate is a bustling and politically charged city on the verge of the Sea of Swords. The Upper and Lower cities, the Seatower of Balduran, and the Basilisk Gate are prominent landmarks in Baldur''s Gate.', 'https://raw.githubusercontent.com/quarkusio/quarkus-super-heroes/characterdata/images/locations/baldurs_gate.png', 'CITY');
INSERT INTO locations(name, description, picture, location_type)
VALUES ('Greyhawk', 'Greyhawk is one of the earliest and most iconic campaign settings for D&D. The World of Greyhawk is a rich, medieval fantasy world filled with a diverse array of nations, cultures, and adventures. Castle Greyhawk, the setting''s namesake, and the Free City of Greyhawk are central to the campaign setting.', 'https://raw.githubusercontent.com/quarkusio/quarkus-super-heroes/characterdata/images/locations/greyhawkcity.jpg', 'CITY');
INSERT INTO locations(name, description, picture, location_type)
VALUES ('Tyr', 'Located in the Tablelands, Tyr is one of the city-states in the harsh world of Athas. Tyr is known for its oppressive sorcerer-king, Kalak, who rules the city with an iron fist.', 'https://raw.githubusercontent.com/quarkusio/quarkus-super-heroes/characterdata/images/locations/tyr.jpg', 'CITY');
INSERT INTO locations(name, description, picture, location_type)
VALUES ('Tarantia', 'Tarantia is the capital city of the kingdom of Aquilonia in the Hyborian Age, a fictional era in the Conan universe. It is a grand and bustling city, known for its opulent architecture, sprawling markets, and busy streets. The Royal Palace, where the Aquilonian king resides, is a prominent landmark in Tarantia.', 'https://raw.githubusercontent.com/quarkusio/quarkus-super-heroes/characterdata/images/locations/tarantia.jpg', 'CITY');
INSERT INTO locations(name, description, picture, location_type)
VALUES ('Aquilonia', 'Aquilonia is a powerful kingdom in the Hyborian Age and one of Conan''s most significant conquests. King Conan became the ruler of Aquilonia, making it the mightiest kingdom of the age. The capital city, Tarantia, is known for its grandeur and influence.', 'https://raw.githubusercontent.com/quarkusio/quarkus-super-heroes/characterdata/images/locations/aquilonia.png', 'COUNTRY');
INSERT INTO locations(name, description, picture, location_type)
VALUES ('Cimmeria', 'Cimmeria is a northern barbarian homeland in the Hyborian Age, a fictional age in the Conan universe. It''s the homeland of Conan the Barbarian, and the Cimmerians are known for their toughness and warrior culture.', 'https://raw.githubusercontent.com/quarkusio/quarkus-super-heroes/characterdata/images/locations/cimmeria.png', 'COUNTRY');
INSERT INTO locations(name, description, picture, location_type)
VALUES ('Asteroid M', 'Originally situated in Earth''s orbit, later relocated to various positions. Created by Magneto, it was a haven for mutants and a launching point for his various schemes. Asteroid M featured advanced technology and was used as a base of operations for mutant activities.', 'https://raw.githubusercontent.com/quarkusio/quarkus-super-heroes/characterdata/images/locations/asteroid_m.png', 'PLANET');
INSERT INTO locations(name, description, picture, location_type)
VALUES ('Avalon', 'Avalon is an island located in the Atlantic Ocean and serves as the headquarters for the British superhero team Excalibur from the Marvel comics universe.', 'https://raw.githubusercontent.com/quarkusio/quarkus-super-heroes/characterdata/images/locations/avalon.png', 'ISLAND');
INSERT INTO locations(name, description, picture, location_type)
VALUES ('Monster Island (Kaiju Island)', 'Monster Island, also known as Kaiju Island, is a remote island in the Pacific Ocean inhabited by giant monsters, including Godzilla and other kaiju.', 'https://raw.githubusercontent.com/quarkusio/quarkus-super-heroes/characterdata/images/locations/monster_island.jpg', 'ISLAND');
INSERT INTO locations(name, description, picture, location_type)
VALUES ('Madripoor', 'Madripoor frequently appears in Marvel comics, especially in X-Men and Wolverine stories, and serves as a backdrop for espionage and intrigue. Madripoor is a fictional island nation located in Southeast Asia. It is known for its diverse culture, lawlessness, and as a haven for criminals and spies.', 'https://raw.githubusercontent.com/quarkusio/quarkus-super-heroes/characterdata/images/locations/madripoor.png', 'ISLAND');
INSERT INTO locations(name, description, picture, location_type)
VALUES ('K''un-Zi', 'K''un-Zi is another of the Seven Capital Cities of Heaven, akin to K''un-Lun, and shares its mystical properties and traditions. Like K''un-Lun, K''un-Zi is known for its deep connection to martial arts, mystical energies, and spiritual enlightenment. It was ruled over by the Crane Mother and was known for its people''s use of dark magics, and it is protected by a line of warriors who bear the name Crane Warrior.', 'https://raw.githubusercontent.com/quarkusio/quarkus-super-heroes/characterdata/images/locations/kunzi.png', 'CITY');
INSERT INTO locations(name, description, picture, location_type)
VALUES ('K''un-Lun', 'K''un-Lun is a mystical, otherworldly city that exists on Earth once every decade through the use of ancient rituals. It''s a place of great spiritual significance and martial training.', 'https://raw.githubusercontent.com/quarkusio/quarkus-super-heroes/characterdata/images/locations/kunlun.png', 'CITY');
INSERT INTO locations(name, description, picture, location_type)
VALUES ('Latveria', 'A small European country ruled by the iron fist of its monarch, Victor Von Doom, also known as Doctor Doom. Latveria is a place of advanced technology, magic, and strict governance. Its citizens live in fear and respect of their ruler.', 'https://raw.githubusercontent.com/quarkusio/quarkus-super-heroes/characterdata/images/locations/latveria.jpg', 'COUNTRY');
INSERT INTO locations(name, description, picture, location_type)
VALUES ('Wakanda', 'A hidden African nation, isolated by advanced technology and powerful natural barriers. Ruler T''Challa, the Black Panther, is the king and protector of Wakanda. Wakanda is known for its reserves of Vibranium, a valuable and powerful metal.', 'https://raw.githubusercontent.com/quarkusio/quarkus-super-heroes/characterdata/images/locations/wakanda.png', 'COUNTRY');
