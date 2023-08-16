import numpy as np
from skimage import io
from skimage.viewer import ImageViewer


def minFilter(in_image, filter, offset=0):
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

    newimg = np.zeros((newx, newy))
    newimg[filter_outside:newx - filter_outside, filter_outside:newy - filter_outside] = in_image

    outimg = np.zeros(np.shape(in_image))

    for x in range(newx - len(filter)):
        print("{:3} / {}".format(x + 1, newx - len(filter)))
        for y in range(newy - len(filter)):
            c = None

            for i in range(len(filter)):
                for j in range(len(filter)):
                    if c is None:
                        c = newimg[x + i][y + j]
                    else:
                        c = min(c, newimg[x + i][y + j])

            outimg[x][y] = c
    return outimg


def maxFilter(in_image, filter, offset=0):
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

    newimg = np.zeros((newx, newy))
    newimg[filter_outside:newx - filter_outside, filter_outside:newy - filter_outside] = in_image
    outimg = np.zeros(np.shape(in_image))

    for x in range(newx - len(filter)):
        print("{:3} / {}".format(x + 1, newx - len(filter)))
        for y in range(newy - len(filter)):
            c = None

            for i in range(len(filter)):
                for j in range(len(filter)):
                    if c is None:
                        c = newimg[x + i][y + j]
                    else:
                        c = max(c, newimg[x + i][y + j])

            outimg[x][y] = c
    return outimg


def medianFilter(in_image, filter, offset=0):
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

    newimg = np.zeros((newx, newy))
    newimg[filter_outside:newx - filter_outside, filter_outside:newy - filter_outside] = in_image

    outimg = np.zeros(np.shape(in_image))

    for x in range(newx - len(filter)):
        print("{:3} / {}".format(x + 1, newx - len(filter)))
        for y in range(newy - len(filter)):
            c = []

            for i in range(len(filter)):
                for j in range(len(filter)):
                    for amount in range(filter[i][j]):
                        c.append(newimg[x + i][y + j])

            np.sort(c, kind='heapsort')
            #c = c[int(len(filter) * len(filter)) - 1]
            c = np.median(c)
            outimg[x][y] = c
    return outimg


def main():
    pepper = io.imread("pepper.jpg", as_gray=True)
    tree = io.imread("tree.png", as_gray=True)
    box = [[1, 1, 1],
           [1, 1, 1],
           [1, 1, 1]]
    bigbox = np.full((11, 11), 1)
    gaussian = [[1, 1, 1],
                [1, 3, 1],
                [1, 1, 1]]
    pepper_min = minFilter(pepper, box)
    pepper_max = maxFilter(pepper, box)
    pepper_median = medianFilter(pepper, box)
    tree_min = minFilter(tree, box)
    tree_max = maxFilter(tree, box)
    tree_median = medianFilter(tree, box)
    io.imsave("pepper_min.jpg", pepper_min)
    io.imsave("pepper_max.jpg", pepper_max)
    io.imsave("pepper_median.jpg", pepper_median)
    io.imsave("tree_min.png", tree_min)
    io.imsave("tree_max.png", tree_max)
    io.imsave("tree_median.png", tree_median)


if __name__ == "__main__":
    main()
