import math
import numpy as np

a_vec = (1, 2, 0)
c_vec = (0, 0, 0)
theta = (0, 0, 0)
e_vec = (0, 0, 0)

x = a_vec[0] - c_vec[0]
y = a_vec[1] - c_vec[1]
z = a_vec[2] - c_vec[2]

cx = math.cos(theta[0])
cy = math.cos(theta[1])
cz = math.cos(theta[2])

sx = math.sin(theta[0])
sy = math.sin(theta[1])
sz = math.sin(theta[2])

dx = cy * (sz * y + cz * x) - sy * z
dy = sx * (cy * z + sy * (sz * y + cz * x)) + cx * (cz * y - sz * x)
dz = cx * (cy * z + sy * (sz * y + cz * x)) - sx * (cz * y - sz * x)

print("d: ", dx, dy, dz)

# bx = e_vec[2] / dz * dx + e_vec[0]
# by = e_vec[2] / dz * dy + e_vec[1]

# print(bx, by)

print("weak")

# y
# |  z
# | /
# |/
# +--------- x

# y
# |
# +-x
objects = []
view = []
#               x, y, z
# objects.append((4, 4, 2))
# objects.append((4, 2, 2))
# objects.append((2, 2, 2))
# objects.append((2, 4, 2))

# objects.append((4, 4, 4))
# objects.append((4, 2, 4))
# objects.append((2, 2, 4))
# objects.append((2, 4, 4))


def Quaternion(angle, v):
    r = math.radians(angle / 2)
    s = math.sin(r)
    c = math.cos(r)

    x = v[0] * s
    y = v[1] * s
    z = v[2] * s
    w = c

    return (x, y, z, w)

def Conj(q):
    return (-q[0], -q[1], -q[2], q[3])

def qmv(q, v):
    w = - (q[0] * v[0]) - (q[1] * v[1]) - (q[2] * v[2])
    x =   (q[3] * v[0]) + (q[1] * v[2]) - (q[2] * v[1])
    y =   (q[3] * v[1]) + (q[2] * v[0]) - (q[0] * v[2])
    z =   (q[3] * v[2]) + (q[0] * v[1]) - (q[1] * v[0])

    return (x, y, z, w)

def qmq(q1, q2):
    w = (q1[3] * q2[3]) - (q1[0] * q2[0]) - (q1[1] * q2[1]) - (q1[2] * q2[2])
    x = (q1[0] * q2[3]) + (q1[3] * q2[0]) + (q1[1] * q2[2]) - (q1[2] * q2[1])
    y = (q1[1] * q2[3]) + (q1[3] * q2[1]) + (q1[2] * q2[0]) - (q1[0] * q2[2])
    z = (q1[2] * q2[3]) + (q1[3] * q2[2]) + (q1[0] * q2[1]) - (q1[1] * q2[0])

    return (x, y, z, w)

def qlen(v):
    length = math.sqrt(v[0] * v[0] + v[1] * v[1] + v[2] * v[2] + v[3] * v[3])

    x = v[0] / length
    y = v[1] / length
    z = v[2] / length
    w = v[3] / length

    return (x, y, z)

objects.append((1, 3, 1))
objects.append((0, 5, 5))

objects.append((1, 1, -1))
objects.append((1, -1, -1))
objects.append((1, 1, 1))
objects.append((1, -1, 1))

objects.append((-1, 1, -1))
objects.append((-1, -1, -1))
objects.append((-1, 1, 1))
objects.append((-1, -1, 1))

camera = (0, 0, 0)
Bz = 15

for object in objects:
    d = math.sqrt( math.pow(object[0] - camera[0], 2) + math.pow(object[1] - camera[1], 2) + math.pow(object[2] - camera[2], 2) )

    Bx = object[0] * Bz / d
    By = object[1] * Bz / d

    print(round(Bx), round(By))
    view.append((round(Bx), round(By)))

print('-'*Bz)
for i in range(0, Bz):
    for j in range(0, Bz):
        if (j, i) in view:
            print('x', end='')
        else:
            print(' ', end='')
    print()
print('-'*Bz)

print('perspective')

height = 24
width = 80
fov = 90
zfar = 1000
znear = 0.1

angle = 90
axis = (0.5, 0.5, -1)

a = height / width
f = 1 / math.tan(math.radians(fov / 2))
scale = zfar / (zfar - znear)

projection_matrix = [
    [a*f, 0, 0, 0],
    [0, f, 0, 0],
    [0, 0, scale, 1], # the -scale and 1 are flipped in this one, but it works
    [0, 0, -scale*znear, 0]
]
np_pmat = np.array(projection_matrix)

viewPort = []

for object in objects:
    q = Quaternion(angle, axis)
    qc = Conj(q)
    s1 = qmv(q, object + (1, ))
    pt = qmq(s1, qc)

    # p = np.array(object + (1, ))
    p = np.array(pt)
    print(object, p)

    p[2] += 3
    # p = (pt[0], pt[1], pt[2]+3, pt[3])

    # multiply by perspective matrix
    pp = np.matmul(p, np_pmat)
    if pp[3] == 0:
        continue
    
    # convert 4d -> 3d
    pp[0] /= pp[3]
    pp[1] /= pp[3]
    pp[2] /= pp[3]
    pointNew = pp[:3]

    # scale into view
    pointNew[0] += 1
    pointNew[1] += 1

    pointNew[0] *= 0.5 * width
    pointNew[1] *= 0.5 * height

    print(pointNew)
    # convert the x and y to a rounded tuple
    viewPort.append(tuple(np.round(pointNew, 0)[:2]))

