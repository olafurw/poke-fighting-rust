from PIL import Image
import os

colors = [
	(34,128,0), (105,140,138), (45,80,89), 
	(19,50,77), (35,63,140), (177,163,217), 
	(87,26,102), (255,128,213), (153,77,107), 
	(229,115,115), (204,143,102), (204,153,51), 
	(121,128,32), (27,51,0), (26,102,66), 
	(191,251,255), (0,102,153), (153,180,204)
]

images = []
for filename in sorted(os.listdir('battle')):
	lines = [line.rstrip('\n') for line in open('battle/' + filename)]
	
	colorList = []
	
	for line in lines:
		for character in line:
			intValue = ord(character) - 65
			
			if intValue >= 0:
				colorList.append(colors[intValue])
	
	im = Image.new('RGB', (512, 512))
	im.putdata(colorList)
	images.append(im)

images[0].save('result.gif', save_all=True, append_images=images[1:], optimize=True, loop=1)