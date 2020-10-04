import matplotlib.pyplot as plt
from knapsack import execute, execute_multiple_time, execute_multiple_time_incremented


def treatement(content):
    attempts = [(str(out[3])) for out in content]
    weight = [(out[2]) for out in content]
    profit = [(out[0]) for out in content]
    print('attempts')
    print(attempts)
    print('profit')
    print(profit)
    print('weight')
    print(weight)
    return (attempts, weight, profit)


def classic_execution():
    print('getting data from rust')
    content = execute_multiple_time(10)
    print('done')
    # print(content)

    res = treatement(content)

    print("Flat")
    plt.figure(figsize=(5, 5))
    legend = plt.subplot(111)
    legend.plot(res[0], res[1], label='weight')
    legend.plot(res[0], res[2], label='profit')
    legend.legend()
    plt.title('execute_multiple_time')
    plt.xlabel('attempts')
    plt.ylabel('weight / profit')

    # Bar
    print('Bar')
    plt.figure(figsize=(10, 10))
    legend = plt.subplot(121)
    legend.bar(res[0], res[1], label='weight')
    legend.bar(res[0], res[2], label='profit')
    plt.title('bar')
    plt.xlabel('attempts')
    plt.ylabel('weight / profit')
    legend.legend()


def execute_for_data_science(incrementations):
    print('getting data from rust')
    content = execute_multiple_time_incremented(incrementations)
    print('done')

    print('incrementations')
    print(incrementations)

    res = treatement(content)

    plt.figure(figsize=(5, 5))
    legend = plt.subplot(111)
    legend.plot(incrementations, res[1], label='weight')
    legend.plot(incrementations, res[2], label='profit')
    legend.legend()
    plt.title('execute_multiple_time')
    plt.xlabel('attempts')
    plt.ylabel('weight / profit')


# classic_execution()
execute_for_data_science([10, 100, 1000, 10000, 100000, 1000000])# 2000000])
plt.show()
