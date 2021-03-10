# NeHe Tutorial Lesson: 42 - Multiple Viewports
#
# Ported to PyOpenGL 2.0 by Brian Leair 18 Jan 2004
#
# This code was created by Jeff Molofee 2000
#
# The port was based on the PyOpenGL tutorials and from
# PyOpenGLContext (tests/glprint.py)
#
# If you've found this code useful, feel free to let me know
# at (Brian Leair telcom_sage@yahoo.com).
#
# See original source and C based tutorial at http://nehe.gamedev.net
#
# Note:
# -----
# This code is not an ideal example of Pythonic coding or use of OO techniques.
# It is a simple and direct exposition of how to use the Open GL API in
# Python via the PyOpenGL package. It also uses GLUT, a high quality
# platform independent library. Due to using these APIs, this code is
# more like a C program using procedural based programming.
#
# To run this example you will need:
# Python     - www.python.org (v 2.3 as of 1/2004)
# PyOpenGL     - pyopengl.sourceforge.net (v 2.0.1.07 as of 1/2004)
# numpy      - numpy.sourceforge.net


# /***************************************************************************************************************
# *                                                      *                                                       *
# *  Lesson 42: Multiple Viewports                       *  Created:  05/17/2003                                 *
# *                                                      *                                                       *
# *  This Program Was Written By Jeff Molofee (NeHe)     *  Runs Much Faster (Many Useless Loops Removed)        *
# *  From http://nehe.gamedev.net.                       *                                                       *
# *                                                      *  Maze Code Is Still Very Unoptimized.  Speed Can Be   *
# *  I Wanted To Create A Maze, And Was Able To Find     *  Increased Considerably By Keeping Track Of Cells     *
# *  Example Code, But Most Of It Was Uncommented And    *  That Have Been Visited Rather Than Randomly          *
# *  Difficult To Figure Out.                            *  Searching For Cells That Still Need To Be Visited.   *
# *                                                      *                                                       *
# *  This Is A Direct Conversion Of Basic Code I Wrote   *  This Tutorial Demonstrates Multiple Viewports In A   *
# *  On The Atari XE Many Years Ago.                     *  Single Window With Both Ortho And Perspective Modes  *
# *                                                      *  Used At The Same Time.  As Well, Two Of The Views    *
# *  It Barely Resembles The Basic Code, But The Idea    *  Have Lighting Enabled, While The Other Two Do Not.   *
# *  Is Exactly The Same.                                *                                                       *
# *                                                      *********************************************************
# *  Branches Are Always Made From An Existing Path      *
# *  So There Should Always Be A Path Through The Maze   *
# *  Although It Could Be Quite Short :)                 *
# *                                                      *
# *  Do Whatever You Want With This Code.  If You Found  *
# *  It Useful Or Have Made Some Nice Changes To It,     *
# *  Send Me An Email: nehe@connect.ab.ca                *
# *                                                      *
# *******************************************************/
#

#
# Ported to PyOpenGL 2.0 by Brian Leair  Feb, 2004
#
from OpenGL.GL import *
from OpenGL.GLUT import *
from OpenGL.GLU import *
from numpy import zeros
import random
from time import sleep
import sys


# Some api in the chain is translating the keystrokes to this octal string
# so instead of saying: ESCAPE = 27, we use the following.
ESCAPE = b'\033'

# Number of the glut window.
window = 0


# User Defined Variables

# General Loops (Used For Seeking)
mx = 0
my = 0
done = False  # Flag To Let Us Know When It's Done

width = 128  # Maze Width  (Must Be A Power Of 2)
height = 128  # Maze Height (Must Be A Power Of 2)

# numarray of unsigned bytes - # // Holds Our RGB Texture Data
tex_data = None

# // The Quadric Object
quadric = None

r = [None] * 4
g = [None] * 4
# // Random Colors (4 Red, 4 Green, 4 Blue)
b = [None] * 4

xrot = 0
yrot = 0
# // Use For Rotation Of Objects
zrot = 0


def UpdateTex(dmx, dmy):
    """ // Update Pixel dmx, dmy On The Texture """
    global tex_data

    # // Set Red Pixel To Full Bright
    tex_data[0+((dmx+(width*dmy))*3)] = 255
    # // Set Green Pixel To Full Bright
    tex_data[1+((dmx+(width*dmy))*3)] = 255
    # // Set Blue Pixel To Full Bright
    tex_data[2+((dmx+(width*dmy))*3)] = 255
    return


