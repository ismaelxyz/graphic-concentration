#!/usr/bin/python

import OpenGL.GL as gl
import OpenGL.GLUT as glut
import OpenGL.Tk as tk
import sys


def redraw(_):
    gl.glClearColor(1, 0, 1, 0)
    gl.glClear(int(gl.GL_COLOR_BUFFER_BIT) | int(gl.GL_DEPTH_BUFFER_BIT))
    gl.glColor3f(0, 1, 0)
    # draw checkerboard
    N = 4
    gl.glDisable(gl.GL_LIGHTING)
    for x in range(-N, N):
        for y in range(-N, N):
            if (x + y) % 2 == 0:
                gl.glColor3f(1, 1, 1)
            else:
                gl.glColor3f(0, 0, 0)
            gl.glRectf(x, y, x + 1, y + 1)
    gl.glEnable(gl.GL_LIGHTING)

    gl.glPushMatrix()
    gl.glTranslatef(0.0, 0.0, 1.0)
    glut.glutSolidSphere(1.0, 20, 20)
    gl.glPopMatrix()


def main():
    frame = tk.Frame()
    frame.pack(side="top")
    gltk = tk.Opengl(width=200, height=200, double=1, depth=1)
    glut.glutInit([])
    gltk.redraw = redraw
    quit = tk.Button(frame, text="Quit", command=sys.exit)
    quit.pack({"side": "top", "side": "left"})
    help = tk.Button(frame, text="Help", command=gltk.help)
    help.pack({"side": "top", "side": "left"})
    reset = tk.Button(frame, text="Reset", command=gltk.reset)
    reset.pack({"side": "top", "side": "left"})

    gltk.pack(side="top", expand=1, fill="both")
    gltk.set_eyepoint(20.0)
    gltk.mainloop()


if __name__ == "__main__":
    main()
