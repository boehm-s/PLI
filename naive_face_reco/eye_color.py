from face_reco import Face, Eye_color_data

import os
import sys
import re
import pickle
import enum

images = os.listdir('./train_img')


colors = Eye_color_data()

for img in images:
    color = re.search(r"\_(.+?)\.", img).group(1)
    face = Face('./train_img/' + img)
    detected_color = face.get_eyes_color()
    getattr(colors, color).append(detected_color)

with open('eyes_data.pkl', 'wb') as output:
    pickle.dump(colors, output, pickle.HIGHEST_PROTOCOL)
