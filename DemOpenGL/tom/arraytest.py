#!/usr/bin/python

import numpy as np
import numpy.random as random
import OpenGL.Tk as tk
import OpenGL.GL as gl
import sys

N_POINTS = 50


def shuffle(a, b):
    return np.ravel(np.transpose(np.reshape(np.concatenate([a, b]), (2, len(a)))))


def redraw(_):
    gl.glClearColor(1, 1, 1, 0)
    gl.glClear(int(gl.GL_COLOR_BUFFER_BIT) | int(gl.GL_DEPTH_BUFFER_BIT))
    gl.glOrtho(-1, 1, -1, 1, -1, 1)
    gl.glDisable(gl.GL_LIGHTING)
    gl.glDrawArrays(gl.GL_LINE_LOOP, 0, N_POINTS)
    gl.glEnable(gl.GL_LIGHTING)


def main():
    frame = tk.Frame()
    frame.pack(side="top", expand=1)
    quit = tk.Button(frame, text="Quit", command=sys.exit)
    quit.pack(side="top")
    gltk = tk.Opengl(width=400, height=400, double=1)
    a = np.arange(0, N_POINTS)
    vertex = shuffle(np.cos(2 * np.pi * a / N_POINTS), np.sin(2 * np.pi * a / N_POINTS))
    vertex.shape = (N_POINTS, 2)
    color = random.random(N_POINTS * 3)
    color.shape = (N_POINTS, 3)

    gl.glVertexPointerd(vertex)
    gl.glColorPointerd(color)
    gl.glEnableClientState(gl.GL_VERTEX_ARRAY)
    gl.glEnableClientState(gl.GL_COLOR_ARRAY)

    gltk.redraw = redraw
    gltk.pack(side="top", expand=1, fill="both")
    gltk.mainloop()


if __name__ == "__main__":
    main()
