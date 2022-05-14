from PIL import Image
import sys
filename = sys.argv[1]
img = Image.open(filename)
height, width = img.size

r = img.getchannel(0)
g = img.getchannel(1)
b = img.getchannel(2)

for i in range(height * width):
    y, x = i // height, i % height
    pixel = bin(r.getpixel((x, y)))[2:]
    r.putpixel((x, y), int(pixel[:-1] + '0', 2))

bands = (r, g, b)
new = Image.merge("RGB", bands)
new.save(filename)
