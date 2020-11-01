import sys

import matplotlib.pyplot as plt
import pandas as pd
import seaborn as sns
import yaml

"""
Graphs and displays the choice and friend distributions in data_output.yaml in a bar chart

Usage:
    python3 graph.py
"""


def load_data(filename: str) -> dict:
    """
    Loads data from YAML file

    Args:
        filename (str): The YAML file to read from.

    Returns:
        The data in the YAML file, parsed using pyyaml's defaults.
    """
    with open(filename) as f:
        data = yaml.safe_load(f)

    return data


def fetch_distribution(
    data: dict, distribution_key: str, independent_label: str
) -> pd.DataFrame:
    """
    Fetches distribution from nested dictionary format.

    Args:
        data (dict): The data, as a dictionary, directly read from the data output file.
        distribution_key (str): The name of the distribution to fetch from the dictionary.
        independent_label (str): The name/label of the independent variable. The dependent variable always represents frequency.

    Returns:
        A Pandas DataFrame of the distribution, ready for graphing by Seaborn.
    """
    distribution = []

    for algo in data.get("algo"):
        for count, choice in enumerate(algo.get(distribution_key)):
            distribution.append((algo.get("name"), count + 1, list(choice.values())[0]))

    return pd.DataFrame(
        distribution, columns=["Algorithm", independent_label, "Frequency"]
    )


if __name__ == "__main__":
    data = load_data("data_output.yaml")
    distributions = [
        ("Number of friends in same house", "friend_distribution"),
        ("House rank", "choice_distribution"),
    ]

    for name, each_distribution in distributions:
        distribution = fetch_distribution(
            data, each_distribution, independent_label=name
        )

        # Graph/plot figure
        plot = sns.barplot(x=name, y="Frequency", hue="Algorithm", data=distribution)
        plot.figure.get_axes()[0].legend(loc="upper right") # snap legend to upper right

        # Save/show figure
        plot.figure.savefig(f"{each_distribution}.png")
        if len(sys.argv) > 1 and sys.argv[1] == "show":
            plt.show()

        plot.clear()
