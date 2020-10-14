import numpy as np
import matplotlib.pyplot as plt
import yaml


filename = "data_output.yaml"
with open(filename, "r") as f:
    file = yaml.load(f, Loader=yaml.FullLoader)

def algorithm(a):
    algorithmz = [file['choice_distribution' + str(a)][1][2], file['choice_distribution' + str(a)][2][3],
                 file['choice_distribution' + str(a)][3][4], file['choice_distribution' + str(a)][4][5],
                 file['choice_distribution' + str(a)][5][6]]
    return algorithmz



def graph_create():
    n_groups = 5
    fig, ax = plt.subplots()
    index = np.arange(n_groups)
    bar_width = 0.2

    bars1 = plt.bar(index, algorithm(1), bar_width, color='b',label='100 Trials')
    bars2 = plt.bar(index + bar_width, algorithm(2), bar_width, color='g', label='1000 Trials')
    bars3 = plt.bar(index + 2*bar_width, algorithm(3), bar_width, color='r', label='10000 Trials')
    bars4 = plt.bar(index + 3*bar_width, algorithm(4), bar_width, color='y', label='100000 Trials')

    plt.ylabel('People')
    plt.title('Housing Allocation')
    plt.xticks(index + bar_width, ('1st Choice', '2nd Choice', '3rd Choice', 'Fourth Choice'))

    plt.show()


graph_create()
