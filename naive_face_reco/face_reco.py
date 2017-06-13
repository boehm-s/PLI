from PIL import Image, ImageDraw
import face_recognition
from colorthief import ColorThief

class Face:
    def __init__(self, path):
        self.image = face_recognition.load_image_file(path)
        self.face_landmarks_list = face_recognition.face_landmarks(self.image)
        emptyImg = Image.new('RGB', (2,2), (255, 255, 255))
        emptyImg.save("empty.png", "PNG")
        self.skin_color = self.__get_skin_color()
        if len(self.face_landmarks_list) == 0:
            raise Exception('No faces found in the image')

    def __get_skin_color(self):
        for face_landmarks in self.face_landmarks_list:
            top_brows = sorted(face_landmarks['left_eyebrow'], key=lambda x: x[1])[0][1]
            bot_nose =  sorted(face_landmarks['nose_tip'], key=lambda x: x[1])[len(face_landmarks['nose_tip']) - 1][1]
            eye_left = sorted(face_landmarks['left_eye'], key=lambda x: x[0])[0][0]
            eye_right = sorted(face_landmarks['right_eye'], key=lambda x: x[0])[len(face_landmarks['right_eye']) - 1][0]
            h3 = bot_nose - top_brows;
            x1 = eye_left + int((eye_right - eye_left) / 10)
            y1 = top_brows - int(3 * (h3 / 6))
            x2 = eye_right - int((eye_right - eye_left) / 10)
            y2 = top_brows - int(h3 / 10)
            rec_skin = (x1, y1, x2, y2)
            print("SKIN REC : {}".format(rec_skin))
            pil_image = Image.fromarray(self.image)
            cropped_skin = pil_image.crop(rec_skin)
            color_thief = ColorThief('./img/empty.png')
            color_thief.image = cropped_skin
            skin_color = color_thief.get_color(quality=1)
            print("SKIN color : {}\n".format(skin_color))
            return skin_color

    def get_eyes_color(self):
        eyes = []
        for face_landmarks in self.face_landmarks_list:

            facial_features = [
                'left_eye',
                'right_eye'
            ]

            for facial_feature in facial_features:
                eyeX = sorted(face_landmarks[facial_feature], key=lambda x: x[0])
                eyeY = sorted(face_landmarks[facial_feature], key=lambda x: x[1])
                eyeLeft = eyeX[0][0]
                eyeTop = eyeY[0][1]
                eyeRight = eyeX[len(eyeX) - 1][0]
                eyeBottom = eyeY[len(eyeY) - 1][1]
                w = (eyeLeft - eyeRight)
                h = (eyeBottom - eyeTop)
                eyeTop -= int(h / 6)
                eyeBottom += int(h / 8)
                eyeLeft -= int(w/4)
                eyeRight += int(w/4)
                rec = (eyeLeft,eyeTop,eyeRight,eyeBottom)
                print("eye REC : {}".format(rec))
                eyes.append(rec)

            pil_image = Image.fromarray(self.image)
            d = ImageDraw.Draw(pil_image)
            skin_color = self.skin_color
            for eye in eyes:
                cropped_im = pil_image.crop(eye)
                cropped_im = cropped_im.convert("RGBA")
                # turn black and white and skin color to transparent
                datas = cropped_im.getdata()
                newData = []
                for item in datas:
                    SKIN = (range(skin_color[0] - 25, skin_color[0] + 25),
                            range(skin_color[1] - 25, skin_color[1] + 25),
                            range(skin_color[2] - 25, skin_color[2] + 25))
                    WB = list(range(0, 30)) + list(range(200, 255))
                    if ((item[0] in WB and item[1] in WB and item[2] in WB)
                        or (len(set(range(item[0]-3, item[0]+3)).intersection(range(item[1]-3, item[1]+3))) != 0
                            and len(set(range(item[1]-3, item[1]+3)).intersection(range(item[2]-3, item[2]+3))) != 0
                            and len(set(range(item[0]-3, item[0]+3)).intersection(range(item[2]-3, item[2]+3))) != 0)
                        or ((item[0] in SKIN[0] and item[1] in SKIN[1] and item[2] in SKIN[2]))):
                        newData.append((255, 255, 255, 0))
                    else:
                        newData.append(item)
                cropped_im.putdata(newData)
                cropped_im.show()
                color_thief = ColorThief('./img/empty.png')
                color_thief.image = cropped_im
                main_color = color_thief.get_color(quality=1)
                print("main color : {}\n".format(main_color))

            for facial_feature in facial_features:
                d.line(face_landmarks[facial_feature], width=5)

            pil_image.show()
        return eyes

path = "./img/portrait.jpg"
face = Face(path)
face.get_eyes_color()
