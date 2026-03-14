#!/usr/bin/python

import OpenGL.GL as gl
import OpenGL.GLUT as glut
import OpenGL.Tk as tk


def init():
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
    gl.glDepthFunc(gl.GL_LESS)
    gl.glEnable(gl.GL_DEPTH_TEST)


def redraw(o):
    gl.glClear(int(gl.GL_COLOR_BUFFER_BIT) | int(gl.GL_DEPTH_BUFFER_BIT))
    gl.glPushMatrix()
    gl.glTranslatef(0, -1, 0)
    gl.glRotatef(250, 1, 0, 0)
    glut.glutSolidCone(1, 2, 50, 10)
    gl.glPopMatrix()


def main():
    gltk = tk.Opengl(width=200, height=200, double=1, depth=1)
    gltk.redraw = redraw
    gltk.autospin_allowed = 1
    gltk.pack(side=tk.TOP, expand=tk.YES, fill=tk.BOTH)
    init()
    gltk.mainloop()


if __name__ == "__main__":
    main()
