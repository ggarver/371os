# Script to convert url to VGA buffer array
import numpy as np
from PIL import Image
import requests
import io 
def download(inp):
    response = requests.get(inp)

    as_file = io.BytesIO(response.content)
    img = Image.open(as_file)
    scaled = img.resize((80, 25))


    np_img = np.array(scaled)
    cols = []

    with Image.open("dump.ppm") as color_img:
        for coordinate in range(0, 720, 45):
            col = color_img.getpixel((coordinate, 0))
            cols.append(col)
    cols = np.array(cols)

    # reshape to pixels instead of rows
    pix = np_img.reshape(-1, 3)
    # add dimensions in order to subtract 16 colors from 1 pix
    # absolute val 
    dist = np.abs(pix[:, np.newaxis, :] - cols[np.newaxis, :, :])
    # sum to make distances easier
    dist = dist.sum(axis = 2)

    # find min dist, index = color 
    indices = np.argmin(dist, axis=1)

    data = ", ".join(map(str, indices))
    with open("image.rs", "w") as w:
        w.write(f"pub const IMG: [u8; {80 * 25}] = [{data}];")



def main():
    download(inp = input("enter img url:"))
    # download(inp = "https://cd-rs.github.io/os/img/rainbow.jpg")



if __name__ == "__main__":
    main()




