# Script to convert url to VGA buffer array
import numpy as np
from PIL import Image
import requests
import io 
def download(inp):
    response = requests.get(inp)

    as_file = io.BytesIO(response.content)
    # img = Image.open(as_file)

    np_img = np.array(Image.open(img))




def main():
    # download(inp = input("enter img url:"))
    download(inp = "https://cd-public.github.io/ai101/images/photo-cat.jpg")



if __name__ == "__main__":
    main()




