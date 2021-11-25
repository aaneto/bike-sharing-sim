import pandas
import matplotlib.pyplot as plt

df = pandas.read_json("metrics.json")
df = df.ewm(span = 200).mean()

plt.plot(df["rider_exits"], color="r", label="Saidas do Sistema")
plt.plot(df["complete_travels"], color="g", label="Viagens Completas")
plt.xlabel("Epoch")
plt.legend(loc="upper left")

plt.plot()
plt.show()
