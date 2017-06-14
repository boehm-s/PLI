from face_reco import Face

import os
import sys
import re
import pickle
import enum

images = os.listdir('./train_img')

class GetAttr(type):
    def __getitem__(cls, x):
        return getattr(cls, x)

class Eye_color_data(object):
    __metaclass__ = GetAttr

    def __init__(self):
        self.brown = []
        self.blue = []
        self.green = []
        self.grey = []

colors = Eye_color_data()

for img in images:
    color = re.search(r"\_(.+?)\.", img).group(1)
    face = Face('./train_img/' + img)
    detected_color = face.get_eyes_color()
    getattr(colors, color).append(detected_color)


print('I AM HERE !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!! ')
with open('eyes_data.pkl', 'wb') as output:
    pickle.dump(colors, output, pickle.HIGHEST_PROTOCOL)
