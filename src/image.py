from PIL import Image
import os
# print("This line will be printed.")
os.chdir('C:/Users/Oliver Chang/Documents/cs181g/Game2DEngine/content')
dims=32
# ​
# ​
seq = [
    ['wiry_side_0002_Still.png', 'wiry_side_0001_Walk-1.png', 'wiry_side_0000_Walk-2.png'],
]
# ​
# ​
row_size=len(max(seq,key = lambda r:len(r)))*dims
col_size=len(seq)*dims
# ​
outfile=Image.new('RGBA',(row_size,col_size))
for y,row in enumerate(seq):
    for x,col in enumerate(row):
        img=Image.open(col)
        outfile.alpha_composite(img,(x*dims,y*dims))
outfile.save('wiry_all.png')