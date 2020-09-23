import matplotlib.pyplot as plt
from knapsack import execute, execute_multiple_time, execute_multiple_time_incremented


__all__ = [
    "execute",
    "execute_multiple_time",
    "execute_multiple_time_incremented"
]

# execute_multiple_time
print("Flat")
content = execute_multiple_time(100)

attempts = [(out[3]) for out in content]
weight = [(out[2]) for out in content]
profit = [(out[0]) for out in content]

plt.figure(figsize=(10, 10))
legend = plt.subplot(111)
legend.plot(attempts, weight, label='weight')
legend.plot(attempts, profit, label='profit')
legend.legend()
plt.title('execute_multiple_time')
plt.xlabel('attempts')
plt.ylabel('weight / profit')


# execute_multiple_time_incremented
print("Incremented")
content2 = execute_multiple_time_incremented(100)
attempts2 = [(out[3]) for out in content2]
weight2 = [(out[2]) for out in content2]
profit2 = [(out[0]) for out in content2]

plt.figure(figsize=(10, 10))
legend = plt.subplot(111)
legend.plot(attempts2, weight2, label='weight')
legend.plot(attempts2, profit2, label='profit')
legend.legend()
plt.title('execute_multiple_time_incremented')
plt.xlabel('attempts')
plt.ylabel('weight / profit')


plt.show()
