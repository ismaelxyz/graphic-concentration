#!/usr/bin/python

import OpenGL.GL as gl
import OpenGL.Tk as tk
import OpenGL.GLUT as glut
import sys


def redraw_checker(o):
    gl.glClearColor(1, 0, 1, 0)
    gl.glClear(int(gl.GL_COLOR_BUFFER_BIT) | int(gl.GL_DEPTH_BUFFER_BIT))
    gl.glColor3f(0, 1, 0)
    # draw checkerboard
    CHECKER_SIZE = 4
    gl.glDisable(gl.GL_LIGHTING)
    for x in range(-CHECKER_SIZE, CHECKER_SIZE):
        for y in range(-CHECKER_SIZE, CHECKER_SIZE):
            if (x + y) % 2 == 0:
                gl.glColor3f(1, 1, 0)
            else:
                gl.glColor3f(0, 0, 0)
            gl.glRectf(x, y, x + 1, y + 1)
    gl.glEnable(gl.GL_LIGHTING)

    gl.glPushMatrix()
    gl.glTranslatef(0.0, 0.0, 1.0)
    glut.glutSolidSphere(1.0, 20, 20)
    gl.glPopMatrix()


def init():
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


def redraw_cone(o):
    gl.glClear(int(gl.GL_COLOR_BUFFER_BIT) | int(gl.GL_DEPTH_BUFFER_BIT))
    gl.glPushMatrix()
    gl.glTranslatef(0, -1, 0)
    gl.glRotatef(250, 1, 0, 0)
    glut.glutSolidCone(1, 2, 50, 10)
    gl.glPopMatrix()


def main():
    frame = tk.Frame()
    frame.pack(side=tk.TOP)
    gltk = tk.Opengl(width=200, height=200, double=1, depth=1)
    glut.glutInit([])
    gltk.redraw = redraw_checker
    quit = tk.Button(frame, text="Quit", command=sys.exit)
    quit.pack(side=tk.TOP, expand=tk.YES, fill=tk.BOTH)
    gltk.pack(side=tk.TOP, expand=tk.YES, fill=tk.BOTH)
    gltk.set_eyepoint(20.0)

    gltk2 = tk.Opengl(width=200, height=200, double=1)
    gltk2.redraw = redraw_cone
    gltk2.autospin_allowed = 1
    gltk2.pack(side=tk.TOP, expand=tk.YES, fill=tk.BOTH)
    init()

    gltk.mainloop()


if __name__ == "__main__":
    main()
