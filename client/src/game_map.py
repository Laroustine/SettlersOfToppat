from my_map.hexagon import CityLink, CityPoints
from my_map.thief import Thief
import json

class Hexagon:
    def __init__(self, entry: int, res: str, die: int) -> None:
        self.entry = entry
        self.res = res
        self.chance = die
    
    def __repr__(self) -> str:
        return f"{self.chance}-{self.res}"

class DiceChance:
    MAXIMUM = 36 * 2
    LUCK = {
        2: 1 * 1,
        3: 2 * 2,
        4: 3 * 2,
        5: 4 * 2,
        6: 5 * 2,
        7: 6 * 2,
        8: 5 * 2,
        9: 4 * 2,
        10: 3 * 2,
        11: 2 * 2,
        12: 1 * 1,
    }

    def __init__(self) -> None:
        self.res = {}

    def die_chance(self, die:int) -> int:
        return float(self.LUCK[die] // 2) / float(self.MAXIMUM // 2)
    
    def item_chance(self, name: str) -> float:
        chance = self.res.get(name) if self.res.get(name) else 0
        return chance/self.MAXIMUM


class GameMap:
    def __init__(self, path: str = "./examples/map.json") -> None:
        self.cities = []
        self.roads = []
        self.hexagons = []
        self.ressources = []
        self.thief = None
        self.chance = DiceChance()
        self.load_map(path)
        self.make_city_link()
        self.check_chances()
    
    def __repr__(self) -> str:
        return f"Map: "

    def load_map(self, path: str) -> None:
        with open(path, "r") as map_file:
            values = json.load(map_file)        
        dice = 0
        for i in range(len(values["tile"])):
            if values["tile"][i]:
                self.hexagons.append(Hexagon(i, values["tile"][i], values["value"][dice + i]))
            else:
                self.hexagons.append(Hexagon(i, None, 0))
                self.thief = Thief(i)
                dice = -1
        self.ressources = list(set(values["tile"]))
        self.ressources = [i for i in self.ressources if i]

    def make_map(self):
        # Make Points
        self.cities.append([CityPoints() for _ in range(7)])
        self.cities.append([CityPoints() for _ in range(9)])
        self.cities.append([CityPoints() for _ in range(11)])
        self.cities.append([CityPoints() for _ in range(9)])
        self.cities.append([CityPoints() for _ in range(7)])
        # Make Roads
        self.roads.append([CityLink() for _ in range(6)])
        self.roads.append([CityLink() for _ in range(4)])
        self.roads.append([CityLink() for _ in range(8)])
        self.roads.append([CityLink() for _ in range(5)])
        self.roads.append([CityLink() for _ in range(10)])
        self.roads.append([CityLink() for _ in range(5)])
        self.roads.append([CityLink() for _ in range(8)])
        self.roads.append([CityLink() for _ in range(4)])
        self.roads.append([CityLink() for _ in range(6)])

    def make_city_link(self):
        self.make_map()

    def check_chances(self):
        for elm in self.ressources:
            stats = 0
            for hex in self.hexagons:
                if elm == hex.res:
                    stats += self.chance.LUCK.get(hex.chance)
            self.chance.res[elm] = stats
