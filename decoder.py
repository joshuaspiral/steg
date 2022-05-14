from PIL import Image
import sys

filename = sys.argv[1]
img = Image.open(filename)
r = [bin(x) for x in (list(img.getdata(band=0)))]

msg = ''.join([str(i)[-1] for i in r])

for i in range(0, len(msg), 8):
    if int(msg[i:i+8]): print(chr(int(msg[i:i+8], 2)), end='')
    else: break
