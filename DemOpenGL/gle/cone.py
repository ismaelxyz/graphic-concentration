#!/usr/bin/python3

from OpenGL.GL import (
    glClear,
    glPushMatrix,
    glTranslatef,
    glRotatef,
    glPopMatrix,
    GL_COLOR_BUFFER_BIT,
    GL_DEPTH_BUFFER_BIT,
)

from OpenGL import GLUT as glut
from OpenGL import GLE as gle
from maintest import Maintest


def draw_polycone_shape(frame: Maintest):

    glClear(int(GL_COLOR_BUFFER_BIT) | int(GL_DEPTH_BUFFER_BIT))
    # set up some matrices so that the object spins with the mouse
    gle.gleSetJoinStyle(
        int(gle.TUBE_NORM_EDGE) | int(gle.TUBE_JN_ANGLE) | int(gle.TUBE_JN_CAP)
    )
    glPushMatrix()
    glTranslatef(0.0, 0.0, -80.0)
    glRotatef(frame.last_x, 0.0, 1.0, 0.0)
    glRotatef(frame.last_y, 1.0, 0.0, 0.0)

    # Phew. FINALLY, Draw the polycone
    gle.glePolyCone(
        (
            (-6.0, 6.0, 0.0),
            (6.0, 6.0, 0.0),
            (6.0, -6.0, 0.0),
            (-6.0, -6.0, 0.0),
            (-6.0, 6.0, 0.0),
            (6.0, 6.0, 0.0),
        ),
        (
            (0.0, 0.0, 0.0),
            (0.0, 0.8, 0.3),
            (0.8, 0.3, 0.0),
            (0.2, 0.3, 0.9),
            (0.2, 0.8, 0.5),
            (0.0, 0.0, 0.0),
        ),
        (1, 1, 3, 0.5, 2, 1),
    )

    glPopMatrix()

    glut.glutSwapBuffers()


def main():
    maintest = Maintest()
    maintest.main_loop(lambda: draw_polycone_shape(maintest))


if __name__ == "__main__":
    main()
