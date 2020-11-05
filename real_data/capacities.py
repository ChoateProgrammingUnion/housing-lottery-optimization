import csv
from collections import defaultdict
from typing import List, Dict, Tuple, Set

import yaml


header = None  # ['Dorm', 'Room', 'Bed', 'Gender', 'Form', 'Capacity']

rooms: Dict[Tuple[str, str], Dict[str, Set[Tuple[str, str]]]]  # (gender, form) -> {dorm: {(room_number, capacity),}}
rooms = defaultdict(lambda: defaultdict(lambda: set()))

capacities: Dict[Tuple[str, str, str], Dict[str, int]]  # (gender, form, room_type) -> {dorm: capacity}
capacities = defaultdict(lambda: defaultdict(lambda: 0))

capacity_to_room_type = {"1": "single", "2": "double", "3": "triple"}
dorm_name_map = defaultdict(lambda: '', {
    'Quantrell': 'Quantrell',
    'Library': 'Library',
    'Hall': 'Hall',
    'Logan Munroe': 'Logan',
    'Clinton Knight': 'CK',
    'Archbold': 'Archbold',
    'Bernhard House': 'Bernhard',
    'McCook': 'McCook',
    'West Wing': 'West Wing',
    'Spencer': 'Spencer',
    'Chapel House': 'Chapel',
    'Hill House': 'Hill',
    'Tenney House': 'Tenney'
})

with open('capacities.csv', newline='') as csv_file:
    reader = csv.reader(csv_file, delimiter=',', quotechar='"')
    for row in reader:
        if header is None:
            header = row
            continue

        form = row[4]
        if form not in ['4', '5']:
            if form in ['3', '6']:
                print("freshman/senior omitted")
            else:
                print("row not valid (form):", str(row))
            continue

        dorm = row[0]
        if dorm == '':
            print("row not valid (dorm):", str(row))
            continue
        dorm = dorm_name_map[dorm]
        if dorm == '':
            print("invalid dorm:", row[0])
            continue

        gender = row[3].lower()
        if gender not in ['m', 'f']:
            print("row not valid (gender):", str(row))
            continue

        room_number = row[1]
        if dorm == '':
            print("row not valid (room_number):", str(row))
            continue

        capacity = row[5]
        if capacity not in ['1', '2', '3']:
            print("row not valid (capacity):", str(row))
            continue

        rooms[gender, form][dorm].add((room_number, capacity))

for classifier, dorms in rooms.items():
    gender = classifier[0]
    form = classifier[1]

    for dorm_name, dorm_rooms in dorms.items():
        for room_number, capacity in dorm_rooms:
            room_type = capacity_to_room_type[capacity]
            capacities[gender, form, room_type][dorm_name] += 1
