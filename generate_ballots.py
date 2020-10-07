import yaml
import random

houses = ["Tenny","CK","East Cottage"] # add houses
capacity = [25,25,25] # unused rn
users = [] # ballots

def algorithm(house,a):
	if house == 0:
		return a ** 2 # popular
	if house == 1:
		return 1 # completely random
	if house == 2:
		return (a-3)**2 # not popular
	# add algorithm for the new houses if needed


def create_weight(a):
	weight = []
	for i in range(10):
		weight.append(int(algorithm(a,1+0.1*len(weight))*100))
	return weight

def randhouse(a):
	global check
	weight = []
	my_list = []
	weight = create_weight(a)
	for answer in range(1,11):
		my_list += [answer]*weight[answer-1]
	return random.choice(my_list)

for i in range(100):
	name = "Person "+str(i)
	ranking = []
	for j in range(len(houses)):
		ranking.append({'name': houses[j], 'weight': randhouse(j)})
	users.append({'name':name, 'ranking':ranking})

house = []
for i in range(len(houses)):
	house.append({'name': houses[i]})

overall = [{'houses':house}, {'ballots':users}]

with open('input2.yaml', 'w') as f:
    data  = yaml.dump(overall,f)