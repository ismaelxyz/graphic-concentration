#!/usr/bin/env python3


"""
This code is not a good example of Python and using OO techniques.  It is a
simple and direct exposition of how to use the Open GL API in Python via the
PyOpenGL package.  It also uses GLUT, which in my opinion is a high quality
library in that it makes my work simpler.  Due to using these APIs, this code
is more like a C program using function based programming (which Python is in
fact based upon, note the use of closures and lambda) than a "good" OO program.

To run this code get and install OpenGL, GLUT, PyOpenGL and numpy.
Installing numpy  means having a C compiler that is configured properly, or so
I found.  For Win32 this assumes VCS, I poked through the setup.py for Numeric,
and chased through disutils code and noticed what seemed to be hard coded
preferences for VCS in the case of a Win32 OS.

BTW, since this is Python make sure you use tabs or spaces to indent, I had
numerous problems since I was using editors that were not sensitive to Python.
"""

from OpenGL import GL as gl
from OpenGL import GLUT as glut
from OpenGL import GLU as glu
import sys

# Some api in the chain is translating the keystrokes to this octal string
# so instead of saying: ESCAPE = 27, we use the following.
ESCAPE = b"\033"

# A general OpenGL initialization function.  Sets all of the initial parameters.
# We call this right after our OpenGL window is created.


def init_gl(width, height):
    # This Will Clear The Background Color To Black
    gl.glClearColor(0.0, 0.0, 0.0, 0.0)
    gl.glClearDepth(1.0)  # Enables Clearing Of The Depth Buffer
    gl.glDepthFunc(gl.GL_LESS)  # The Type Of Depth Test To Do
    gl.glEnable(gl.GL_DEPTH_TEST)  # Enables Depth Testing
    gl.glShadeModel(gl.GL_SMOOTH)  # Enables Smooth Color Shading

    gl.glMatrixMode(gl.GL_PROJECTION)
    gl.glLoadIdentity()  # Reset The Projection Matrix

    # Calculate The Aspect Ratio Of The Window
    glu.gluPerspective(45.0, float(width) / float(height), 0.1, 100.0)

    gl.glMatrixMode(gl.GL_MODELVIEW)


def resize_scene(width, height):
    """
    The function called when our window is resized (which shouldn't happen if you
    enable fullscreen, below)
    """
    if height == 0:  # Prevent A Divide By Zero If The Window Is Too Small
        height = 1

    # Reset The Current Viewport And Perspective Transformation
    gl.glViewport(0, 0, width, height)

    gl.glMatrixMode(gl.GL_PROJECTION)
    gl.glLoadIdentity()
    glu.gluPerspective(45.0, float(width) / float(height), 0.1, 100.0)
    gl.glMatrixMode(gl.GL_MODELVIEW)


def draw_scene():
    """The main drawing function."""
    # Clear The Screen And The Depth Buffer
    gl.glClear(int(gl.GL_COLOR_BUFFER_BIT) | int(gl.GL_DEPTH_BUFFER_BIT))
    gl.glLoadIdentity()  # Reset The View

    #  since this is double buffered, swap the buffers to display what just got
    #  drawn.
    glut.glutSwapBuffers()


def key_pressed(window, *args):
    """
    The function called whenever a key is pressed. Note the use of Python tuples
    to pass in: (key, x, y)
    """
    # If escape is pressed, kill everything.
    if args[0] == ESCAPE:
        glut.glutDestroyWindow(window)


def main():

    # pass arguments to init
    glut.glutInit(sys.argv)

    # Select type of Display mode:
    #  Double buffer
    #  RGBA color
    # Alpha components supported
    # Depth buffer
    glut.glutInitDisplayMode(
        int(glut.GLUT_RGBA) | int(glut.GLUT_DOUBLE) | int(glut.GLUT_DEPTH)
    )

    # get a 640 x 480 window
    glut.glutInitWindowSize(640, 480)

    # the window starts at the upper left corner of the screen
    glut.glutInitWindowPosition(0, 0)

    # Okay, like the C version we retain the window id to use when closing, but for those of you new
    # to Python (like myself), remember this assignment would make the variable local and not global
    # if it weren't for the global declaration at the start of main.
    window = glut.glutCreateWindow("Jeff Molofee's GL Code Tutorial ... NeHe '99")

    # Register the drawing function with glut, BUT in Python land, at least using PyOpenGL, we need to
    # set the function pointer and invoke a function to actually register the callback, otherwise it
    # would be very much like the C version of the code.
    glut.glutDisplayFunc(draw_scene)

    # Uncomment this line to get full screen.
    # glutFullScreen()

    # When we are doing nothing, redraw the scene.
    glut.glutIdleFunc(draw_scene)

    # Register the function called when our window is resized.
    glut.glutReshapeFunc(resize_scene)

    # Register the function called when the keyboard is pressed.
    glut.glutKeyboardFunc(lambda key, x, y: key_pressed(window, key, x, y))

    # Print message to console, and kick off the main to get it rolling.
    print("Hit ESC key to quit.")

    # Initialize our window.
    init_gl(640, 480)

    # Start Event Processing Engine
    glut.glutMainLoop()


if __name__ == "__main__":
    main()
