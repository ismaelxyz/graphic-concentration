import OpenGL.GLUT as GLUT
from OpenGL.GLUT import (
    glutInit,
    glutGet,
    glutInitDisplayMode,
    glutInitWindowSize,
    glutCreateWindow,
    glutDisplayFunc,
    glutBitmapCharacter,
    glutKeyboardFunc,
    glutTimerFunc,
    glutLeaveMainLoop,
    glutPostRedisplay,
    glutSwapBuffers,
    glutMainLoop,
    GLUT_DOUBLE,
    GLUT_RGBA,
    GLUT_WINDOW_WIDTH,
    GLUT_WINDOW_HEIGHT,
)
from OpenGL import GL as gl

# Some GLUT bitmap font symbols are provided dynamically by PyOpenGL and may not
# exist in static type information, so fetch them via getattr to keep Pylance happy.
GLUT_BITMAP_8_BY_13 = getattr(GLUT, "GLUT_BITMAP_8_BY_13")

ESCAPE = "\033"  # ASCII ESC

PROMPT = ("Press keys '1' - '0' to start callbacks", "Press ESCAPE to exit.")


class TimerCBOwner:
    def __init__(self, name, delay, repeat, all_timers):
        self.all_timers = all_timers
        self.name = name
        self.delay = delay
        self.repeat = repeat
        self.state = "WAITING"
        glutTimerFunc(self.delay, self.cb, 0)
        self.all_timers.append(self)
        glutPostRedisplay()

    def get_description(self):
        return "%s: %s" % (self.name, self.state)

    def cb(self, value):
        self.state = "CALL %d" % value
        if value + 1 == self.repeat:
            self.state += " (LAST!)"

        if value < self.repeat:
            glutTimerFunc(self.delay, self.cb, value + 1)
        else:
            self.all_timers.remove(self)

        glutPostRedisplay()


def keyboard(key, all_timers):
    # PyOpenGL/GLUT may deliver `key` as `bytes` (common on Python 3).
    if isinstance(key, (bytes, bytearray)):
        key = key.decode("utf-8")

    if key == ESCAPE:
        glutLeaveMainLoop()
    else:
        TimerCBOwner(key, 2000, 5, all_timers)


def display(all_timers):
    w = float(glutGet(GLUT_WINDOW_WIDTH))
    h = float(glutGet(GLUT_WINDOW_HEIGHT))
    gl.glViewport(0, 0, int(w), int(h))
    gl.glClearColor(0.0, 0.0, 0.0, 0.0)
    gl.glClear(gl.GL_COLOR_BUFFER_BIT)
    gl.glColor4f(1.0, 1.0, 0.5, 1.0)
    gl.glMatrixMode(gl.GL_PROJECTION)
    gl.glLoadIdentity()
    gl.glMatrixMode(gl.GL_MODELVIEW)
    gl.glLoadIdentity()
    gl.glTranslate(-1.0, 1.0, 0.0)
    scale = 1.0 / w
    gl.glScale(scale, -scale * w / h, 1.0)
    gl.glTranslate(1.0, 1.0, 0.0)
    y = 25.0

    for s in PROMPT:
        gl.glRasterPos(40.0, y)
        y += 30.0
        for c in s:
            # GLUT bitmap font constants are pointers; don't coerce to int.
            glutBitmapCharacter(GLUT_BITMAP_8_BY_13, ord(c))

    y = 100.0
    for t in all_timers:
        gl.glRasterPos(80.0, y)
        for c in t.get_description():
            glutBitmapCharacter(GLUT_BITMAP_8_BY_13, ord(c))
        y += 30.0

    glutSwapBuffers()


def main():
    all_timers = []

    glutInit([])
    glutInitWindowSize(640, 480)
    glutInitDisplayMode(int(GLUT_RGBA) | int(GLUT_DOUBLE))
    glutCreateWindow("glutTimerFunc")
    glutDisplayFunc(lambda: display(all_timers))
    glutKeyboardFunc(lambda key, _x, _y: keyboard(key, all_timers))
    glutMainLoop()


if __name__ == "__main__":
    main()
