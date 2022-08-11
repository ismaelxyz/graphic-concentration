#!/usr/bin/python

from OpenGL.GL import *
from OpenGL.Tk import *
from OpenGL.GLUT import *
from tkinter import *
from PIL import Image
import sys


class App(Frame):
    def __init__(self):
        self.frame = Frame()
        self.frame.pack()

        self.gl = Opengl(width=400, height=400, double=1, depth=1)
        self.gl.redraw = self.redraw
        self.gl.autospin_allowed = 1
        self.gl.pack(side=TOP, expand=YES, fill=BOTH)
        self.gl.set_background(255, 255, 255)
        self.init()

        self.b = Button(self.frame, text="Save", command=self.save)
        self.b.pack(side='top')
        self.quit = Button(self.frame, text='Quit', command=sys.exit)
        self.quit.pack(side='top')
        self.gl.mainloop()

    def init(self):
        glutInit([])
        glMaterialfv(GL_FRONT, GL_AMBIENT, [0.2, 0.2, 0.2, 1.0])
        glMaterialfv(GL_FRONT, GL_DIFFUSE, [0.8, 0.8, 0.8, 1.0])
        glMaterialfv(GL_FRONT, GL_SPECULAR, [1.0, 0.0, 1.0, 1.0])
        glMaterialfv(GL_FRONT, GL_SHININESS, 50.0)
        glLightfv(GL_LIGHT0, GL_AMBIENT, [0.0, 1.0, 0.0, 1.0])
        glLightfv(GL_LIGHT0, GL_DIFFUSE, [1.0, 1.0, 1.0, 1.0])
        glLightfv(GL_LIGHT0, GL_SPECULAR, [1.0, 1.0, 1.0, 1.0])
        glLightfv(GL_LIGHT0, GL_POSITION, [1.0, 1.0, 1.0, 0.0])
        glLightModelfv(GL_LIGHT_MODEL_AMBIENT, [0.2, 0.2, 0.2, 1.0])
        glEnable(GL_LIGHTING)
        glEnable(GL_LIGHT0)

    def redraw(self, o):
        glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT)
        glPushMatrix()
        glTranslatef(0, -1, 0)
        glRotatef(250, 1, 0, 0)
        glutSolidCone(1, 2, 50, 10)
        glPopMatrix()

    def save(self, filename='test.jpg', format="JPEG"):
        width, height = 400, 400
        glPixelStorei(GL_PACK_ALIGNMENT, 1)
        data = glReadPixelsub(0, 0, width, height, GL_RGB)
        image = Image.frombytes("RGB", (width, height), data)
        image = image.transpose(Image.Transpose.FLIP_TOP_BOTTOM)
        image.save(filename, format)
        print('Saved image to %s' % (os.path.abspath(filename)))
        return image
        

if __name__ == '__main__':
    App()
