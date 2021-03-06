from numpy import add, array, sqrt, arccos, zeros, pi as PI


def vv3(x, y):
    return add.reduce(x*y)


def vxv3(a, b):
    return array([
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0]
    ])


def dist3(x, y):
    return sqrt(add.reduce((y-x)**2))


def angle(x, y, z):
    d1 = dist3(x, y)
    d2 = dist3(z, y)

    if (d1 <= 0 or d2 <= 0):
        return 0.0

    acc = add.reduce((y-x) * (y-z)) / (d1*d2)

    if (acc > 1):
        acc = 1.

    elif (acc < -1):
        acc = -1.

    return arccos(acc)


def dihedral(crds):
    q = crds[1] - crds[0]
    r = crds[1] - crds[2]
    s = crds[2] - crds[3]

    t = vxv3(q, r)
    u = vxv3(s, r)
    v = vxv3(u, t)

    w = vv3(v, r)
    z = zeros(3)
    acc = angle(t, z, u)
    ang = acc / PI * 180.

    if (w < 0):
        ang = 360 - ang

    return ang