def Reset():
    """ // Reset The Maze, Colors, Start Point, Etc    """
    global tex_data, r, g, b, mx, my

    # ZeroMemory(tex_data, width * height *3);                            // Clear Out The Texture Memory With 0's
    # This creates or array of unsigned bytes for our texture data. All values initialized to 0
    # tex_data = numarray.zeros ((width * height * 3), type="u1")
    tex_data = zeros((width * height * 3), "b")

    # This Will seed the random num stream with current system time.
    random.seed()

    # // Loop So We Can Assign 4 Random Colors
    for loop in range(4):
        # // Pick A Random Red Color (Bright)
        r[loop] = 128 + random.randint(0, 127)
        # // Pick A Random Green Color (Bright)
        g[loop] = 128 + random.randint(0, 127)
        # // Pick A Random Blue Color (Bright)
        b[loop] = 128 + random.randint(0, 127)

    # // Pick A New Random X Position
    mx = random.randint(0, (width/2) - 1) * 2
    # // Pick A New Random Y Position
    my = random.randint(0, (width/2) - 1) * 2
    return


# // Any GL Init Code & User Initialiazation Goes Here
# We call this right after our OpenGL window is created.
def InitGL(Width, Height):
    global tex_data, width, height, quadric

    # // Call Reset To Build Our Initial Texture, Etc.
    Reset()

    glEnable(GL_TEXTURE_2D)  # // Enable Texture Mapping
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP)
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP)
    glTexParameterf(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR)
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR)
    glTexImage2D(GL_TEXTURE_2D, 0, GL_RGB, width, height, 0,
                 GL_RGB, GL_UNSIGNED_BYTE, tex_data.tobytes())

    # // Black Background
    glClearColor(0.0, 0.0, 0.0, 0.0)
    glClearDepth(1.0)          # // Depth Buffer Setup
    glDepthFunc(GL_LEQUAL)  # // The Type Of Depth Testing
    glEnable(GL_DEPTH_TEST)  # // Enable Depth Testing

    # // Enable Color Material (Allows Us To Tint Textures)
    glEnable(GL_COLOR_MATERIAL)

    quadric = gluNewQuadric()  # // Create A Pointer To The Quadric Object
    # // Create Smooth Normals
    gluQuadricNormals(quadric, GLU_SMOOTH)
    # // Create Texture Coords
    gluQuadricTexture(quadric, GL_TRUE)

    glEnable(GL_LIGHT0)      # // Enable Light0 (Default GL Light)

    return True              # // Return TRUE (Initialization Successful)


