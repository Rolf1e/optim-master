import csv


def read_csv(file_name):
    with open(file_name) as csv_file:
        return [row for row in csv.reader(csv_file, delimiter=';')]
