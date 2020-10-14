import yaml

filename = "data_output.yaml"
with open(filename, "r") as f:
    file = yaml.load(f, Loader=yaml.FullLoader)
    print(file['choice_distribution'])
