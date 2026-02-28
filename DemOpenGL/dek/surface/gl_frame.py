#!/usr/bin/env python3

from OpenGL.GL import *  # type: ignore
from OpenGL.Tk import *  # type: ignore
from numpy import *  # type: ignore
from PIL import Image

WIDTH = 800
HEIGHT = 600


class GLFrame:

    def __init__(self, master=None, redraw=None, *args, **keywords):
        self.image_name = "photo.ppm"
        self.master = master
        self.keywords = keywords
        self.ogl_frame = Frame(self.master, width=320, height=200)
        self.ogl_frame.pack(side="top", expand=1, fill="both")
        self.keywords["double"] = 1
        self.ogl = Opengl(self.ogl_frame, self.keywords)
        self.ogl.pack(side="top", expand=1, fill="both")
        self.ogl.bind("<Shift-Button-2>", self.photo)
        self.ogl.bind("<Button-2>", self.ogl.tkRecordMouse)
        self.ogl.bind("<B2-Motion>", self.ogl.tkTranslate)
        self.ogl.bind("<Button-1>", self.ogl.StartRotate)
        self.ogl.bind("<B1-Motion>", self.ogl.tkRotate)
        self.ogl.bind("<Button-3>", self.ogl.tkRecordMouse)
        self.ogl.bind("<B3-Motion>", self.ogl.tkScale)

        glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA)
        glEnable(GL_BLEND)
        glEnable(GL_LINE_SMOOTH)

        self.ogl.set_background(0, 0, 0)

        if redraw == None:
            self.ogl.redraw = self.redraw
        else:
            self.ogl.redraw = redraw

        self.main_loop = self.ogl.mainloop

        self.axis_points = identity((3))

    def redraw(self, event=None):
        glDisable(GL_LIGHTING)
        glBegin(GL_LINES)

        # +x axis points left
        glColor3f(1, 0, 0)
        glVertex3fv([0, 0, 0])
        glVertex3fv(self.axis_points[0].tolist())

        # +y axis points up
        glColor3f(0, 1, 0)
        glVertex3fv([0, 0, 0])
        glVertex3fv(self.axis_points[1].tolist())

        # +z-axis points away
        glColor3f(0, 0, 1)
        glVertex3fv([0, 0, 0])
        glVertex3fv(self.axis_points[2].tolist())

        glEnd()

    def photo(self, event=None):
        glPixelStorei(GL_UNPACK_ALIGNMENT, 1)
        pixels = glReadPixels(
            0,
            0,
            self.keywords["width"],
            self.keywords["height"],
            GL_RGBA,
            GL_UNSIGNED_BYTE,
        )
        if pixels is None:
            pixels = b""

        im = Image.new("RGB", (self.keywords["width"], self.keywords["height"]))
        im.fromstring(pixels)
        im.save(self.image_name)


if __name__ == "__main__":
    x = GLFrame(None, None, width=320, height=200, double=1, depth=1)
    x.main_loop()