print('-'*width)
for i in range(height):
    for j in range(width):
        if (j, i) in viewPort:
            print('x', end='')
        else:
            print(' ', end='')
    print()
print('-'*width)

print('######### quaternums ##########')



# P' = QPQ*
a = 180
v = (0, 0, 0)
q = Quaternion(a, v)
p = (0, 0, 2, 0)
qc = Conj(q)
s1 = qmv(q, p)
pp = qmq(s1, qc)
print('P:', pp)
print('len:', qlen(pp))

print('something')

def norm(v):
    l = np.linalg.norm(v)
    return np.array(( v[0] / l, v[1] / l, v[2] / l ))

def point_at(pos, target, up):
    # calculate new forward vector
    forward = target - pos

    # calculate new up
    a = forward * np.dot(up, forward)
    tmpUp = np.subtract(up, a)
    newUp = norm(tmpUp)
    
    newRight = np.cross(newUp, forward)

    print('stuff', newRight, newUp, forward)

    m = np.zeros((4, 4))

    m[0][0] = newRight[0]
    m[1][0] = newUp[0]
    m[2][0] = forward[0]
    m[3][0] = pos[0]

    m[0][1] = newRight[1]
    m[1][1] = newUp[1]
    m[2][1] = forward[1]
    m[3][1] = pos[1]

    m[0][2] = newRight[2]
    m[1][2] = newUp[2]
    m[2][2] = forward[2]
    m[3][2] = pos[2]

    m[0][3] = 0
    m[1][3] = 0
    m[2][3] = 0
    m[3][3] = 1

    print('point at\n', m)

    return m

def quick_inverse(mat):
    m = np.zeros((4, 4))

    m[0][0] = mat[0][0]
    m[1][0] = mat[0][1]
    m[2][0] = mat[0][2]

    m[0][1] = mat[1][0]
    m[1][1] = mat[1][1]
    m[2][0] = mat[1][2]
    
    m[0][2] = mat[2][0]
    m[1][2] = mat[2][1]
    m[2][2] = mat[2][2]

    m[3][0] = - np.dot(mat[3][:3], mat[0][:3])
    m[3][1] = - np.dot(mat[3][:3], mat[1][:3])
    m[3][2] = - np.dot(mat[3][:3], mat[2][:3])
    m[3][3] = 1

    print('inverse\n', m)

    return m

def rotY(yaw):
    m = np.zeros((3, 3))
    y = math.radians(yaw)

    m[0][0] = math.cos(y)
    m[0][2] = -math.sin(y)
    m[1][1] = 1
    m[0][2] = math.sin(y)
    m[2][2] = math.cos(y)
    # m[3][3] = 1

    return m

# control these
vcam = np.array((0, 0, 0))
yaw = 0

target = np.array((0, 0, 1))
camRot = rotY(yaw)
print('camRot\n', camRot)
print('target', target)
lookDir = np.dot(camRot, target) #np.array((0, 0.5, 1))
print(f'{target[0]} * {camRot[0][0]} + {target[1]} * {camRot[1][0]} + ({target[2]}) * {camRot[2][0]} =', target[0] * camRot[0][0] + target[1] * camRot[1][0] + (target[2]) * camRot[2][0])
print('lookdir', lookDir)
up = np.array((0, 1, 0))
target = vcam + lookDir

matCamera = point_at(vcam, target, up)
matView = quick_inverse(matCamera)

viewPort = []
points = []
points.append((1, 1, -1))
points.append((1, -1, -1))
points.append((1, 1, 1))
points.append((1, -1, 1))

points.append((-1, 1, -1))
points.append((-1, -1, -1))
points.append((-1, 1, 1))
points.append((-1, -1, 1))


for point in points:
    p = np.array(point + (1, ))

    p[2] += 3

    # world -> view
    p = np.matmul(matView, p) # remember this is JUST a point, not TRIANGLE
    print('view', p)

    # multiply by perspective matrix
    pp = np.matmul(p, np_pmat)
    if pp[3] == 0:
        continue
    print('###########', pp)
    
    # convert 4d -> 3d
    pp[0] /= pp[3]
    pp[1] /= pp[3]
    pp[2] /= pp[3]
    pointNew = pp[:3]

    # 3d -> 2d
    # pointNew[0] /= pointNew[2]
    # pointNew[1] /= pointNew[2]

    # scale into view
    pointNew[0] += 1
    pointNew[1] += 1

    pointNew[0] *= 0.5 * width
    pointNew[1] *= 0.5 * height

    print(f'{point} -> {pointNew}')
    # convert the x and y to a rounded tuple
    viewPort.append(tuple(np.round(pointNew, 0)[:2]))

print('-'*width)
for i in range(height):
    for j in range(width):
        if (j, i) in viewPort:
            print('x', end='')
        else:
            print(' ', end='')
    print()
print('-'*width)
