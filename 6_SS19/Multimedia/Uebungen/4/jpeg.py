import matplotlib.pyplot as plt
import numpy as np
from PIL import Image
from scipy.fftpack import dct, idct

# This function demonstrates the jpeg encoding
#
#   input  :   original image
#   output :   matrix with quantised DCT values
def jpegEncode(input):

    image = plt.imread(input)
    quantized = np.zeros((len(image),len(image)))
    print(type(image[0,0]))

    fig, ax = plt.subplots()
    ax.imshow(image,cmap='gray')

    # DCT matrix
    ar = np.array([range(8)])

    T = np.array(0.5 * np.cos(ar.T * (2*ar+1) * np.pi / 16))

    T[0,:] = np.sqrt(1/8)

    # Luminance quantized matrix
    q = [   [16, 11, 10, 16, 24, 40, 51, 61],
            [12, 12, 14, 19, 26, 58, 60, 55],
            [14, 13, 16, 24, 40, 57, 69, 56],
            [14, 17, 22, 29, 51, 87, 80, 62],
            [18, 22, 37, 56, 68, 109, 103, 77],
            [24, 35, 55, 64, 81, 104, 113, 92],
            [49, 64, 78, 87, 103, 121, 120, 101],
            [72, 92, 95, 98, 112, 100, 103, 99]]


    for x in range(0, len(image), 8):
        for y in range(0, len(image), 8):
            test = image[x:x+8,y:y+8]
            #print(test)
            test = dct(dct(test.T).T)
            #print(test)
            test = test / q
            test = np.around(test)
            quantized[x:x+8,y:y+8] = test
    #print(quantized)
    return quantized

# This function demonstrates the jpeg decoding
#
#   input  :   matrix with quantised DCT values
#   output :   reconstructed image
def jpegDecode(input):
    # DCT matrix
    ar = np.array([range(8)])
    T = np.array(0.5 * np.cos(ar.T * (2 * ar + 1) * np.pi / 16))

    # Luminance quantized matrix
    q = [[16, 11, 10, 16, 24, 40, 51, 61],
         [12, 12, 14, 19, 26, 58, 60, 55],
         [14, 13, 16, 24, 40, 57, 69, 56],
         [14, 17, 22, 29, 51, 87, 80, 62],
         [18, 22, 37, 56, 68, 109, 103, 77],
         [24, 35, 55, 64, 81, 104, 113, 92],
         [49, 64, 78, 87, 103, 121, 120, 101],
         [72, 92, 95, 98, 112, 100, 103, 99]]

    newImage = np.zeros((len(input),len(input)))
    for x in range(0, len(input), 8):
        for y in range(0, len(input), 8):
            test = input[x:x+8,y:y+8]
            test = test * q

            test = idct(idct(test.T).T)
            test = np.around(test)
            newImage[x:x+8,y:y+8] = test
    fig, ax = plt.subplots()

    plt.imsave('baboon-new.jpg', newImage, cmap='gray')
    Image.open('baboon-new.jpg').convert('L').save('baboon-new.jpg')

    ax.imshow(newImage,cmap='gray')
    #plt.show()
    print(newImage)
   # print(type(newImage[0,0]))
    return newImage


def main():
    input = "baboon.jpg"

    encoded = jpegEncode(input)
    decoded = jpegDecode(encoded)
    
    for i in range(0, 10):
        encoded2 = jpegEncode("baboon-new.jpg")
        jpegDecode(encoded2)
    
    

if __name__ == "__main__":
    main()
