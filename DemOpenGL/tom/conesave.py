#!/usr/bin/python

import OpenGL.GL as gl
from OpenGL.Tk import Opengl, Frame, Button, TOP, YES, BOTH
import OpenGL.GLUT as glut

from PIL import Image
import sys
import os


class App(Frame):
    def __init__(self):
        self.frame = Frame()
        self.frame.pack()

        self.gltk = Opengl(width=400, height=400, double=1, depth=1)
        self.gltk.redraw = self.redraw
        self.gltk.autospin_allowed = 1
        self.gltk.pack(side=TOP, expand=YES, fill=BOTH)
        self.gltk.set_background(255, 255, 255)
        self.init()

        self.b = Button(self.frame, text="Save", command=self.save)
        self.b.pack(side="top")
        self.quit_btn = Button(self.frame, text="Quit", command=sys.exit)
        self.quit_btn.pack(side="top")
        self.gltk.mainloop()

    def init(self):
        glut.glutInit([])
        gl.glMaterialfv(gl.GL_FRONT, gl.GL_AMBIENT, [0.2, 0.2, 0.2, 1.0])
        gl.glMaterialfv(gl.GL_FRONT, gl.GL_DIFFUSE, [0.8, 0.8, 0.8, 1.0])
        gl.glMaterialfv(gl.GL_FRONT, gl.GL_SPECULAR, [1.0, 0.0, 1.0, 1.0])
        gl.glMaterialfv(gl.GL_FRONT, gl.GL_SHININESS, 50.0)
        gl.glLightfv(gl.GL_LIGHT0, gl.GL_AMBIENT, [0.0, 1.0, 0.0, 1.0])
        gl.glLightfv(gl.GL_LIGHT0, gl.GL_DIFFUSE, [1.0, 1.0, 1.0, 1.0])
        gl.glLightfv(gl.GL_LIGHT0, gl.GL_SPECULAR, [1.0, 1.0, 1.0, 1.0])
        gl.glLightfv(gl.GL_LIGHT0, gl.GL_POSITION, [1.0, 1.0, 1.0, 0.0])
        gl.glLightModelfv(gl.GL_LIGHT_MODEL_AMBIENT, [0.2, 0.2, 0.2, 1.0])
        gl.glEnable(gl.GL_LIGHTING)
        gl.glEnable(gl.GL_LIGHT0)

    def redraw(self, o):
        gl.glClear(int(gl.GL_COLOR_BUFFER_BIT) | int(gl.GL_DEPTH_BUFFER_BIT))
        gl.glPushMatrix()
        gl.glTranslatef(0, -1, 0)
        gl.glRotatef(250, 1, 0, 0)
        glut.glutSolidCone(1, 2, 50, 10)
        gl.glPopMatrix()

    def save(self, filename="test.jpg", format="JPEG"):
        width, height = 400, 400
        gl.glPixelStorei(gl.GL_PACK_ALIGNMENT, 1)
        data = gl.glReadPixelsub(0, 0, width, height, gl.GL_RGB)
        image = Image.frombytes("RGB", (width, height), data)
        image = image.transpose(Image.Transpose.FLIP_TOP_BOTTOM)
        image.save(filename, format)
        print("Saved image to %s" % (os.path.abspath(filename)))
        return image


if __name__ == "__main__":
    App()
