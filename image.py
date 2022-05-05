from PIL import Image
import os

colors = [
	(168,168,120), # Normal,
    (240,128,48), # Fire,
    (104,144,240), # Water,
	(248,208,48), # Electric,
    (120,200,80), # Grass,
    (152,216,216), # Ice,
	(192,48,40), # Fighting,
    (160,64,160), # Poison,
    (224,192,104), # Ground,
	(168,144,240), # Flying,
    (248,88,136), # Psychic,
    (168,184,32), # Bug,
    (184,160,56), # Rock,
	(112,88,152), # Ghost,
    (112,56,248), # Dragon,
	(112,88,72), # Dark,
    (184,184,208), # Steel,
    (240,182,188) # Fairy
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