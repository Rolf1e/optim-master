import reader
import matplotlib.pyplot as plt


attempts = []
weight = []
content = reader.read_csv('output/output.csv')
parsed_content = [(row[0], row[1]) for row in content]
for x in parsed_content:
    attempts.append(x[0].split('-')[0])
    weight.append(x[1])

print(attempts)
print(weight)

# sort this thing
plt.figure(figsize=(10, 10))
plt.plot(attempts, weight, label="perf")
plt.xlabel("number of attempts")
plt.ylabel("max weight")
plt.show()

