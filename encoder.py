from PIL import Image
import sys
filename = sys.argv[1]
print("Enter your input, (Ctrl + D for EOF)")
msg = sys.stdin.read()
msg = ''.join([bin(ord(i))[2:].rjust(8, '0') for i in msg])

img = Image.open(filename)
height, width = img.size

r = img.getchannel(0)
g = img.getchannel(1)
b = img.getchannel(2)

for i in range(len(msg)):
    y, x = i // height, i % height
    pixel = bin(r.getpixel((x, y)))[2:]
    if pixel[-1] != msg[i]:
        r.putpixel((x, y), int(pixel[:-1] + msg[i], 2))
    else:
        r.putpixel((x, y), int(pixel, 2))

bands = (r, g, b)
new = Image.merge("RGB", bands)
new.save('out.png')
