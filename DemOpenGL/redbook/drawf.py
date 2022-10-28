#!/usr/bin/env python3

'''
    drawf.c from the Redbook examples.  
    Converted to Python by Jason L. Petrone 7/00


    Draws the bitmapped letter F on the screen(several times).
    This demonstrates use of the glBitmap() call.
'''

import sys
import struct
from OpenGL.GLUT import *
from OpenGL.GL import *

window = None
RASTERBYTES = (
    0xc0, 0x00, 0xc0, 0x00, 0xc0, 0x00, 0xc0, 0x00, 0xc0, 0x00,
    0xff, 0x00, 0xff, 0x00, 0xc0, 0x00, 0xc0, 0x00, 0xc0, 0x00,
    0xff, 0xc0, 0xff, 0xc0
)
RASTERS =  b''.join([struct.pack('B', byte) for byte in RASTERBYTES])


def init():
    glPixelStorei(GL_UNPACK_ALIGNMENT, 1)
    glClearColor(0.0, 0.0, 0.0, 0.0)


def display():
    print('Display')
    glClear(GL_COLOR_BUFFER_BIT)
    glColor3f(1.0, 1.0, 1.0)
    glRasterPos2i(20, 20)
    print('Beginning bitmaps')
    glBitmap(10, 12, 0.0, 0.0, 11.0, 0.0, RASTERS)
    glBitmap(10, 12, 0.0, 0.0, 11.0, 0.0, RASTERS)
    glBitmap(10, 12, 0.0, 0.0, 11.0, 0.0, RASTERS)
    print('Flushing')
    glFlush()


def reshape(w, h):
    print('Reshape')
    glViewport(0, 0, w, h)
    glMatrixMode(GL_PROJECTION)
    glLoadIdentity()
    glOrtho(0, w, 0, h, -1.0, 1.0)
    glMatrixMode(GL_MODELVIEW)


def keyboard(key, x, y):
    print('Keyboard: ', key, x, y)
    if key == b'\x1b':
        glutDestroyWindow(window)


def main():
    '''
        Main Loop
        Open window with initial window size, title bar,
        RGBA display mode, and handle input events.
    '''
    global window
    glutInit(sys.argv)
    glutInitDisplayMode(GLUT_SINGLE | GLUT_RGB)
    glutInitWindowSize(100, 100)
    glutInitWindowPosition(100, 100)
    window = glutCreateWindow('Drawf')
    init()
    glutReshapeFunc(reshape)
    glutKeyboardFunc(keyboard)
    glutDisplayFunc(display)
    glutMainLoop()


if __name__ == '__main__':
    main()
