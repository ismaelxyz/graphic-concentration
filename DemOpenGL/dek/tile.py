#!/usr/bin/env python3

import sys
from PIL.Image import open as pil_open
from OpenGL.GL import *
from OpenGL.Tk import *
import math

const = math.pi


class Checker:

    def __init__(self):
        self.filename = 'image.ppm'

        if len(sys.argv) == 2:
            self.filename = sys.argv[1]

        elif len(sys.argv) > 2 or '-h' in sys.argv:

            sys.stderr.write('usage: <name> ppmfilename\n')
            exit(0)

        self.SetupWindow()
        self.SetupTexture()
        self.ogl.mainloop()

    def make_image(self):
        with pil_open(self.filename) as im:
            self.imageWidth = im.size[0]
            self.imageHeight = im.size[1]
            self.image = im.tobytes("raw", "RGBX", 0, -1)

    def display(self, event=None):
        glClearColor(0.0, 0.0, 0.0, 0)
        glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT)
        glBegin(GL_QUADS)

        glTexCoord2f(0.0, 0.0)
        glVertex3f(0.0, 0.0, 0.0)
        glTexCoord2f(0.0, 2.0)
        glVertex3f(0.0, 10., 0.0)
        glTexCoord2f(2.0, 2.0)
        glVertex3f(10., 10., 0.0)
        glTexCoord2f(2.0, 0.0)
        glVertex3f(10., 0.0, 0.0)

        glEnd()
        glFlush()

    def SetupWindow(self):
        self.OglFrame = Frame()
        self.OglFrame.pack(side='top')
        self.QuitButton = Button(self.OglFrame, {'text': 'Quit'})
        self.QuitButton.bind('<ButtonRelease-1>', sys.exit)
        self.QuitButton.pack({'side': 'top'})
        self.ogl = Opengl(master=self.OglFrame,
                          width=500, height=500, double=1)
        self.ogl.pack(side='top', expand=1, fill='both')
        # self.ogl.set_eyepoint(900.)
        # self.ogl.set_centerpoint(0, 0, 0)
        self.ogl.redraw = self.display

    def SetupTexture(self):
        self.make_image()
        glPixelStorei(GL_UNPACK_ALIGNMENT, 1)
        glTexImage2D(GL_TEXTURE_2D, 0, 3, self.imageWidth,
                     self.imageHeight, 0, GL_RGBA, GL_UNSIGNED_BYTE,  self.image)
        # glTexParameterf(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP)
        # glTexParameterf(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP)
        glTexParameterf(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_REPEAT)
        glTexParameterf(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_REPEAT)
        glTexParameterf(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST)
        glTexParameterf(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST)
        glTexEnvf(GL_TEXTURE_ENV, GL_TEXTURE_ENV_MODE, GL_DECAL)
        glEnable(GL_TEXTURE_2D)
        glShadeModel(GL_FLAT)


if __name__ == '__main__':
    Checker()
