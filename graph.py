import numpy as np
import matplotlib.pyplot as plt
import yaml


filename = "data_output.yaml"
color = ['b', 'g', 'r', 'y']
with open(filename, "r") as f:
    file = yaml.load(f, Loader=yaml.FullLoader)


def algorithm(a):
    algorithmz = [file['choice_distribution' + str(a)][0][1], file['choice_distribution' + str(a)][1][2],
                 file['choice_distribution' + str(a)][2][3], file['choice_distribution' + str(a)][3][4],
                 file['choice_distribution' + str(a)][4][5]]
    return algorithmz


def graph_create():
    n_groups = 5
    fig, ax = plt.subplots()
    index = np.arange(n_groups)
    bar_width = 0.2
    
    for i in range(1,5):
        bars = plt.bar(index + (i-1)*bar_width, algorithm(i), bar_width, color = color[i-1], label = str(10*10**i) + " Trials")

    plt.ylabel('People')
    plt.title('Housing Allocation')
    plt.xticks(index + bar_width, ('1st Choice', '2nd Choice', '3rd Choice', '4th Choice', "5th Choice"))
    plt.legend()
    plt.show()


graph_create()
