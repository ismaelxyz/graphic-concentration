#!/usr/bin/python

from OpenGL import GL as gl
from OpenGL import GLUT as glut
from OpenGL import GLE as gle
from maintest import Maintest


def draw_helicoid(frame: Maintest):
    """Draw the scene"""
    gl.glClear((int(gl.GL_COLOR_BUFFER_BIT)) | int(gl.GL_DEPTH_BUFFER_BIT))
    # set up some matrices so that the object spins with the mouse
    gle.gleSetJoinStyle(
        (int(gle.TUBE_NORM_EDGE)) | int(gle.TUBE_JN_ANGLE) | int(gle.TUBE_JN_CAP)
    )
    gl.glColor3f(0.6, 0.8, 0.3)

    gl.glPushMatrix()
    gl.glTranslatef(0.0, 0.0, -80.0)
    gl.glRotatef(frame.last_x, 0.0, 1.0, 0.0)
    gl.glRotatef(frame.last_y, 1.0, 0.0, 0.0)

    gle.gleHelicoid(1.0, 6.0, 2.0, -3.0, 4.0, None, None, 0.0, 1080.0)

    gl.glPopMatrix()

    glut.glutSwapBuffers()


def main():
    frame = Maintest(121.0, 121.0)
    frame.main_loop(lambda: draw_helicoid(frame))


if __name__ == "__main__":
    main()
