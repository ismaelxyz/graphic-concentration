#!/usr/bin/env python3
# -*- coding: utf-8 -*-

from OpenGL.GL import *
from OpenGL.GLUT import *
from OpenGL.Tk import *


class Fog:

    def __init__(self):
        self.gl = Opengl(width=250, height=140, double=1, depth=1)
        self.gl.redraw = self.redraw
        self.gl.pack(side='top', expand=1, fill='both')
        self.mode = IntVar(self.gl)
        self.mode.set(int(GL_EXP))
        r1 = Radiobutton(text='GL_LINEAR', anchor=W, variable=self.mode,
                         value=int(GL_LINEAR), command=self.selectFog)
        r1.pack(side='top', expand=1, fill='both')
        r2 = Radiobutton(text='GL_EXP', anchor=W, variable=self.mode,
                         value=int(GL_EXP), command=self.selectFog)
        r2.pack(side='top', expand=1, fill='both')

    def run(self):
        self.init()
        self.gl.mainloop()

    def selectFog(self):
        val = self.mode.get()

        if val == int(GL_LINEAR):
            glFogf(GL_FOG_START, 1.0)
            glFogf(GL_FOG_END, 5.0)
            glFogi(GL_FOG_MODE, val)
        elif val == int(GL_EXP):
            glFogi(GL_FOG_MODE, val)

        self.gl.tkRedraw()

    def init(self):
        glutInit([])
        glDisable(GL_DITHER)
        glEnable(GL_DEPTH_TEST)
        glDepthFunc(GL_LESS)
        glLightfv(GL_LIGHT0, GL_POSITION, [0.0, 3.0, 3.0, 0.0])
        glLightModelf(GL_LIGHT_MODEL_LOCAL_VIEWER, 0.0)
        glFrontFace(GL_CW)
        glEnable(GL_LIGHTING)
        glEnable(GL_LIGHT0)
        glEnable(GL_AUTO_NORMAL)
        glEnable(GL_NORMALIZE)
        glEnable(GL_FOG)
        fogColor = [0.5, 0.5, 0.5, 1.0]
        glFogi(GL_FOG_MODE, GL_EXP)
        glFogfv(GL_FOG_COLOR, fogColor)
        glFogf(GL_FOG_DENSITY, 0.35)
        glHint(GL_FOG_HINT, GL_DONT_CARE)
        glClearColor(0.5, 0.5, 0.5, 1.0)

    def drawTorus(self, x, y, z):
        glPushMatrix()
        glTranslatef(x, y, z)
        glMaterialfv(GL_FRONT, GL_AMBIENT, [0.1745, 0.01175, 0.01175, 1.0])
        glMaterialfv(GL_FRONT, GL_DIFFUSE, [0.61424, 0.04136, 0.04136, 1.0])
        glMaterialfv(GL_FRONT, GL_SPECULAR, [
                     0.727811, 0.626959, 0.626959, 1.0])
        glMaterialfv(GL_FRONT, GL_SHININESS, 0.6 * 128.0)
        glutSolidTorus(0.275, 0.85, 20, 20)
        glPopMatrix()

    def redraw(self, o):
        glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT)
        self.drawTorus(-4.0, -0.5, -1.0)
        self.drawTorus(-2.0, -0.5, -2.0)
        self.drawTorus(0.0, -0.5, -3.0)
        self.drawTorus(2.0, -0.5, -4.0)
        self.drawTorus(4.0, -0.5, -5.0)


def main():
    fog = Fog()
    fog.run()


if __name__ == '__main__':
    main()
