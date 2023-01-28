class CityPoints:
    def __init__(self) -> None:
        self.res = (None, None, None)
        self.owner = None
        self.port = None
        self.present = False
        self.thief = False
    
    def __repr__(self) -> str:
        return f"City: {self.owner}-{self.present}"

    def build(self, new_owner) -> bool:
        if self.owner:
            return self.owner
        self.owner = new_owner
        self.present = True
        return self.present

class CityLink:
    def __init__(self) -> None:
        self.cities = (None, None)
        self.owner = None
        self.present = False

    def __repr__(self) -> str:
        return f"Road: {self.owner}-{self.present}"

    def build(self, new_owner) -> bool:
        if self.owner:
            return self.owner
        self.owner = new_owner
        self.present = True
        return self.present
