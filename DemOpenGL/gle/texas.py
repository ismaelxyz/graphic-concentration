#!/usr/bin/python

from OpenGL import GL as gl
from OpenGL import GLUT as glut
from OpenGL import GLE as gle
from math import sqrt
from maintest import Maintest


class Texas:
    """A class representing the Texas-shaped 3D extrusion."""

    SCALE = 0.8
    TSCALE = 4

    def __init__(self, last_x: float = 121.0, last_y: float = 121.0):
        """Initialize the Texas object with given rotation angles."""
        self.last_x = last_x
        self.last_y = last_y
        self._init_geometry()

    def _init_geometry(self):
        """Initialize all geometry data as instance attributes."""
        # Define the 2D points that make up the Texas shape
        points = (
            (-1.5, 2.0),
            (-0.75, 2.0),
            (-0.75, 1.38),
            (-0.5, 1.25),
            (0.88, 1.12),
            (1.0, 0.62),
            (1.12, 0.1),
            (0.5, -0.5),
            (0.2, -1.12),
            (0.3, -1.5),
            (-0.25, -1.45),
            (-1.06, -0.3),
            (-1.38, -0.3),
            (-1.65, -0.6),
            (-2.5, 0.5),
            (-1.5, 0.5),
            (-1.5, 2.0),
            (-0.75, 2.0),
        )

        # Brand points for the extrusion
        self.brand_points = list(
            map(lambda x: (0, 0, self.TSCALE * x), (0.1, 0.0, -5.0, -5.1))
        )
        self.brand_colors = ((1.0, 0.3, 0.0),) * 4

        # Create the spine (path) for the extrusion
        self.tspine = list(
            map(lambda x: (self.TSCALE * x[0], self.TSCALE * x[1], 0), points)
        )

        # Create the cross-section for the extrusion
        self.texas_xsection = list(
            map(lambda x: (self.SCALE * x[0], self.SCALE * x[1]), points[1:])
        )

        # Generate colors for the extrusion
        self.tcolors = []
        for i in range(len(self.texas_xsection)):
            self.tcolors.append(
                (
                    ((i * 33) % 255) / 255.0,
                    ((i * 47) % 255) / 255.0,
                    ((i * 89) % 255) / 255.0,
                )
            )

        # Calculate normals for the cross-section
        self.texas_normal = []
        for i in range(1, len(self.texas_xsection)):
            ax = self.texas_xsection[i][0] - self.texas_xsection[i - 1][0]
            ay = self.texas_xsection[i][1] - self.texas_xsection[i - 1][1]
            alen = sqrt(ax * ax + ay * ay)
            self.texas_normal.append((-ay / alen, ax / alen))

        # Close the contour by copying the last normal to the first position
        self.texas_normal.insert(0, self.texas_normal[-1])

    def draw(self):
        """Render the Texas-shaped 3D extrusion."""
        gl.glClear(int(gl.GL_COLOR_BUFFER_BIT) | int(gl.GL_DEPTH_BUFFER_BIT))

        # Set up join style for the extrusion
        gle.gleSetJoinStyle(
            int(gle.TUBE_NORM_FACET)
            | int(gle.TUBE_JN_ANGLE)
            | int(gle.TUBE_CONTOUR_CLOSED)
            | int(gle.TUBE_JN_CAP)
        )

        gl.glPushMatrix()
        gl.glTranslatef(0.0, 0.0, -80.0)
        gl.glRotatef(self.last_x, 0.0, 1.0, 0.0)
        gl.glRotatef(self.last_y, 1.0, 0.0, 0.0)

        # Draw the main Texas shape extrusion
        gle.gleExtrusion(
            self.texas_xsection, self.texas_normal, None, self.tspine, self.tcolors
        )

        # Draw the brand extrusion
        gle.gleExtrusion(
            self.texas_xsection,
            self.texas_normal,
            None,
            self.brand_points,
            self.brand_colors,
        )

        gl.glPopMatrix()

        glut.glutSwapBuffers()

    def mouse_motion(self, x: int, y: int):
        """Update rotation angles based on mouse motion."""
        self.last_x = x
        self.last_y = y
        glut.glutPostRedisplay()


def draw_wrapper(frame: Maintest, texas: Texas):
    """Wrapper that syncs rotation values and calls Texas.draw()."""
    texas.last_x = frame.last_x
    texas.last_y = frame.last_y
    texas.draw()


def main():
    """Main entry point for the Texas demo."""
    texas = Texas()
    frame = Maintest(121.0, 121.0)
    frame.last_x = texas.last_x
    frame.last_y = texas.last_y

    frame.main_loop(lambda: draw_wrapper(frame, texas))


if __name__ == "__main__":
    import logging

    logging.basicConfig()
    main()
