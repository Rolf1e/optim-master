import matplotlib.pyplot as plt
import timeit
from knapsack import resolve_random, resolve_walk, resolve_random_multi_threaded


def treat_rw(content):
    weight = [(out[2]) for out in content]
    profit = [(out[0]) for out in content]
    return (weight, profit)


def data_science_walk(number_execution):
    print('getting data from rust')
    start_time = timeit.default_timer()
    content = resolve_walk(number_execution)
    print('took : ',  timeit.default_timer() - start_time)

    res = treat_rw(content)
    # print(res)

    n = range(0, number_execution)
    plt.figure(figsize=(5, 5))
    legend = plt.subplot(111)
    legend.plot(n, res[0], label='weight')
    legend.plot(n, res[1], label='profit')
    legend.legend()
    plt.title('Random Walk')
    plt.xlabel('attempts')
    plt.ylabel('weight / profit')

    # Histogram
    fig, axs = plt.subplots(1, 2, sharey=True, tight_layout=True)
    axs[0].hist(res[0], bins=20)
    axs[1].hist(res[1], bins=20)


def treat_rand(content):
    attempts = [(str(out[3])) for out in content]
    weight = [(out[2]) for out in content]
    profit = [(out[0]) for out in content]
    return (attempts, weight, profit)


def data_science(number_execution, iterations):
    print('getting data from rust')
    start_time = timeit.default_timer()
    content = resolve_random_multi_threaded(number_execution, iterations)
    print('took : ',  timeit.default_timer() - start_time)

    res = treat_rand(content)

    print("Flat")
    plt.figure(figsize=(5, 5))
    legend = plt.subplot(111)
    legend.plot(res[0], res[1], label='weight')
    legend.plot(res[0], res[2], label='profit')
    legend.legend()
    plt.title('Random')
    plt.xlabel('attempts')
    plt.ylabel('weight / profit')

    # Bar
    print('Bar')
    plt.figure(figsize=(10, 10))
    legend = plt.subplot(121)
    legend.bar(res[0], res[1], label='weight')
    legend.bar(res[0], res[2], label='profit')
    plt.title('Random bar')
    plt.xlabel('attempts')
    plt.ylabel('weight / profit')
    legend.legend()


data_science(10, 100000)

# data_science_walk(100000)
plt.show()
