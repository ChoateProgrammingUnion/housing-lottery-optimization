from __future__ import annotations

from typing import Dict, Tuple, List


class Student:
    form: str          # 3/4
    gender: str        # m/f
    room_type: str     # single/double/triple
    ballot: List[int]  # house id as int

    def __init__(self, form: str, gender: str, room_type: str, ballot: List[int]):
        self.form = form
        self.gender = gender
        self.room_type = room_type
        self.ballot = ballot


class LotteryStudent:
    ballot: List[int]  # house id as int

    def __init__(self, ballot: List[int]):
        self.ballot = ballot


class House:
    name: str
    capacity: int

    def __init__(self, name: str, capacity: int):
        self.name = name
        self.capacity = capacity

    @staticmethod
    def from_tuple(x: Tuple[str, int]) -> House:
        return House(x[0], x[1])

    def __eq__(self, o: House) -> bool:
        return o.name == self.name


house_name_map: Dict[str, str] = {
    "arch": "Archbold",
    "hall": "Hall",
    "lbry": "Library",
    "bernhs": "Bernhard",
    "mccook": "McCook",
    "wwing": "West Wing",
    "chaphs": "Chapel",
    "ck": "CK",
    "tennhs": "Tenney",
    "logan": "Logan",
    "spencr": "Spencer",
    "quan": "Quantrell",
    "hill": "Hill",
}


house_map: Dict[Tuple[str, str, str], Dict[str, Tuple[str, int]]] = {
    ("double", "3", "f"): {
        "arch": ("Archbold", 10),
        "hall": ("Hall", 10),
        "lbry": ("Library", 10),
        "bernhs": ("Bernhard", 10),
        "mccook": ("McCook", 10),
        "wwing": ("West Wing", 10)
    },
    ("double", "3", "m"): {
        "ck": ("CK", 10),
        "tennhs": ("Tenney", 10),
        "logan": ("Logan", 10),
        "spencr": ("Spencer", 10),
        "quan": ("Quantrell", 10),
        "hill": ("Hill", 10)
    },
    ("double", "4", "f"): {
        "arch": ("Archbold", 10),
        "wwing": ("West Wing", 10),
        "bernhs": ("Bernhard", 10),
        "lbry": ("Library", 10),
        "hall": ("Hall", 10),
        "mccook": ("McCook", 10),
        "chaphs": ("Chapel", 10)
    },
    ("double", "4", "m"): {
        "tennhs": ("Tenney", 10),
        "logan": ("Logan", 10),
        "hill": ("Hill", 10),
        "ck": ("CK", 10),
        "spencr": ("Spencer", 10),
        "quan": ("Quantrell", 10)
    },
    ("single", "3", "f"): {
        "arch": ("Archbold", 10),
        "hall": ("Hall", 10),
        "lbry": ("Library", 10),
        "bernhs": ("Bernhard", 10),
        "mccook": ("McCook", 10),
        "wwing": ("West Wing", 10)
    },
    ("single", "3", "m"): {
        "ck": ("CK", 10),
        "tennhs": ("Tenney", 10),
        "logan": ("Logan", 10),
        "spencr": ("Spencer", 10),
        "quan": ("Quantrell", 10),
        "hill": ("Hill", 10)
    },
    ("single", "4", "f"): {
        "arch": ("Archbold", 10),
        "wwing": ("West Wing", 10),
        "bernhs": ("Bernhard", 10),
        "lbry": ("Library", 10),
        "hall": ("Hall", 10),
        "mccook": ("McCook", 10),
        "chaphs": ("Chapel", 10)
    },
    ("single", "4", "m"): {
        "tennhs": ("Tenney", 7),
        "logan": ("Logan", 5),
        "hill": ("Hill", 6),
        "ck": ("CK", 5),
        "spencr": ("Spencer", 4),
        "quan": ("Quantrell", 3)
    },
    ("triple", "3", "f"): {
        "arch": ("Archbold", 10),
        "hall": ("Hall", 10),
        "lbry": ("Library", 10),
        "bernhs": ("Bernhard", 10),
        "mccook": ("McCook", 10),
        "wwing": ("West Wing", 10)
    },
    ("triple", "3", "m"): {
        "ck": ("CK", 10),
        "tennhs": ("Tenney", 10),
        "logan": ("Logan", 10),
        "spencr": ("Spencer", 10),
        "quan": ("Quantrell", 10),
        "hill": ("Hill", 10)
    },
    ("triple", "4", "f"): {
        "arch": ("Archbold", 10),
        "wwing": ("West Wing", 10),
        "bernhs": ("Bernhard", 10),
        "lbry": ("Library", 10),
        "hall": ("Hall", 10),
        "mccook": ("McCook", 10),
        "chaphs": ("Chapel", 10)
    },
    ("triple", "4", "m"): {
        "tennhs": ("Tenney", 10),
        "logan": ("Logan", 10),
        "hill": ("Hill", 10),
        "ck": ("CK", 10),
        "spencr": ("Spencer", 10),
        "quan": ("Quantrell", 10)
    }
}
