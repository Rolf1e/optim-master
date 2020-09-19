import matplotlib.pyplot as plt
from knapsack import execute, execute_multiple_time

__all__ = [
    "execute",
    "execute_multiple_time"
]

# content = execute(100)
content = execute_multiple_time(100)
# print(content)

attempts = [(out[3]) for out in content]
weight = [(out[2]) for out in content]
profit = [(out[0]) for out in content]
# print(attempts)
# print(weight)

plt.figure(figsize=(10, 10))
plt.plot(attempts, weight)
plt.xlabel("attempt")
plt.ylabel("weight")
plt.show()

plt.plot(attempts, profit)
plt.xlabel("attempt")
plt.ylabel("profit")
plt.show()
