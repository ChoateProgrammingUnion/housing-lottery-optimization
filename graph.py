import numpy as np
import matplotlib.pyplot as plt
import yaml


filename = "data_output.yaml"
color = ['b', 'g', 'r', 'y']
with open(filename, "r") as f:
    file = yaml.load(f, Loader=yaml.FullLoader)


def algorithm(house,n_house):
    algorithmz = []
    for i in range(0,n_house):
        algorithmz.append(file['choice_distribution' + str(house)][i][i+1])
    return algorithmz


def graph_create(n_house):
    fig, ax = plt.subplots()
    index = np.arange(n_house)
    bar_width = 0.2
    
    for i in range(1,n_house):
        bars = plt.bar(index + (i-1)*bar_width, algorithm(i,n_house), bar_width, color = color[(i-1)%4], label = str(10*10**i) + " Trials")

    plt.ylabel('People')
    plt.title('Housing Allocation')
    plt.xticks(index + bar_width, ("Choice "+str(i+1) for i in range(n_house)))
    plt.legend()
    plt.show()


graph_create(len(file['choice_distribution1']))
