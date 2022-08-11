#!/usr/bin/python

from OpenGL.GL import *
from OpenGL.Tk import *
from numpy import *

n = 50
a = arange(0, n)
vertices = transpose(
    reshape(
        array(
            (
                cos(2*pi*a/float(n)),
                sin(3*2*pi*a/float(n))
            )
        ),
        (2, n)
    )
)

colors = ones((n, 3))
colors[0] = [1, 0, 0]
colors[25] = [1, 1, 0]
colors.shape = (n, 3)


def redraw(_):
    global n
    glClearColor(0.5, 0.5, 0.5, 0)
    glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT)
    glOrtho(-1, 1, -1, 1, -1, 1)
    glDisable(GL_LIGHTING)
    glDrawArrays(GL_LINE_LOOP, 0, n)
    glEnable(GL_LIGHTING)


def main():
    global n, colors, vertices
    gl = Opengl(width=400, height=400, double=1)
    gl.redraw = redraw
    gl.autospin_allowed = 1

    glVertexPointerd(vertices)
    glColorPointerd(colors)
    glEnableClientState(GL_VERTEX_ARRAY)
    glEnableClientState(GL_COLOR_ARRAY)

    gl.pack(side='top', expand=1, fill='both')
    gl.mainloop()


if __name__ == "__main__":
    main()
