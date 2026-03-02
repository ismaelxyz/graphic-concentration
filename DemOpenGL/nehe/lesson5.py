#!/usr/bin/env python3

import sys

import OpenGL.GL as gl
import OpenGL.GLUT as glut
import OpenGL.GLU as glu


class Lesson5:
    """OpenGL lesson 5 - 3D shapes (pyramid and cube) rendering."""

    ESCAPE = b"\033"

    def __init__(self):
        """Initialize the lesson with default values."""
        self.window = 0
        # Rotation angle for the triangle (pyramid)
        self.rtri = 0.0
        # Rotation angle for the quadrilateral (cube)
        self.rquad = 0.0

    def init_gl(self, width, height):
        """A general OpenGL initialization function.

        Sets all of the initial parameters.
        We call this right after our OpenGL window is created.
        """
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

    def resize_scene(self, width, height):
        """The function called when our window is resized.

        This shouldn't happen if you enable fullscreen.
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

    def draw_scene(self):
        """The main drawing function."""
        # Clear The Screen And The Depth Buffer
        gl.glClear(int(gl.GL_COLOR_BUFFER_BIT) | int(gl.GL_DEPTH_BUFFER_BIT))
        gl.glLoadIdentity()  # Reset The View

        # Move Left And Into The Screen
        gl.glTranslatef(-1.5, 0.0, -6.0)

        # Rotate The Pyramid On It's Y Axis
        gl.glRotatef(self.rtri, 0.0, 1.0, 0.0)

        gl.glBegin(gl.GL_TRIANGLES)  # Start Drawing The Pyramid

        gl.glColor3f(1.0, 0.0, 0.0)  # Red
        gl.glVertex3f(0.0, 1.0, 0.0)  # Top Of Triangle (Front)
        gl.glColor3f(0.0, 1.0, 0.0)  # Green
        gl.glVertex3f(-1.0, -1.0, 1.0)  # Left Of Triangle (Front)
        gl.glColor3f(0.0, 0.0, 1.0)  # Blue
        gl.glVertex3f(1.0, -1.0, 1.0)

        gl.glColor3f(1.0, 0.0, 0.0)  # Red
        gl.glVertex3f(0.0, 1.0, 0.0)  # Top Of Triangle (Right)
        gl.glColor3f(0.0, 0.0, 1.0)  # Blue
        gl.glVertex3f(1.0, -1.0, 1.0)  # Left Of Triangle (Right)
        gl.glColor3f(0.0, 1.0, 0.0)  # Green
        gl.glVertex3f(1.0, -1.0, -1.0)  # Right

        gl.glColor3f(1.0, 0.0, 0.0)  # Red
        gl.glVertex3f(0.0, 1.0, 0.0)  # Top Of Triangle (Back)
        gl.glColor3f(0.0, 1.0, 0.0)  # Green
        gl.glVertex3f(1.0, -1.0, -1.0)  # Left Of Triangle (Back)
        gl.glColor3f(0.0, 0.0, 1.0)  # Blue
        gl.glVertex3f(-1.0, -1.0, -1.0)  # Right Of

        gl.glColor3f(1.0, 0.0, 0.0)  # Red
        gl.glVertex3f(0.0, 1.0, 0.0)  # Top Of Triangle (Left)
        gl.glColor3f(0.0, 0.0, 1.0)  # Blue
        gl.glVertex3f(-1.0, -1.0, -1.0)  # Left Of Triangle (Left)
        gl.glColor3f(0.0, 1.0, 0.0)  # Green
        gl.glVertex3f(-1.0, -1.0, 1.0)  # Right Of Triangle (Left)
        gl.glEnd()

        gl.glLoadIdentity()
        gl.glTranslatef(1.5, 0.0, -7.0)  # Move Right And Into The Screen
        gl.glRotatef(self.rquad, 1.0, 1.0, 1.0)  # Rotate The Cube On X, Y & Z
        gl.glBegin(gl.GL_QUADS)  # Start Drawing The Cube

        gl.glColor3f(0.0, 1.0, 0.0)  # Set The Color To Blue
        gl.glVertex3f(1.0, 1.0, -1.0)  # Top Right Of The Quad (Top)
        gl.glVertex3f(-1.0, 1.0, -1.0)  # Top Left Of The Quad (Top)
        gl.glVertex3f(-1.0, 1.0, 1.0)  # Bottom Left Of The Quad (Top)
        gl.glVertex3f(1.0, 1.0, 1.0)  # Bottom Right Of The Quad (Top)

        gl.glColor3f(1.0, 0.5, 0.0)  # Set The Color To Orange
        gl.glVertex3f(1.0, -1.0, 1.0)  # Top Right Of The Quad (Bottom)
        gl.glVertex3f(-1.0, -1.0, 1.0)  # Top Left Of The Quad (Bottom)
        gl.glVertex3f(-1.0, -1.0, -1.0)  # Bottom Left Of The Quad (Bottom)
        gl.glVertex3f(1.0, -1.0, -1.0)  # Bottom Right Of The Quad (Bottom)

        gl.glColor3f(1.0, 0.0, 0.0)  # Set The Color To Red
        gl.glVertex3f(1.0, 1.0, 1.0)  # Top Right Of The Quad (Front)
        gl.glVertex3f(-1.0, 1.0, 1.0)  # Top Left Of The Quad (Front)
        gl.glVertex3f(-1.0, -1.0, 1.0)  # Bottom Left Of The Quad (Front)
        gl.glVertex3f(1.0, -1.0, 1.0)  # Bottom Right Of The Quad (Front)

        gl.glColor3f(1.0, 1.0, 0.0)  # Set The Color To Yellow
        gl.glVertex3f(1.0, -1.0, -1.0)  # Bottom Left Of The Quad (Back)
        gl.glVertex3f(-1.0, -1.0, -1.0)  # Bottom Right Of The Quad (Back)
        gl.glVertex3f(-1.0, 1.0, -1.0)  # Top Right Of The Quad (Back)
        gl.glVertex3f(1.0, 1.0, -1.0)  # Top Left Of The Quad (Back)

        gl.glColor3f(0.0, 0.0, 1.0)  # Set The Color To Blue
        gl.glVertex3f(-1.0, 1.0, 1.0)  # Top Right Of The Quad (Left)
        gl.glVertex3f(-1.0, 1.0, -1.0)  # Top Left Of The Quad (Left)
        gl.glVertex3f(-1.0, -1.0, -1.0)  # Bottom Left Of The Quad (Left)
        gl.glVertex3f(-1.0, -1.0, 1.0)  # Bottom Right Of The Quad (Left)

        gl.glColor3f(1.0, 0.0, 1.0)  # Set The Color To Violet
        gl.glVertex3f(1.0, 1.0, -1.0)  # Top Right Of The Quad (Right)
        gl.glVertex3f(1.0, 1.0, 1.0)  # Top Left Of The Quad (Right)
        gl.glVertex3f(1.0, -1.0, 1.0)  # Bottom Left Of The Quad (Right)
        gl.glVertex3f(1.0, -1.0, -1.0)  # Bottom Right Of The Quad (Right)
        gl.glEnd()  # Done Drawing The Quad

        # What values to use?  Well, if you have a FAST machine and a FAST 3D Card, then
        # large values make an unpleasant display with flickering and tearing.  I found that
        # smaller values work better, but this was based on my experience.
        self.rtri = self.rtri + 0.2  # Increase The Rotation Variable For The Triangle
        self.rquad = self.rquad - 0.15  # Decrease The Rotation Variable For The Quad

        # since this is double buffered, swap the buffers to display what just got drawn.
        glut.glutSwapBuffers()

    def key_pressed(self, *args):
        """The function called whenever a key is pressed.

        Note the use of Python tuples to pass in: (key, x, y)
        """
        # If escape is pressed, kill everything.
        if args[0] == self.ESCAPE:
            glut.glutDestroyWindow(self.window)

    def run(self):
        """Main entry point for the lesson."""
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

        # Create the window
        self.window = glut.glutCreateWindow(
            "Jeff Molofee's GL Code Tutorial ... NeHe '99"
        )

        # Register the drawing function with glut
        glut.glutDisplayFunc(self.draw_scene)

        # Uncomment this line to get full screen.
        # glut.glutFullScreen()

        # When we are doing nothing, redraw the scene.
        glut.glutIdleFunc(self.draw_scene)

        # Register the function called when our window is resized.
        glut.glutReshapeFunc(self.resize_scene)

        # Register the function called when the keyboard is pressed.
        glut.glutKeyboardFunc(self.key_pressed)

        # Print message to console, and kick off the main to get it rolling.
        print("Hit ESC key to quit.")

        # Initialize our window.
        self.init_gl(640, 480)

        # Start Event Processing Engine
        glut.glutMainLoop()


def main():
    """Entry point for the lesson."""
    lesson = Lesson5()
    lesson.run()


if __name__ == "__main__":
    main()
