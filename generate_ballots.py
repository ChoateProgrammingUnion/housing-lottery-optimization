import yaml
import random
from numpy.random import zipf
import unittest

houses = ["Tenny", "CK", "East Cottage", "KEC", "Pratt"]  # add houses


def randhouse(house):
    x = zipf(
        a=1.01 + random.random() / 2, size=1000
    )  # use zipf distribution with some randomness in the variability of the distribution, a = 1-2 because larger number of distribution makes one number too popular
    return (
        10
        - (int(random.choice(x[x < 6])) * 2 - random.randint(0, 1))
        - int(house / (len(houses) - 1) * 8)
        - random.randint(-1, 1)
    ) % 10 + 1.0  # return a number between 1-10, change popularity between houses so that some more are popular than others


# chooses a random number between 1-5 from the zipf list, *2 to make it 2,4,6,8,10, then minus 1 randomly to allow all integers from 1-10. Then, shift the distribution to make different popularities.


def create():
    users = []  # ballots
    people = 100
    num_friends = 5
    people_list = [person_num for person_num in range(people)]

    capacity = [int(people / len(houses)) + 1] * (people % len(houses)) + [
        int(people / len(houses))
    ] * (len(houses) - (people % len(houses)))
    for i in range(people):
        name = "Person " + str(i)
        ranking = []
        for j in range(len(houses)):
            ranking.append({"name": houses[j], "weight": randhouse(j)})
        users.append({"name": name, "ranking": ranking, "friends": []})

    # times of loop is the number of possible friend pair
    for friend_pairs in range(int(people * num_friends / 2)):
        # pick two student
        person1 = random.choice(people_list)
        people_list.remove(person1)
        person2 = random.choice(people_list)

        stuck = 0  # variable to use to quit if loop becomes infinite

        # repick friend pair: cap number of friends for each person, and don't allow duplicate friend pairs
        while (
            (len(users[person1]["friends"]) >= num_friends)
            or (len(users[person2]["friends"]) >= num_friends)
            or "Person " + str(person2) in users[person1]["friends"]
        ):
            stuck += 1
            people_list = [person_num for person_num in range(people)]
            person1 = random.choice(people_list)
            people_list.remove(person1)
            person2 = random.choice(people_list)

            # break in case the loop becomes infinite for some unforseen reason, quiting after people*(people-1)/2 since this is the number of 2 pair combinations possible
            if stuck >= people * (people - 1) / 2:
                break

        if stuck < people * (people - 1) / 2:
            users[person1]["friends"].append("Person " + str(person2))
            users[person2]["friends"].append("Person " + str(person1))
        people_list = [person_num for person_num in range(people)]

    house = []
    for i in range(len(houses)):
        house.append({"name": houses[i], "capacity": capacity[i]})

    return [{"houses": house}, {"ballots": users}, num_friends]


name = "input.yaml"
overall = create()
with open(name, "w") as f:
    data = yaml.dump(overall[0], f)
    data = yaml.dump(overall[1], f)
