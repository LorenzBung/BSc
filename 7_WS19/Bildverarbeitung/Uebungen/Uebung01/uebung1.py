import numpy as np
from PIL import Image


def main():
    image = np.asarray(Image.open("monkey.jpg"))
    print(np.shape(image))
    r = image[:, :, 0]
    g = image[:, :, 1]
    b = image[:, :, 2]
    hmirrored = mirror(image)
    vmirrored = mirror(image, vertical=True)
    imv = Image.fromarray(hmirrored)
    # imv.save("monkey-hmirrored.jpg")
    imv.show()
    imh = Image.fromarray(vmirrored)
    # imh.save("monkey-vmirrored.jpg")
    imh.show()


def mirror(image, vertical=False):
    im = image.copy()
    if vertical:
        counter = 0
        while counter < len(image) // 2:
            im[counter] = image[-(counter + 1)]
            im[-(counter + 1)] = image[counter]
            counter += 1
    else:
        counter = 0
        while counter < len(image[0]) // 2:
            im[:, counter] = image[:, -(counter + 1)]
            im[:, -(counter + 1)] = image[:, counter]
            counter += 1
    return im


if __name__ == "__main__":
    main()
