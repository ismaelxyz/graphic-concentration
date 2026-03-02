#!/usr/bin/env python3

import sys
from pathlib import Path

import OpenGL.GL as gl
import OpenGL.GLUT as glut
import OpenGL.GLU as glu
from PIL import Image


class Lesson6:
    """OpenGL lesson 6 - Textured cube rendering."""

    ESCAPE = b"\033"

    def __init__(self):
        """Initialize the lesson with default values."""
        self.window = 0
        # Rotations for cube
        self.xrot = 0.0
        self.yrot = 0.0
        self.zrot = 0.0
        self.texture = 0

    def load_textures(self):
        """Load textures from file."""
        image_path = Path(__file__).parent / "NeHe.bmp"
        image = Image.open(image_path)

        ix = image.size[0]
        iy = image.size[1]
        image = image.tobytes("raw", "RGBA", 0, -1)

        # Create Texture
        # 2d texture (x and y size)
        gl.glBindTexture(gl.GL_TEXTURE_2D, gl.glGenTextures(1))

        gl.glPixelStorei(gl.GL_UNPACK_ALIGNMENT, 1)
        gl.glTexImage2D(
            gl.GL_TEXTURE_2D, 0, 3, ix, iy, 0, gl.GL_RGBA, gl.GL_UNSIGNED_BYTE, image
        )
        gl.glTexParameterf(gl.GL_TEXTURE_2D, gl.GL_TEXTURE_WRAP_S, gl.GL_CLAMP)
        gl.glTexParameterf(gl.GL_TEXTURE_2D, gl.GL_TEXTURE_WRAP_T, gl.GL_CLAMP)
        gl.glTexParameterf(gl.GL_TEXTURE_2D, gl.GL_TEXTURE_WRAP_S, gl.GL_REPEAT)
        gl.glTexParameterf(gl.GL_TEXTURE_2D, gl.GL_TEXTURE_WRAP_T, gl.GL_REPEAT)
        gl.glTexParameterf(gl.GL_TEXTURE_2D, gl.GL_TEXTURE_MAG_FILTER, gl.GL_NEAREST)
        gl.glTexParameterf(gl.GL_TEXTURE_2D, gl.GL_TEXTURE_MIN_FILTER, gl.GL_NEAREST)
        gl.glTexEnvf(gl.GL_TEXTURE_ENV, gl.GL_TEXTURE_ENV_MODE, gl.GL_DECAL)

    def init_gl(self, width, height):
        """A general OpenGL initialization function.

        Sets all of the initial parameters.
        We call this right after our OpenGL window is created.
        """
        self.load_textures()
        gl.glEnable(gl.GL_TEXTURE_2D)
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
        if height == 0:  # Prevent A Divide By Zero If The Window Is Too Small
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
        gl.glTranslatef(0.0, 0.0, -5.0)  # Move Into The Screen

        gl.glRotatef(self.xrot, 1.0, 0.0, 0.0)  # Rotate The Cube On It's X Axis
        gl.glRotatef(self.yrot, 0.0, 1.0, 0.0)  # Rotate The Cube On It's Y Axis
        gl.glRotatef(self.zrot, 0.0, 0.0, 1.0)  # Rotate The Cube On It's Z Axis

        gl.glBegin(gl.GL_QUADS)  # Start Drawing The Cube

        # Front Face (note that the texture's corners have to match the quad's corners)
        gl.glTexCoord2f(0.0, 0.0)
        gl.glVertex3f(-1.0, -1.0, 1.0)  # Bottom Left Of The Texture and Quad
        gl.glTexCoord2f(1.0, 0.0)
        gl.glVertex3f(1.0, -1.0, 1.0)  # Bottom Right Of The Texture and Quad
        gl.glTexCoord2f(1.0, 1.0)
        gl.glVertex3f(1.0, 1.0, 1.0)  # Top Right Of The Texture and Quad
        gl.glTexCoord2f(0.0, 1.0)
        gl.glVertex3f(-1.0, 1.0, 1.0)  # Top Left Of The Texture and Quad

        # Back Face
        gl.glTexCoord2f(1.0, 0.0)
        gl.glVertex3f(-1.0, -1.0, -1.0)  # Bottom Right Of The Texture and Quad
        gl.glTexCoord2f(1.0, 1.0)
        gl.glVertex3f(-1.0, 1.0, -1.0)  # Top Right Of The Texture and Quad
        gl.glTexCoord2f(0.0, 1.0)
        gl.glVertex3f(1.0, 1.0, -1.0)  # Top Left Of The Texture and Quad
        gl.glTexCoord2f(0.0, 0.0)
        gl.glVertex3f(1.0, -1.0, -1.0)  # Bottom Left Of The Texture and Quad

        # Top Face
        gl.glTexCoord2f(0.0, 1.0)
        gl.glVertex3f(-1.0, 1.0, -1.0)  # Top Left Of The Texture and Quad
        gl.glTexCoord2f(0.0, 0.0)
        gl.glVertex3f(-1.0, 1.0, 1.0)  # Bottom Left Of The Texture and Quad
        gl.glTexCoord2f(1.0, 0.0)
        gl.glVertex3f(1.0, 1.0, 1.0)  # Bottom Right Of The Texture and Quad
        gl.glTexCoord2f(1.0, 1.0)
        gl.glVertex3f(1.0, 1.0, -1.0)  # Top Right Of The Texture and Quad

        # Bottom Face
        gl.glTexCoord2f(1.0, 1.0)
        gl.glVertex3f(-1.0, -1.0, -1.0)  # Top Right Of The Texture and Quad
        gl.glTexCoord2f(0.0, 1.0)
        gl.glVertex3f(1.0, -1.0, -1.0)  # Top Left Of The Texture and Quad
        gl.glTexCoord2f(0.0, 0.0)
        gl.glVertex3f(1.0, -1.0, 1.0)  # Bottom Left Of The Texture and Quad
        gl.glTexCoord2f(1.0, 0.0)
        gl.glVertex3f(-1.0, -1.0, 1.0)  # Bottom Right Of The Texture and Quad

        # Right face
        gl.glTexCoord2f(1.0, 0.0)
        gl.glVertex3f(1.0, -1.0, -1.0)  # Bottom Right Of The Texture and Quad
        gl.glTexCoord2f(1.0, 1.0)
        gl.glVertex3f(1.0, 1.0, -1.0)  # Top Right Of The Texture and Quad
        gl.glTexCoord2f(0.0, 1.0)
        gl.glVertex3f(1.0, 1.0, 1.0)  # Top Left Of The Texture and Quad
        gl.glTexCoord2f(0.0, 0.0)
        gl.glVertex3f(1.0, -1.0, 1.0)  # Bottom Left Of The Texture and Quad

        # Left Face
        gl.glTexCoord2f(0.0, 0.0)
        gl.glVertex3f(-1.0, -1.0, -1.0)  # Bottom Left Of The Texture and Quad
        gl.glTexCoord2f(1.0, 0.0)
        gl.glVertex3f(-1.0, -1.0, 1.0)  # Bottom Right Of The Texture and Quad
        gl.glTexCoord2f(1.0, 1.0)
        gl.glVertex3f(-1.0, 1.0, 1.0)  # Top Right Of The Texture and Quad
        gl.glTexCoord2f(0.0, 1.0)
        gl.glVertex3f(-1.0, 1.0, -1.0)  # Top Left Of The Texture and Quad

        gl.glEnd()  # Done Drawing The Cube

        self.xrot = self.xrot + 0.2  # X rotation
        self.yrot = self.yrot + 0.2  # Y rotation
        self.zrot = self.zrot + 0.2  # Z rotation

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
    lesson = Lesson6()
    lesson.run()


if __name__ == "__main__":
    main()
