from PIL import Image
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


# Gibt normiertes kumuliertes Histogram (CDF) zurück
def compute_cumHisto(img, binSize=1):

    histo = img
    k = len(histo)
    n = 0

    for i in range(0, k):
        n += histo[i]

    p = np.zeros(int(np.ceil(256 / binSize)))
    c = float(histo[0])  # first value
    for i in range(1, k):
        c += histo[i]
        p[i // binSize] = c / n

    return p

# Führt diskrete Histogrammanpassung durch
def match_Histo(img_histo, ref_histo):

    k = len(img_histo)
    pa = compute_cumHisto(img_histo)
    pr = compute_cumHisto(ref_histo)

    lut = np.zeros(k)

    for a in range(0, k):
        j = k - 1

        while True:
            lut[a] = j
            j -= 1
            if j < 0 or pa[a] > pr[j]:
                break

    return lut


def apply_LUT(img, lut):

    new = np.zeros(np.shape(img)[:2])

    for i, row in enumerate(img):
        for j, pixel in enumerate(row):
            new[i][j] = lut[int(img[i][j])]

    return Image.fromarray(new)


def rgb2gray(rgb):

    img = np.asarray(rgb)
    gray = np.zeros(np.shape(img)[:2])

    for i, row in enumerate(img):
        for j, pixel in enumerate(row):
            gray[i][j] = (int(pixel[0]) + int(pixel[1]) + int(pixel[2])) // 3

    return gray


if __name__ == "__main__":

    # read img
    im = Image.open("bild01.jpg")
    ref = Image.open("bild02.jpg")

    # convert to numpy array
    arr_im = rgb2gray(im)
    arr_ref = rgb2gray(ref)

    # compute histograms
    histo_im = compute_Histo(arr_im)
    histo_ref = compute_Histo(arr_ref)

    # compute new image with lut
    lut = match_Histo(histo_im, histo_ref)
    im_new = apply_LUT(arr_im, lut)

    # compute new histogram of new image
    histo_new = compute_Histo(im_new)

    # plot information
    N = histo_new.size
    x = range(N)
    width = 1

    # plot histogram of image
    plt.figure(1)
    plt.subplot(211)
    plt.bar(x, histo_im, width, color="blue")
    plt.xlim([0, N-1])
    # plot new img
    plt.figure(1)
    plt.subplot(212)
    plt.imshow(im, cmap=cm.Greys_r)

    # plot histogram of new image
    plt.figure(2)
    plt.subplot(211)
    plt.bar(x, histo_new, width, color="blue")
    plt.xlim([0, N-1])
    # plot new img
    plt.figure(2)
    plt.subplot(212)
    plt.imshow(im_new, cmap=cm.Greys_r)

    # plot reference histogram
    plt.figure(3)
    plt.subplot(211)
    plt.bar(x, histo_ref, width, color="blue")
    plt.xlim([0,N-1])
    # plot reference image
    plt.figure(3)
    plt.subplot(212)
    plt.imshow(ref, cmap=cm.Greys_r)

    plt.show()
