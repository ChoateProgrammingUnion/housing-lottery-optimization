import csv
import yaml

def create_csv(i):
	filename = "visual"+i+".csv"
	with open(filename, 'w') as f:
		writer = csv.writer(f)
		fieldnames = ["name"]
		for key in houses:
			for house in houses[key]:
				for info in house:
					if isinstance(house[info],str): 
						fieldnames.append(house[info])# add house name

		writer.writerow(fieldnames)
		for ballots in users:
			for person in users[ballots]: # each person
				info = []
				for key in person:
					if isinstance(person[key], list):
						for ballot in person[key]:
							for rank in ballot:
								if isinstance(ballot[rank],int) or isinstance(ballot[rank],float): # add ballot number
									info.append(ballot[rank])
					else:
						info.append(person[key]) # add person name
				writer.writerow(info) # add info of the person(name and ballot)


for i in range(1,11): # change number of files
	filename = "input"+str(i)+".yaml"
	with open(filename, 'r') as f:
		file = yaml.load(f,Loader=yaml.FullLoader)

	houses = file[0] # info of houses
	users = file[1] # info of user input
	create_csv(str(i))
    