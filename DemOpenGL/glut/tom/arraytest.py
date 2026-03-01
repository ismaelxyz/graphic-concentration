"""GLUT demo: draw a rotating polyline using vertex/color arrays.

Refactor notes:
- Avoids wildcard imports.
- Encapsulates state (arrays + timer) in a small class.
"""

from dataclasses import dataclass, field
from typing import Optional

import sys
import time

import numpy as np
from OpenGL import GL, GLUT


def shuffle(a: np.ndarray, b: np.ndarray) -> np.ndarray:
    return np.ravel(np.transpose(np.reshape(np.concatenate([a, b]), (2, len(a)))))


def build_arrays(n: int) -> tuple[np.ndarray, np.ndarray]:
    a = np.arange(0, n)
    vertex = shuffle(np.cos(2 * np.pi * a / n), np.sin(2 * np.pi * a / n))
    vertex = vertex.reshape((n, 2))
    color = np.random.random(n * 3)
    color = color.reshape((n, 3))

    return vertex, color


@dataclass
class ArrayDemo:
    n: int = 50
    title: str = "Array Drawing Demo"
    rotation_period: float = 10.0

    start_time: float = field(default_factory=time.time)
    vertex: np.ndarray = field(init=False)
    color: np.ndarray = field(init=False)
    window: Optional[int] = None

    def __post_init__(self) -> None:
        self.vertex, self.color = build_arrays(self.n)

    def draw_arrays(self) -> None:
        GL.glVertexPointerd(self.vertex)
        GL.glColorPointerd(self.color)
        GL.glEnableClientState(GL.GL_VERTEX_ARRAY)
        GL.glEnableClientState(GL.GL_COLOR_ARRAY)

        GL.glDisable(GL.GL_LIGHTING)
        try:
            GL.glDrawArrays(GL.GL_LINE_LOOP, 0, self.n)
        finally:
            GL.glEnable(GL.GL_LIGHTING)

    def apply_rotation(self) -> float:
        angle = (
            ((time.time() - self.start_time) % self.rotation_period)
            / self.rotation_period
        ) * 360.0
        GL.glRotatef(angle, 0.0, 1.0, 0.0)
        return angle

    def display(self) -> None:
        GL.glClearColor(0.5, 0.5, 0.5, 0.0)
        GL.glClear(int(GL.GL_COLOR_BUFFER_BIT) | int(GL.GL_DEPTH_BUFFER_BIT))

        GL.glMatrixMode(GL.GL_PROJECTION)
        GL.glLoadIdentity()
        GL.glOrtho(-1, 1, -1, 1, -1, 1)

        GL.glMatrixMode(GL.GL_MODELVIEW)
        GL.glLoadIdentity()
        self.apply_rotation()
        self.draw_arrays()

        GLUT.glutSwapBuffers()

    def idle(self) -> None:
        GLUT.glutPostRedisplay()

    def run(self) -> None:
        print("You should see a polynomial curve rotating about the origin.")

        GLUT.glutInit(sys.argv)
        GLUT.glutInitDisplayMode(
            int(GLUT.GLUT_RGBA) | int(GLUT.GLUT_DOUBLE) | int(GLUT.GLUT_DEPTH)
        )
        self.window = GLUT.glutCreateWindow(self.title)

        GLUT.glutDisplayFunc(self.display)
        GLUT.glutIdleFunc(self.idle)
        GLUT.glutMainLoop()


def main() -> None:
    ArrayDemo().run()


if __name__ == "__main__":
    main()
