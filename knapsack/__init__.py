import matplotlib.pyplot as plt
from knapsack import resolve_random_multithread, resolve_walk_multithread, resolve_hill_climber_multithread


def treatement(content):
    res = content[0]
    weight = [(out[1]) for out in res]
    profit = [(out[0]) for out in res]
    return (weight, profit, content[1])


def data_science_walk(number_execution):
    print('getting data from rust')

    content = resolve_walk_multithread(number_execution)

    res = treatement(content)
    print('Time : ', res[2], 'ms')
    # print(res)

    n = range(0, number_execution)
    plt.figure(figsize=(5, 5))
    legend = plt.subplot(111)
    legend.plot(n, res[0], label='profit')
    legend.plot(n, res[1], label='weight')
    legend.legend()
    plt.title('Random Walk')
    plt.xlabel('attempts')
    plt.ylabel('weight / profit')

    # Histogram
    # fig, axs = plt.subplots(1, 2, sharey=True, tight_layout=True)
    # axs[0].hist(res[0], bins=20)
    # axs[1].hist(res[1], bins=20)


def data_science_random(number_execution, iterations):
    print('getting data from rust')

    content = resolve_random_multithread(number_execution, iterations)

    res = treatement(content)
    print('Time : ', res[2], 'ms')

    plt.figure(figsize=(5, 5))
    legend = plt.subplot(111)
    legend.plot(res[0], res[0], label='profit')
    legend.plot(res[0], res[1], label='weight')
    legend.legend()
    plt.title('Random')
    plt.xlabel('attempts')
    plt.ylabel('profit / weight')

    # Bar
    # plt.figure(figsize=(10, 10))
    # legend = plt.subplot(121)
    # legend.bar(res[0], res[0], label='profit')
    # legend.bar(res[0], res[1], label='weight')
    # plt.title('Random bar')
    # plt.xlabel('attempts')
    # plt.ylabel('profit / weight')
    # legend.legend()

def data_science_hill_climber(number_execution):
    print('getting data from rust')

    content = resolve_hill_climber_multithread(number_execution)

    res = treatement(content)
    print('Time : ', res[2], 'ms')

    n = range(0, number_execution)
    plt.figure(figsize=(5, 5))
    legend = plt.subplot(111)
    legend.plot(n, res[0], label='weight')
    legend.plot(n, res[1], label='profit')
    legend.legend()
    plt.title('Hill Climber')
    plt.xlabel('attempts')
    plt.ylabel('weight / profit')


print('Random')
data_science_random(10, 100000)

print('Walk')
data_science_walk(100000)

print('Hill climber')
data_science_hill_climber(1000)

plt.show()

