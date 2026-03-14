#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import OpenGL.GL as gl
import OpenGL.GLUT as glut
import OpenGL.Tk as tk
from tkinter import IntVar, Radiobutton, W


class Fog:

    def __init__(self):
        self.gltk = tk.Opengl(width=250, height=140, double=1, depth=1)
        self.gltk.redraw = self.redraw
        self.gltk.pack(side="top", expand=1, fill="both")  # type: ignore[call-arg]
        self.mode = IntVar(self.gltk)  # type: ignore[arg-type]
        self.mode.set(int(gl.GL_EXP))
        r1 = Radiobutton(
            text="GL_LINEAR",
            anchor=W,
            variable=self.mode,
            value=int(gl.GL_LINEAR),
            command=self.select_fog,
        )
        r1.pack(side="top", expand=1, fill="both")
        r2 = Radiobutton(
            text="GL_EXP",
            anchor=W,
            variable=self.mode,
            value=int(gl.GL_EXP),
            command=self.select_fog,
        )
        r2.pack(side="top", expand=1, fill="both")

    def run(self):
        self.init()
        self.gltk.mainloop()

    def select_fog(self):
        val = self.mode.get()

        if val == int(gl.GL_LINEAR):
            gl.glFogf(gl.GL_FOG_START, 1.0)
            gl.glFogf(gl.GL_FOG_END, 5.0)
            gl.glFogi(gl.GL_FOG_MODE, val)
        elif val == int(gl.GL_EXP):
            gl.glFogi(gl.GL_FOG_MODE, val)

        self.gltk.tkRedraw()

    def init(self):
        glut.glutInit([])
        gl.glDisable(gl.GL_DITHER)
        gl.glEnable(gl.GL_DEPTH_TEST)
        gl.glDepthFunc(gl.GL_LESS)
        gl.glLightfv(gl.GL_LIGHT0, gl.GL_POSITION, [0.0, 3.0, 3.0, 0.0])
        gl.glLightModelf(gl.GL_LIGHT_MODEL_LOCAL_VIEWER, 0.0)
        gl.glFrontFace(gl.GL_CW)
        gl.glEnable(gl.GL_LIGHTING)
        gl.glEnable(gl.GL_LIGHT0)
        gl.glEnable(gl.GL_AUTO_NORMAL)
        gl.glEnable(gl.GL_NORMALIZE)
        gl.glEnable(gl.GL_FOG)

        fog_color = [0.5, 0.5, 0.5, 1.0]
        gl.glFogi(gl.GL_FOG_MODE, gl.GL_EXP)
        gl.glFogfv(gl.GL_FOG_COLOR, fog_color)
        gl.glFogf(gl.GL_FOG_DENSITY, 0.35)
        gl.glHint(gl.GL_FOG_HINT, gl.GL_DONT_CARE)
        gl.glClearColor(0.5, 0.5, 0.5, 1.0)

    def draw_torus(self, x, y, z):
        gl.glPushMatrix()
        gl.glTranslatef(x, y, z)
        gl.glMaterialfv(gl.GL_FRONT, gl.GL_AMBIENT, [0.1745, 0.01175, 0.01175, 1.0])
        gl.glMaterialfv(gl.GL_FRONT, gl.GL_DIFFUSE, [0.61424, 0.04136, 0.04136, 1.0])
        gl.glMaterialfv(
            gl.GL_FRONT, gl.GL_SPECULAR, [0.727811, 0.626959, 0.626959, 1.0]
        )
        gl.glMaterialfv(gl.GL_FRONT, gl.GL_SHININESS, 0.6 * 128.0)
        glut.glutSolidTorus(0.275, 0.85, 20, 20)
        gl.glPopMatrix()

    def redraw(self, o):
        gl.glClear(int(gl.GL_COLOR_BUFFER_BIT) | int(gl.GL_DEPTH_BUFFER_BIT))

        self.draw_torus(-4.0, -0.5, -1.0)
        self.draw_torus(-2.0, -0.5, -2.0)
        self.draw_torus(0.0, -0.5, -3.0)
        self.draw_torus(2.0, -0.5, -4.0)
        self.draw_torus(4.0, -0.5, -5.0)


def main():
    fog = Fog()
    fog.run()


if __name__ == "__main__":
    main()
