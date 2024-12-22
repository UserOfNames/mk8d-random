class Course():
    __slots__ = ("rank", "coord", "name")

    def __init__(self, rank, coord, name):
        self.rank = rank
        self.coord = coord
        self.name = name

    def __repr__(self):
        return f"({self.coord}) {self.name} ({self.rank})"
