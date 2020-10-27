from generate_ballots import create
import unittest

overall = create()

class TestBallot(unittest.TestCase):
    def test_friend(self):
        # test that number of friends equals the actual expected number of friends
        self.assertEqual(len(overall[1]["ballots"][0]["friends"]), overall[2])
        self.assertEqual(len(overall[1]["ballots"][99]["friends"]), overall[2])
        self.assertEqual(len(overall[1]["ballots"][1]["friends"]), overall[2])

    def test_mutal_friends(self):
        # test that friends are mutual
        mutual = 0
        for i in range(overall[2]):
            friend1 = int(overall[1]["ballots"][0]["friends"][i][-2:])
            friend2_list = overall[1]["ballots"][friend1]["friends"]
            for friend in friend2_list:
                if friend == "Person 0":
                    mutual += 1
                    break
        self.assertTrue(mutual == overall[2])



    def test_ballot(self):
        # test that weights are between 0.0 and 10.0
        self.assertGreaterEqual(overall[1]["ballots"][0]["ranking"][0]["weight"], 0.0)
        self.assertLessEqual(overall[1]["ballots"][0]["ranking"][0]["weight"], 10.0)

    def test_house(self):
        # test that number of students less than or equal to the total capacity
        capacity = 0
        for i in range(len(overall[0]["houses"])):
            capacity += overall[0]["houses"][i]["capacity"]
        self.assertGreaterEqual(capacity,len(overall[1]["ballots"]))