import csv
from collections import defaultdict, OrderedDict
from typing import List, Set, Dict, Tuple

import yaml

from real_data.utils import house_map, House, LotteryStudent, Student

header = None  # ['Form', 'M/F', 'B/D', 'House', 'Room/Bed', 'Rm Type', '1', '2', '3', '4', '5', '6', '']


houses: Dict[Tuple[str, str, str], List[House]] = defaultdict(lambda: [])  # (form,gender) -> [houses]
students: List[Student] = []

with open('data.csv', newline='') as csv_file:
    reader = csv.reader(csv_file, delimiter=',', quotechar='"')
    for row in reader:
        if header is None:
            header = row
            continue

        form = row[0]
        if form not in ["3", "4"]:
            print("row not valid (form):", str(row))
            continue

        gender = row[1].lower()
        if gender not in ["m", "f"]:
            print("row not valid (gender):", str(row))
            continue

        room_type = row[5].lower()
        if room_type not in ["single", "double", "triple"]:
            print("row not valid (room_type):", str(row))
            continue

        ballot: List[int] = []
        for i in range(6):
            house_raw_name = row[6 + i].lower()
            if house_raw_name == '':
                break
            house_object = House.from_tuple(house_map[room_type, form, gender][house_raw_name])
            if house_object not in houses[form, gender, room_type]:
                houses[form, gender, room_type].append(house_object)
            ballot.append(houses[form, gender, room_type].index(house_object))

        students.append(Student(form, gender, room_type, ballot))

lotteries: Dict[Tuple[str, str, str], List[LotteryStudent]] = defaultdict(lambda: [])  # (form,gender,room_type)
for student in students:
    lotteries[student.form, student.gender, student.room_type].append(LotteryStudent(student.ballot))

for classifier, lottery in lotteries.items():
    filename = f"yaml/{classifier[2]}_{classifier[0]}_{classifier[1]}.yaml"

    house_list: List[House] = houses[classifier]

    def house_weight(student: LotteryStudent, house_id: int) -> float:
        if house_id in student.ballot:
            rank = student.ballot.index(house_id)
        else:
            rank = len(student.ballot)

        return float(1 + len(student.ballot) - rank)

    students_dict = [{
        "name": f"Student {i}",
        "ranking": [{
            "name": house.name,
            "weight": house_weight(lottery[i], house_id)
        } for house_id, house in enumerate(house_list)]
    } for i in range(len(lottery))]

    output = {
        "houses": list(map(lambda x: {"name": x.name, "capacity": x.capacity}, house_list)),
        "ballots": students_dict
    }

    with open(filename, "w") as f:
        data = yaml.dump(output, f)

