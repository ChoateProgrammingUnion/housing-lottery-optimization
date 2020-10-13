import yaml
import random
from numpy.random import zipf

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
    capacity = [int(people / len(houses))] * (len(houses) - 1) + [
        people - int(people / len(houses)) * (len(houses) - 1)
    ] * 1
    for i in range(people):
        name = "Person " + str(i)
        ranking = []
        for j in range(len(houses)):
            ranking.append({"name": houses[j], "weight": randhouse(j)})
        users.append({"name": name, "ranking": ranking})

    house = []
    for i in range(len(houses)):
        house.append({"name": houses[i], "capacity": capacity[i]})

    return [{"houses": house}, {"ballots": users}]


name = "input.yaml"
overall = create()
with open(name, "w") as f:
    data = yaml.dump(overall[0], f)
    data = yaml.dump(overall[1], f)
