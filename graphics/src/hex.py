import numpy as np
from PIL import Image

IMG_NAME = "dump.ppm"

img = np.array(Image.open(IMG_NAME))
print(img[0][:10])