def Update():
    """ Solves/builds the maze. """
    global width, height, done, mx, my

    done = True                          # // Set done To True
    # // Loop Through All The Rooms
    for x in range(0, width, 2):
        for y in range(0, height, 2):                                    # // On X And Y Axis
            # // If Current Texture Pixel (Room) Is Blank
            if (tex_data[((x+(width*y))*3)] == 0):
                # // We Have To Set done To False (Not Finished Yet)
                done = False

    # // If done Is True Then There Were No Unvisited Rooms
    if (done):
        # // Display A Message At The Top Of The Window, Pause For A Bit And Then Start Building A New Maze!
        glutSetWindowTitle(
            "Lesson 42: Multiple Viewports... 2003 NeHe Productions... Maze Complete!")
        sleep(5)
        glutSetWindowTitle(
            "Lesson 42: Multiple Viewports... 2003 NeHe Productions... Building Maze!")
        Reset()

    # // Check To Make Sure We Are Not Trapped (Nowhere Else To Move)
    if (((mx > (width-4) or tex_data[(((mx+2)+(width*my))*3)] == 255)) and ((mx < 2 or tex_data[(((mx-2)+(width*my))*3)] == 255)) and
            ((my > (height-4) or tex_data[((mx+(width*(my+2)))*3)] == 255)) and ((my < 2 or tex_data[((mx+(width*(my-2)))*3)] == 255))):
        while True:                                                # // If We Are Trapped
            # // Pick A New Random X Position
            mx = random.randint(0, (width/2) - 1) * 2
            # // Pick A New Random Y Position
            my = random.randint(0, (height/2) - 1) * 2
            if (tex_data[((mx+(width*my))*3)] == 0):
                break
                # // Keep Picking A Random Position Until We Find
                # // One That Has Already Been Tagged (Safe Starting Point)

    # // Pick A Random Direction
    dir = random.randint(0, 3)

    # // If The Direction Is 0 (Right) And We Are Not At The Far Right
    if ((dir == 0) and (mx <= (width-4))):
        # // And If The Room To The Right Has Not Already Been Visited
        if (tex_data[(((mx+2)+(width*my))*3)] == 0):
            # // Update The Texture To Show Path Cut Out Between Rooms
            UpdateTex(mx+1, my)
            # // Move To The Right (Room To The Right)
            mx += 2

    # // If The Direction Is 1 (Down) And We Are Not At The Bottom
    if ((dir == 1) and (my <= (height-4))):
        # // And If The Room Below Has Not Already Been Visited
        if (tex_data[((mx+(width*(my+2)))*3)] == 0):
            # // Update The Texture To Show Path Cut Out Between Rooms
            UpdateTex(mx, my+1)
            my += 2                      # // Move Down (Room Below)

    # // If The Direction Is 2 (Left) And We Are Not At The Far Left
    if ((dir == 2) and (mx >= 2)):
        # // And If The Room To The Left Has Not Already Been Visited
        if (tex_data[(((mx-2)+(width*my))*3)] == 0):
            # // Update The Texture To Show Path Cut Out Between Rooms
            UpdateTex(mx-1, my)
            # // Move To The Left (Room To The Left)
            mx -= 2

    # // If The Direction Is 3 (Up) And We Are Not At The Top
    if ((dir == 3) and (my >= 2)):
        # // And If The Room Above Has Not Already Been Visited
        if (tex_data[((mx+(width*(my-2)))*3)] == 0):
            # // Update The Texture To Show Path Cut Out Between Rooms
            UpdateTex(mx, my-1)
            my -= 2                      # // Move Up (Room Above)

    UpdateTex(mx, my)                  # // Update Current Room
    return


last_milliseconds = 0


