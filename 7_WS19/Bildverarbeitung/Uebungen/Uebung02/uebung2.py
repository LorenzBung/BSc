from skimage.io import imread
import numpy as np
import matplotlib.pyplot as plt
import matplotlib.cm as cm


def compute_Histo(img):
    img = np.asarray(img)
    histogram = np.zeros(256)

    for i, row in enumerate(img):
        for j, value in enumerate(row):
            histogram[int(value)] += 1

    return histogram


def bin_Histo(img, bin=1):
    assert 0 < bin < 257
    img = np.asarray(img)
    histogram = np.zeros(int(np.ceil(256 / bin)))

    for i, row in enumerate(img):
        for j, value in enumerate(row):
            histogram[int(value) // bin] += 1

    return histogram


def brighten(img, offset):
    # add offset to img
    for i, row in enumerate(img):
        for j, value in enumerate(row):
            img[i][j] += offset
            # check clamping
            if img[i][j] > 255:
                img[i][j] = 255
            elif img[i][j] < 0:
                img[i][j] = 0

    return img


def get_lut(k=256):

    # create lut-table
    # which only brightens the darker pixel values (e.g. < 200)
    # bright pixel values should not change that much
    lut = {}
    for i in range(k):
        if i < 200:
            lut[i] = 40
        else:
            lut[i] = 5

    return lut


def brighten_with_lut(img, lut):
    # brighten image using the lookup-table
    for i, row in enumerate(img):
        for j, value in enumerate(row):
            img[i][j] += lut[value]
            # check clamping
            if img[i][j] > 255:
                img[i][j] = 255
            elif img[i][j] < 0:
                img[i][j] = 0
    return img


def rgb2gray(img):

    img = np.asarray(img)
    gray = np.zeros(np.shape(img)[:2])

    for i, row in enumerate(img):
        for j, pixel in enumerate(row):
            gray[i][j] = (int(pixel[0]) + int(pixel[1]) + int(pixel[2])) // 3

    return gray


if __name__ == "__main__":

    # plot processed img
    fig, axs = plt.subplots(8, 2)

    # read img

    for i in range(0, 5):
        im = rgb2gray(imread("bild0" + str(i + 1) + ".jpg"))
        data = compute_Histo(im)

        axs[i, 0].imshow(im, cmap=cm.Greys_r)
        axs[i, 0].set_xticks([])
        axs[i, 0].set_yticks([])
        axs[i, 1].bar(range(data.size), data, 1, color="blue")
        axs[i, 1].set_xlim([0, data.size-1])

    # brighten image without lut
    im = rgb2gray(imread("bild01.jpg"))
    im = brighten(im, 40)
    data = compute_Histo(im)
    axs[5, 0].imshow(im, cmap=cm.Greys_r)
    axs[5, 0].set_xticks([])
    axs[5, 0].set_yticks([])
    axs[5, 1].bar(range(data.size), data, 1, color="blue")
    axs[5, 1].set_xlim([0, data.size-1])

    # brighten image with lut-table
    im = rgb2gray(imread("bild01.jpg"))
    lut = get_lut()
    im = brighten_with_lut(im, lut)
    data = compute_Histo(im)
    axs[6, 0].imshow(im, cmap=cm.Greys_r)
    axs[6, 0].set_xticks([])
    axs[6, 0].set_yticks([])
    axs[6, 1].bar(range(data.size), data, 1, color="blue")
    axs[6, 1].set_xlim([0, data.size-1])

    # compute histogram (with bin-size)
    im = rgb2gray(imread("bild01.jpg"))
    data_binned = bin_Histo(im, 5)
    axs[7, 0].imshow(im, cmap=cm.Greys_r)
    axs[7, 0].set_xticks([])
    axs[7, 0].set_yticks([])
    axs[7, 1].bar(range(data_binned.size), data_binned, 1, color="blue")
    axs[7, 1].set_xlim([0, data_binned.size-1])

    plt.show()
