import matplotlib.pyplot as plt
from knapsack import random_execution

__all__ = [
    "random_execution"
]

content = random_execution(10)
# print(content)

attempts = [(out[0]) for out in content]
weight = [(out[1]) for out in content]
print(attempts)
print(weight)

plt.figure(figsize=(10, 10))
plt.plot(attempts, weight, label="perf")
plt.xlabel("number of attempts")
plt.ylabel("max weight")
plt.show()
