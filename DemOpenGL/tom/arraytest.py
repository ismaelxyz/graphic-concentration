#!/usr/bin/python

from numpy import *
from numpy.random import *
from OpenGL.Tk import *
from OpenGL.GL import *
import sys

N = 50


def shuffle(a, b):
    return ravel(transpose(reshape(concatenate([a, b]), (2, len(a)))))


def redraw(_):
    glClearColor(1, 1, 1, 0)
    glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT)
    glOrtho(-1, 1, -1, 1, -1, 1)
    glDisable(GL_LIGHTING)
    glDrawArrays(GL_LINE_LOOP, 0, N)
    glEnable(GL_LIGHTING)


def main():
    frame = Frame()
    frame.pack(side='top', expand=1)
    quit = Button(frame, text='Quit', command=sys.exit)
    quit.pack(side='top')
    gl = Opengl(width=400, height=400, double=1)
    a = arange(0, N)
    vertex = shuffle(cos(2 * pi * a / N), sin(2 * pi * a / N))
    vertex.shape = (N, 2)
    color = random(N * 3)
    color.shape = (N, 3)

    glVertexPointerd(vertex)
    glColorPointerd(color)
    glEnableClientState(GL_VERTEX_ARRAY)
    glEnableClientState(GL_COLOR_ARRAY)

    gl.redraw = redraw
    gl.pack(side='top', expand=1, fill='both')
    gl.mainloop()


if __name__ == '__main__':
    main()
