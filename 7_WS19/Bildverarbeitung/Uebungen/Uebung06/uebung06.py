import numpy as np
from skimage import io
from skimage.viewer import ImageViewer


def linearHT(im_edge, angle_steps, radius_steps):
  pass
  acc = np.zeros((angle_steps, radius_steps))
  3:Let (uc,vc) be the center coordinates of the image I
  4:for all image coordinates (u, v) do
    5:if I(u, v) is an edge point then
      6:(x, y)←(u−uc,v−vc)  # relative coordinate to center
      7:for θi=0...π do
        8:ri = xcos(θi) + ysin(θi)
        9:IncrementAcc[θi,ri]
  max_lines = find_max_lines(acc, K)
  # return the list of parameter pairs (θj, rj) for K strongest lines
  return max_lines


def main():
  noisy_lines = io.imread("noisy-lines.png", as_gray=True)
  hough_array = linearHT(noisy_lines, 100, 100)


if __name__ == "__main__":
  main()
