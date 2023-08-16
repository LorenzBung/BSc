from skimage import io
from skimage.viewer import ImageViewer
import numpy as np


def filter(in_image, filter, off, edge="min"):
    for i in range(len(filter)):
        # Assert that filter is of shape NxN
        assert(len(filter) == len(filter[i]))
    # Assert that N is of type (2k + 1)
    assert(len(filter) % 2 == 1)

    filter_outside = len(filter) // 2
    newx = np.shape(in_image)[0] + 2 * filter_outside
    newy = np.shape(in_image)[1] + 2 * filter_outside

    innerx = newx - filter_outside
    innery = newy - filter_outside

    if edge == "min":
        newimg = np.zeros((newx, newy))
        newimg[filter_outside:newx - filter_outside, filter_outside:newy - filter_outside] = in_image
    elif edge == "max":
        newimg = np.full((newx, newy), 1.0)
        newimg[filter_outside:newx - filter_outside, filter_outside:newy - filter_outside] = in_image
    elif edge == "mirror":
        newimg = np.zeros((newx, newy))
        newimg[filter_outside:newx - filter_outside, filter_outside:newy - filter_outside] = in_image

        # Edge cases
        newimg[filter_outside:innerx, 0:filter_outside] = np.fliplr(newimg[filter_outside:innerx, filter_outside:2*filter_outside])
        newimg[filter_outside:innerx, innery:newy] = np.fliplr(newimg[filter_outside:innerx, innery - filter_outside:innery])
        newimg[0:filter_outside, 0:newy] = np.flipud(newimg[filter_outside:filter_outside * 2, 0:newy])
        newimg[innerx:newx, 0:newy] = np.flipud(newimg[innerx - filter_outside:innerx, 0:newy])

    elif edge == "continue":
        newimg = np.zeros((newx, newy))
        newimg[filter_outside:newx - filter_outside, filter_outside:newy - filter_outside] = in_image

        # Edge cases
        newimg[filter_outside:innerx, 0:filter_outside] = newimg[filter_outside:innerx, filter_outside:filter_outside + 1]
        newimg[filter_outside:innerx, innery:newy] = newimg[filter_outside:innerx, innery - 1:innery]
        newimg[0:filter_outside, filter_outside:innery] = newimg[filter_outside:filter_outside + 1, filter_outside:innery]
        newimg[innerx:newx, filter_outside:innery] = newimg[innerx - 1:innerx, filter_outside:innery]

        # Corner cases
        newimg[0:filter_outside, 0:filter_outside] = in_image[0][0]
        newimg[0:filter_outside, innery:newy] = in_image[0][np.shape(in_image)[1] - 1]
        newimg[innerx:newx, 0:filter_outside] = in_image[np.shape(in_image)[0] - 1][0]
        newimg[innerx:newx, innery:newy] = in_image[np.shape(in_image)[0] - 1][np.shape(in_image)[1] - 1]

    outimg = np.zeros(np.shape(in_image))

    for x in range(newx - len(filter)):
        print("{:3} / {}".format(x + 1, newx - len(filter)))
        for y in range(newy - len(filter)):
            c = 0

            for i in range(len(filter)):
                for j in range(len(filter)):
                    c += filter[i][j] * newimg[x + i][y + j]

            outimg[x][y] = (c / np.sum(filter))

    return outimg


def main():
    im = io.imread("lena.jpg", as_gray=True)  # Open as grayscale image
    box = [[1, 1, 1],
           [1, 1, 1],
           [1, 1, 1]]
    bigbox = np.full((11, 11), 1)
    gaussian = [[1, 1, 1],
                [1, 3, 1],
                [1, 1, 1]]

    newimg = filter(im, bigbox, 0, "min")
    ImageViewer(newimg).show()

if __name__ == "__main__":
    main()