def DrawGLScene():
    """ Our Drawing Routine """
    global xrot, yrot, zrot
    global width, height, tex_data
    global quadric
    global r, g, b
    global last_milliseconds

    cur_milliseconds = glutGet(GLUT_ELAPSED_TIME)
    milliseconds = cur_milliseconds - last_milliseconds
    last_milliseconds = cur_milliseconds

    xrot += milliseconds * .02  # Increase Rotation On The X-Axis
    yrot += milliseconds * .03  # Increase Rotation On The Y-Axis
    zrot += milliseconds * .015  # Increase Rotation On The Z-Axis

    Update()

    # // Get Window Dimensions
    window_width = glutGet(GLUT_WINDOW_WIDTH)
    window_height = glutGet(GLUT_WINDOW_HEIGHT)

    # // Update Our Texture... This Is The Key To The Programs Speed... Much Faster Than Rebuilding The Texture Each Time
    glTexSubImage2D(GL_TEXTURE_2D, 0, 0, 0, width, height,
                    GL_RGB, GL_UNSIGNED_BYTE, tex_data.tobytes())

    glClear(GL_COLOR_BUFFER_BIT)      # // Clear Screen

    # // Loop To Draw Our 4 Views
    for loop in range(4):
        # // Assign Color To Current View
        glColor3ub(r[loop], g[loop], b[loop])

        if loop == 0:                                                    # // If We Are Drawing The First Scene
            # // Set The Viewport To The Top Left.  It Will Take Up Half The Screen Width And Height

            glViewport(0, window_height//2, window_width//2, window_height//2)
            # // Select The Projection Matrix
            glMatrixMode(GL_PROJECTION)
            glLoadIdentity()  # Reset The Projection Matrix
            # // Set Up Ortho Mode To Fit 1/4 The Screen (Size Of A Viewport)
            gluOrtho2D(0, window_width//2, window_height/7/2, 0)

        if loop == 1:  # If We Are Drawing The Second Scene
            # // Set The Viewport To The Top Right.  It Will Take Up Half The Screen Width And Height
            glViewport(window_width//2, window_height//2,
                       window_width//2, window_height//2)
            # // Select The Projection Matrix
            glMatrixMode(GL_PROJECTION)
            glLoadIdentity()          # // Reset The Projection Matrix
            # // Set Up Perspective Mode To Fit 1/4 The Screen (Size Of A Viewport)
            gluPerspective(45.0, float(width) / float(height), 0.1, 500.0)

        if loop == 2:  # If We Are Drawing The Third Scene
            # // Set The Viewport To The Bottom Right.  It Will Take Up Half The Screen Width And Height
            glViewport(window_width//2, 0, window_width//2, window_height//2)
            # // Select The Projection Matrix
            glMatrixMode(GL_PROJECTION)
            glLoadIdentity()          # // Reset The Projection Matrix
            # // Set Up Perspective Mode To Fit 1/4 The Screen (Size Of A Viewport)
            gluPerspective(45.0, float(width) / float(height), 0.1, 500.0)

        if loop == 3:                                                    # // If We Are Drawing The Fourth Scene
            # // Set The Viewport To The Bottom Left.  It Will Take Up Half The Screen Width And Height
            glViewport(0, 0, window_width//2, window_height//2)
            # // Select The Projection Matrix
            glMatrixMode(GL_PROJECTION)
            glLoadIdentity()          # // Reset The Projection Matrix
            # // Set Up Perspective Mode To Fit 1/4 The Screen (Size Of A Viewport)
            gluPerspective(45.0, float(width) / float(height), 0.1, 500.0)

        glMatrixMode(GL_MODELVIEW)  # // Select The Modelview Matrix
        glLoadIdentity()              # // Reset The Modelview Matrix

        glClear(GL_DEPTH_BUFFER_BIT)  # // Clear Depth Buffer

        # Are We Drawing The First Image?  (Original Texture... Ortho)
        if loop == 0:
            glBegin(GL_QUADS)  # Begin Drawing A Single Quad

            # We Fill The Entire 1/4 Section With A Single Textured Quad.
            glTexCoord2f(1.0, 0.0)
            glVertex2i(window_width//2, 0)
            glTexCoord2f(0.0, 0.0)
            glVertex2i(0, 0)
            glTexCoord2f(0.0, 1.0)
            glVertex2i(0, window_height//2)
            glTexCoord2f(1.0, 1.0)
            glVertex2i(window_width//2, window_height//2)

            glEnd()  # Done Drawing The Textured Quad

        # // Are We Drawing The Second Image?  (3D Texture Mapped Sphere... Perspective)
        if (loop == 1):
            # // Move 14 Units Into The Screen
            glTranslatef(0.0, 0.0, -14.0)

            # // Rotate By xrot On The X-Axis
            glRotatef(xrot, 1.0, 0.0, 0.0)
            # // Rotate By yrot On The Y-Axis
            glRotatef(yrot, 0.0, 1.0, 0.0)
            # // Rotate By zrot On The Z-Axis
            glRotatef(zrot, 0.0, 0.0, 1.0)

            glEnable(GL_LIGHTING)      # // Enable Lighting
            # // Draw A Sphere
            gluSphere(quadric, 4.0, 32, 32)
            glDisable(GL_LIGHTING)      # // Disable Lighting

        # // Are We Drawing The Third Image?  (Texture At An Angle... Perspective)
        if (loop == 2):
            glTranslatef(0.0, 0.0, -2.0)  # // Move 2 Units Into The Screen
            # // Tilt The Quad Below Back 45 Degrees.
            glRotatef(-45.0, 1.0, 0.0, 0.0)
            # // Rotate By zrot/1.5 On The Z-Axis
            glRotatef(zrot/1.5, 0.0, 0.0, 1.0)

            glBegin(GL_QUADS)          # // Begin Drawing A Single Quad

            glTexCoord2f(1.0, 1.0)
            glVertex3f(1.0,  1.0, 0.0)
            glTexCoord2f(0.0, 1.0)
            glVertex3f(-1.0,  1.0, 0.0)
            glTexCoord2f(0.0, 0.0)
            glVertex3f(-1.0, -1.0, 0.0)
            glTexCoord2f(1.0, 0.0)
            glVertex3f(1.0, -1.0, 0.0)

            glEnd()                  # // Done Drawing The Textured Quad

        # // Are We Drawing The Fourth Image?  (3D Texture Mapped Cylinder... Perspective)
        if (loop == 3):
            glTranslatef(0.0, 0.0, -7.0)  # // Move 7 Units Into The Screen
            # // Rotate By -xrot/2 On The X-Axis
            glRotatef(-xrot/2, 1.0, 0.0, 0.0)
            # // Rotate By -yrot/2 On The Y-Axis
            glRotatef(-yrot/2, 0.0, 1.0, 0.0)
            # // Rotate By -zrot/2 On The Z-Axis
            glRotatef(-zrot/2, 0.0, 0.0, 1.0)

            glEnable(GL_LIGHTING)      # // Enable Lighting
            # // Translate -2 On The Z-Axis (To Rotate Cylinder Around The Center, Not An End)
            glTranslatef(0.0, 0.0, -2.0)
            # // Draw A Cylinder
            gluCylinder(quadric, 1.5, 1.5, 4.0, 32, 16)
            glDisable(GL_LIGHTING)      # // Disable Lighting

    # // Flush The GL Rendering Pipeline
    glutSwapBuffers()
    return True

# The function called when our window is resized (which shouldn't happen if you enable fullscreen, below)


def ReSizeGLScene(Width, Height):
    if Height == 0:                        # Prevent A Divide By Zero If The Window Is Too Small
        Height = 1

    # Reset The Current Viewport And Perspective Transformation
    glViewport(0, 0, Width, Height)
    glMatrixMode(GL_PROJECTION)
    glLoadIdentity()
    # // field of view, aspect ratio, near and far
    # This will squash and stretch our objects as the window is resized.
    gluPerspective(45.0, float(Width)/float(Height), 0.1, 100.0)

    glMatrixMode(GL_MODELVIEW)
    glLoadIdentity()


# The function called whenever a key is pressed. Note the use of Python tuples to pass in: (key, x, y)
def keyPressed(*args):
    global window

    # If escape is pressed, kill everything.
    if args[0] == ESCAPE:
        glutDestroyWindow(window)

    # Check To See If Spacebar Is Pressed
    if (args[0] == b' '):
        Reset()                      # // If So, Call Reset And Start A New Maze

    return


def main():
    global window
    # pass arguments to init
    glutInit(sys.argv)

    # Select type of Display mode:
    #  Double buffer
    #  RGBA color
    # Alpha components supported
    # Depth buffer
    glutInitDisplayMode(GLUT_RGBA | GLUT_DOUBLE | GLUT_ALPHA | GLUT_DEPTH)

    glutInitWindowSize(1024, 768)

    # the window starts at the upper left corner of the screen
    glutInitWindowPosition(0, 0)

    # Okay, like the C version we retain the window id to use when closing,
    # but for those of you new
    # to Python, remember this assignment would make the variable local and
    # not global
    # if it weren't for the global declaration at the start of main.
    window = glutCreateWindow(
        "Lesson 42: Multiple Viewports... 2003 NeHe Productions... Building Maze!")

    # Register the drawing function with glut, BUT in Python land, at least using PyOpenGL, we need to
    # set the function pointer and invoke a function to actually register the callback, otherwise it
    # would be very much like the C version of the code.
    glutDisplayFunc(DrawGLScene)

    # Uncomment this line to get full screen.
    # glutFullScreen()

    # When we are doing nothing, redraw the scene.
    glutIdleFunc(DrawGLScene)

    # Register the function called when our window is resized.
    glutReshapeFunc(ReSizeGLScene)

    # Register the function called when the keyboard is pressed.
    # The call setup glutSpecialFunc () is needed to receive
    # "keyboard function or directional keys."
    glutKeyboardFunc(keyPressed)

    # Print message to console, and kick off the main to get it rolling.
    print('Hit ESC key to quit.')

    glutSpecialFunc(keyPressed)

    # We've told Glut the type of window we want, and we've told glut about
    # various functions that we want invoked (idle, resizing, keyboard events).
    # Glut has done the hard work of building up thw windows DC context and
    # tying in a rendering context, so we are ready to start making immediate mode
    # GL calls.
    # Call to perform inital GL setup (the clear colors, enabling modes, and most releveant -
    # consturct the displays lists for the bitmap font.
    InitGL(640, 480)

    # Start Event Processing Engine
    glutMainLoop()


if __name__ == "__main__":
    main()
