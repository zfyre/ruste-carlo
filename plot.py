import matplotlib.pyplot as plt
import numpy as np

# Read the binary file and interpret as float32
data = np.fromfile("data.bin", dtype=np.float32)

print(data)


plt.hist(data, bins=100)
plt.show()


