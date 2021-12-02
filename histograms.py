import sys
import matplotlib.pyplot as plt

demands = [4.0, 12.0, 13.0, 3.0, 4.0, 5.0, 3.0, 4.0, 3.0, 3.0, 4.0, 5.0, 3.0, 9.0, 7.0, 3.0, 4.0, 8.0, 2.0, 3.0]
bikes = [9] * 20

fig, axs = plt.subplots(2)
fig.suptitle("Histograma das entradas")

axs[0].bar(range(20), demands, label="Demanda")
axs[0].legend(loc="lower left")

axs[1].bar(range(20), bikes, label="Bicicletas")
axs[1].legend(loc="lower left")

plt.plot()
plt.savefig(sys.argv[-1])
