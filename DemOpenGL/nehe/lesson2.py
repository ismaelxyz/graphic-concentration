#!/usr/bin/env python3

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
    """The function called when our window is resized (which shouldn't happen
    if you enable fullscreen, below)
    """
    # Prevent A Divide By Zero If The Window Is Too Small
    if height == 0:
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

    # Move Left 1.5 units and into the screen 6.0 units.
    gl.glTranslatef(-1.5, 0.0, -6.0)

    # Draw a triangle
    gl.glBegin(gl.GL_POLYGON)  # Start drawing a polygon
    gl.glVertex3f(0.0, 1.0, 0.0)  # Top
    gl.glVertex3f(1.0, -1.0, 0.0)  # Bottom Right
    gl.glVertex3f(-1.0, -1.0, 0.0)  # Bottom Left
    gl.glEnd()  # We are done with the polygon

    # Move Right 3.0 units.
    gl.glTranslatef(3.0, 0.0, 0.0)

    # Draw a square (quadrilateral)
    gl.glBegin(gl.GL_QUADS)  # Start drawing a 4 sided polygon
    gl.glVertex3f(-1.0, 1.0, 0.0)  # Top Left
    gl.glVertex3f(1.0, 1.0, 0.0)  # Top Right
    gl.glVertex3f(1.0, -1.0, 0.0)  # Bottom Right
    gl.glVertex3f(-1.0, -1.0, 0.0)  # Bottom Left
    gl.glEnd()  # We are done with the polygon

    #  since this is double buffered, swap the buffers to display what just got drawn.
    glut.glutSwapBuffers()


# The function called whenever a key is pressed. Note the use of Python tuples to pass in: (key, x, y)


def key_pressed(window, *args):
    # If escape is pressed, kill everything.
    if args[0] == ESCAPE:
        glut.glutDestroyWindow(window)


def main():

    # For now we just pass glutInit one empty argument. I wasn't sure what should or could be passed in (tuple, list, ...)
    # Once I find out the right stuff based on reading the PyOpenGL source, I'll address this.
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
